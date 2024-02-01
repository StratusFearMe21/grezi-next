use cairo::freetype;
use cairo::freetype::face::LoadFlag;
use cairo::FontFace;
use cairo::ImageSurface;
use ecolor::Color32;
use eframe::egui;
use eframe::epaint::FontFamily;
use eframe::epaint::TextureId;
use egui::FontDefinitions;
use std::collections::HashMap;

pub fn font_defs_to_ft(
    font_defs: FontDefinitions,
    ft: freetype::Library,
) -> HashMap<FontFamily, (freetype::Face, cairo::FontFace)> {
    font_defs
        .families
        .into_iter()
        .map(|f| {
            let data = font_defs.font_data.get(&f.1[0]).unwrap();
            let face = ft
                .new_memory_face(data.font.clone().into_owned(), data.index as isize)
                .unwrap();
            let cairo_face = FontFace::create_from_ft(&face).unwrap();
            (f.0, (face, cairo_face))
        })
        .collect()
}

pub fn cairo_draw(
    output: egui::FullOutput,
    textures: &mut HashMap<TextureId, ImageSurface>,
    ctx: &cairo::Context,
    fonts: &HashMap<FontFamily, (freetype::Face, cairo::FontFace)>,
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
    fonts: &HashMap<FontFamily, (freetype::Face, cairo::FontFace)>,
) {
    use cairo::{SurfacePattern, TextCluster};

    match shape {
        egui::Shape::Noop => {}
        egui::Shape::Vec(shapes) => {
            for shape in shapes {
                cairo_draw_shape(ctx, shape, textures, fonts);
            }
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
        egui::Shape::Text(text) => {
            let origin = text.pos;

            for row in &text.galley.rows {
                let mut row_iter = row.glyphs.iter().map(|f| f.chr);

                let mut section = row.section_index_at_start;
                let mut next_section = section;

                let mut glyphs_iter = row.glyphs.iter();
                let mut raw_glyphs_iter = row.glyphs.iter().map(|g| g.section_index).peekable();

                let mut chars_in_section = 0;
                while let Some(g) = raw_glyphs_iter.peek().copied() {
                    if g != section {
                        next_section = g;
                        break;
                    } else {
                        raw_glyphs_iter.next();
                        chars_in_section += 1;
                    }
                }

                loop {
                    let layout_section = &text.galley.job.sections[section as usize];

                    let row_str: String = (&mut row_iter).take(chars_in_section).collect();

                    ctx.set_font_size(layout_section.format.font_id.size as f64);
                    let font = fonts.get(&layout_section.format.font_id.family).unwrap();
                    ctx.set_font_face(&font.1);
                    let format_color: palette::Srgba<u8> = palette::cast::from_array(
                        if layout_section.format.color == Color32::PLACEHOLDER {
                            text.fallback_color
                        } else {
                            layout_section.format.color
                        }
                        .to_srgba_unmultiplied(),
                    );
                    let format_color: palette::Srgba<f64> = format_color.into_format();
                    ctx.set_source_rgba(
                        format_color.red,
                        format_color.green,
                        format_color.blue,
                        format_color.alpha,
                    );
                    let glyphs: Vec<_> = (&mut glyphs_iter)
                        .take(chars_in_section)
                        .map(|glyph| {
                            cairo::Glyph::new(
                                font.0.get_char_index(glyph.chr as usize) as u64,
                                origin.x as f64 + glyph.pos.x as f64,
                                origin.y as f64 + glyph.pos.y as f64,
                            )
                        })
                        .collect();

                    ctx.show_text_glyphs(
                        &row_str,
                        &glyphs,
                        &[TextCluster::new(
                            row_str.as_bytes().len() as i32,
                            glyphs.len() as i32,
                        )],
                        cairo::TextClusterFlags::None,
                    )
                    .unwrap();

                    if layout_section.format.underline.width > 0.0
                        && layout_section.format.underline.color.a() > 0
                    {
                        ctx.set_line_width(layout_section.format.underline.width as f64);
                        let underline_color: palette::Srgba<u8> = palette::cast::from_array(
                            if layout_section.format.underline.color == Color32::PLACEHOLDER {
                                text.fallback_color
                            } else {
                                layout_section.format.underline.color
                            }
                            .to_srgba_unmultiplied(),
                        );
                        let underline_color: palette::Srgba<f64> = underline_color.into_format();
                        ctx.set_source_rgba(
                            underline_color.red,
                            underline_color.green,
                            underline_color.blue,
                            underline_color.alpha,
                        );
                        let first_glyph = glyphs.first().unwrap();
                        ctx.move_to(first_glyph.x(), first_glyph.y());
                        let last_glyph = glyphs.last().unwrap();
                        font.0
                            .load_glyph(last_glyph.index() as u32, LoadFlag::NO_SCALE)
                            .unwrap();
                        ctx.line_to(
                            last_glyph.x()
                                + (font.0.glyph().advance().x as f64
                                    * (layout_section.format.font_id.size as f64 / 72.0))
                                    / 16.0,
                            last_glyph.y(),
                        );
                        ctx.stroke().unwrap();
                    }

                    if layout_section.format.strikethrough.width > 0.0
                        && layout_section.format.strikethrough.color.a() > 0
                    {
                        ctx.set_line_width(layout_section.format.strikethrough.width as f64);
                        let strikethrough_color: palette::Srgba<u8> = palette::cast::from_array(
                            if layout_section.format.strikethrough.color == Color32::PLACEHOLDER {
                                text.fallback_color
                            } else {
                                layout_section.format.strikethrough.color
                            }
                            .to_srgba_unmultiplied(),
                        );
                        let strikethrough_color: palette::Srgba<f64> =
                            strikethrough_color.into_format();
                        ctx.set_source_rgba(
                            strikethrough_color.red,
                            strikethrough_color.green,
                            strikethrough_color.blue,
                            strikethrough_color.alpha,
                        );
                        let first_glyph = glyphs.first().unwrap();
                        font.0
                            .load_glyph(first_glyph.index() as u32, LoadFlag::NO_SCALE)
                            .unwrap();
                        let font_plus = ((font.0.glyph().metrics().height as f64
                            * (layout_section.format.font_id.size as f64 / 72.0))
                            / 16.0)
                            / 2.0;
                        let last_glyph = glyphs.last().unwrap();
                        font.0
                            .load_glyph(last_glyph.index() as u32, LoadFlag::NO_SCALE)
                            .unwrap();
                        ctx.move_to(first_glyph.x(), first_glyph.y() - font_plus);
                        ctx.line_to(
                            last_glyph.x()
                                + (font.0.glyph().advance().x as f64
                                    * (layout_section.format.font_id.size as f64 / 72.0))
                                    / 16.0,
                            last_glyph.y() - font_plus,
                        );
                        ctx.stroke().unwrap();
                    }

                    section = next_section;
                    if raw_glyphs_iter.next().is_none() {
                        break;
                    }
                    chars_in_section = 1;
                    while let Some(g) = raw_glyphs_iter.peek().copied() {
                        if g != section {
                            next_section = g;
                            break;
                        } else {
                            raw_glyphs_iter.next();
                            chars_in_section += 1;
                        }
                    }
                }
            }
        }
        // egui::Shape::Circle(circle) => {
        //     let matrix = ctx.matrix();
        //     let color = circle.fill.to_srgba_unmultiplied();
        //     ctx.set_source_rgba(
        //         color[0] as f64 / 255.0,
        //         color[1] as f64 / 255.0,
        //         color[2] as f64 / 255.0,
        //         color[3] as f64 / 255.0,
        //     );

        //     ctx.move_to(circle.center.x as f64, circle.center.y as f64);
        //     ctx.scale(, )
        // }
        _ => {}
    }
}
