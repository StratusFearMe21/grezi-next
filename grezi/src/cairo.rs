use cairo::freetype;
use cairo::FontFace;
use cairo::ImageSurface;
use cairo::TextClusterFlags;
use eframe::egui;
use eframe::epaint::mutex::Mutex;
use eframe::epaint::TextureId;
use egui_glyphon::glyphon::fontdb::ID;
use egui_glyphon::glyphon::FontSystem;
use egui_glyphon::glyphon::LayoutGlyph;
use egui_glyphon::glyphon::LayoutRun;
use egui_glyphon::glyphon::LayoutRunIter;
use egui_glyphon::GlyphonRendererCallback;
use indexmap::IndexSet;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;

use crate::parser::objects::Editor;

pub fn fonts_to_ft(
    font_system: Arc<Mutex<FontSystem>>,
    used_fonts: &IndexSet<ID, ahash::RandomState>,
    ft: freetype::Library,
) -> HashMap<ID, (freetype::Face, cairo::FontFace)> {
    let mut font_system = font_system.lock();
    used_fonts
        .iter()
        .copied()
        .map(|f| {
            let data = unsafe { font_system.db_mut().make_shared_face_data(f) }.unwrap();
            let face = ft
                .new_memory_face(Rc::new(data.0.deref().as_ref().to_owned()), data.1 as isize)
                .unwrap();
            let cairo_face = FontFace::create_from_ft(&face).unwrap();
            (f, (face, cairo_face))
        })
        .collect()
}

pub fn cairo_draw(
    output: egui::FullOutput,
    textures: &mut HashMap<TextureId, ImageSurface>,
    ctx: &cairo::Context,
    fonts: &HashMap<ID, (freetype::Face, cairo::FontFace)>,
) {
    for (id, tex) in output.textures_delta.set {
        let surface = match tex.image {
            egui::ImageData::Color(c) => ImageSurface::create_for_data(
                c.pixels
                    .iter()
                    .flat_map(|c| {
                        let c = c.to_array();
                        [c[2], c[1], c[0], c[3]]
                    })
                    .collect::<Vec<_>>(),
                cairo::Format::ARgb32,
                c.width() as i32,
                c.height() as i32,
                cairo::Format::ARgb32
                    .stride_for_width(c.width() as u32)
                    .unwrap(),
            )
            .unwrap(),
            _ => continue,
        };

        if let Some(pos) = tex.pos {
            let texture = textures.get_mut(&id).unwrap();

            let ctx = cairo::Context::new(texture).unwrap();

            ctx.set_source_surface(&surface, 0.0, 0.0).unwrap();
            ctx.rectangle(
                pos[0] as f64,
                pos[1] as f64,
                surface.width() as f64,
                surface.height() as f64,
            );
            ctx.fill().unwrap();
        } else {
            textures.insert(id, surface);
        }
    }

    for shape in output.shapes {
        ctx.reset_clip();

        ctx.rectangle(
            shape.clip_rect.min.x as f64,
            shape.clip_rect.min.y as f64,
            shape.clip_rect.width() as f64,
            shape.clip_rect.height() as f64,
        );

        ctx.clip();

        cairo_draw_shape(ctx, shape.shape, textures, fonts);
    }

    for id in output.textures_delta.free {
        textures.remove(&id);
    }
}

pub fn cairo_draw_shape(
    ctx: &cairo::Context,
    shape: eframe::epaint::Shape,
    textures: &HashMap<TextureId, ImageSurface>,
    fonts: &HashMap<ID, (freetype::Face, cairo::FontFace)>,
) {
    use cairo::{SurfacePattern, TextCluster};

    match shape {
        egui::Shape::Noop => {}
        egui::Shape::Vec(shapes) => {
            for shape in shapes {
                cairo_draw_shape(ctx, shape, textures, fonts);
            }
        }
        egui::Shape::LineSegment { points, stroke } => {
            let color: palette::Srgba<u8> =
                palette::cast::from_array(stroke.color.to_srgba_unmultiplied());
            let color: palette::Srgba<f64> = color.into_format();
            // let color: palette::LinSrgba<f64> = color.into_linear();
            ctx.set_source_rgba(color.red, color.green, color.blue, color.alpha);
            ctx.set_line_width(stroke.width as f64);

            ctx.move_to(points[0].x as f64, points[0].y as f64);
            ctx.line_to(points[1].x as f64, points[1].y as f64);
            ctx.stroke().unwrap();
        }
        egui::Shape::Rect(rect) => {
            let texture = textures.get(&rect.fill_texture_id);
            if let Some(texture) = texture {
                ctx.save().unwrap();
                ctx.translate(rect.rect.min.x as f64, rect.rect.min.y as f64);
                let ratio = rect.rect.width() as f64 / texture.width() as f64;
                ctx.scale(ratio, ratio);
                ctx.set_source(&SurfacePattern::create(texture)).unwrap();
                ctx.paint().unwrap();
                ctx.restore().unwrap();
                ctx.set_operator(cairo::Operator::Multiply);
            }
            let color: palette::Srgba<u8> =
                palette::cast::from_array(rect.fill.to_srgba_unmultiplied());
            let color: palette::Srgba<f64> = color.into_format();
            // let color: palette::LinSrgba<f64> = color.into_linear();
            ctx.set_source_rgba(color.red, color.green, color.blue, color.alpha);

            ctx.rectangle(
                rect.rect.min.x as f64,
                rect.rect.min.y as f64,
                rect.rect.width() as f64,
                rect.rect.height() as f64,
            );
            ctx.fill().unwrap();

            if texture.is_some() {
                ctx.set_operator(cairo::Operator::Over);
            }

            ctx.set_line_width(rect.stroke.width as f64);
            if rect.stroke.width > 0.0 {
                let color: palette::Srgba<u8> =
                    palette::cast::from_array(rect.stroke.color.to_srgba_unmultiplied());
                let color: palette::Srgba<f64> = color.into_format();
                // let color: palette::LinSrgba<f64> = color.into_linear();
                ctx.set_source_rgba(color.red, color.green, color.blue, color.alpha);

                ctx.rectangle(
                    rect.rect.min.x as f64,
                    rect.rect.min.y as f64,
                    rect.rect.width() as f64,
                    rect.rect.height() as f64,
                );
                ctx.stroke().unwrap();
            }
        }
        egui::Shape::Callback(glyphon_callback) => {
            let callback = glyphon_callback
                .callback
                .downcast_ref::<GlyphonRendererCallback<Editor>>()
                .unwrap();

            struct RunIter<'a> {
                run: Rc<LayoutRun<'a>>,
                glyphs: std::slice::Iter<'a, LayoutGlyph>,
                iter: LayoutRunIter<'a>,
            }

            impl<'a> Iterator for RunIter<'a> {
                type Item = (Rc<LayoutRun<'a>>, &'a LayoutGlyph);

                fn next(&mut self) -> Option<Self::Item> {
                    let next = self.glyphs.next().or_else(|| {
                        self.run = Rc::new(self.iter.next()?);
                        self.glyphs = self.run.glyphs.iter();
                        self.glyphs.next()
                    })?;

                    Some((Rc::clone(&self.run), next))
                }
            }

            impl<'a> RunIter<'a> {
                fn new(mut iter: LayoutRunIter<'a>) -> RunIter<'a> {
                    let run = Rc::new(iter.next().unwrap());
                    let glyphs = run.glyphs.iter();
                    RunIter { run, glyphs, iter }
                }
            }

            for buffer in callback.buffers.iter() {
                let buffer_read = buffer.buffer.read();
                ctx.set_font_size(buffer_read.as_ref().metrics().font_size as f64);
                let mut glyphs = RunIter::new(buffer_read.as_ref().layout_runs()).peekable();

                while glyphs.peek().is_some() {
                    let color;
                    let font_id;
                    let rtl;
                    if let Some(glyph) = glyphs.peek() {
                        color = glyph.1.color_opt;
                        font_id = glyph.1.font_id;
                        rtl = glyph.0.rtl;
                    } else {
                        glyphs.next();
                        continue;
                    }
                    let color_rgba: palette::Srgba<u8> = palette::cast::from_array(
                        color
                            .unwrap_or(egui_glyphon::glyphon::Color::rgb(255, 255, 255))
                            .as_rgba(),
                    );
                    let color_rgba: palette::Srgba<f64> = color_rgba.into_format();
                    let font = fonts.get(&font_id).unwrap();

                    ctx.set_source_rgba(
                        color_rgba.red,
                        color_rgba.green,
                        color_rgba.blue,
                        color_rgba.alpha,
                    );
                    ctx.set_font_face(&font.1);

                    let mut new_glyphs = Vec::new();
                    let mut text = String::new();
                    let mut clusters = Vec::new();
                    while let Some(g) = glyphs.peek() {
                        if g.1.color_opt != color || g.1.font_id != font_id || g.0.rtl != rtl {
                            break;
                        }

                        let t = &g.0.text[g.1.start..g.1.end];
                        text.push_str(&t);
                        clusters.push(TextCluster::new(t.len() as i32, 1));
                        let glyph = cairo::Glyph::new(
                            g.1.glyph_id as u64,
                            buffer.rect.left() as f64 + g.1.x as f64,
                            buffer.rect.top() as f64 + g.0.line_y as f64 + g.1.y as f64,
                        );
                        new_glyphs.push(glyph);
                        glyphs.next();
                    }

                    ctx.show_text_glyphs(
                        &text,
                        &new_glyphs,
                        &clusters,
                        if rtl {
                            TextClusterFlags::Backward
                        } else {
                            TextClusterFlags::None
                        },
                    )
                    .unwrap();
                }
            }
        }
        _ => {}
    }
}
