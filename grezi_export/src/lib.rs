use std::{
    collections::HashMap,
    fs::File,
    io::{BufWriter, Write},
    ops::{Deref, DerefMut, Range},
    path::Path,
    rc::Rc,
    sync::Arc,
};

use cairo::{
    FontFace, ImageSurface, PdfSurface, PsSurface, SurfacePattern, SvgSurface, TextCluster,
    TextClusterFlags, freetype,
};
use egui::{Color32, Pos2, Rect, TextureId, Vec2, mutex::Mutex};
use egui_glyphon::{
    BufferWithTextArea,
    cosmic_text::{FontSystem, fontdb::ID},
};
use eyre::{Context, ContextCompat, OptionExt, bail};
use grezi_egui::{GrzResolvedSlide, ResolvedTextTag};
use grezi_font_serde::{FontRef, IndexSliceSerializer};
use grezi_parser::{GrzRoot, HIGHLIGHT_COLOR_DEFAULT};
use image::ImageFormat;
use indexmap::IndexSet;
use keyframe::functions::EaseOutCubic;
use smallvec::SmallVec;
use tracing::instrument;

pub struct GrzExporter<'a> {
    file: &'a GrzRoot,
    font_system: Arc<Mutex<FontSystem>>,
    ft: cairo::freetype::Library,
    used_faces: HashMap<ID, (freetype::Face, cairo::FontFace)>,
}

impl<'a> GrzExporter<'a> {
    pub fn new(file: &'a GrzRoot, font_system: Arc<Mutex<FontSystem>>) -> Self {
        Self {
            file,
            font_system,
            used_faces: HashMap::new(),
            ft: cairo::freetype::Library::init().unwrap(),
        }
    }

    #[instrument(skip(self, out_path), fields(out_path = %out_path.as_ref().display()))]
    pub fn export(
        &mut self,
        out_path: impl AsRef<Path>,
        size: Vec2,
        index_range: Range<usize>,
    ) -> eyre::Result<()> {
        let egui_ctx = egui::Context::default();
        egui_extras::install_image_loaders(&egui_ctx);
        let mut cairo_ctx;
        let mut image_data = Vec::new();
        match out_path.as_ref().extension().and_then(|e| e.to_str()) {
            Some("slideshow") => {
                let mut output_file =
                    BufWriter::new(File::create(&out_path).wrap_err("Failed to open output file")?);
                let mut all_fonts_used: IndexSet<FontRef, ahash::RandomState> = IndexSet::default();
                let mut font_system = self.font_system.lock();
                for i in 0..self.file.slides.len() {
                    let slide = GrzResolvedSlide::resolve_slide(
                        self.file,
                        font_system.deref_mut(),
                        &egui_ctx,
                        i,
                    )
                    .ok_or_eyre("Failed to resolve slide during serialization")?;

                    let fonts = slide.fonts_used();

                    for font in fonts {
                        all_fonts_used.insert(FontRef(unsafe {
                            font_system.db_mut().make_shared_face_data(font).unwrap().0
                        }));
                    }
                }
                postcard::to_io(
                    &(IndexSliceSerializer(all_fonts_used.as_slice()), self.file),
                    &mut output_file,
                )
                .wrap_err("Failed to write slideshow in binary format to file")?;
                output_file
                    .flush()
                    .wrap_err("Failed to flush output file")?;
                return Ok(());
            }
            Some("pdf") => {
                cairo_ctx = cairo::Context::new(
                    &PdfSurface::new(size.x as f64, size.y as f64, &out_path)
                        .wrap_err("Error creating pdf surface")?,
                )
                .unwrap();
            }
            Some("ps") => {
                cairo_ctx = cairo::Context::new(
                    &PsSurface::new(size.x as f64, size.y as f64, &out_path)
                        .wrap_err("Error creating ps surface")?,
                )
                .unwrap();
            }
            Some("svg") => {
                cairo_ctx = cairo::Context::new(
                    &SvgSurface::new(size.x as f64, size.y as f64, Some(&out_path))
                        .wrap_err("Error creating svg surface")?,
                )
                .unwrap();
            }
            Some(ext) => {
                if ImageFormat::from_extension(ext).is_none() {
                    bail!("Unsupported export extension `{}`", ext)
                }
                let stride = cairo::Format::ARgb32
                    .stride_for_width(size.x as u32)
                    .wrap_err_with(|| {
                        format!("Error calculating ARGB stride for width: {}", size.x as u32)
                    })?;
                image_data = vec![0; stride as usize * size.y as usize];
                cairo_ctx = unsafe {
                    cairo::Context::new(
                        &ImageSurface::create_for_data_unsafe(
                            image_data.as_mut_ptr(),
                            cairo::Format::ARgb32,
                            size.x as i32,
                            size.y as i32,
                            stride,
                        )
                        .wrap_err("Error creating svg surface")?,
                    )
                    .unwrap()
                };
            }
            None => bail!("File extension was not valid UTF-8"),
        }

        let input = egui::RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, size)),
            //pixels_per_point: Some(2.0),
            ..Default::default()
        };

        let mut textures = HashMap::new();

        for index in index_range {
            let mut text_buffers = Vec::new();
            let resolved_slide = grezi_egui::GrzResolvedSlide::resolve_slide(
                self.file,
                self.font_system.lock().deref_mut(),
                &egui_ctx,
                index,
            )
            .wrap_err_with(|| format!("Slide index {} doesn't exist or contained errors", index))?;
            fonts_to_ft(
                &mut self.used_faces,
                Arc::clone(&self.font_system),
                resolved_slide.fonts_used(),
                &self.ft,
            );
            let mut tags = Vec::new();
            let output = egui_ctx.run(input.clone(), |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    resolved_slide.draw(
                        Rect::from_min_size(Pos2::ZERO, size),
                        ui,
                        f64::MAX,
                        &EaseOutCubic,
                        &mut text_buffers,
                        Some(&mut tags),
                    )
                });
            });

            cairo_draw(
                output,
                &mut textures,
                &cairo_ctx,
                &self.ft,
                Arc::clone(&self.font_system),
                &mut self.used_faces,
                text_buffers,
                tags,
            );

            cairo_ctx.show_page().unwrap();

            if !image_data.is_empty() {
                cairo_ctx.target().finish();
                image_data.chunks_mut(4).for_each(|chunk| {
                    chunk.swap(0, 2);
                });
                image::save_buffer(
                    &out_path,
                    &image_data,
                    size.x as u32,
                    size.y as u32,
                    image::ColorType::Rgba8,
                )
                .wrap_err_with(|| {
                    format!(
                        "Error saving image {} with size {:?}",
                        out_path.as_ref().display(),
                        size
                    )
                })?;
                image_data.iter_mut().for_each(|n| *n = 0);
                let stride = cairo::Format::ARgb32
                    .stride_for_width(size.x as u32)
                    .unwrap();
                let image_surface = unsafe {
                    ImageSurface::create_for_data_unsafe(
                        image_data.as_mut_ptr(),
                        cairo::Format::ARgb32,
                        size.x as i32,
                        size.y as i32,
                        stride,
                    )
                    .unwrap()
                };
                cairo_ctx = cairo::Context::new(&image_surface).unwrap();
            }
        }
        cairo_ctx.target().finish();

        Ok(())
    }
}

pub fn fonts_to_ft(
    used_faces: &mut HashMap<ID, (freetype::Face, cairo::FontFace)>,
    font_system: Arc<Mutex<FontSystem>>,
    used_fonts: SmallVec<[ID; 8]>,
    ft: &freetype::Library,
) {
    let mut font_system = font_system.lock();
    for f in used_fonts {
        used_faces.entry(f).or_insert_with(|| {
            let data = unsafe { font_system.db_mut().make_shared_face_data(f) }.unwrap();
            let face = ft
                .new_memory_face(Rc::new(data.0.deref().as_ref().to_owned()), data.1 as isize)
                .unwrap();
            let cairo_face = FontFace::create_from_ft(&face).unwrap();
            (face, cairo_face)
        });
    }
}

pub fn cairo_draw(
    output: egui::FullOutput,
    textures: &mut HashMap<TextureId, (ImageSurface, bool)>,
    ctx: &cairo::Context,
    ft: &freetype::Library,
    font_system: Arc<Mutex<FontSystem>>,
    fonts: &mut HashMap<ID, (freetype::Face, cairo::FontFace)>,
    buffers: Vec<BufferWithTextArea>,
    tags: Vec<ResolvedTextTag>,
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
        };

        if let Some(pos) = tex.pos {
            let texture = textures.get_mut(&id).unwrap();

            let ctx = cairo::Context::new(&mut texture.0).unwrap();

            ctx.set_source_surface(&surface, 0.0, 0.0).unwrap();
            ctx.rectangle(
                pos[0] as f64,
                pos[1] as f64,
                surface.width() as f64,
                surface.height() as f64,
            );
            ctx.fill().unwrap();
        } else {
            textures.insert(id, (surface, false));
        }
    }

    ctx.tag_begin("Document", "");

    for shape in output.shapes {
        ctx.reset_clip();

        ctx.rectangle(
            shape.clip_rect.min.x as f64,
            shape.clip_rect.min.y as f64,
            shape.clip_rect.width() as f64,
            shape.clip_rect.height() as f64,
        );

        ctx.clip();

        cairo_draw_shape(
            ctx,
            shape.shape,
            textures,
            ft,
            Arc::clone(&font_system),
            fonts,
        );
    }

    ctx.reset_clip();
    cairo_draw_text(ctx, ft, Arc::clone(&font_system), fonts, buffers, tags);

    ctx.tag_end("Document");

    for id in output.textures_delta.free {
        textures.remove(&id);
    }
}

pub fn cairo_draw_shape(
    ctx: &cairo::Context,
    shape: egui::Shape,
    textures: &mut HashMap<TextureId, (ImageSurface, bool)>,
    ft: &freetype::Library,
    font_system: Arc<Mutex<FontSystem>>,
    fonts: &mut HashMap<ID, (freetype::Face, cairo::FontFace)>,
) {
    match shape {
        egui::Shape::Noop | egui::Shape::Text(_) => {}
        egui::Shape::Vec(shapes) => {
            for shape in shapes {
                cairo_draw_shape(ctx, shape, textures, ft, Arc::clone(&font_system), fonts);
            }
        }
        egui::Shape::LineSegment { points, stroke } => {
            let stroke_color = stroke.color;
            let color = stroke_color.to_normalized_gamma_f32();
            ctx.set_source_rgba(
                color[0] as f64,
                color[1] as f64,
                color[2] as f64,
                color[3] as f64,
            );
            ctx.set_line_width(stroke.width as f64);

            ctx.move_to(points[0].x as f64, points[0].y as f64);
            ctx.line_to(points[1].x as f64, points[1].y as f64);
            ctx.stroke().unwrap();
        }
        egui::Shape::Rect(rect) => {
            /// If someone could explain why cairo's alpha is different
            /// than egui's alpha, I'd love to know
            pub const HIGHLIGHT_COLOR_CAIRO: Color32 = {
                let color = Color32::LIGHT_YELLOW;
                Color32::from_rgba_premultiplied(
                    (color.r() as f32 * 0.5 + 0.5) as u8,
                    (color.g() as f32 * 0.5 + 0.5) as u8,
                    (color.b() as f32 * 0.5 + 0.5) as u8,
                    (color.a() as f32 * 0.5 + 0.5) as u8,
                )
            };

            let fill = if rect.fill == HIGHLIGHT_COLOR_DEFAULT {
                HIGHLIGHT_COLOR_CAIRO
            } else {
                rect.fill
            };
            let color = fill.to_normalized_gamma_f32();

            if let Some(texture) = rect.brush.and_then(|brush| {
                let texture = textures.get_mut(&brush.fill_texture_id)?;
                Some((&mut texture.0, &mut texture.1))
            }) {
                if !*texture.1 {
                    let texture_ctx = cairo::Context::new(&texture.0).unwrap();
                    texture_ctx.set_operator(cairo::Operator::Multiply);
                    texture_ctx.set_source_rgba(
                        color[0] as f64,
                        color[1] as f64,
                        color[2] as f64,
                        color[3] as f64,
                    );
                    texture_ctx.mask_surface(&texture.0, 0.0, 0.0).unwrap();
                    *texture.1 = true;
                }
                ctx.save().unwrap();
                ctx.translate(rect.rect.min.x as f64, rect.rect.min.y as f64);
                let ratio = rect.rect.width() as f64 / texture.0.width() as f64;
                ctx.scale(ratio, ratio);
                ctx.set_source(&SurfacePattern::create(&texture.0)).unwrap();
                ctx.paint().unwrap();
                ctx.restore().unwrap();
            } else {
                ctx.set_source_rgba(
                    color[0] as f64,
                    color[1] as f64,
                    color[2] as f64,
                    color[3] as f64,
                );

                ctx.rectangle(
                    rect.rect.min.x as f64,
                    rect.rect.min.y as f64,
                    rect.rect.width() as f64,
                    rect.rect.height() as f64,
                );

                ctx.fill().unwrap();
            }

            ctx.set_line_width(rect.stroke.width as f64);
            if rect.stroke.width > 0.0 {
                let color = rect.stroke.color.to_normalized_gamma_f32();
                ctx.set_source_rgba(
                    color[0] as f64,
                    color[1] as f64,
                    color[2] as f64,
                    color[3] as f64,
                );

                ctx.rectangle(
                    rect.rect.min.x as f64,
                    rect.rect.min.y as f64,
                    rect.rect.width() as f64,
                    rect.rect.height() as f64,
                );
                ctx.stroke().unwrap();
            }
        }
        egui::Shape::Circle(circle) => {
            let color = circle.fill.to_normalized_gamma_f32();
            ctx.set_source_rgba(
                color[0] as f64,
                color[1] as f64,
                color[2] as f64,
                color[3] as f64,
            );

            ctx.arc(
                circle.center.x as f64,
                circle.center.y as f64,
                circle.radius as f64,
                0.0,
                2.0 * std::f64::consts::PI,
            );
            ctx.fill().unwrap();

            ctx.set_line_width(circle.stroke.width as f64);
            if circle.stroke.width > 0.0 {
                let color = circle.stroke.color.to_normalized_gamma_f32();
                ctx.set_source_rgba(
                    color[0] as f64,
                    color[1] as f64,
                    color[2] as f64,
                    color[3] as f64,
                );

                ctx.arc(
                    circle.center.x as f64,
                    circle.center.y as f64,
                    circle.radius as f64,
                    0.0,
                    2.0 * std::f64::consts::PI,
                );
                ctx.stroke().unwrap();
            }
        }
        egui::Shape::Ellipse(ellipse) => {
            let color = ellipse.fill.to_normalized_gamma_f32();
            ctx.set_source_rgba(
                color[0] as f64,
                color[1] as f64,
                color[2] as f64,
                color[3] as f64,
            );
            let matrix = ctx.matrix();
            ctx.translate(ellipse.center.x as f64, ellipse.center.y as f64);
            ctx.scale(ellipse.radius.x as f64, ellipse.radius.y as f64);
            ctx.translate(-ellipse.center.x as f64, -ellipse.center.y as f64);

            ctx.arc(
                ellipse.center.x as f64,
                ellipse.center.y as f64,
                1.0,
                0.0,
                2.0 * std::f64::consts::PI,
            );
            ctx.set_matrix(matrix);
            ctx.fill().unwrap();

            ctx.set_line_width(ellipse.stroke.width as f64);
            if ellipse.stroke.width > 0.0 {
                let color = ellipse.stroke.color.to_normalized_gamma_f32();
                ctx.set_source_rgba(
                    color[0] as f64,
                    color[1] as f64,
                    color[2] as f64,
                    color[3] as f64,
                );
                let matrix = ctx.matrix();
                ctx.translate(ellipse.center.x as f64, ellipse.center.y as f64);
                ctx.scale(ellipse.radius.x as f64, ellipse.radius.y as f64);
                ctx.translate(-ellipse.center.x as f64, -ellipse.center.y as f64);

                ctx.arc(
                    ellipse.center.x as f64,
                    ellipse.center.y as f64,
                    1.0,
                    0.0,
                    2.0 * std::f64::consts::PI,
                );
                ctx.set_matrix(matrix);
                ctx.stroke().unwrap();
            }
        }
        egui::Shape::Path(_path) => {}
        egui::Shape::Mesh(_mesh) => {}
        egui::Shape::QuadraticBezier(_qb) => {}
        egui::Shape::CubicBezier(_cb) => {}
        egui::Shape::Callback(_cb) => {}
    }
}

fn cairo_draw_text(
    ctx: &cairo::Context,
    ft: &freetype::Library,
    font_system: Arc<Mutex<FontSystem>>,
    fonts: &mut HashMap<ID, (freetype::Face, cairo::FontFace)>,
    buffers: Vec<BufferWithTextArea>,
    tags: Vec<ResolvedTextTag>,
) {
    for tag in tags {
        match tag {
            ResolvedTextTag::SectStart => {
                ctx.tag_begin("Sect", "");
                continue;
            }
            ResolvedTextTag::SectEnd => {
                ctx.tag_end("Sect");
                continue;
            }
            ResolvedTextTag::ListStart => {
                ctx.tag_begin("L", "");
                continue;
            }
            ResolvedTextTag::ListEnd => {
                ctx.tag_end("L");
                continue;
            }
            ResolvedTextTag::ListBodyStart => {
                ctx.tag_begin("LBody", "");
                continue;
            }
            ResolvedTextTag::ListBodyEnd => {
                ctx.tag_end("LBody");
                continue;
            }
            ResolvedTextTag::ListItemStart => {
                ctx.tag_begin("LI", "");
                continue;
            }
            ResolvedTextTag::ListItemEnd => {
                ctx.tag_end("LI");
                continue;
            }
            ResolvedTextTag::BlockquoteStart => {
                ctx.tag_begin("BlockQuote", "");
                continue;
            }
            ResolvedTextTag::BlockquoteEnd => {
                ctx.tag_end("BlockQuote");
                continue;
            }
            ResolvedTextTag::Paragraph(p) => {
                ctx.tag_begin("P", "");
                let buffer = buffers.get(p).unwrap();
                cairo_draw_buffer(ctx, buffer, ft, Arc::clone(&font_system), fonts);
                ctx.tag_end("P");
            }
            ResolvedTextTag::Code(p) => {
                ctx.tag_begin("Code", "");
                let buffer = buffers.get(p).unwrap();
                cairo_draw_buffer(ctx, buffer, ft, Arc::clone(&font_system), fonts);
                ctx.tag_end("Code");
            }
            ResolvedTextTag::Label(lbl) => {
                ctx.tag_begin("Lbl", "");
                let buffer = buffers.get(lbl).unwrap();
                cairo_draw_buffer(ctx, buffer, ft, Arc::clone(&font_system), fonts);
                ctx.tag_end("Lbl");
            }
            ResolvedTextTag::Heading(level, h) => {
                let header_tag = format!("H{}", level);
                ctx.tag_begin(&header_tag, "");
                let buffer = buffers.get(h).unwrap();
                cairo_draw_buffer(ctx, buffer, ft, Arc::clone(&font_system), fonts);
                ctx.tag_end(&header_tag);
            }
            ResolvedTextTag::Untagged(p) => {
                ctx.tag_begin("Figure", "");
                let buffer = buffers.get(p).unwrap();
                cairo_draw_buffer(ctx, buffer, ft, Arc::clone(&font_system), fonts);
                ctx.tag_end("Figure");
            }
        }
    }
}

pub fn cairo_draw_buffer(
    ctx: &cairo::Context,
    buffer: &BufferWithTextArea,
    ft: &freetype::Library,
    font_system: Arc<Mutex<FontSystem>>,
    fonts: &mut HashMap<ID, (freetype::Face, cairo::FontFace)>,
) {
    let buffer_read = buffer.buffer.read();
    ctx.set_font_size((buffer_read.metrics().font_size * buffer.scale) as f64);

    for run in buffer_read.layout_runs() {
        let mut glyphs = run.glyphs.iter().peekable();
        while glyphs.peek().is_some() {
            let color;
            let orig_color;
            let font_id;
            if let Some(glyph) = glyphs.peek() {
                orig_color = glyph.color_opt.unwrap_or(buffer.default_color);
                color = egui_glyphon::cosmic_text::Color::rgba(
                    orig_color.r(),
                    orig_color.g(),
                    orig_color.b(),
                    (buffer.opacity * orig_color.a() as f32) as u8,
                );
                font_id = glyph.font_id;
            } else {
                glyphs.next();
                continue;
            }
            let color_rgba = color.as_rgba();
            let color_rgba = Color32::from_rgba_unmultiplied(
                color_rgba[0],
                color_rgba[1],
                color_rgba[2],
                color_rgba[3],
            )
            .to_normalized_gamma_f32();
            let font = fonts.entry(font_id).or_insert_with(|| {
                let mut font_system = font_system.lock();
                let data = unsafe { font_system.db_mut().make_shared_face_data(font_id) }.unwrap();
                let face = ft
                    .new_memory_face(Rc::new(data.0.deref().as_ref().to_owned()), data.1 as isize)
                    .unwrap();
                let cairo_face = FontFace::create_from_ft(&face).unwrap();
                (face, cairo_face)
            });

            ctx.set_source_rgba(
                color_rgba[0] as f64,
                color_rgba[1] as f64,
                color_rgba[2] as f64,
                color_rgba[3] as f64,
            );
            ctx.set_font_face(&font.1);

            let mut new_glyphs = Vec::new();
            let mut text = String::new();
            let mut clusters = Vec::new();
            while let Some(g) = glyphs.peek() {
                if g.color_opt.unwrap_or(buffer.default_color) != orig_color || g.font_id != font_id
                {
                    break;
                }

                let t = &run.text[g.start..g.end];
                text.push_str(&t);
                clusters.push(TextCluster::new(t.len() as i32, 1));
                let glyph = g.physical((buffer.rect.left(), buffer.rect.top()), buffer.scale);
                let glyph = cairo::Glyph::new(
                    g.glyph_id as _,
                    glyph.x as f64,
                    ((run.line_y * buffer.scale).round() as i32 + glyph.y) as f64,
                );
                new_glyphs.push(glyph);
                glyphs.next();
            }

            ctx.show_text_glyphs(&text, &new_glyphs, &clusters, TextClusterFlags::None)
                .unwrap();
        }
    }
}
