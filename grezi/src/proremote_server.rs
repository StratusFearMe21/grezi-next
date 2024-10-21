use std::{
    borrow::Cow,
    collections::HashMap,
    io::Cursor,
    net::TcpListener,
    ops::{Deref, DerefMut},
    path::Path,
    sync::{atomic::Ordering, Arc},
    time::UNIX_EPOCH,
};

use base64::Engine;
use cairo::ImageSurface;
use eframe::egui::{PaintCallback, Pos2, Rect, TextureId, Vec2};
use egui_glyphon::{BufferWithTextArea, GlyphonRendererCallback};
use image::{DynamicImage, ImageFormat, RgbaImage};
use mdns_sd::{ServiceDaemon, ServiceInfo};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use serde::{Deserialize, Serialize};
use tungstenite::{
    accept_hdr,
    handshake::server::{Request, Response},
    Message,
};

use crate::{
    parser::{objects::cosmic_jotdown, AstObject},
    resolver::Resolved,
    MyEguiApp,
};

#[derive(Deserialize, Debug)]
#[serde(tag = "action")]
#[serde(rename_all = "camelCase")]
enum ProMessage<'a> {
    Authenticate {
        password: Cow<'a, str>,
        protocol: i32,
    },
    LibraryRequest,
    PlaylistRequestAll,
    #[serde(rename_all = "camelCase")]
    PresentationCurrent {
        // presentation_slide_quality: i32,
        presentation_destination: i32,
        #[serde(default)]
        presentation_path: Cow<'a, str>,
    },
    #[serde(rename_all = "camelCase")]
    PresentationRequest {
        presentation_path: Cow<'a, str>,
        #[serde(default)]
        presentation_destination: i32,
    },
    #[serde(rename_all = "camelCase")]
    PresentationTriggerNext {
        presentation_destination: i32,
    },
    #[serde(rename_all = "camelCase")]
    PresentationTriggerPrevious {
        presentation_destination: i32,
    },
    #[serde(rename_all = "camelCase")]
    PresentationSlideIndex {
        presentation_destination: i32,
    },
    #[serde(rename_all = "camelCase")]
    PresentationTriggerIndex {
        slide_index: Cow<'a, str>,
        // presentation_path: Cow<'a, str>,
        #[serde(default)]
        presentation_destination: i32,
    },
    Ping,
    ClearText,
}

#[derive(Serialize, Debug)]
#[serde(tag = "action")]
#[serde(rename_all = "camelCase")]
enum ProResponse<'a> {
    #[serde(rename_all = "camelCase")]
    Authenticate {
        controller: i32,
        authenticated: i32,
        error: Cow<'a, str>,
        major_version: i32,
        minor_version: i32,
        patch_version: i32,
        protocol: i32,
    },
    LibraryRequest {
        library: &'a [Cow<'a, str>],
    },
    #[serde(rename_all = "camelCase")]
    PlaylistRequestAll {
        playlist_all: &'a [Playlist<'a>],
    },
    #[serde(rename_all = "camelCase")]
    PresentationCurrent {
        presentation_path: Cow<'a, str>,
        presentation: Presentation<'a>,
    },
    #[serde(rename_all = "camelCase")]
    PresentationTriggerIndex {
        slide_index: usize,
        presentation_path: Cow<'a, str>,
        presentation_destination: i32,
    },
    #[serde(rename_all = "camelCase")]
    PresentationSlideIndex {
        slide_index: usize,
        presentation_path: Cow<'a, str>,
        presentation_destination: i32,
    },
    ClearText,
}

impl<'a> TryFrom<ProResponse<'a>> for Message {
    type Error = serde_json::Error;
    fn try_from(value: ProResponse<'a>) -> Result<Self, Self::Error> {
        let value = serde_json::to_string(&value)?;
        Ok(Message::Text(value))
    }
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Presentation<'a> {
    presentation_name: Cow<'a, str>,
    presentation_path: Cow<'a, str>,
    presentation_current_location: Cow<'a, str>,
    presentation_has_timeline: bool,
    presentation_slide_groups: Cow<'a, [PresentationSlideGroup<'a>]>,
    presentation_destination: i32,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct PresentationSlideGroup<'a> {
    group_name: Cow<'a, str>,
    group_color: Cow<'a, str>,
    group_slides: Cow<'a, [ProSlide<'a>]>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct ProSlide<'a> {
    slide_enabled: bool,
    slide_notes: Arc<str>,
    slide_attachment_mask: i32,
    slide_text: Cow<'a, str>,
    slide_image: Cow<'a, str>,
    slide_index: usize,
    slide_transition_type: i32,
    slide_label: Cow<'a, str>,
    slide_color: Cow<'a, str>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct Playlist<'a> {
    playlist_location: Cow<'a, str>,
    playlist_type: Cow<'a, str>,
    playlist_name: Cow<'a, str>,
    playlist: Cow<'a, [serde_json::Value]>,
}

pub fn start_server(
    app: MyEguiApp,
    ctx: eframe::egui::Context,
    passwd: Arc<str>,
) -> tungstenite::Result<()> {
    let server = TcpListener::bind("0.0.0.0:1599").unwrap();

    // Create a daemon
    let mdns = ServiceDaemon::new().expect("Failed to create daemon");

    // Create a service info.
    let service_type = "_pro7proremote._tcp.local.";
    let instance_name = "grezi";
    let host_name = format!("{}.local.", hostname::get().unwrap().to_string_lossy());
    let ip = local_ip_address::local_ip().unwrap().to_string();
    let port = 1599;

    let my_service = ServiceInfo::new::<_, HashMap<String, String>>(
        service_type,
        instance_name,
        &host_name,
        ip,
        port,
        HashMap::default(),
    )
    .unwrap();

    // Register with the daemon, which publishes the service.
    mdns.register(my_service)
        .expect("Failed to register our service");

    let (stream, _) = server.accept()?;
    mdns.shutdown().unwrap();
    let callback = |_: &Request, response: Response| Ok(response);

    let Ok(mut websocket) = accept_hdr(stream, callback) else {
        return Ok(());
    };

    #[cfg(not(windows))]
    let file_name = format!(
        "{}",
        std::fs::canonicalize(Path::new(app.file_name.deref()))
            .unwrap()
            .display()
    );
    #[cfg(windows)]
    let file_name = format!(
        "{}",
        dunce::canonicalize(Path::new(app.file_name.deref()))
            .unwrap()
            .display()
    );
    let mut current_presentation: Option<Message> = None;

    loop {
        let msg = websocket.read()?;
        let Ok(msg) = msg.to_text() else {
            continue;
        };
        let Ok(msg): Result<ProMessage<'_>, _> = (if msg.is_empty() {
            Ok(ProMessage::Ping)
        } else {
            serde_json::from_str(msg)
        }) else {
            dbg!(msg);
            continue;
        };
        match msg {
            ProMessage::Authenticate { password, protocol } => {
                if password.as_ref() == passwd.deref() && protocol == 701 {
                    websocket.write(
                        ProResponse::Authenticate {
                            error: Cow::Borrowed(""),
                            authenticated: 1,
                            controller: 1,
                            major_version: 7,
                            minor_version: 17,
                            patch_version: 1,
                            protocol,
                        }
                        .try_into()
                        .unwrap(),
                    )?;
                } else {
                    dbg!("Authentication error");

                    websocket.write(
                        ProResponse::Authenticate {
                            error: Cow::Borrowed("Uh oh, stinky, poooooop"),
                            authenticated: 0,
                            controller: 0,
                            major_version: 7,
                            minor_version: 17,
                            patch_version: 1,
                            protocol,
                        }
                        .try_into()
                        .unwrap(),
                    )?;
                }
            }
            ProMessage::LibraryRequest => {
                websocket.write(
                    ProResponse::LibraryRequest {
                        library: &[
                            Cow::Borrowed(&file_name),
                            Cow::Borrowed("/Announcements.grz"),
                        ],
                    }
                    .try_into()
                    .unwrap(),
                )?;
            }
            ProMessage::PlaylistRequestAll => {
                websocket.write(
                    ProResponse::PlaylistRequestAll {
                        playlist_all: &[Playlist {
                            playlist_location: Cow::Borrowed("0"),
                            playlist_type: Cow::Borrowed("playlistTypePlaylist"),
                            playlist_name: Cow::Borrowed("Default"),
                            playlist: Cow::Borrowed(&[]),
                        }],
                    }
                    .try_into()
                    .unwrap(),
                )?;
            }
            ProMessage::PresentationCurrent {
                presentation_destination,
                presentation_path,
                ..
            }
            | ProMessage::PresentationRequest {
                presentation_destination,
                presentation_path,
                ..
            } => {
                let size = Vec2::new(854.0, 480.0);

                let input = eframe::egui::RawInput {
                    screen_rect: Some(Rect::from_min_size(Pos2::ZERO, size)),
                    time: Some(25_849_200.0),
                    ..Default::default()
                };
                if presentation_destination == 0
                    && presentation_path.as_ref() != "/Announcements.grz"
                {
                    let response = current_presentation.get_or_insert_with(|| {
                        let mut group_slides = Vec::new();
                        let slide_show = app.slide_show.load();
                        let slide_show = slide_show.read();
                        let used_fonts = slide_show.used_fonts(app.font_system.lock().deref_mut());

                        slide_show
                            .slide_show
                            .par_iter()
                            .enumerate()
                            .map_init(
                                {
                                    let app = app.clone();
                                    move || {
                                        let ft = cairo::freetype::Library::init().unwrap();

                                        let font_defs_ft = crate::cairo::fonts_to_ft(
                                            Arc::clone(&app.font_system),
                                            &used_fonts,
                                            &ft,
                                        );
                                        let textures: HashMap<TextureId, (ImageSurface, bool)> =
                                            HashMap::default();

                                        let export_ctx = eframe::egui::Context::default();
                                        egui_extras::install_image_loaders(&export_ctx);
                                        if !export_ctx
                                            .is_loader_installed(egui_anim::AnimLoader::ID)
                                        {
                                            export_ctx.add_image_loader(Arc::new(
                                                egui_anim::AnimLoader::default(),
                                            ));
                                        }
                                        (font_defs_ft, textures, ft, export_ctx)
                                    }
                                },
                                |(font_defs_ft, textures, ft, export_ctx), (index, (_, obj))| {
                                    let mut speaker_notes: Option<Arc<str>> = None;
                                    let slide_image = {
                                        let stride = cairo::Format::ARgb32
                                            .stride_for_width(size.x as u32)
                                            .unwrap();
                                        let mut image_data =
                                            vec![0; stride as usize * size.y as usize];
                                        let surface = unsafe {
                                            ImageSurface::create_for_data_unsafe(
                                                image_data.as_mut_ptr(),
                                                cairo::Format::ARgb32,
                                                size.x as i32,
                                                size.y as i32,
                                                stride,
                                            )
                                            .unwrap()
                                        };
                                        let cairo_ctx = cairo::Context::new(&surface).unwrap();

                                        let output = export_ctx.run(input.clone(), |ctx| {
                                            let color = match obj {
                                                AstObject::Slide { bg, .. } => {
                                                    if let Some(color) = bg.1 {
                                                        color.1
                                                    } else {
                                                        bg.0
                                                    }
                                                }
                                                AstObject::Action { slide_in_ast, .. } => {
                                                    let Some(AstObject::Slide { bg, .. }) =
                                                        slide_show.slide_show.get(slide_in_ast)
                                                    else {
                                                        unreachable!()
                                                    };
                                                    if let Some(color) = bg.1 {
                                                        color.1
                                                    } else {
                                                        bg.0
                                                    }
                                                }
                                            };
                                            eframe::egui::CentralPanel::default()
                                    .frame(eframe::egui::Frame::default().fill(color.into()))
                                    .show(ctx, |ui| {
                                        let resolved = match obj {
                                            AstObject::Slide {
                                                objects: slide,
                                                actions,
                                                ..
                                            } => {
                                                let mut font_system = app.font_system.lock();
                                                let resolved = Arc::new(Resolved::resolve(
                                                    slide,
                                                    (actions, None),
                                                    ui,
                                                    Rect::from_min_size(Pos2::ZERO, size),
                                                    &slide_show,
                                                    font_system.deref_mut(),
                                                    Arc::clone(&app.resolved_images),
                                                ));
                                                resolved
                                            }
                                            AstObject::Action {
                                                actions,
                                                slide_in_ast,
                                                ..
                                            } => {
                                                let slide = slide_show
                                                    .slide_show
                                                    .get(slide_in_ast)
                                                    .unwrap();
                                                match slide {
                                                    AstObject::Slide {
                                                        objects: slide,
                                                        actions: slide_actions,
                                                        ..
                                                    } => {
                                                        let mut font_system =
                                                            app.font_system.lock();
                                                        let resolved = Arc::new(Resolved::resolve(
                                                            slide,
                                                            (slide_actions, Some(actions)),
                                                            ui,
                                                            Rect::from_min_size(Pos2::ZERO, size),
                                                            &slide_show,
                                                            font_system.deref_mut(),
                                                            Arc::clone(&app.resolved_images),
                                                        ));
                                                        resolved
                                                    }
                                                    _ => Arc::new(Resolved::slideshow_end(
                                                        Rect::from_min_size(Pos2::ZERO, size),
                                                    )),
                                                }
                                            }
                                };

                                        let mut buffers = Vec::new();
                                        resolved.draw_slide(
                                            ui,
                                            f32::MAX,
                                            &mut buffers,
                                            app.font_system.lock().deref_mut(),
                                            true,
                                        );
                                        resolved.draw_actions(ui, f32::MAX, true);

                                        ui.painter().add(PaintCallback {
                                            rect: Rect::from_min_size(Pos2::ZERO, size),
                                            callback: Arc::new(GlyphonRendererCallback { buffers }),
                                        });

                                        speaker_notes = resolved.speaker_notes.clone();
                                    });
                                        });

                                        crate::cairo::cairo_draw(
                                            output,
                                            textures,
                                            &cairo_ctx,
                                            &ft,
                                            Arc::clone(&app.font_system),
                                            font_defs_ft,
                                        );

                                        cairo_ctx.target().finish();

                                        image_data.chunks_mut(4).for_each(|chunk| {
                                            chunk.swap(0, 2);
                                        });
                                        let mut jpeg = Cursor::new(Vec::new());

                                        DynamicImage::ImageRgba8(
                                            RgbaImage::from_raw(
                                                size.x as u32,
                                                size.y as u32,
                                                image_data,
                                            )
                                            .unwrap(),
                                        )
                                        .to_rgb8()
                                        .write_to(&mut jpeg, ImageFormat::Jpeg)
                                        .unwrap();

                                        base64::prelude::BASE64_STANDARD.encode(jpeg.into_inner())
                                    };

                                    ProSlide {
                                        slide_enabled: true,
                                        slide_notes: speaker_notes
                                            .clone()
                                            .unwrap_or_else(|| String::new().into()),
                                        slide_attachment_mask: 0,
                                        slide_text: Cow::Borrowed(""),
                                        slide_image: Cow::Owned(slide_image),
                                        slide_index: index,
                                        slide_transition_type: -1,
                                        slide_label: Cow::Borrowed(""),
                                        slide_color: Cow::Borrowed(""),
                                    }
                                },
                            )
                            .collect_into_vec(&mut group_slides);

                        let presentation_slide_groups = [PresentationSlideGroup {
                            group_name: Cow::Borrowed("Group"),
                            group_color: Cow::Borrowed(""),
                            group_slides: Cow::Owned(group_slides),
                        }];

                        let o = app.file_name.rsplit_once('.').unwrap();

                        ProResponse::PresentationCurrent {
                            presentation_path: Cow::Borrowed(&file_name),
                            presentation: Presentation {
                                presentation_name: Cow::Borrowed(o.0),
                                presentation_path: Cow::Borrowed(&file_name),
                                presentation_current_location: Cow::Borrowed(&file_name),
                                presentation_has_timeline: false,
                                presentation_slide_groups: Cow::Borrowed(
                                    &presentation_slide_groups,
                                ),
                                presentation_destination,
                            },
                        }
                        .try_into()
                        .unwrap()
                    });

                    websocket.write(response.clone())?;
                    app.resolved_images.lock().clear();
                } else {
                    let ft = cairo::freetype::Library::init().unwrap();

                    let export_ctx = eframe::egui::Context::default();
                    egui_extras::install_image_loaders(&export_ctx);
                    if !export_ctx.is_loader_installed(egui_anim::AnimLoader::ID) {
                        export_ctx.add_image_loader(Arc::new(egui_anim::AnimLoader::default()));
                    }

                    let slide_image = {
                        let stride = cairo::Format::ARgb32
                            .stride_for_width(size.x as u32)
                            .unwrap();
                        let mut image_data = vec![0; stride as usize * size.y as usize];
                        let surface = unsafe {
                            ImageSurface::create_for_data_unsafe(
                                image_data.as_mut_ptr(),
                                cairo::Format::ARgb32,
                                size.x as i32,
                                size.y as i32,
                                stride,
                            )
                            .unwrap()
                        };
                        let cairo_ctx = cairo::Context::new(&surface).unwrap();

                        let output = export_ctx.run(input.clone(), |ctx| {
                            eframe::egui::CentralPanel::default().show(ctx, |ui| {
                                let time = std::time::SystemTime::now()
                                    .duration_since(UNIX_EPOCH)
                                    .unwrap();
                                let time = format!("*{:?}*", time);
                                let buffers = cosmic_jotdown::jotdown_into_buffers(
                                    jotdown::Parser::new(&time),
                                    &egui_glyphon::glyphon::Attrs::new(),
                                )
                                .collect::<Vec<_>>();
                                let font_size = 24.0;
                                let (size, resolved_job) =
                                    crate::parser::objects::cosmic_jotdown::resolve_paragraphs(
                                        &buffers,
                                        size,
                                        app.font_system.lock().deref_mut(),
                                        egui_glyphon::glyphon::Metrics::new(
                                            font_size,
                                            font_size * 1.2,
                                        ),
                                        None,
                                        crate::parser::objects::VerticalSpacing::Normal,
                                    );

                                let buffers = resolved_job
                                    .into_iter()
                                    .map(|buffer| {
                                        let text_rect = buffer
                                            .relative_bounds
                                            .translate(Vec2::splat(8.0))
                                            .expand(1.0);

                                        // ui.painter().debug_rect(text_rect, Color32::GREEN, "");

                                        BufferWithTextArea::new(
                                            Arc::clone(&buffer.buffer),
                                            text_rect,
                                            1.0,
                                            egui_glyphon::glyphon::Color::rgb(255, 255, 255),
                                            ui.ctx(),
                                            buffer.url_map.clone(),
                                        )
                                    })
                                    .collect::<Vec<_>>();

                                ui.painter().add(PaintCallback {
                                    rect: Rect::from_min_size(Pos2::ZERO, size),
                                    callback: Arc::new(GlyphonRendererCallback { buffers }),
                                });
                            });
                        });

                        crate::cairo::cairo_draw(
                            output,
                            &mut HashMap::default(),
                            &cairo_ctx,
                            &ft,
                            Arc::clone(&app.font_system),
                            &mut HashMap::default(),
                        );

                        cairo_ctx.target().finish();

                        image_data.chunks_mut(4).for_each(|chunk| {
                            chunk.swap(0, 2);
                        });
                        let mut jpeg = Cursor::new(Vec::new());

                        DynamicImage::ImageRgba8(
                            RgbaImage::from_raw(size.x as u32, size.y as u32, image_data).unwrap(),
                        )
                        .to_rgb8()
                        .write_to(&mut jpeg, ImageFormat::Jpeg)
                        .unwrap();

                        base64::prelude::BASE64_STANDARD.encode(jpeg.into_inner())
                    };

                    let group_slides = [ProSlide {
                        slide_enabled: true,
                        slide_notes: String::new().into(),
                        slide_attachment_mask: 0,
                        slide_text: Cow::Borrowed(""),
                        slide_image: Cow::Owned(slide_image),
                        slide_index: 0,
                        slide_transition_type: -1,
                        slide_label: Cow::Borrowed(""),
                        slide_color: Cow::Borrowed(""),
                    }];

                    let presentation_slide_groups = [PresentationSlideGroup {
                        group_name: Cow::Borrowed("Group"),
                        group_color: Cow::Borrowed(""),
                        group_slides: Cow::Borrowed(&group_slides),
                    }];

                    websocket.write(
                        ProResponse::PresentationCurrent {
                            presentation_path: Cow::Borrowed("/Announcements.grz"),
                            presentation: Presentation {
                                presentation_name: Cow::Borrowed("Announcements"),
                                presentation_path: Cow::Borrowed("/Announcements.grz"),
                                presentation_current_location: Cow::Borrowed("/Announcements.grz"),
                                presentation_has_timeline: false,
                                presentation_slide_groups: Cow::Borrowed(
                                    &presentation_slide_groups,
                                ),
                                presentation_destination,
                            },
                        }
                        .try_into()
                        .unwrap(),
                    )?;
                }
            }
            ProMessage::PresentationSlideIndex {
                presentation_destination,
            } => {
                if presentation_destination == 0 {
                    websocket
                        .write(
                            ProResponse::PresentationSlideIndex {
                                presentation_path: Cow::Borrowed(&file_name),
                                slide_index: app.index.load(Ordering::Relaxed),
                                presentation_destination,
                            }
                            .try_into()
                            .unwrap(),
                        )
                        .unwrap();
                } else {
                    websocket
                        .write(
                            ProResponse::PresentationSlideIndex {
                                presentation_path: Cow::Borrowed("/Announcements.grz"),
                                slide_index: 0,
                                presentation_destination,
                            }
                            .try_into()
                            .unwrap(),
                        )
                        .unwrap();
                }
            }
            ProMessage::PresentationTriggerNext {
                presentation_destination,
            } => {
                if presentation_destination == 0 {
                    let _ = app
                        .index
                        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |index| {
                            app.resolved.store(None);
                            app.speaker_view.clear_resolved();
                            app.vb_dbg.store(0, Ordering::Relaxed);
                            app.obj_dbg.store(0, Ordering::Relaxed);
                            app.restart_timer.store(1, Ordering::Relaxed);
                            app.next.store(true, Ordering::Relaxed);
                            ctx.request_repaint();
                            Some(index + 1)
                        });
                    websocket.write(
                        ProResponse::PresentationTriggerIndex {
                            presentation_path: Cow::Borrowed(&file_name),
                            slide_index: app.index.load(Ordering::Relaxed),
                            presentation_destination,
                        }
                        .try_into()
                        .unwrap(),
                    )?;
                } else {
                    websocket
                        .write(
                            ProResponse::PresentationTriggerIndex {
                                presentation_path: Cow::Borrowed("/Announcements.grz"),
                                slide_index: 0,
                                presentation_destination,
                            }
                            .try_into()
                            .unwrap(),
                        )
                        .unwrap();
                }
            }
            ProMessage::PresentationTriggerPrevious {
                presentation_destination,
            } => {
                if presentation_destination == 0 {
                    let _ = app.index.fetch_update(
                        Ordering::Relaxed,
                        Ordering::Relaxed,
                        |mut index| {
                            if index != 0 {
                                app.resolved.store(None);
                                app.speaker_view.clear_resolved();
                                app.vb_dbg.store(0, Ordering::Relaxed);
                                app.obj_dbg.store(0, Ordering::Relaxed);
                                app.next.store(false, Ordering::Relaxed);
                                index -= 1;
                                let slide_show = app.slide_show.load();
                                let slide_show = slide_show.read();
                                while matches!(
                                    slide_show.slide_show.get_index(index).map(|o| o.1),
                                    Some(
                                        AstObject::Action { next: true, .. }
                                            | AstObject::Slide { next: true, .. }
                                    )
                                ) {
                                    index -= 1;
                                }
                                ctx.request_repaint();
                                return Some(index);
                            }
                            None
                        },
                    );
                    websocket.write(
                        ProResponse::PresentationTriggerIndex {
                            presentation_path: Cow::Borrowed(&file_name),
                            slide_index: app.index.load(Ordering::Relaxed),
                            presentation_destination,
                        }
                        .try_into()
                        .unwrap(),
                    )?;
                } else {
                    websocket
                        .write(
                            ProResponse::PresentationTriggerIndex {
                                presentation_path: Cow::Borrowed("/Announcements.grz"),
                                slide_index: 0,
                                presentation_destination,
                            }
                            .try_into()
                            .unwrap(),
                        )
                        .unwrap();
                }
            }
            ProMessage::PresentationTriggerIndex {
                slide_index,
                presentation_destination,
                ..
            } => {
                if presentation_destination == 0 {
                    app.index
                        .swap(slide_index.parse().unwrap(), Ordering::Relaxed);
                    app.next.store(false, Ordering::Relaxed);
                    app.vb_dbg.store(0, Ordering::Relaxed);
                    app.obj_dbg.store(0, Ordering::Relaxed);
                    app.resolved.store(None);
                    ctx.request_repaint();

                    websocket.send(
                        ProResponse::PresentationTriggerIndex {
                            presentation_path: Cow::Borrowed(&file_name),
                            slide_index: app.index.load(Ordering::Relaxed),
                            presentation_destination,
                        }
                        .try_into()
                        .unwrap(),
                    )?;
                } else {
                    websocket
                        .write(
                            ProResponse::PresentationTriggerIndex {
                                presentation_path: Cow::Borrowed("/Announcements.grz"),
                                slide_index: 0,
                                presentation_destination,
                            }
                            .try_into()
                            .unwrap(),
                        )
                        .unwrap();
                }
            }
            ProMessage::Ping => {
                websocket.write(Message::Text(String::new()))?;
            }
            ProMessage::ClearText => {
                websocket.write(ProResponse::ClearText.try_into().unwrap())?;
            }
        }

        websocket.flush()?;
    }
}
