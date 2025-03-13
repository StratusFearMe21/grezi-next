use std::{
    ops::DerefMut,
    sync::{mpsc::Sender, Arc},
};

use arc_swap::ArcSwapOption;
use egui::{self, mutex::Mutex, Modifiers};
use egui_glyphon::cosmic_text::FontSystem;
use grezi_egui::GrzResolvedSlide;
use grezi_parser::parse::GrzFile;

/// The `App` struct cannot have ownership over the `GrzFile`,
/// because in the case of the language server, it needs mutable
/// access to that data.
///
/// Instead what we do is we assign a struct
/// to own the data, and then when a new slide needs to be resolved,
/// it asks the owner of the `GrzFile` via a `crossbeam_channel::Sender`.
///
/// This is a stub for when the language server is not active
pub struct DefaultOwner {
    pub root: GrzFile,
    pub slide_index: usize,
    pub message_receiver: crossbeam_channel::Receiver<FileOwnerMessage>,
    pub shared_data: AppHandle,
}

impl DefaultOwner {
    pub fn run(mut self) {
        for message in self.message_receiver.iter() {
            let reset_time;
            match message {
                FileOwnerMessage::Index {
                    index,
                    reset_time: rt,
                } => {
                    self.slide_index = index;
                    reset_time = rt;
                }
                FileOwnerMessage::Next(trigger_was_action) => {
                    self.slide_index += 1;
                    reset_time = true;
                    if trigger_was_action && self.root.slideshow.slides.len() <= self.slide_index {
                        self.slide_index = 0;
                    }
                }
                FileOwnerMessage::Previous => {
                    self.slide_index = self.slide_index.saturating_sub(1);
                    reset_time = false;
                }
                FileOwnerMessage::ResetFile => {
                    let parse_result = self.root.update_file().unwrap();
                    if parse_result.has_errors() {
                        eprintln!("{:?}", parse_result);
                        continue;
                    }
                    reset_time = true;
                }
            }

            let mut new_slide;
            loop {
                new_slide = GrzResolvedSlide::resolve_slide(
                    &self.root.slideshow,
                    self.shared_data.font_system.lock().deref_mut(),
                    &self.shared_data.egui_ctx,
                    self.slide_index,
                );

                if new_slide.is_none() {
                    self.slide_index = self.slide_index.saturating_sub(1);
                    if self.slide_index == 0 {
                        break;
                    }
                } else {
                    break;
                }
            }
            self.shared_data.resolved.store(new_slide.map(Arc::new));

            if reset_time {
                self.shared_data
                    .custom_key_sender
                    .send(egui::Event::Key {
                        key: egui::Key::R,
                        physical_key: None,
                        pressed: true,
                        repeat: false,
                        modifiers: Modifiers::NONE,
                    })
                    .unwrap();
            }

            self.shared_data.egui_ctx.request_repaint();
        }
    }
}

/// Messages to be passed to the
/// owner of the current `GrzFile`
#[derive(Debug)]
pub enum FileOwnerMessage {
    Index { index: usize, reset_time: bool },
    ResetFile,
    Next(bool),
    Previous,
}

#[derive(Clone)]
pub struct AppHandle {
    pub resolved: Arc<ArcSwapOption<GrzResolvedSlide>>,
    pub font_system: Arc<Mutex<FontSystem>>,
    pub custom_key_sender: Sender<egui::Event>,
    pub root_owner_sender: crossbeam_channel::Sender<FileOwnerMessage>,
    pub egui_ctx: egui::Context,
}

impl AppHandle {
    pub fn new(
        custom_key_sender: Sender<egui::Event>,
        root_owner_sender: crossbeam_channel::Sender<FileOwnerMessage>,
        context: egui::Context,
        font_system: Arc<Mutex<FontSystem>>,
    ) -> Self {
        Self {
            resolved: Arc::new(ArcSwapOption::empty()),
            root_owner_sender,
            font_system,
            custom_key_sender,
            egui_ctx: context,
        }
    }
}
