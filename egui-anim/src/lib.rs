use std::{
    fmt::Debug,
    io::Cursor,
    sync::{atomic::AtomicUsize, Arc},
};

use atomic_float::AtomicF64;
use egui::{
    ahash::HashMap,
    load::{BytesPoll, ImageLoader, ImagePoll},
    ColorImage,
};
use image::{
    codecs::{gif::GifDecoder, png::PngDecoder, webp::WebPDecoder},
    AnimationDecoder, Delay,
};
use parking_lot::Mutex;

static MOVE: Mutex<Option<Arc<[(Arc<ColorImage>, Delay)]>>> = Mutex::new(None);

#[derive(Clone, Debug)]
pub struct Anim {
    frames: Arc<[(String, Delay)]>,
    delta: Arc<AtomicF64>,
    frame_on: Arc<AtomicUsize>,
}

impl Anim {
    pub fn new(ctx: &egui::Context, uri: &str, size_hint: egui::SizeHint) -> Anim {
        {
            let mut poll = ImagePoll::Pending { size: None };
            while matches!(poll, ImagePoll::Pending { .. }) {
                poll = ctx.try_load_image(uri, size_hint).unwrap();
            }
        }

        let frames = Arc::clone(&MOVE.lock().as_ref().unwrap());

        Anim {
            frames: frames
                .iter()
                .enumerate()
                .map(|(index, (_, delay))| (format!("{}\0{}", uri, index), *delay))
                .collect::<Vec<_>>()
                .into(),
            delta: Arc::new(ctx.input(|i| i.time).into()),
            frame_on: Arc::new(0.into()),
        }
    }
}

#[derive(Clone)]
pub struct AnimEntry {
    frames: Arc<[(Arc<ColorImage>, Delay)]>,
}

#[derive(Default)]
pub struct AnimLoader {
    cache: Mutex<HashMap<String, AnimEntry>>,
}

impl AnimLoader {
    pub const ID: &'static str = egui::generate_loader_id!(ImageLoader);
}

impl ImageLoader for AnimLoader {
    fn id(&self) -> &str {
        Self::ID
    }

    fn load(
        &self,
        ctx: &egui::Context,
        uri: &str,
        _: egui::SizeHint,
    ) -> egui::load::ImageLoadResult {
        let mut uri_split = uri.split('\0');

        let Some(uri) = uri_split.next() else {
            return Err(egui::load::LoadError::NotSupported);
        };

        let Some(mime) = uri_split.next() else {
            return Err(egui::load::LoadError::NotSupported);
        };

        let index = uri_split.next().unwrap_or("0");

        let mut cache = self.cache.lock();
        if let Some(entry) = cache.get(uri).cloned() {
            Ok(ImagePoll::Ready {
                image: Arc::clone(
                    &entry
                        .frames
                        .get(index.parse::<usize>().unwrap())
                        .or_else(|| entry.frames.last())
                        .unwrap()
                        .0,
                ),
            })
        } else {
            match ctx.try_load_bytes(uri) {
                Ok(BytesPoll::Ready { bytes, .. }) => {
                    let result = match mime {
                        "gif" => {
                            AnimEntry::new(GifDecoder::new(Cursor::new(bytes.as_ref())).unwrap())
                        }
                        "apng" => AnimEntry::new(
                            PngDecoder::new(Cursor::new(bytes.as_ref()))
                                .unwrap()
                                .apng()
                                .unwrap(),
                        ),
                        "webp" => {
                            AnimEntry::new(WebPDecoder::new(Cursor::new(bytes.as_ref())).unwrap())
                        }
                        _ => return Err(egui::load::LoadError::NotSupported),
                    };
                    *MOVE.lock() = Some(Arc::clone(&result.frames));
                    cache.insert(uri.into(), result.clone());
                    Ok(ImagePoll::Ready {
                        image: Arc::clone(&result.frames[0].0),
                    })
                }
                Ok(BytesPoll::Pending { size }) => Ok(ImagePoll::Pending { size }),
                Err(e) => Err(e),
            }
        }
    }

    fn forget(&self, uri: &str) {
        let _ = self.cache.lock().remove(uri);
    }

    fn forget_all(&self) {
        self.cache.lock().clear();
    }

    fn byte_size(&self) -> usize {
        self.cache.lock().values().map(|_| 0).sum()
    }
}

impl Debug for AnimEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Gif").finish()
    }
}

impl AnimEntry {
    pub fn new<'a, T: AnimationDecoder<'a>>(file: T) -> Self {
        let raw_frames = file.into_frames().collect_frames().unwrap();
        let mut frames = Vec::new();
        for frame in raw_frames {
            let delay = frame.delay();
            let buffer = frame.into_buffer();
            let s = [buffer.width(), buffer.height()];

            let pixels = buffer.as_flat_samples();
            let texture = ColorImage::from_rgba_unmultiplied(s.map(|f| f as _), pixels.as_slice());

            frames.push((Arc::new(texture), delay));
        }
        AnimEntry {
            frames: frames.into(),
        }
    }
}

impl Anim {
    pub fn find_img(&self, ctx: &egui::Context) -> &str {
        let delta = self.delta.load(std::sync::atomic::Ordering::Relaxed);
        let raw_time = ctx.input(|i| i.time);
        let time = if delta > 0.1 {
            raw_time % delta
        } else {
            raw_time
        } * 1000.0;
        let mut frame_on = self.frame_on.load(std::sync::atomic::Ordering::Relaxed);
        let frame_time = self.frames[frame_on].1.numer_denom_ms();
        let frame_time = frame_time.0 as f64 / frame_time.1 as f64;

        if time >= frame_time {
            self.delta
                .store(raw_time, std::sync::atomic::Ordering::Relaxed);
            frame_on += 1;
            if frame_on == self.frames.len() {
                frame_on = 0;
            }
            self.frame_on
                .store(frame_on, std::sync::atomic::Ordering::Relaxed);
        }
        ctx.request_repaint();
        &self.frames[frame_on].0
    }
}
