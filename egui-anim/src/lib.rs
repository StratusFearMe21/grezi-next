use std::{
    fmt::Debug,
    sync::{atomic::AtomicUsize, Arc},
    time::Duration,
};

use web_time::Instant;

use egui::{
    epaint::RectShape, mutex::Mutex, Color32, ColorImage, Rect, Rounding, Stroke, TextureHandle,
    TextureOptions, Ui, Vec2,
};
use image::{AnimationDecoder, Delay};

#[derive(Clone)]
pub struct Anim {
    frames: Arc<[(TextureHandle, Delay)]>,
    pub size: Vec2,
    last_instant: Arc<Mutex<Instant>>,
    frame_on: Arc<AtomicUsize>,
}

impl Debug for Anim {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Gif").finish()
    }
}

impl Anim {
    pub fn new<'a, T: AnimationDecoder<'a>>(
        ctx: &egui::Context,
        file: T,
        name: Option<impl Debug>,
    ) -> Self {
        let raw_frames = file.into_frames().collect_frames().unwrap();
        let mut frames = Vec::new();
        let mut size = [0; 2];
        for (num, frame) in raw_frames.into_iter().enumerate() {
            let delay = frame.delay();
            let buffer = frame.into_buffer();
            let s = [buffer.width(), buffer.height()];
            if s[0] > size[0] || s[1] > size[1] {
                size = s;
            }

            let pixels = buffer.as_flat_samples();
            let texture = ctx.load_texture(
                format!("{:?}-{}", name, num),
                ColorImage::from_rgba_unmultiplied(s.map(|f| f as _), pixels.as_slice()),
                TextureOptions::default(),
            );

            frames.push((texture, delay));
        }
        Anim {
            frames: frames.into(),
            last_instant: Arc::new(Mutex::new(Instant::now())),
            frame_on: Arc::new(0.into()),
            size: egui::vec2(size[0] as _, size[1] as _),
        }
    }
}

impl Anim {
    pub fn ui(&self, ui: &mut Ui, rect: Rect, tint: Color32) {
        let mut last_instant = self.last_instant.lock();
        let elapsed = last_instant.elapsed();
        let mut frame_on = self.frame_on.load(std::sync::atomic::Ordering::Relaxed);

        if (elapsed.as_millis() as u32, elapsed.subsec_nanos() as u32)
            >= self.frames[frame_on].1.numer_denom_ms()
        {
            *last_instant = Instant::now();
            frame_on += 1;
            if frame_on == self.frames.len() {
                frame_on = 0;
            }
            self.frame_on
                .store(frame_on, std::sync::atomic::Ordering::Relaxed);
            ui.ctx().request_repaint_after(Duration::from_millis(
                self.frames[frame_on].1.numer_denom_ms().0 as u64,
            ));
        } else {
            ui.ctx().request_repaint_after(Duration::from_millis(
                self.frames[frame_on].1.numer_denom_ms().0 as u64
                    - last_instant.elapsed().as_millis() as u64,
            ));
        }

        ui.painter().add(RectShape {
            rect,
            rounding: Rounding::ZERO,
            fill: tint,
            stroke: Stroke::NONE,
            fill_texture_id: self.frames[frame_on].0.id(),
            uv: Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
        });
    }
}
