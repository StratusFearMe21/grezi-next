use egui::{epaint::ClippedShape, Color32, Context, ImageData, Pos2, Shape, TextureId};
use tiny_skia::{BlendMode, Color, Mask, PathBuilder, Pattern, Pixmap, PixmapMut, Rect, Transform};

use std::collections::HashMap;

pub fn rasterize(size: (f32, f32), ui: impl FnOnce(&Context)) -> Pixmap {
    let mut backend = TinySkiaBackend::new();

    let input = egui::RawInput {
        screen_rect: Some([Pos2::default(), Pos2::new(size.0, size.1)].into()),
        //pixels_per_point: Some(2.0),
        ..Default::default()
    };

    let output = backend.output_to_pixmap(input, ui);

    // while output.1 {
    //     output = self.output_to_pixmap(input, ui);
    // }

    output.1
}

pub struct TinySkiaBackend {
    ctx: Context,
    textures: HashMap<TextureId, Pixmap>,
    clip_mask: Mask,
}

impl TinySkiaBackend {
    pub fn new() -> Self {
        Self {
            ctx: Context::default(),
            textures: HashMap::new(),
            clip_mask: Mask::new(5, 5).unwrap(),
        }
    }

    pub fn context(&self) -> &Context {
        &self.ctx
    }

    pub fn output_to_pixmap(
        &mut self,
        input: egui::RawInput,
        ui: impl FnOnce(&Context),
    ) -> (egui::output::PlatformOutput, Pixmap) {
        use egui::output::FullOutput;

        let rect = input
            .screen_rect
            .unwrap_or_else(|| self.ctx.input(|i| i.screen_rect));
        let scale = input
            .pixels_per_point
            .unwrap_or_else(|| self.ctx.input(|i| i.pixels_per_point));

        let mut pixmap =
            Pixmap::new((rect.max.x * scale) as u32, (rect.max.y * scale) as u32).unwrap();

        let FullOutput {
            platform_output,
            textures_delta,
            shapes,
            ..
        } = self.ctx.run(input, ui);

        for (id, tex) in textures_delta.set {
            let pixmap = data_to_pixmap(&tex.image);

            if let Some(pos) = tex.pos {
                let texture = self.textures.get_mut(&id).unwrap();
                texture.draw_pixmap(
                    pos[0] as i32,
                    pos[1] as i32,
                    pixmap.as_ref(),
                    &Default::default(),
                    tiny_skia::Transform::identity(),
                    None,
                );
            } else {
                self.textures.insert(id, pixmap);
            }
        }

        for shape in shapes {
            // TODO: set clip rect
            self.draw_shape(&mut pixmap.as_mut(), shape, scale)
        }

        for id in textures_delta.free {
            self.textures.remove(&id);
        }

        (platform_output, pixmap)
    }

    fn draw_shape(&mut self, pixmap: &mut PixmapMut, shape: ClippedShape, scale: f32) {
        let mut clip = PathBuilder::new();
        clip.push_rect(
            tiny_skia::Rect::from_ltrb(
                shape.clip_rect.left(),
                shape.clip_rect.top(),
                shape.clip_rect.right(),
                shape.clip_rect.bottom(),
            )
            .unwrap(),
        );
        if let Some(clip) = clip.finish() {
            self.clip_mask =
                Mask::new(clip.bounds().width() as u32, clip.bounds().height() as u32).unwrap();
            self.clip_mask.fill_path(
                &clip,
                tiny_skia::FillRule::EvenOdd,
                false,
                Default::default(),
            );
        }

        match shape.shape {
            Shape::Noop => {}
            Shape::Vec(v) => {
                for inner_shape in v {
                    self.draw_shape(
                        pixmap,
                        ClippedShape {
                            clip_rect: shape.clip_rect,
                            shape: inner_shape,
                        },
                        scale,
                    );
                }
            }
            Shape::Mesh(mesh) => {
                // TODO
                // println!("skipping mesh... ({} vertices)", mesh.vertices.len());
                let mut tris = mesh.indices.chunks(3);
                while let Some(&[a, b, c]) = tris.next() {
                    let mut path = PathBuilder::new();
                    let a = mesh.vertices[a as usize];
                    let b = mesh.vertices[b as usize];
                    let c = mesh.vertices[c as usize];
                    path.move_to(a.pos.x, a.pos.y);
                    path.line_to(b.pos.x, b.pos.y);
                    path.line_to(c.pos.x, c.pos.y);
                    path.close();

                    self.draw_path(
                        &self.clip_mask,
                        pixmap,
                        Some(mesh.texture_id),
                        path,
                        Some(a.color),
                        None,
                    );
                }
            }
            Shape::Rect(rect) => {
                let mut path = PathBuilder::new();
                if rect.rounding == egui::epaint::Rounding::default() {
                    path.push_rect(
                        Rect::from_ltrb(
                            rect.rect.left(),
                            rect.rect.top(),
                            rect.rect.right(),
                            rect.rect.bottom(),
                        )
                        .unwrap(),
                    );
                } else {
                    let r = rect.rounding;
                    let rect = rect.rect;

                    path.move_to(rect.left(), rect.top() + r.nw);
                    path.quad_to(rect.left(), rect.top(), rect.left() + r.nw, rect.top());
                    path.line_to(rect.right() - r.ne, rect.top());
                    path.quad_to(rect.right(), rect.top(), rect.right(), rect.top() + r.ne);
                    path.line_to(rect.right(), rect.bottom() - r.se);
                    path.quad_to(
                        rect.right(),
                        rect.bottom(),
                        rect.right() - r.se,
                        rect.bottom(),
                    );
                    path.line_to(rect.left() + r.sw, rect.bottom());
                    path.quad_to(
                        rect.left(),
                        rect.bottom(),
                        rect.left(),
                        rect.bottom() - r.sw,
                    );
                    path.close();
                }

                self.draw_path(
                    &self.clip_mask,
                    pixmap,
                    Some(rect.fill_texture_id),
                    path,
                    Some(rect.fill),
                    Some(rect.stroke),
                );
            }
            Shape::LineSegment { points, stroke } => {
                let mut path = PathBuilder::new();
                path.move_to(points[0].x, points[0].y);
                path.line_to(points[1].x, points[1].y);

                self.draw_path(&self.clip_mask, pixmap, None, path, None, Some(stroke));
            }
            Shape::Circle(circle) => {
                let mut path = PathBuilder::new();
                path.push_oval(
                    tiny_skia::Rect::from_ltrb(
                        circle.center.x - circle.radius,
                        circle.center.y - circle.radius,
                        circle.center.x + circle.radius,
                        circle.center.y + circle.radius,
                    )
                    .unwrap(),
                );

                self.draw_path(
                    &self.clip_mask,
                    pixmap,
                    None,
                    path,
                    Some(circle.fill),
                    Some(circle.stroke),
                );
            }
            Shape::Path(path_shape) => {
                if path_shape.points.is_empty() {
                    return;
                }
                let mut path = PathBuilder::new();
                path.move_to(path_shape.points[0].x, path_shape.points[0].y);
                for p in &path_shape.points[1..] {
                    path.line_to(p.x, p.y);
                }
                if path_shape.closed {
                    path.close();
                }

                self.draw_path(
                    &self.clip_mask,
                    pixmap,
                    None,
                    path,
                    Some(path_shape.fill),
                    Some(path_shape.stroke),
                );
            }
            Shape::Text(ts) => {
                let font_pixmap = self.textures.get(&TextureId::Managed(0)).unwrap();

                //println!("{:?}", ts.pos);
                let origin = ts.pos;

                for row in &ts.galley.rows {
                    //println!("row: {:?}", row.rect);

                    for g in &row.glyphs {
                        let mut path = PathBuilder::new();
                        //println!("- glyph {} {:?} {:?} {:?}", g.chr, g.pos, g.size, g.uv_rect);
                        path.push_rect(
                            Rect::from_xywh(
                                origin.x + g.pos.x + g.uv_rect.offset.x + 0.1,
                                origin.y + g.pos.y + g.uv_rect.offset.y - 0.1,
                                g.uv_rect.size.x,
                                g.uv_rect.size.y,
                            )
                            .unwrap(),
                        );

                        let path = path.finish().unwrap();

                        let uv = tiny_skia::IntRect::from_ltrb(
                            g.uv_rect.min[0] as i32,
                            g.uv_rect.min[1] as i32,
                            g.uv_rect.max[0] as i32,
                            g.uv_rect.max[1] as i32,
                        );
                        if uv.is_none() {
                            continue;
                        }
                        let uv = uv.unwrap();
                        let mut glyph_pixmap = font_pixmap.clone_rect(uv).unwrap();
                        let format = &ts.galley.job.sections[g.section_index as usize].format;
                        let color = if let Some(color) = ts.override_text_color {
                            color
                        } else {
                            format.color
                        };

                        let rect = tiny_skia::Rect::from_xywh(
                            0.0,
                            0.0,
                            glyph_pixmap.width() as f32,
                            glyph_pixmap.height() as f32,
                        )
                        .unwrap();

                        glyph_pixmap.fill_rect(
                            rect,
                            &tiny_skia::Paint {
                                shader: tiny_skia::Shader::SolidColor(Color::from_rgba8(
                                    color.r(),
                                    color.g(),
                                    color.b(),
                                    color.a(),
                                )),
                                blend_mode: tiny_skia::BlendMode::SourceIn,
                                ..Default::default()
                            },
                            tiny_skia::Transform::identity(),
                            None,
                        );

                        let fill_shader = Pattern::new(
                            glyph_pixmap.as_ref(),
                            tiny_skia::SpreadMode::Pad,
                            tiny_skia::FilterQuality::Bicubic,
                            1.0,
                            tiny_skia::Transform::from_translate(
                                origin.x + g.pos.x + g.uv_rect.offset.x,
                                origin.y + g.pos.y + g.uv_rect.offset.y,
                            )
                            .pre_scale(1.0 / scale, 1.0 / scale),
                        );

                        pixmap.fill_path(
                            &path,
                            &tiny_skia::Paint {
                                shader: fill_shader,
                                anti_alias: true,
                                force_hq_pipeline: true,
                                ..Default::default()
                            },
                            tiny_skia::FillRule::EvenOdd,
                            tiny_skia::Transform::identity(),
                            Some(&self.clip_mask),
                        );
                    }
                }
            }
            Shape::QuadraticBezier(qb) => {
                let mut path = PathBuilder::new();
                path.move_to(qb.points[0].x, qb.points[0].y);
                path.quad_to(
                    qb.points[1].x,
                    qb.points[1].y,
                    qb.points[2].x,
                    qb.points[2].y,
                );

                if qb.closed {
                    path.close();
                }

                self.draw_path(
                    &self.clip_mask,
                    pixmap,
                    None,
                    path,
                    Some(qb.fill),
                    Some(qb.stroke),
                );
            }
            Shape::CubicBezier(cb) => {
                let mut path = PathBuilder::new();
                path.move_to(cb.points[0].x, cb.points[0].y);
                path.cubic_to(
                    cb.points[1].x,
                    cb.points[1].y,
                    cb.points[2].x,
                    cb.points[2].y,
                    cb.points[3].x,
                    cb.points[3].y,
                );

                if cb.closed {
                    path.close();
                }

                self.draw_path(
                    &self.clip_mask,
                    pixmap,
                    None,
                    path,
                    Some(cb.fill),
                    Some(cb.stroke),
                );
            }
            Shape::Callback(_) => unimplemented!(),
        }
    }

    fn draw_path(
        &self,
        mask: &Mask,
        pixmap: &mut PixmapMut,
        texture: Option<TextureId>,
        path: PathBuilder,
        fill: Option<Color32>,
        stroke: Option<egui::epaint::Stroke>,
    ) {
        let path = path.finish().unwrap();

        if let Some(fill) = fill {
            let fill_shader = tiny_skia::Shader::SolidColor(Color::from_rgba8(
                fill.r(),
                fill.g(),
                fill.b(),
                fill.a(),
            ));

            pixmap.fill_path(
                &path,
                &tiny_skia::Paint {
                    shader: fill_shader,
                    anti_alias: false,
                    ..Default::default()
                },
                tiny_skia::FillRule::EvenOdd,
                tiny_skia::Transform::identity(),
                Some(&mask),
            );
            //.unwrap();
        }

        if let Some(texture) = texture {
            let texture = self.textures.get(&texture).unwrap().as_ref();
            let shader = Pattern::new(
                texture,
                tiny_skia::SpreadMode::Pad,
                tiny_skia::FilterQuality::Bilinear,
                1.0,
                // Transform::default(),
                Transform::from_translate(path.bounds().x(), path.bounds().y()).pre_scale(
                    path.bounds().width() / texture.width() as f32,
                    path.bounds().height() / texture.height() as f32,
                ),
            );
            pixmap.fill_path(
                &path,
                &tiny_skia::Paint {
                    shader,
                    blend_mode: if fill.is_some() {
                        BlendMode::Multiply
                    } else {
                        BlendMode::Source
                    },
                    anti_alias: false,
                    force_hq_pipeline: true,
                },
                tiny_skia::FillRule::EvenOdd,
                Transform::default(),
                Some(&mask),
            );
        }

        if let Some(stroke) = stroke {
            let stroke_color = stroke.color;
            let stroke_shader = tiny_skia::Shader::SolidColor(Color::from_rgba8(
                stroke_color.r(),
                stroke_color.g(),
                stroke_color.b(),
                stroke_color.a(),
            ));

            let sw = if stroke.width == 0.0 {
                0.0001
            } else {
                stroke.width
            };

            //println!("{:?}", path);

            pixmap.stroke_path(
                &path,
                &tiny_skia::Paint {
                    shader: stroke_shader,
                    anti_alias: true,
                    ..Default::default()
                },
                &tiny_skia::Stroke {
                    width: sw,
                    ..Default::default()
                },
                tiny_skia::Transform::identity(),
                Some(&mask),
            );
            //.unwrap();
        }
    }
}

fn data_to_pixmap(data: &ImageData) -> Pixmap {
    let mut image_data: Vec<u8> = match data {
        ImageData::Color(c) => {
            // println!("Image");
            let pixels: Vec<u8> = c
                .pixels
                .iter()
                .flat_map(|c| [c[0], c[1], c[2], c[3]])
                .collect();

            // image::save_buffer(
            //     format!(
            //         "{}.png",
            //         u32::from_ne_bytes((&pixels[..4]).try_into().unwrap())
            //     ),
            //     &pixels,
            //     c.size[0] as u32,
            //     c.size[1] as u32,
            //     image::ColorType::Rgba8,
            // )
            // .unwrap();
            pixels
        }
        ImageData::Font(f) => f
            .srgba_pixels(Some(1.0 / 1.8))
            .flat_map(|c| [c[0], c[1], c[2], c[3]])
            .collect(),
    };

    PixmapMut::from_bytes(&mut image_data, data.width() as u32, data.height() as u32)
        .unwrap()
        .to_owned()
}
