use cairo::freetype::freetype;
use cairo::FontFace;
use cairo::ImageSurface;
use eframe::egui;
use eframe::epaint::FontFamily;
use eframe::epaint::TextureId;
use egui::FontDefinitions;
use libc::{c_long, size_t};
use std::collections::HashMap;
use std::ffi::c_void;

pub fn font_defs_to_ft(
    font_defs: FontDefinitions,
    ft: cairo::freetype::freetype::FT_Library,
) -> HashMap<FontFamily, (cairo::freetype::freetype::FT_Face, cairo::FontFace)> {
    font_defs
        .families
        .into_iter()
        .map(|f| {
            let data = font_defs.font_data.get(&f.1[0]).unwrap();
            let mut face = std::ptr::null_mut();
            unsafe {
                freetype::FT_New_Memory_Face(
                    ft,
                    data.font.as_ptr(),
                    data.font.len() as i64,
                    data.index as i64,
                    &mut face,
                );
            }
            (
                f.0,
                (face, unsafe { FontFace::create_from_ft(face).unwrap() }),
            )
        })
        .collect()
}

pub fn new_ft() -> cairo::freetype::freetype::FT_Library {
    extern "C" fn alloc_library(_memory: freetype::FT_Memory, size: c_long) -> *mut c_void {
        unsafe { libc::malloc(size as size_t) }
    }

    extern "C" fn free_library(_memory: freetype::FT_Memory, block: *mut c_void) {
        unsafe { libc::free(block) }
    }

    extern "C" fn realloc_library(
        _memory: freetype::FT_Memory,
        _cur_size: c_long,
        new_size: c_long,
        block: *mut c_void,
    ) -> *mut c_void {
        unsafe { libc::realloc(block, new_size as size_t) }
    }

    static mut MEMORY: freetype::FT_MemoryRec_ = freetype::FT_MemoryRec_ {
        user: 0 as *mut c_void,
        alloc: Some(alloc_library),
        free: Some(free_library),
        realloc: Some(realloc_library),
    };

    let mut ft = core::ptr::null_mut();

    unsafe {
        freetype::FT_New_Library(&mut MEMORY, &mut ft);
        freetype::FT_Add_Default_Modules(ft);
    }

    ft
}

pub fn cairo_draw(
    output: egui::FullOutput,
    textures: &mut HashMap<TextureId, ImageSurface>,
    ctx: &cairo::Context,
    fonts: &HashMap<FontFamily, (cairo::freetype::freetype::FT_Face, cairo::FontFace)>,
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
    fonts: &HashMap<FontFamily, (cairo::freetype::freetype::FT_Face, cairo::FontFace)>,
) {
    use cairo::{freetype::freetype::FT_Get_Char_Index, SurfacePattern, TextCluster};

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
                    let ref layout_section = text.galley.job.sections[section as usize];

                    let row_str: String = (&mut row_iter).take(chars_in_section).collect();

                    ctx.set_font_size(layout_section.format.font_id.size as f64);
                    let font = fonts.get(&layout_section.format.font_id.family).unwrap();
                    ctx.set_font_face(&font.1);
                    ctx.set_source_rgba(
                        layout_section.format.color.r() as f64 / 255.0,
                        layout_section.format.color.g() as f64 / 255.0,
                        layout_section.format.color.b() as f64 / 255.0,
                        layout_section.format.color.a() as f64 / 255.0,
                    );
                    let glyphs: Vec<_> = (&mut glyphs_iter)
                        .take(chars_in_section)
                        .map(|glyph| cairo::Glyph {
                            index: unsafe { FT_Get_Char_Index(font.0, glyph.chr as u64) as u64 },
                            x: origin.x as f64 + glyph.pos.x as f64,
                            y: origin.y as f64 + glyph.pos.y as f64,
                        })
                        .collect();

                    ctx.show_text_glyphs(
                        &row_str,
                        &glyphs,
                        &[TextCluster {
                            num_bytes: row_str.as_bytes().len() as i32,
                            num_glyphs: glyphs.len() as i32,
                        }],
                        cairo::TextClusterFlags::None,
                    )
                    .unwrap();
                    section = next_section;
                    if (&mut raw_glyphs_iter).next().is_none() {
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
