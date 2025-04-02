use std::{
    borrow::Cow,
    net::TcpStream,
    ops::{Deref, DerefMut},
    sync::{
        mpsc::{Receiver, Sender},
        Arc,
    },
};

use arc_swap::ArcSwapOption;
use eframe::{
    egui::{self, mutex::Mutex, Modifiers, Vec2},
    egui_wgpu,
};
use egui_glyphon::{glyphon::FontSystem, GlyphonRendererCallback};
use grezi_egui::GrzResolvedSlide;
use grezi_font_serde::FontSystemDeserializer;
use grezi_parser::GrzRoot;
use keyframe::functions::EaseOutCubic;
use serde::{Deserialize, Serialize};
use tungstenite::{stream::MaybeTlsStream, WebSocket};

#[derive(Serialize, Deserialize)]
enum Message {
    Index { index: usize, reset_time: bool },
    Get,
}

impl TryInto<tungstenite::Message> for Message {
    type Error = postcard::Error;

    fn try_into(self) -> Result<tungstenite::Message, Self::Error> {
        Ok(tungstenite::Message::binary(postcard::to_allocvec(&self)?))
    }
}

pub enum App {
    Connection {
        ip: String,
        font_system: Arc<Mutex<FontSystem>>,
    },
    InShow {
        root: GrzRoot,
        resolved: ArcSwapOption<GrzResolvedSlide>,
        next_resolved: ArcSwapOption<GrzResolvedSlide>,
        font_system: Arc<Mutex<FontSystem>>,
        custom_key_events: Receiver<egui::Event>,
        custom_key_sender: Sender<egui::Event>,
        index: usize,
        time: f64,
        socket: WebSocket<MaybeTlsStream<TcpStream>>,
    },
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            match self {
                App::Connection { ip, font_system } => {
                    ui.label("Enter IP address for presentation:");
                    ui.text_edit_singleline(ip);
                    if ui.button("Connect").clicked() {
                        let (connection, _) =
                            tungstenite::connect(format!("ws://{}:3000/subscribe", ip)).unwrap();

                        // connection.send(Message::Get.try_into().unwrap()).unwrap();

                        // let slideshow = connection.read().unwrap().into_data();
                        let slideshow = ureq::get(format!("http://{}:3000/slideshow", ip))
                            .call()
                            .unwrap()
                            .into_body()
                            .with_config()
                            .limit(u64::MAX)
                            .read_to_vec()
                            .unwrap();

                        let mut result: (FontSystemDeserializer, GrzRoot) =
                            postcard::from_bytes(&slideshow).unwrap();

                        result.0 .0.db_mut().set_sans_serif_family("Ubuntu");
                        result.0 .0.db_mut().set_monospace_family("Fira Code");
                        result.0 .0.db_mut().set_serif_family("DejaVu Serif");

                        *font_system.lock() = result.0 .0;
                        // ctx.set_fonts(result.0 .1);

                        let (key_tx, key_rx) = std::sync::mpsc::channel();

                        // std::thread::spawn({
                        //     let key_tx = key_tx.clone();
                        //     let egui_ctx = ctx.clone();
                        //     move || {
                        //         let message = connection.read().unwrap().into_data();
                        //         let message: Message = postcard::from_bytes(&message).unwrap();

                        //         match message {
                        //             Message::Next => {
                        //                 key_tx
                        //                     .send(egui::Event::Key {
                        //                         key: egui::Key::ArrowRight,
                        //                         pressed: true,
                        //                         physical_key: None,
                        //                         repeat: false,
                        //                         modifiers: Modifiers::NONE,
                        //                     })
                        //                     .unwrap();
                        //                 egui_ctx.request_repaint();
                        //             }
                        //             Message::Previous => {
                        //                 key_tx
                        //                     .send(egui::Event::Key {
                        //                         key: egui::Key::ArrowLeft,
                        //                         pressed: true,
                        //                         physical_key: None,
                        //                         repeat: false,
                        //                         modifiers: Modifiers::NONE,
                        //                     })
                        //                     .unwrap();
                        //                 egui_ctx.request_repaint();
                        //             }
                        //             _ => {}
                        //         }
                        //     }
                        // });

                        *self = Self::InShow {
                            root: result.1,
                            resolved: ArcSwapOption::empty(),
                            next_resolved: ArcSwapOption::empty(),
                            font_system: Arc::clone(font_system),
                            custom_key_events: key_rx,
                            custom_key_sender: key_tx,
                            index: 0,
                            time: 0.0,
                            socket: connection,
                        };
                    }
                }
                App::InShow {
                    root,
                    resolved,
                    next_resolved,
                    font_system,
                    index,
                    time,
                    custom_key_events,
                    custom_key_sender,
                    socket,
                } => {
                    let max_rect = ui.max_rect();

                    ctx.input(|input| {
                        if resolved.load().is_none() || next_resolved.load().is_none() {
                            *time = input.time;
                        }
                        for event in input
                            .events
                            .iter()
                            .map(|e| Cow::Borrowed(e))
                            .chain(custom_key_events.try_iter().map(|e| Cow::Owned(e)))
                        {
                            match event.as_ref() {
                                egui::Event::PointerButton {
                                    pressed: true, pos, ..
                                } => {
                                    let mut top_half_rect = max_rect;
                                    top_half_rect.max.y /= 2.0;

                                    if top_half_rect.contains(*pos) {
                                        *index += 1;
                                        resolved.store(None);
                                        next_resolved.store(None);
                                        *time = input.time;
                                        socket
                                            .send(
                                                Message::Index {
                                                    index: *index,
                                                    reset_time: true,
                                                }
                                                .try_into()
                                                .unwrap(),
                                            )
                                            .unwrap();
                                    } else {
                                        *index = index.saturating_sub(1);
                                        // When navigating to the previous slide
                                        // skip over all slides marked with the
                                        // `next()` action
                                        while root
                                            .slides
                                            .get_index(*index)
                                            .map(|s| s.1.slide_params.next.is_some())
                                            .unwrap_or_default()
                                        {
                                            *index = index.saturating_sub(1);
                                        }
                                        resolved.store(None);
                                        next_resolved.store(None);
                                        socket
                                            .send(
                                                Message::Index {
                                                    index: *index,
                                                    reset_time: false,
                                                }
                                                .try_into()
                                                .unwrap(),
                                            )
                                            .unwrap();
                                    }
                                }
                                egui::Event::Key {
                                    key: egui::Key::ArrowRight | egui::Key::Space,
                                    pressed: true,
                                    modifiers,
                                    ..
                                } => {
                                    *index += 1;
                                    resolved.store(None);
                                    next_resolved.store(None);
                                    *time = input.time;
                                    if !modifiers.ctrl {
                                        socket
                                            .send(
                                                Message::Index {
                                                    index: *index,
                                                    reset_time: true,
                                                }
                                                .try_into()
                                                .unwrap(),
                                            )
                                            .unwrap();
                                    }
                                }
                                egui::Event::Key {
                                    key: egui::Key::ArrowLeft,
                                    pressed: true,
                                    ..
                                } => {
                                    *index = index.saturating_sub(1);
                                    // When navigating to the previous slide
                                    // skip over all slides marked with the
                                    // `next()` action
                                    while root
                                        .slides
                                        .get_index(*index)
                                        .map(|s| s.1.slide_params.next.is_some())
                                        .unwrap_or_default()
                                    {
                                        *index = index.saturating_sub(1);
                                    }
                                    resolved.store(None);
                                    next_resolved.store(None);
                                    socket
                                        .send(
                                            Message::Index {
                                                index: *index,
                                                reset_time: false,
                                            }
                                            .try_into()
                                            .unwrap(),
                                        )
                                        .unwrap();
                                }
                                egui::Event::Key {
                                    key: egui::Key::B,
                                    pressed: true,
                                    ..
                                } => {
                                    *index = 0;
                                    resolved.store(None);
                                    next_resolved.store(None);
                                }
                                egui::Event::Key {
                                    key: egui::Key::R,
                                    pressed: true,
                                    ..
                                } => {
                                    *time = input.time;
                                }
                                _ => {}
                            }
                        }
                    });

                    if resolved.load().is_none() {
                        let mut new_slide;
                        loop {
                            new_slide = GrzResolvedSlide::resolve_slide(
                                &root,
                                font_system.lock().deref_mut(),
                                ui.ctx(),
                                *index,
                            );

                            if new_slide.is_none() {
                                *index = index.saturating_sub(1);
                                if *index == 0 {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                        resolved.store(new_slide.map(Arc::new));
                        let next_slide = GrzResolvedSlide::resolve_slide(
                            &root,
                            font_system.lock().deref_mut(),
                            ui.ctx(),
                            *index + 1,
                        );
                        next_resolved.store(next_slide.map(Arc::new));
                    }

                    let mut buffers = Vec::new();
                    let time = ui.input(|i| i.time - *time);

                    let mut first_slide_rect = max_rect;
                    let mut second_slide_rect = max_rect;
                    let mut speaker_rect = max_rect;

                    if max_rect.height() > max_rect.width() {
                        first_slide_rect.max.y /= 3.0;
                        speaker_rect =
                            first_slide_rect.translate(Vec2::new(0.0, first_slide_rect.max.y));
                        second_slide_rect =
                            speaker_rect.translate(Vec2::new(0.0, first_slide_rect.max.y));
                    } else {
                        first_slide_rect.max.x /= 2.0;
                        speaker_rect.min.x = first_slide_rect.max.x;
                        second_slide_rect.min.x = first_slide_rect.max.x;
                        second_slide_rect.max.y /= 2.0;
                        speaker_rect.min.y = second_slide_rect.max.y;
                    }
                    if let Some(resolved) = resolved.load().deref() {
                        resolved.draw(
                            first_slide_rect,
                            ui,
                            time,
                            &EaseOutCubic,
                            &mut buffers,
                            None,
                        );
                        if resolved.max_time > time {
                            ctx.request_repaint();
                        } else if let Some(next) = resolved.params.next {
                            if next > time {
                                ctx.request_repaint_after_secs((next - time) as f32);
                            } else {
                                custom_key_sender
                                    .send(egui::Event::Key {
                                        key: egui::Key::Space,
                                        pressed: true,
                                        physical_key: None,
                                        repeat: false,
                                        modifiers: Modifiers::CTRL,
                                    })
                                    .unwrap();
                            }
                        }
                        egui::Window::new("Speaker Notes")
                            .fixed_rect(speaker_rect)
                            .fixed_size(speaker_rect.size())
                            .fixed_pos(speaker_rect.min)
                            .min_size(speaker_rect.size())
                            .show(ui.ctx(), |ui| {
                                ui.label(
                                    resolved
                                        .params
                                        .speaker_notes
                                        .as_ref()
                                        .map(|sn| sn.as_ref())
                                        .unwrap_or_default(),
                                )
                            });
                    }

                    if let Some(resolved) = next_resolved.load().deref() {
                        resolved.draw(
                            second_slide_rect,
                            ui,
                            time,
                            &EaseOutCubic,
                            &mut buffers,
                            None,
                        );
                        if resolved.max_time > time {
                            ctx.request_repaint();
                        }
                    }

                    ui.painter().add(egui_wgpu::Callback::new_paint_callback(
                        max_rect,
                        GlyphonRendererCallback { buffers },
                    ));
                }
            }
        });
    }
}
