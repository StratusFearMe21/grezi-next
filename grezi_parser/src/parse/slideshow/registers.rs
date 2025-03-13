use std::{borrow::Cow, io, sync::Arc};

use grezi_layout::Flex;
use smart_default::SmartDefault;
use tracing::instrument;

use crate::{
    parse::{
        cursor::GrzCursorGuardRaw,
        error::{ErrsWithSource, ParseError},
    },
    slide::BgColor,
};

use super::object::{parse_color_raw, ObjParamParser};

#[derive(SmartDefault)]
pub struct Registers {
    #[default = 15.0]
    pub margin: f32,
    pub margin_per: f32,
    #[default(Flex::Legacy)]
    pub flex: Flex,
    pub bg: BgColor,
    pub create_edges: bool,
}

impl Registers {
    #[instrument(skip_all, fields(source = %cursor.parent_source()?, type = std::any::type_name::<Self>()))]
    pub fn parse(
        &mut self,
        mut cursor: GrzCursorGuardRaw,
        errors: Arc<ErrsWithSource>,
    ) -> io::Result<()> {
        let mut obj_params = ObjParamParser::new(&mut cursor)?;
        if let Some(param) = obj_params.next() {
            let param = param?;

            match param.0 {
                x if x == "MARGIN" => {
                    let margin_str: Cow<'_, str> = param.1.into();
                    match margin_str.parse() {
                        Ok(c) => self.margin = c,
                        Err(_) => errors.append_error(
                            ParseError::Syntax(obj_params.char_range(), "Not a valid float"),
                            obj_params.error_info(),
                        ),
                    }
                }
                x if x == "MARGIN_PER" => {
                    let margin_per_str: Cow<'_, str> = param.1.into();
                    match margin_per_str.parse() {
                        Ok(c) => self.margin_per = c,
                        Err(_) => errors.append_error(
                            ParseError::Syntax(obj_params.char_range(), "Not a valid float"),
                            obj_params.error_info(),
                        ),
                    }
                }
                x if x == "BACKGROUND" => {
                    let bg_str: Cow<'_, str> = param.1.into();
                    let bg = parse_color_raw(
                        bg_str.as_ref(),
                        obj_params.char_range(),
                        obj_params.error_info(),
                        Arc::clone(&errors),
                    )?;
                    let oklab_bg = oklab::srgb_f32_to_oklab(oklab::Rgb {
                        r: bg.red,
                        g: bg.green,
                        b: bg.blue,
                    });
                    self.bg = BgColor {
                        bg_l: oklab_bg.l,
                        bg_a: oklab_bg.a,
                        bg_b: oklab_bg.b,
                        alpha: bg.alpha,
                    };
                }
                x if x == "CREATE_EDGES" => {
                    let create_edges_str: Cow<'_, str> = param.1.into();
                    match create_edges_str.parse() {
                        Ok(c) => self.create_edges = c,
                        Err(_) => errors.append_error(
                            ParseError::Syntax(obj_params.char_range(), "Not a valid float"),
                            obj_params.error_info(),
                        ),
                    }
                }
                x if x == "FLEX" => {
                    let flex_str: Cow<'_, str> = param.1.into();
                    match flex_str.parse() {
                        Ok(c) => self.flex = c,
                        Err(_) => errors.append_error(
                            ParseError::Syntax(obj_params.char_range(), "Not a valid flex"),
                            obj_params.error_info(),
                        ),
                    }
                }
                x if x == "INVERT" => {
                    // unimplemented
                }
                _ => errors.append_error(
                    ParseError::NotFound(cursor.char_range()?, "That register does not exist"),
                    cursor.error_info(),
                ),
            }
        }
        Ok(())
    }
}
