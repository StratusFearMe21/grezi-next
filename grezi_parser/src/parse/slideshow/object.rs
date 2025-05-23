use std::{borrow::Cow, io, num::NonZeroU16, ops::DerefMut, str::FromStr, sync::Arc};

use css_color::Srgb;
use ecolor::Color32;
use ropey::RopeSlice;
use tracing::instrument;
use tree_sitter_grz::{FieldName, NodeKind};
use url::Url;

use crate::{
    object::{ObjInner, Object},
    parse::{
        cursor::{ErrorInfo, GrzCursor, GrzCursorGuard},
        error::{ErrsWithSource, ParseError},
        CharRange,
    },
    text::Align,
};

use super::{
    registers::Registers,
    text::{StringLiteral, TextJobParams},
};

impl Object {
    /// Parse name before entering this function
    #[instrument(skip_all, fields(source = %cursor.parent_source()?, type = std::any::type_name::<Self>()))]
    pub fn parse(
        &mut self,
        mut cursor: GrzCursorGuard,
        path_to_grz: &str,
        errors: Arc<ErrsWithSource>,
    ) -> io::Result<()> {
        if let Some(obj_inner_cursor) = cursor.goto_first_child(NodeKind::SymObjInner)? {
            self.parameters
                .parse(obj_inner_cursor, path_to_grz, Arc::clone(&errors))?;
        } else {
            errors.append_error(
                ParseError::Missing(cursor.char_range()?, "Missing inner details of object"),
                cursor.error_info(),
            );
        }

        Ok(())
    }

    pub fn apply_registers(&mut self, _registers: &Registers) {}
}

impl ObjInner {
    #[instrument(skip_all, fields(source = %cursor.parent_source()?, type = std::any::type_name::<Self>()))]
    pub fn parse(
        &mut self,
        mut cursor: GrzCursorGuard,
        path_to_grz: &str,
        errors: Arc<ErrsWithSource>,
    ) -> io::Result<()> {
        let obj_type = cursor.rope_slice()?;
        let mut obj_params = ObjParamParser::new(cursor.deref_mut())?;
        match obj_type {
            x if x == "Rect" => {
                let mut color = Color32::WHITE;
                let mut stroke = Color32::TRANSPARENT;
                let mut height = 0.0;
                while let Some(param) = obj_params.next() {
                    let param = param?;

                    match param.0 {
                        x if x.map(|x| x == "color").unwrap_or_default() => {
                            let color_str: Cow<'_, str> = param.1.into();
                            color = parse_color(
                                color_str.as_ref(),
                                obj_params.char_range(),
                                obj_params.error_info(),
                                Arc::clone(&errors),
                            )?;
                        }
                        x if x.map(|x| x == "stroke").unwrap_or_default() => {
                            let stroke_str: Cow<'_, str> = param.1.into();
                            stroke = parse_color(
                                stroke_str.as_ref(),
                                obj_params.char_range(),
                                obj_params.error_info(),
                                Arc::clone(&errors),
                            )?;
                        }
                        x if x.map(|x| x == "height").unwrap_or_default() || x.is_none() => {
                            let height_str: Cow<'_, str> = param.1.into();
                            match height_str.parse() {
                                Ok(c) => height = c,
                                Err(_) => errors.append_error(
                                    ParseError::Syntax(
                                        obj_params.char_range(),
                                        "Not a valid float",
                                    ),
                                    obj_params.error_info(),
                                ),
                            }
                        }
                        _ => {}
                    }
                }

                *self = Self::Rect {
                    color,
                    stroke,
                    height,
                };
            }
            x if x == "Image" => {
                let mut tint = Color32::WHITE;
                let mut scale = None;
                let mut url = Url::parse("file:/bruh/moment").unwrap();
                let mut bytes = None;
                while let Some(param) = obj_params.next() {
                    let param = param?;

                    match param.0 {
                        x if x.map(|x| x == "tint").unwrap_or_default() => {
                            let tint_str: Cow<'_, str> = param.1.into();
                            tint = parse_color(
                                tint_str.as_ref(),
                                obj_params.char_range(),
                                obj_params.error_info(),
                                Arc::clone(&errors),
                            )?;
                        }
                        x if x.map(|x| x == "scale").unwrap_or_default() => {
                            let scale_str: Cow<'_, str> = param.1.into();
                            match scale_str.parse() {
                                Ok(c) => scale = Some(c),
                                Err(_) => errors.append_error(
                                    ParseError::Syntax(
                                        obj_params.char_range(),
                                        "Not a valid float",
                                    ),
                                    obj_params.error_info(),
                                ),
                            }
                        }
                        x if x.map(|x| x == "value").unwrap_or_default() || x.is_none() => {
                            let url_str: Cow<'_, str> = param.1.into();
                            match dunce::canonicalize(path_to_grz)
                                .or(Err(()))
                                .and_then(|path| Url::from_file_path(path))
                                .and_then(|u| u.join(url_str.as_ref()).or(Err(())))
                            {
                                Ok(u) => url = u,
                                Err(_) => errors.append_error(
                                    ParseError::Syntax(obj_params.char_range(), "Not a valid URL"),
                                    obj_params.error_info(),
                                ),
                            }

                            bytes = std::fs::read(url.path()).ok().map(Arc::<[u8]>::from)
                        }
                        _ => {}
                    }
                }

                if let Some(data) = bytes {
                    *self = Self::Image {
                        url,
                        scale,
                        tint,
                        data,
                    };
                } else {
                    errors.append_error(
                        ParseError::NotFound(
                            cursor.char_range()?,
                            "Could not find value parameter, or image specified in value parameter",
                        ),
                        cursor.error_info(),
                    );
                }
            }
            x if x == "Header" || x == "Paragraph" => {
                let mut line_height = None;
                let mut align = Align::Left;
                let mut text_job_params = TextJobParams {
                    default_font_size: match x {
                        x if x == "Header" => 64.0,
                        _ => 48.0,
                    },
                    ..Default::default()
                };

                while let Some(param) = obj_params.next() {
                    let param = param?;

                    match param.0 {
                        x if x.map(|x| x == "value").unwrap_or_default()
                            || x.map(|x| x == "code").unwrap_or_default()
                            || x.is_none() =>
                        {
                            text_job_params.value = param.1
                        }
                        x if x.map(|x| x == "language").unwrap_or_default() => {
                            text_job_params.language = param.1
                        }
                        x if x.map(|x| x == "align").unwrap_or_default() => {
                            align = match param.1.as_rope_slice() {
                                x if x == "left" => Align::Left,
                                x if x == "center" => Align::Center,
                                x if x == "right" => Align::Right,
                                x if x == "justified" => Align::Justified,
                                x if x == "end" => Align::End,
                                _ => {
                                    errors.append_error(
                                        ParseError::NotFound(
                                            obj_params.char_range(),
                                            "That alignment does not exist",
                                        ),
                                        obj_params.error_info(),
                                    );
                                    Align::Left
                                }
                            }
                        }
                        x if x.map(|x| x == "color").unwrap_or_default() => {
                            let color_str: Cow<'_, str> = param.1.into();
                            text_job_params.default_attrs.color = parse_color(
                                color_str.as_ref(),
                                obj_params.char_range(),
                                obj_params.error_info(),
                                Arc::clone(&errors),
                            )?;
                        }
                        x if x.map(|x| x == "font_family").unwrap_or_default() => {
                            let family: Cow<'_, str> = param.1.into();
                            text_job_params.default_attrs.apply_fontstr(family.as_ref());
                        }
                        x if x.map(|x| x == "font_size").unwrap_or_default() => {
                            let font_size_str: Cow<'_, str> = param.1.into();
                            match font_size_str.parse() {
                                Ok(c) => text_job_params.default_font_size = c,
                                Err(_) => errors.append_error(
                                    ParseError::Syntax(
                                        obj_params.char_range(),
                                        "Not a valid float",
                                    ),
                                    obj_params.error_info(),
                                ),
                            }
                        }
                        x if x.map(|x| x == "tagged").unwrap_or_default() => {
                            let tagged_str: Cow<'_, str> = param.1.into();
                            match tagged_str.parse() {
                                Ok(c) => text_job_params.tagged = c,
                                Err(_) => errors.append_error(
                                    ParseError::Syntax(
                                        obj_params.char_range(),
                                        "Not a valid boolean",
                                    ),
                                    obj_params.error_info(),
                                ),
                            }
                        }
                        x if x.map(|x| x == "line_height").unwrap_or_default() => {
                            let line_height_str: Cow<'_, str> = param.1.into();
                            match line_height_str.parse() {
                                Ok(c) => {
                                    if c <= 0.0 {
                                        errors.append_error(
                                            ParseError::Syntax(
                                                obj_params.char_range(),
                                                "Line height must be greater than 0",
                                            ),
                                            obj_params.error_info(),
                                        );
                                    } else {
                                        line_height = Some(c)
                                    }
                                }
                                Err(_) => errors.append_error(
                                    ParseError::Syntax(
                                        obj_params.char_range(),
                                        "Not a valid float",
                                    ),
                                    obj_params.error_info(),
                                ),
                            }
                        }
                        _ => {}
                    }
                }

                *self = Self::Text {
                    job: text_job_params.into(),
                    line_height,
                    align,
                }
            }
            _ => errors.append_error(
                ParseError::NotFound(cursor.char_range()?, "That object type does not exist"),
                cursor.error_info(),
            ),
        }
        Ok(())
    }
}

pub struct ObjParamParser<'a, 'b> {
    cursor: &'b mut GrzCursor<'a>,
    char_range: CharRange,
    span: Option<tracing::span::EnteredSpan>,
}

impl<'a, 'b> ObjParamParser<'a, 'b> {
    pub fn new(cursor: &'b mut GrzCursor<'a>) -> io::Result<ObjParamParser<'a, 'b>> {
        Ok(Self {
            char_range: cursor.char_range()?,
            cursor,
            span: None,
        })
    }

    pub fn char_range(&self) -> CharRange {
        self.char_range.clone()
    }

    pub fn error_info(&self) -> ErrorInfo {
        self.cursor.error_info()
    }
}

impl<'a> Iterator for ObjParamParser<'a, '_> {
    type Item = io::Result<(Option<RopeSlice<'a>>, StringLiteral<'a>)>;

    fn next(&mut self) -> Option<Self::Item> {
        self.span = None;
        match self.cursor.goto_next_sibling() {
            Ok(result) => {
                if !result {
                    return None;
                }
            }
            Err(e) => return Some(Err(e)),
        }
        self.span = Some(
            tracing::info_span!("next",
                source = %self.cursor.rope_slice().unwrap_or_else(|_| RopeSlice::from("")),
                type = std::any::type_name::<Self>()
            )
            .entered(),
        );
        match self.cursor.goto_first_child(NodeKind::SymObjParam) {
            Ok(c) => match parse_obj_param(c?) {
                Ok(params) => {
                    self.char_range = params.0;
                    Some(Ok(params.1))
                }
                Err(e) => Some(Err(e)),
            },
            Err(e) => Some(Err(e)),
        }
    }
}

#[instrument(skip_all, fields(source = %cursor.parent_source()?))]
pub fn parse_obj_param<'a>(
    mut cursor: GrzCursorGuard<'a, '_>,
) -> io::Result<(CharRange, (Option<RopeSlice<'a>>, StringLiteral<'a>))> {
    let key = if cursor.field_id() == NonZeroU16::new(FieldName::FieldKey as u16) {
        let key = cursor.rope_slice()?;
        cursor.goto_next_sibling()?;
        Some(key)
    } else {
        None
    };
    let char_range = cursor.char_range()?;
    let value = cursor.node_to_string_literal()?;
    Ok((char_range, (key, value)))
}

#[instrument(skip_all, fields(source = color))]
pub fn parse_color_raw<'a>(
    color: &str,
    mut range: CharRange,
    error_info: ErrorInfo,
    errors: Arc<ErrsWithSource>,
) -> io::Result<Srgb> {
    match Srgb::from_str(color) {
        Ok(c) => Ok(c),
        Err(e) => {
            range.byte_range.start += e.span.start;
            range.byte_range.end = range.byte_range.start + e.span.len();
            // The parser doesn't support unicode
            // so this is all we can do
            range.start_character += e.span.start;
            range.end_character = range.start_character + e.span.len();
            errors.append_error(
                ParseError::ColorSyntax {
                    range,
                    error: e.inner_error,
                    expected_none: e.expected_none,
                },
                error_info,
            );
            Ok(Srgb {
                red: 0.0,
                green: 0.0,
                blue: 0.0,
                alpha: 0.0,
            })
        }
    }
}

#[instrument(skip_all, fields(source = color))]
pub fn parse_color<'a>(
    color: &str,
    range: CharRange,
    error_info: ErrorInfo,
    errors: Arc<ErrsWithSource>,
) -> io::Result<Color32> {
    let c = parse_color_raw(color, range, error_info, errors)?;
    Ok(Color32::from_rgba_unmultiplied(
        (c.red * 255.0) as u8,
        (c.green * 255.0) as u8,
        (c.blue * 255.0) as u8,
        (c.alpha * 255.0) as u8,
    ))
}
