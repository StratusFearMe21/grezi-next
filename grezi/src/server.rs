use std::{
    borrow::Cow,
    collections::HashMap,
    io::Cursor,
    net::TcpListener,
    ops::{Deref, DerefMut},
    sync::{atomic::Ordering, Arc},
};

use base64::Engine;
use cairo::ImageSurface;
use eframe::egui::{PaintCallback, Pos2, Rect, TextureId, Vec2};
use egui_glyphon::GlyphonRendererCallback;
use image::{DynamicImage, ImageFormat, RgbaImage};
use mdns_sd::{ServiceDaemon, ServiceInfo};
use serde::{Deserialize, Serialize};
use tungstenite::{
    accept_hdr,
    handshake::server::{Request, Response},
    Message,
};

use crate::{parser::AstObject, resolver::Resolved, MyEguiApp};

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
    },
    #[serde(rename_all = "camelCase")]
    PresentationRequest {
        // presentation_path: Cow<'a, str>,
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
}

#[derive(Serialize, Debug)]
#[serde(tag = "action")]
#[serde(rename_all = "camelCase")]
enum ProResponse<'a> {
    #[serde(rename_all = "camelCase")]
    Authenticate {
        error: Cow<'a, str>,
        authenticated: i32,
        controller: i32,
        major_version: i32,
        minor_version: i32,
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
    group_slides: Vec<ProSlide<'a>>,
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
    let server = TcpListener::bind("0.0.0.0:8080").unwrap();

    // Create a daemon
    let mdns = ServiceDaemon::new().expect("Failed to create daemon");

    // Create a service info.
    let service_type = "_pro7proremote._tcp.local.";
    let instance_name = "grezi";
    let host_name = format!("{}.local.", hostname::get().unwrap().to_string_lossy());
    let ip = local_ip_address::local_ip().unwrap().to_string();
    let port = 8080;

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
        std::fs::canonicalize(app.file_name.deref())
            .unwrap()
            .display()
    );
    #[cfg(windows)]
    let file_name = format!("{}", dunce::canonicalize(&app.file_name).unwrap().display());
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
                            minor_version: 6,
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
                            minor_version: 6,
                        }
                        .try_into()
                        .unwrap(),
                    )?;
                }
            }
            ProMessage::LibraryRequest => {
                websocket.write(
                    ProResponse::LibraryRequest {
                        library: &[Cow::Borrowed(&file_name)],
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
                ..
            }
            | ProMessage::PresentationRequest {
                presentation_destination,
                ..
            } => {
                let response = current_presentation.get_or_insert_with(|| {
                    let mut group_slides = Vec::new();
                    let slide_show = app.slide_show.load();
                    let slide_show = slide_show.read();
                    let used_fonts = slide_show.used_fonts(app.font_system.lock().deref_mut());
                    let size = Vec2::new(854.0, 480.0);
                    let export_ctx = eframe::egui::Context::default();
                    egui_extras::install_image_loaders(&export_ctx);
                    if !export_ctx.is_loader_installed(egui_anim::AnimLoader::ID) {
                        export_ctx.add_image_loader(Arc::new(egui_anim::AnimLoader::default()));
                    }

                    let ft = cairo::freetype::Library::init().unwrap();

                    let mut font_defs_ft =
                        crate::cairo::fonts_to_ft(Arc::clone(&app.font_system), &used_fonts, &ft);
                    let mut textures: HashMap<TextureId, (ImageSurface, bool)> = HashMap::default();
                    let input = eframe::egui::RawInput {
                        screen_rect: Some(Rect::from_min_size(Pos2::ZERO, size)),
                        time: Some(25_849_200.0),
                        ..Default::default()
                    };
                    let mut speaker_notes: Option<Arc<str>> = None;

                    for (index, (_, obj)) in slide_show.slide_show.iter().enumerate() {
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
                                &mut textures,
                                &cairo_ctx,
                                &ft,
                                Arc::clone(&app.font_system),
                                &mut font_defs_ft,
                            );

                            cairo_ctx.target().finish();

                            image_data.chunks_mut(4).for_each(|chunk| {
                                chunk.swap(0, 2);
                            });
                            let mut jpeg = Cursor::new(Vec::new());

                            DynamicImage::ImageRgba8(
                                RgbaImage::from_raw(size.x as u32, size.y as u32, image_data)
                                    .unwrap(),
                            )
                            .to_rgb8()
                            .write_to(&mut jpeg, ImageFormat::Jpeg)
                            .unwrap();

                            base64::prelude::BASE64_STANDARD.encode(jpeg.into_inner())
                        };

                        group_slides.push(ProSlide {
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
                        });
                    }

                    let presentation_slide_groups = [PresentationSlideGroup {
                        group_name: Cow::Borrowed("Group"),
                        group_color: Cow::Borrowed(""),
                        group_slides,
                    }];

                    let o = app.file_name.rsplit_once('.').unwrap();

                    ProResponse::PresentationCurrent {
                        presentation_path: Cow::Borrowed(&file_name),
                        presentation: Presentation {
                            presentation_name: Cow::Borrowed(o.0),
                            presentation_path: Cow::Borrowed(&file_name),
                            presentation_current_location: Cow::Borrowed(&file_name),
                            presentation_has_timeline: false,
                            presentation_slide_groups: Cow::Borrowed(&presentation_slide_groups),
                            presentation_destination,
                        },
                    }
                    .try_into()
                    .unwrap()
                });

                websocket.write(response.clone())?;
                app.resolved_images.lock().clear();
            }
            ProMessage::PresentationSlideIndex {
                presentation_destination,
            } => {
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
            }
            ProMessage::PresentationTriggerNext {
                presentation_destination,
            } => {
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
                websocket
                    .write(
                        ProResponse::PresentationTriggerIndex {
                            presentation_path: Cow::Borrowed(&file_name),
                            slide_index: app.index.load(Ordering::Relaxed),
                            presentation_destination,
                        }
                        .try_into()
                        .unwrap(),
                    )
                    .unwrap();
            }
            ProMessage::PresentationTriggerPrevious {
                presentation_destination,
            } => {
                let _ =
                    app.index
                        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |mut index| {
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
            }
            ProMessage::PresentationTriggerIndex {
                slide_index,
                presentation_destination,
                ..
            } => {
                app.index
                    .swap(slide_index.parse().unwrap(), Ordering::Relaxed);
                app.next.store(false, Ordering::Relaxed);
                app.vb_dbg.store(0, Ordering::Relaxed);
                app.obj_dbg.store(0, Ordering::Relaxed);
                app.resolved.store(None);
                ctx.request_repaint();

                websocket.write(
                    ProResponse::PresentationTriggerIndex {
                        presentation_path: Cow::Borrowed(&file_name),
                        slide_index: app.index.load(Ordering::Relaxed),
                        presentation_destination,
                    }
                    .try_into()
                    .unwrap(),
                )?;
            }
            ProMessage::Ping => {
                websocket.write(Message::Text(String::new()))?;
            }
        }

        websocket.flush()?;
    }
}
