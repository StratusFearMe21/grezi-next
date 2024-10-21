use std::{
    borrow::Cow,
    collections::HashMap,
    fmt::Display,
    io::{BufRead, BufReader, BufWriter, Cursor, Write},
    net::TcpListener,
    ops::{Deref, DerefMut},
    path::Path,
    str::FromStr,
    sync::{atomic::Ordering, Arc, Mutex},
};

use base64::Engine;
use cairo::ImageSurface;
use eframe::egui::{PaintCallback, Pos2, Rect, TextureId, Vec2};
use egui_glyphon::GlyphonRendererCallback;
use image::{DynamicImage, ImageFormat, RgbaImage};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::{parser::AstObject, resolver::Resolved, MyEguiApp};

enum ImpressMessage {
    TransitionNext,
    TransitionPrevious,
    GotoSlide { slide_number: usize },
    PresentationStart,
    PresentationStop,
    PresentationResume,
    PresentationBlankScreen,
    PointerStarted { initial_x: f32, initial_y: f32 },
    PointerDismissed,
    PointerCoordination { current_x: f32, current_y: f32 },
}

impl FromStr for ImpressMessage {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        match lines.next() {
            Some("transition_next") => Ok(Self::TransitionNext),
            Some("transition_previous") => Ok(Self::TransitionPrevious),
            Some("goto_slide") => Ok(Self::GotoSlide {
                slide_number: lines.next().ok_or(())?.parse().or(Err(()))?,
            }),
            Some("presentation_start") => Ok(Self::PresentationStart),
            Some("presentation_stop") => Ok(Self::PresentationStop),
            Some("presentation_resume") => Ok(Self::PresentationResume),
            Some("presentation_blank_screen") => Ok(Self::PresentationBlankScreen),
            Some("pointer_started") => Ok(Self::PointerStarted {
                initial_x: lines.next().ok_or(())?.parse().or(Err(()))?,
                initial_y: lines.next().ok_or(())?.parse().or(Err(()))?,
            }),
            Some("pointer_dismissed") => Ok(Self::PointerDismissed),
            Some("pointer_coordination") => Ok(Self::PointerCoordination {
                current_x: lines.next().ok_or(())?.parse().or(Err(()))?,
                current_y: lines.next().ok_or(())?.parse().or(Err(()))?,
            }),
            _ => Err(()),
        }
    }
}

enum ImpressResponse<'a> {
    SlideshowFinished,
    SlideshowStarted {
        number_of_slides: usize,
        current_slide_number: usize,
    },
    SlideNotes {
        slide_number: usize,
        notes: Cow<'a, str>,
    },
    SlideUpdated {
        current_slide_number: usize,
    },
    SlidePreview {
        slide_number: usize,
        image: Cow<'a, str>,
    },
    SlideshowInfo {
        title: Cow<'a, str>,
    },
}

impl<'a> Display for ImpressResponse<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SlideshowFinished => write!(f, "slideshow_finished\n\n"),
            Self::SlideshowStarted {
                number_of_slides,
                current_slide_number,
            } => write!(
                f,
                "slideshow_started\n{}\n{}\n\n",
                number_of_slides, current_slide_number
            ),
            Self::SlideNotes {
                slide_number,
                notes,
            } => write!(f, "slide_notes\n{}\n{}\n\n", slide_number, notes),
            Self::SlideUpdated {
                current_slide_number,
            } => write!(f, "slide_updated\n{}\n\n", current_slide_number),
            Self::SlidePreview {
                slide_number,
                image,
            } => write!(f, "slide_preview\n{}\n{}\n\n", slide_number, image),
            Self::SlideshowInfo { title } => write!(f, "slideshow_info\n{}\n\n", title),
        }
    }
}

pub fn start_server(app: MyEguiApp, ctx: eframe::egui::Context) -> std::io::Result<()> {
    let server = TcpListener::bind("0.0.0.0:1599").unwrap();

    let (mut stream, _) = server.accept()?;
    stream.write_fmt(format_args!(
        "LO_SERVER_SERVER_PAIRED\n\nLO_SERVER_INFO\n24.8.2.1\n\n",
    ))?;
    stream.write_fmt(format_args!(
        "{}",
        ImpressResponse::SlideshowInfo {
            title: Path::new(app.file_name.deref())
                .file_name()
                .unwrap()
                .to_string_lossy()
        }
    ))?;

    let mut stream_reader = BufReader::new(&stream);
    let mut stream_writer = BufWriter::new(&stream);

    loop {
        let mut msg = String::new();
        while stream_reader.read_line(&mut msg)? > 1 {}
        if msg.is_empty() {
            break;
        }
        let Ok(msg): Result<ImpressMessage, ()> = msg.parse() else {
            dbg!(msg);
            continue;
        };
        match msg {
            ImpressMessage::PresentationStart => {
                stream_writer.write_fmt(format_args!(
                    "{}",
                    ImpressResponse::SlideshowStarted {
                        number_of_slides: app.slide_show.load().read().slide_show.len(),
                        current_slide_number: app.index.load(Ordering::Relaxed)
                    }
                ))?;
                stream_writer.flush()?;
                let stream = Mutex::new(BufWriter::new(&stream));
                let size = Vec2::new(854.0, 480.0);

                let input = eframe::egui::RawInput {
                    screen_rect: Some(Rect::from_min_size(Pos2::ZERO, size)),
                    time: Some(25_849_200.0),
                    ..Default::default()
                };
                let slide_show = app.slide_show.load();
                let slide_show = slide_show.read();
                let used_fonts = slide_show.used_fonts(app.font_system.lock().deref_mut());

                slide_show.slide_show.par_iter().enumerate().for_each_init(
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
                            if !export_ctx.is_loader_installed(egui_anim::AnimLoader::ID) {
                                export_ctx
                                    .add_image_loader(Arc::new(egui_anim::AnimLoader::default()));
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
                                RgbaImage::from_raw(size.x as u32, size.y as u32, image_data)
                                    .unwrap(),
                            )
                            .write_to(&mut jpeg, ImageFormat::Png)
                            .unwrap();

                            base64::prelude::BASE64_STANDARD.encode(jpeg.into_inner())
                        };

                        {
                            let mut stream = stream.lock().unwrap();
                            stream
                                .write_fmt(format_args!(
                                    "{}",
                                    ImpressResponse::SlidePreview {
                                        slide_number: index,
                                        image: Cow::Owned(slide_image)
                                    }
                                ))
                                .unwrap();
                            stream
                                .write_fmt(format_args!(
                                    "{}",
                                    ImpressResponse::SlideNotes {
                                        slide_number: index,
                                        notes: Cow::Owned(format!(
                                            "<html><body><p>{}</p></body></html>",
                                            speaker_notes
                                                .clone()
                                                .unwrap_or_else(|| String::new().into()),
                                        ))
                                    }
                                ))
                                .unwrap();
                        }
                    },
                );

                stream.lock().unwrap().flush()?;
                app.resolved_images.lock().clear();
            }
            ImpressMessage::TransitionNext => {
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
                stream_writer.write_fmt(format_args!(
                    "{}",
                    ImpressResponse::SlideUpdated {
                        current_slide_number: app.index.load(Ordering::Relaxed)
                    }
                ))?;
            }
            ImpressMessage::TransitionPrevious => {
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
                stream_writer.write_fmt(format_args!(
                    "{}",
                    ImpressResponse::SlideUpdated {
                        current_slide_number: app.index.load(Ordering::Relaxed)
                    }
                ))?;
            }
            ImpressMessage::GotoSlide { slide_number } => {
                if app.index.swap(slide_number, Ordering::Relaxed) == slide_number.saturating_sub(1)
                {
                    app.speaker_view.clear_resolved();
                    app.restart_timer.store(1, Ordering::Relaxed);
                    app.next.store(true, Ordering::Relaxed);
                } else {
                    app.next.store(false, Ordering::Relaxed);
                }
                app.vb_dbg.store(0, Ordering::Relaxed);
                app.obj_dbg.store(0, Ordering::Relaxed);
                app.resolved.store(None);
                ctx.request_repaint();

                stream_writer.write_fmt(format_args!(
                    "{}",
                    ImpressResponse::SlideUpdated {
                        current_slide_number: app.index.load(Ordering::Relaxed)
                    }
                ))?;
            }
            ImpressMessage::PresentationStop => {}
            ImpressMessage::PresentationResume => {}
            ImpressMessage::PresentationBlankScreen => {}
            ImpressMessage::PointerStarted { .. } => {}
            ImpressMessage::PointerDismissed => {}
            ImpressMessage::PointerCoordination { .. } => {}
        }

        stream_writer.flush()?;
    }

    Ok(())
}
