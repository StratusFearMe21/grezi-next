/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! Fairly complete css-color implementation.
//! Relative colors, color-mix, system colors, and other such things require better calc() support
//! and integration.

#[cfg(not(target_arch = "wasm32"))]
use cssparser::color::{
    clamp_floor_256_f32, clamp_unit_f32, parse_hash_color, PredefinedColorSpace, OPAQUE,
};
#[cfg(not(target_arch = "wasm32"))]
use cssparser::{match_ignore_ascii_case, CowRcStr, ParseError, Parser, Token};
use ecolor::Color32;
use palette::{
    chromatic_adaptation::AdaptInto,
    encoding::{Linear, Srgb},
    rgb::{Rgb, Rgba},
    white_point::{D50, D65},
    Alpha, Hsl, Hwb, IntoColor, Xyz, Xyza,
};
use serde::{Deserialize, Serialize};
use std::f32::consts::PI;
use std::str::FromStr;

/// Return the named color with the given name.
///
/// Matching is case-insensitive in the ASCII range.
/// CSS escaping (if relevant) should be resolved before calling this function.
/// (For example, the value of an `Ident` token is fine.)
#[inline]
#[cfg(not(target_arch = "wasm32"))]
pub fn parse_color_keyword(ident: &str) -> Result<Color, ()> {
    Ok(match_ignore_ascii_case! { ident ,
        "transparent" => Color::from_rgba(0, 0, 0, 0.0),
        "currentcolor" => Color::from_current_color(),
        _ => {
            let (r, g, b) = cssparser::color::parse_named_color(ident)?;
            Color::from_rgba(r, g, b, OPAQUE)
        }
    })
}

/// Parse a CSS color using the specified [`ColorParser`] and return a new color
/// value on success.
#[cfg(not(target_arch = "wasm32"))]
pub fn parse_color_with<'i, 't, P>(
    color_parser: &mut DefaultColorParser<'_>,
    input: &mut Parser<'i, 't>,
) -> Result<(std::time::Duration, Color), ParseError<'i, super::Error>> {
    use std::time::Duration;

    let mut duration = Duration::from_millis(500);
    let mut colorspace_in = None;
    let mut color_result = loop {
        let location = input.current_source_location();
        let token = input.next()?;
        match *token {
            Token::Dimension {
                value, ref unit, ..
            } => {
                match_ignore_ascii_case! { unit.as_ref(),
                    "s" => duration = Duration::from_secs_f32(value),
                    "ms" => duration = Duration::from_secs_f32(value / 1000.0),
                    _ => return Err(location.new_unexpected_token_error(Token::Ident(unit.clone())))
                }
                continue;
            }
            Token::Hash(ref value) | Token::IDHash(ref value) => {
                break parse_hash_color(value.as_bytes()).map_or_else(
                    |()| Err(location.new_unexpected_token_error(token.clone())),
                    |(r, g, b, a)| Ok((duration, Color::from_rgba(r, g, b, a))),
                )?;
            }
            Token::Ident(ref value) => {
                match_ignore_ascii_case! { value.as_ref(),
                    "bg" => continue,
                    "in" => {
                        colorspace_in = Some((location, input.expect_ident()?.clone()));
                        continue;
                    },
                    _ => {}
                }
                break parse_color_keyword(value.as_ref()).map_or_else(
                    |()| Err(location.new_unexpected_token_error(token.clone())),
                    |v| Ok((duration, v)),
                )?;
            }
            Token::Function(ref name) => {
                let name = name.clone();
                let t = token.clone();
                break input
                    .parse_nested_block(|arguments| {
                        parse_color_function(color_parser, name, arguments)
                    })
                    .map_or_else(
                        |_| Err(location.new_unexpected_token_error(t)),
                        |v| Ok((duration, v)),
                    )?;
            }
            _ => return Err(location.new_unexpected_token_error(token.clone())),
        };
    };
    match colorspace_in {
        Some((location, color)) => match_ignore_ascii_case! { color.as_ref(),
            "rgb" | "rgba" => {
                if let Some(ref mut reference) = color_parser.reference {
                    let color: palette::LinSrgba = (**reference).into_color();
                    **reference = Color::LinSrgb(color);
                }
                let color: palette::LinSrgba = color_result.1.into_color();
                color_result.1 = Color::LinSrgb(color);
            },

            "hsl" | "hsla" => {
                if let Some(ref mut reference) = color_parser.reference {
                    let color: Alpha<Hsl<Linear<Srgb>>, f32> = (**reference).into_color();
                    **reference = Color::Hsl(color);
                }
                let color: Alpha<Hsl<Linear<Srgb>>, f32> = color_result.1.into_color();
                color_result.1 = Color::Hsl(color);
            },

            "hwb" => {
                if let Some(ref mut reference) = color_parser.reference {
                    let color: Alpha<Hwb<Linear<Srgb>>, f32> = (**reference).into_color();
                    **reference = Color::Hwb(color);
                }
                let color: Alpha<Hwb<Linear<Srgb>>, f32> = color_result.1.into_color();
                color_result.1 = Color::Hwb(color);
            },

            // for L: 0% = 0.0, 100% = 100.0
            // for a and b: -100% = -125, 100% = 125
            "lab" => {
                if let Some(ref mut reference) = color_parser.reference {
                    let color: palette::Laba = (**reference).into_color();
                    **reference = Color::Lab(color);
                }
                let color: palette::Laba = color_result.1.into_color();
                color_result.1 = Color::Lab(color);
            },

            // for L: 0% = 0.0, 100% = 100.0
            // for C: 0% = 0, 100% = 150
            "lch" => {
                if let Some(ref mut reference) = color_parser.reference {
                    let color: palette::Lcha = (**reference).into_color();
                    **reference = Color::Lch(color);
                }
                let color: palette::Lcha = color_result.1.into_color();
                color_result.1 = Color::Lch(color);
            },

            // for L: 0% = 0.0, 100% = 1.0
            // for a and b: -100% = -0.4, 100% = 0.4
            "oklab" => {
                if let Some(ref mut reference) = color_parser.reference {
                    let color: palette::Oklaba = (**reference).into_color();
                    **reference = Color::Oklab(color);
                }
                let color: palette::Oklaba = color_result.1.into_color();
                color_result.1 = Color::Oklab(color);
            },

            // for L: 0% = 0.0, 100% = 1.0
            // for C: 0% = 0.0 100% = 0.4
            "oklch" => {
                if let Some(ref mut reference) = color_parser.reference {
                    let color: palette::Oklcha = (**reference).into_color();
                    **reference = Color::Oklch(color);
                }
                let color: palette::Oklcha = color_result.1.into_color();
                color_result.1 = Color::Oklch(color);
            },
            _ => return Err(location.new_unexpected_token_error(Token::Ident(color.clone()))),
        },
        None => {}
    }
    Ok(color_result)
}

/// Parse one of the color functions: rgba(), lab(), color(), etc.
#[inline]
#[cfg(not(target_arch = "wasm32"))]
fn parse_color_function<'i, 't>(
    color_parser: &mut DefaultColorParser<'_>,
    name: CowRcStr<'i>,
    arguments: &mut Parser<'i, 't>,
) -> Result<Color, ParseError<'i, super::Error>> {
    let color = match_ignore_ascii_case! { &name,
        "rgb" | "rgba" => {
            if let Some(ref mut reference) = color_parser.reference {
                let color: palette::LinSrgba = (**reference).into_color();
                **reference = Color::LinSrgb(color);
            }
            parse_rgb(color_parser, arguments)
        },

        "hsl" | "hsla" => {
            if let Some(ref mut reference) = color_parser.reference {
                let color: Alpha<Hsl<Linear<Srgb>>, f32> = (**reference).into_color();
                **reference = Color::Hsl(color);
            }
            parse_hsl(color_parser, arguments)
        },

        "hwb" => {
            if let Some(ref mut reference) = color_parser.reference {
                let color: Alpha<Hwb<Linear<Srgb>>, f32> = (**reference).into_color();
                **reference = Color::Hwb(color);
            }
            parse_hwb(color_parser, arguments)
        },

        // for L: 0% = 0.0, 100% = 100.0
        // for a and b: -100% = -125, 100% = 125
        "lab" => {
            if let Some(ref mut reference) = color_parser.reference {
                let color: palette::Laba = (**reference).into_color();
                **reference = Color::Lab(color);
            }
            parse_lab_like(color_parser, arguments, 100.0, 125.0, Color::from_lab)
        },

        // for L: 0% = 0.0, 100% = 100.0
        // for C: 0% = 0, 100% = 150
        "lch" => {
            if let Some(ref mut reference) = color_parser.reference {
                let color: palette::Lcha = (**reference).into_color();
                **reference = Color::Lch(color);
            }
            parse_lch_like(color_parser, arguments, 100.0, 150.0, Color::from_lch)
        },

        // for L: 0% = 0.0, 100% = 1.0
        // for a and b: -100% = -0.4, 100% = 0.4
        "oklab" => {
            if let Some(ref mut reference) = color_parser.reference {
                let color: palette::Oklaba = (**reference).into_color();
                **reference = Color::Oklab(color);
            }
            parse_lab_like(color_parser, arguments, 1.0, 0.4, Color::from_oklab)
        },

        // for L: 0% = 0.0, 100% = 1.0
        // for C: 0% = 0.0 100% = 0.4
        "oklch" => {
            if let Some(ref mut reference) = color_parser.reference {
                let color: palette::Oklcha = (**reference).into_color();
                **reference = Color::Oklch(color);
            }
            parse_lch_like(color_parser, arguments, 1.0, 0.4, Color::from_oklch)
        },

        "color" => parse_color_with_color_space(color_parser, arguments),

        _ => return Err(arguments.new_unexpected_token_error(Token::Ident(name))),
    }?;

    arguments.expect_exhausted()?;

    Ok(color)
}

/// Parse the alpha component by itself from either number or percentage,
/// clipping the result to [0.0..1.0].
#[inline]
#[cfg(not(target_arch = "wasm32"))]
fn parse_alpha_component<'i, 't>(
    color_parser: &mut DefaultColorParser<'_>,
    arguments: &mut Parser<'i, 't>,
) -> Result<f32, ParseError<'i, super::Error>>
where
{
    color_parser.in_alpha = true;
    let res = color_parser
        .parse_number_or_percentage(arguments)?
        .unit_value()
        .clamp(0.0, OPAQUE);
    color_parser.in_alpha = false;
    Ok(res)
}

#[cfg(not(target_arch = "wasm32"))]
fn parse_legacy_alpha<'i, 't>(
    color_parser: &mut DefaultColorParser<'_>,
    arguments: &mut Parser<'i, 't>,
) -> Result<f32, ParseError<'i, super::Error>> {
    Ok(if !arguments.is_exhausted() {
        arguments.expect_comma()?;
        parse_alpha_component(color_parser, arguments)?
    } else {
        OPAQUE
    })
}

#[cfg(not(target_arch = "wasm32"))]
fn parse_modern_alpha<'i, 't>(
    color_parser: &mut DefaultColorParser<'_>,
    arguments: &mut Parser<'i, 't>,
) -> Result<Option<f32>, ParseError<'i, super::Error>> {
    if !arguments.is_exhausted() {
        arguments.expect_delim('/')?;
        parse_none_or(arguments, |p| parse_alpha_component(color_parser, p))
    } else {
        Ok(Some(OPAQUE))
    }
}

#[inline]
#[cfg(not(target_arch = "wasm32"))]
fn parse_rgb<'i, 't>(
    color_parser: &mut DefaultColorParser<'_>,
    arguments: &mut Parser<'i, 't>,
) -> Result<Color, ParseError<'i, super::Error>> {
    let maybe_red = parse_none_or(arguments, |p| color_parser.parse_number_or_percentage(p))?;

    // If the first component is not "none" and is followed by a comma, then we
    // are parsing the legacy syntax.
    let is_legacy_syntax = maybe_red.is_some() && arguments.try_parse(|p| p.expect_comma()).is_ok();

    let (red, green, blue, alpha) = if is_legacy_syntax {
        let (red, green, blue) = match maybe_red.unwrap() {
            NumberOrPercentage::Number { value } => {
                let red = clamp_floor_256_f32(value);
                let green = clamp_floor_256_f32(color_parser.parse_number(arguments)?);
                arguments.expect_comma()?;
                let blue = clamp_floor_256_f32(color_parser.parse_number(arguments)?);
                (red, green, blue)
            }
            NumberOrPercentage::Percentage { unit_value } => {
                let red = clamp_unit_f32(unit_value);
                let green = clamp_unit_f32(color_parser.parse_percentage(arguments)?);
                arguments.expect_comma()?;
                let blue = clamp_unit_f32(color_parser.parse_percentage(arguments)?);
                (red, green, blue)
            }
        };

        let alpha = parse_legacy_alpha(color_parser, arguments)?;

        (red, green, blue, alpha)
    } else {
        #[inline]
        fn get_component_value(c: Option<NumberOrPercentage>) -> u8 {
            c.map(|c| match c {
                NumberOrPercentage::Number { value } => clamp_floor_256_f32(value),
                NumberOrPercentage::Percentage { unit_value } => clamp_unit_f32(unit_value),
            })
            .unwrap_or(0)
        }

        let red = get_component_value(maybe_red);

        let green = get_component_value(parse_none_or(arguments, |p| {
            color_parser.parse_number_or_percentage(p)
        })?);

        let blue = get_component_value(parse_none_or(arguments, |p| {
            color_parser.parse_number_or_percentage(p)
        })?);

        let alpha = parse_modern_alpha(color_parser, arguments)?.unwrap_or(0.0);

        (red, green, blue, alpha)
    };

    Ok(Color::from_rgba(red, green, blue, alpha))
}

/// Parses hsl syntax.
///
/// <https://drafts.csswg.org/css-color/#the-hsl-notation>
#[inline]
#[cfg(not(target_arch = "wasm32"))]
fn parse_hsl<'i, 't>(
    color_parser: &mut DefaultColorParser<'_>,
    arguments: &mut Parser<'i, 't>,
) -> Result<Color, ParseError<'i, super::Error>> {
    let maybe_hue = parse_none_or(arguments, |p| color_parser.parse_angle_or_number(p))?;

    // If the hue is not "none" and is followed by a comma, then we are parsing
    // the legacy syntax.
    let is_legacy_syntax = maybe_hue.is_some() && arguments.try_parse(|p| p.expect_comma()).is_ok();

    let saturation: Option<f32>;
    let lightness: Option<f32>;

    let alpha = if is_legacy_syntax {
        saturation = Some(color_parser.parse_percentage(arguments)?);
        arguments.expect_comma()?;
        lightness = Some(color_parser.parse_percentage(arguments)?);
        Some(parse_legacy_alpha(color_parser, arguments)?)
    } else {
        saturation = parse_none_or(arguments, |p| color_parser.parse_percentage(p))?;
        lightness = parse_none_or(arguments, |p| color_parser.parse_percentage(p))?;

        parse_modern_alpha(color_parser, arguments)?
    };

    let hue = maybe_hue.map(|h| normalize_hue(h.degrees()));
    let saturation = saturation.map(|s| s.clamp(0.0, 1.0));
    let lightness = lightness.map(|l| l.clamp(0.0, 1.0));

    Ok(Color::from_hsl(hue, saturation, lightness, alpha))
}

/// Parses hwb syntax.
///
/// <https://drafts.csswg.org/css-color/#the-hbw-notation>
#[inline]
#[cfg(not(target_arch = "wasm32"))]
fn parse_hwb<'i, 't>(
    color_parser: &mut DefaultColorParser<'_>,
    arguments: &mut Parser<'i, 't>,
) -> Result<Color, ParseError<'i, super::Error>> {
    let (hue, whiteness, blackness, alpha) = parse_components(
        color_parser,
        arguments,
        DefaultColorParser::parse_angle_or_number,
        DefaultColorParser::parse_percentage,
        DefaultColorParser::parse_percentage,
    )?;

    let hue = hue.map(|h| normalize_hue(h.degrees()));
    let whiteness = whiteness.map(|w| w.clamp(0.0, 1.0));
    let blackness = blackness.map(|b| b.clamp(0.0, 1.0));

    Ok(Color::from_hwb(hue, whiteness, blackness, alpha))
}

type IntoColorFn<Output> =
    fn(l: Option<f32>, a: Option<f32>, b: Option<f32>, alpha: Option<f32>) -> Output;

#[inline]
#[cfg(not(target_arch = "wasm32"))]
fn parse_lab_like<'i, 't>(
    color_parser: &mut DefaultColorParser<'_>,
    arguments: &mut Parser<'i, 't>,
    lightness_range: f32,
    a_b_range: f32,
    into_color: IntoColorFn<Color>,
) -> Result<Color, ParseError<'i, super::Error>> {
    let (lightness, a, b, alpha) = parse_components(
        color_parser,
        arguments,
        DefaultColorParser::parse_number_or_percentage,
        DefaultColorParser::parse_number_or_percentage,
        DefaultColorParser::parse_number_or_percentage,
    )?;

    let lightness = lightness.map(|l| l.value(lightness_range));
    let a = a.map(|a| a.value(a_b_range));
    let b = b.map(|b| b.value(a_b_range));

    Ok(into_color(lightness, a, b, alpha))
}

#[inline]
#[cfg(not(target_arch = "wasm32"))]
fn parse_lch_like<'i, 't>(
    color_parser: &mut DefaultColorParser<'_>,
    arguments: &mut Parser<'i, 't>,
    lightness_range: f32,
    chroma_range: f32,
    into_color: IntoColorFn<Color>,
) -> Result<Color, ParseError<'i, super::Error>> {
    let (lightness, chroma, hue, alpha) = parse_components(
        color_parser,
        arguments,
        DefaultColorParser::parse_number_or_percentage,
        DefaultColorParser::parse_number_or_percentage,
        DefaultColorParser::parse_angle_or_number,
    )?;

    let lightness = lightness.map(|l| l.value(lightness_range));
    let chroma = chroma.map(|c| c.value(chroma_range));
    let hue = hue.map(|h| normalize_hue(h.degrees()));

    Ok(into_color(lightness, chroma, hue, alpha))
}

/// Parse the color() function.
#[inline]
#[cfg(not(target_arch = "wasm32"))]
fn parse_color_with_color_space<'i, 't>(
    color_parser: &mut DefaultColorParser<'_>,
    arguments: &mut Parser<'i, 't>,
) -> Result<Color, ParseError<'i, super::Error>> {
    let color_space = {
        let location = arguments.current_source_location();

        let ident = arguments.expect_ident()?;
        PredefinedColorSpace::from_str(ident)
            .map_err(|_| location.new_unexpected_token_error(Token::Ident(ident.clone())))?
    };

    match color_space {
        PredefinedColorSpace::Srgb | PredefinedColorSpace::SrgbLinear => {
            if let Some(ref mut reference) = color_parser.reference {
                let color: palette::LinSrgba = (**reference).into_color();
                **reference = Color::LinSrgb(color);
            }
        }
        PredefinedColorSpace::XyzD50 => {
            if let Some(ref mut reference) = color_parser.reference {
                let color: palette::Xyza<D65, f32> = (**reference).into_color();
                **reference = Color::XyzD50(color.adapt_into());
            }
        }
        PredefinedColorSpace::XyzD65 => {
            if let Some(ref mut reference) = color_parser.reference {
                let color: palette::Xyza<D65, f32> = (**reference).into_color();
                **reference = Color::XyzD65(color);
            }
        }
        _ => return Err(arguments.new_error_for_next_token()),
    }

    let (c1, c2, c3, alpha) = parse_components(
        color_parser,
        arguments,
        DefaultColorParser::parse_number_or_percentage,
        DefaultColorParser::parse_number_or_percentage,
        DefaultColorParser::parse_number_or_percentage,
    )?;

    let c1 = c1.map(|c| c.unit_value());
    let c2 = c2.map(|c| c.unit_value());
    let c3 = c3.map(|c| c.unit_value());

    Ok(Color::from_color_function(color_space, c1, c2, c3, alpha))
}

#[cfg(not(target_arch = "wasm32"))]
type ComponentParseResult<'i, R1, R2, R3, Error> =
    Result<(Option<R1>, Option<R2>, Option<R3>, Option<f32>), ParseError<'i, Error>>;

/// Parse the color components and alpha with the modern [color-4] syntax.
#[cfg(not(target_arch = "wasm32"))]
pub fn parse_components<'a, 'i, 't, F1, F2, F3, R1, R2, R3>(
    color_parser: &mut DefaultColorParser<'a>,
    input: &mut Parser<'i, 't>,
    f1: F1,
    f2: F2,
    f3: F3,
) -> ComponentParseResult<'i, R1, R2, R3, super::Error>
where
    F1: FnOnce(
        &mut DefaultColorParser<'a>,
        &mut Parser<'i, 't>,
    ) -> Result<R1, ParseError<'i, super::Error>>,
    F2: FnOnce(
        &mut DefaultColorParser<'a>,
        &mut Parser<'i, 't>,
    ) -> Result<R2, ParseError<'i, super::Error>>,
    F3: FnOnce(
        &mut DefaultColorParser<'a>,
        &mut Parser<'i, 't>,
    ) -> Result<R3, ParseError<'i, super::Error>>,
{
    let r1 = parse_none_or(input, |p| f1(color_parser, p))?;
    let r2 = parse_none_or(input, |p| f2(color_parser, p))?;
    let r3 = parse_none_or(input, |p| f3(color_parser, p))?;

    let alpha = parse_modern_alpha(color_parser, input)?;

    Ok((r1, r2, r3, alpha))
}

#[cfg(not(target_arch = "wasm32"))]
fn parse_none_or<'i, 't, F, T, E>(input: &mut Parser<'i, 't>, thing: F) -> Result<Option<T>, E>
where
    F: FnOnce(&mut Parser<'i, 't>) -> Result<T, E>,
{
    match input.try_parse(|p| p.expect_ident_matching("none")) {
        Ok(_) => Ok(None),
        Err(_) => Ok(Some(thing(input)?)),
    }
}

// Guaratees hue in [0..360)
fn normalize_hue(hue: f32) -> f32 {
    // <https://drafts.csswg.org/css-values/#angles>
    // Subtract an integer before rounding, to avoid some rounding errors:
    hue - 360.0 * (hue / 360.0).floor()
}

/// Either a number or a percentage.
pub enum NumberOrPercentage {
    /// `<number>`.
    Number {
        /// The numeric value parsed, as a float.
        value: f32,
    },
    /// `<percentage>`
    Percentage {
        /// The value as a float, divided by 100 so that the nominal range is
        /// 0.0 to 1.0.
        unit_value: f32,
    },
}

impl NumberOrPercentage {
    /// Return the value as a percentage.
    pub fn unit_value(&self) -> f32 {
        match *self {
            NumberOrPercentage::Number { value } => value,
            NumberOrPercentage::Percentage { unit_value } => unit_value,
        }
    }

    /// Return the value as a number with a percentage adjusted to the
    /// `percentage_basis`.
    pub fn value(&self, percentage_basis: f32) -> f32 {
        match *self {
            Self::Number { value } => value,
            Self::Percentage { unit_value } => unit_value * percentage_basis,
        }
    }
}

/// Either an angle or a number.
pub enum AngleOrNumber {
    /// `<number>`.
    Number {
        /// The numeric value parsed, as a float.
        value: f32,
    },
    /// `<angle>`
    Angle {
        /// The value as a number of degrees.
        degrees: f32,
    },
}

impl AngleOrNumber {
    /// Return the angle in degrees. `AngleOrNumber::Number` is returned as
    /// degrees, because it is the canonical unit.
    pub fn degrees(&self) -> f32 {
        match *self {
            AngleOrNumber::Number { value } => value,
            AngleOrNumber::Angle { degrees } => degrees,
        }
    }
}

/// A trait that can be used to hook into how `cssparser` parses color
/// components, with the intention of implementing more complicated behavior.
///
/// For example, this is used by Servo to support calc() in color.
#[cfg(not(target_arch = "wasm32"))]
impl<'a, 'i> DefaultColorParser<'a> {
    /// Parse an `<angle>` or `<number>`.
    ///
    /// Returns the result in degrees.
    fn parse_angle_or_number<'t>(
        &mut self,
        input: &mut Parser<'i, 't>,
    ) -> Result<AngleOrNumber, ParseError<'i, super::Error>> {
        let location = input.current_source_location();
        Ok(match *input.next()? {
            Token::Number { value, .. } => AngleOrNumber::Number { value },
            Token::Dimension {
                value: v, ref unit, ..
            } => {
                let degrees = match_ignore_ascii_case! { unit,
                    "deg" => v,
                    "grad" => v * 360. / 400.,
                    "rad" => v * 360. / (2. * PI),
                    "turn" => v * 360.,
                    _ => {
                        return Err(location.new_unexpected_token_error(Token::Ident(unit.clone())))
                    }
                };

                AngleOrNumber::Angle { degrees }
            }
            ref t @ Token::Ident(ref i) => match self.reference {
                Some(Color::Hsl(hsl)) => {
                    match_ignore_ascii_case! { i.as_ref(),
                        "h" => AngleOrNumber::Angle { degrees: hsl.hue.into_positive_degrees() },
                        _ => return Err(location.new_unexpected_token_error(t.clone()))
                    }
                }
                Some(Color::Hwb(hwb)) => {
                    match_ignore_ascii_case! { i.as_ref(),
                        "h" => AngleOrNumber::Angle { degrees: hwb.hue.into_positive_degrees() },
                        _ => return Err(location.new_unexpected_token_error(t.clone()))
                    }
                }
                Some(Color::Lch(lch)) => {
                    match_ignore_ascii_case! { i.as_ref(),
                        "h" => AngleOrNumber::Angle { degrees: lch.hue.into_positive_degrees() },
                        _ => return Err(location.new_unexpected_token_error(t.clone()))
                    }
                }
                Some(Color::Oklch(oklch)) => {
                    match_ignore_ascii_case! { i.as_ref(),
                        "h" => AngleOrNumber::Angle { degrees: oklch.hue.into_positive_degrees() },
                        _ => return Err(location.new_unexpected_token_error(t.clone()))
                    }
                }
                _ => return Err(location.new_unexpected_token_error(t.clone())),
            },
            ref t => return Err(location.new_unexpected_token_error(t.clone())),
        })
    }

    /// Parse a `<percentage>` value.
    ///
    /// Returns the result in a number from 0.0 to 1.0.
    fn parse_percentage<'t>(
        &mut self,
        input: &mut Parser<'i, 't>,
    ) -> Result<f32, ParseError<'i, super::Error>> {
        let location = input.current_source_location();
        Ok(match *input.next()? {
            Token::Percentage { unit_value, .. } => unit_value,
            ref t @ Token::Ident(ref i) => match self.reference {
                Some(Color::LinSrgb(srgb)) => {
                    let non_linear: palette::Srgba = (*srgb).into_color();
                    match_ignore_ascii_case! { i.as_ref(),
                        "r" => non_linear.red,
                        "g" => non_linear.green,
                        "b" => non_linear.blue,
                        _ => return Err(location.new_unexpected_token_error(t.clone()))
                    }
                }
                Some(Color::Hsl(hsl)) => {
                    let non_linear: palette::Hsla<Srgb> = (*hsl).into_color();
                    match_ignore_ascii_case! { i.as_ref(),
                        "s" => non_linear.saturation,
                        "l" => non_linear.lightness,
                        _ => return Err(location.new_unexpected_token_error(t.clone()))
                    }
                }
                Some(Color::Hwb(hwb)) => {
                    let non_linear: palette::Hwba<Srgb> = (*hwb).into_color();
                    match_ignore_ascii_case! { i.as_ref(),
                        "w" => non_linear.whiteness,
                        "b" => non_linear.blackness,
                        _ => return Err(location.new_unexpected_token_error(t.clone()))
                    }
                }
                _ => return Err(location.new_unexpected_token_error(t.clone())),
            },
            ref t => return Err(location.new_unexpected_token_error(t.clone())),
        })
    }

    /// Parse a `<number>` value.
    fn parse_number<'t>(
        &mut self,
        input: &mut Parser<'i, 't>,
    ) -> Result<f32, ParseError<'i, super::Error>> {
        let location = input.current_source_location();
        Ok(match *input.next()? {
            Token::Number { value, .. } => value,
            ref t @ Token::Ident(ref i) => match self.reference {
                Some(Color::LinSrgb(srgb)) => {
                    let non_linear: palette::Srgba = (*srgb).into_color();
                    match_ignore_ascii_case! { i.as_ref(),
                        "r" => non_linear.red * 255.0,
                        "g" => non_linear.green * 255.0,
                        "b" => non_linear.blue * 255.0,
                        _ => return Err(location.new_unexpected_token_error(t.clone()))
                    }
                }
                _ => return Err(location.new_unexpected_token_error(t.clone())),
            },
            ref t => return Err(location.new_unexpected_token_error(t.clone())),
        })
    }

    /// Parse a `<number>` value or a `<percentage>` value.
    fn parse_number_or_percentage<'t>(
        &mut self,
        input: &mut Parser<'i, 't>,
    ) -> Result<NumberOrPercentage, ParseError<'i, super::Error>> {
        let location = input.current_source_location();
        Ok(match *input.next()? {
            Token::Number { value, .. } => NumberOrPercentage::Number { value },
            Token::Percentage { unit_value, .. } => NumberOrPercentage::Percentage { unit_value },
            ref t @ Token::Ident(ref i) => match self.reference {
                Some(Color::LinSrgb(srgb)) => {
                    let non_linear: palette::Srgba = (*srgb).into_color();
                    match_ignore_ascii_case! { i.as_ref(),
                        "r" => NumberOrPercentage::Percentage { unit_value: non_linear.red },
                        "g" => NumberOrPercentage::Percentage { unit_value: non_linear.green },
                        "b" => NumberOrPercentage::Percentage { unit_value: non_linear.blue },
                        "a" => NumberOrPercentage::Percentage { unit_value: non_linear.alpha },
                        _ => return Err(location.new_unexpected_token_error(t.clone()))
                    }
                }
                Some(Color::Hsl(hsl)) => {
                    match_ignore_ascii_case! { i.as_ref(),
                        "a" => NumberOrPercentage::Percentage { unit_value: hsl.alpha },
                        _ => return Err(location.new_unexpected_token_error(t.clone()))
                    }
                }
                Some(Color::Hwb(hwb)) => {
                    match_ignore_ascii_case! { i.as_ref(),
                        "a" => NumberOrPercentage::Percentage { unit_value: hwb.alpha },
                        _ => return Err(location.new_unexpected_token_error(t.clone()))
                    }
                }
                Some(Color::Lab(lab)) => {
                    match_ignore_ascii_case! { i.as_ref(),
                        "l" => NumberOrPercentage::Number { value: lab.l },
                        "b" => NumberOrPercentage::Number { value: lab.b },
                        "a" if self.in_alpha => NumberOrPercentage::Percentage { unit_value: lab.alpha },
                        "a" => NumberOrPercentage::Number { value: lab.a },
                        _ => return Err(location.new_unexpected_token_error(t.clone()))
                    }
                }
                Some(Color::Lch(lch)) => {
                    match_ignore_ascii_case! { i.as_ref(),
                        "l" => NumberOrPercentage::Number { value: lch.l },
                        "c" => NumberOrPercentage::Number { value: lch.chroma },
                        "a" => NumberOrPercentage::Percentage { unit_value: lch.alpha },
                        _ => return Err(location.new_unexpected_token_error(t.clone()))
                    }
                }
                Some(Color::Oklab(oklab)) => {
                    match_ignore_ascii_case! { i.as_ref(),
                        "l" => NumberOrPercentage::Number { value: oklab.l },
                        "b" => NumberOrPercentage::Number { value: oklab.b },
                        "a" if self.in_alpha => NumberOrPercentage::Percentage { unit_value: oklab.alpha },
                        "a" => NumberOrPercentage::Number { value: oklab.a },
                        _ => return Err(location.new_unexpected_token_error(t.clone()))
                    }
                }
                Some(Color::Oklch(oklch)) => {
                    match_ignore_ascii_case! { i.as_ref(),
                        "l" => NumberOrPercentage::Number { value: oklch.l },
                        "c" => NumberOrPercentage::Number { value: oklch.chroma },
                        "a" => NumberOrPercentage::Percentage { unit_value: oklch.alpha },
                        _ => return Err(location.new_unexpected_token_error(t.clone()))
                    }
                }
                _ => return Err(location.new_unexpected_token_error(t.clone())),
            },
            ref t => return Err(location.new_unexpected_token_error(t.clone())),
        })
    }
}

/// Default implementation of a [`ColorParser`]
pub struct DefaultColorParser<'a> {
    in_alpha: bool,
    reference: Option<&'a mut Color>,
}

impl<'a> DefaultColorParser<'a> {
    pub fn new(reference: Option<&'a mut Color>) -> Self {
        Self {
            in_alpha: false,
            reference,
        }
    }
}

macro_rules! into_color_impl {
    ($color:ty) => {
        impl IntoColor<$color> for Color {
            fn into_color(self) -> $color {
                match self {
                    Color::Hsl(hsl) => hsl.into_color(),
                    Color::Hwb(hwb) => hwb.into_color(),
                    Color::Lab(lab) => lab.into_color(),
                    Color::Lch(lch) => lch.into_color(),
                    Color::Oklab(oklab) => oklab.into_color(),
                    Color::Oklch(oklch) => oklch.into_color(),
                    Color::LinSrgb(srgb) => srgb.into_color(),
                    Color::XyzD50(xyz) => {
                        let xyz: Xyza<D65, f32> = xyz.adapt_into();
                        xyz.into_color()
                    }
                    Color::XyzD65(xyz) => xyz.into_color(),
                }
            }
        }
    };
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum Color {
    #[serde(with = "palette::serde::as_array")]
    LinSrgb(palette::LinSrgba),
    #[serde(with = "palette::serde::as_array")]
    Hsl(Alpha<Hsl<Linear<Srgb>>, f32>),
    #[serde(with = "palette::serde::as_array")]
    Hwb(Alpha<Hwb<Linear<Srgb>>, f32>),
    #[serde(with = "palette::serde::as_array")]
    Lab(palette::Laba),
    #[serde(with = "palette::serde::as_array")]
    Lch(palette::Lcha),
    #[serde(with = "palette::serde::as_array")]
    Oklab(palette::Oklaba),
    #[serde(with = "palette::serde::as_array")]
    Oklch(palette::Oklcha),
    #[serde(with = "palette::serde::as_array")]
    XyzD50(palette::Xyza<D50>),
    #[serde(with = "palette::serde::as_array")]
    XyzD65(palette::Xyza<D65>),
}

into_color_impl!(palette::LinSrgba);
into_color_impl!(Alpha<Hsl<Linear<Srgb>>, f32>);
into_color_impl!(Alpha<Hwb<Linear<Srgb>>, f32>);
into_color_impl!(palette::Laba);
into_color_impl!(palette::Lcha);
into_color_impl!(palette::Oklaba);
into_color_impl!(palette::Oklcha);
// into_color_impl!(palette::Xyza<D50>);
into_color_impl!(palette::Xyza<D65>);

impl Color {
    pub fn interpolate(self, to_c: Color, a: f32, max: f32) -> Color {
        let from: [f32; 4] = self.into();
        let to: [f32; 4] = to_c.into();

        let new_color = keyframe::ease_with_scaled_time(
            keyframe::functions::Linear,
            from,
            to,
            a.clamp(0.0, max),
            max,
        );

        match to_c {
            Color::LinSrgb(_) => Color::LinSrgb(From::from(new_color)),
            Color::Hsl(_) => Color::Hsl(From::from(new_color)),
            Color::Hwb(_) => Color::Hwb(From::from(new_color)),
            Color::Lab(_) => Color::Lab(From::from(new_color)),
            Color::Lch(_) => Color::Lch(From::from(new_color)),
            Color::Oklab(_) => Color::Oklab(From::from(new_color)),
            Color::Oklch(_) => Color::Oklch(From::from(new_color)),
            Color::XyzD50(_) => Color::XyzD50(From::from(new_color)),
            Color::XyzD65(_) => Color::XyzD65(From::from(new_color)),
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::LinSrgb(
            Alpha::<Rgb<Srgb>, f32>::from_components((
                27.0 / 255.0,
                27.0 / 255.0,
                27.0 / 255.0,
                1.0,
            ))
            .into_linear(),
        )
    }
}

type UnmultipliedPaletteColor32 = palette::Alpha<Rgb<Srgb, u8>, f32>;
type UnmultipliedPaletteColorF32 = palette::Alpha<Rgb<Linear<Srgb>, f32>, f32>;
type PaletteColor32 = palette::blend::PreAlpha<Rgb<Linear<Srgb>, f32>>;

impl Color {
    fn from_current_color() -> Self {
        Self::default()
    }

    fn from_rgba(red: u8, green: u8, blue: u8, alpha: f32) -> Self {
        Color::LinSrgb(
            UnmultipliedPaletteColor32::from_components((red, green, blue, alpha)).into_linear(),
        )
    }

    fn from_hsl(
        hue: Option<f32>,
        saturation: Option<f32>,
        lightness: Option<f32>,
        alpha: Option<f32>,
    ) -> Self {
        Color::Hsl(
            palette::Hsla::<Srgb, f32>::new(
                hue.unwrap_or(1.0),
                saturation.unwrap_or(1.0),
                lightness.unwrap_or(1.0),
                alpha.unwrap_or(1.0),
            )
            .into_color(),
        )
    }

    fn from_hwb(
        hue: Option<f32>,
        whiteness: Option<f32>,
        blackness: Option<f32>,
        alpha: Option<f32>,
    ) -> Self {
        Color::Hwb(
            palette::Hwba::<Srgb, f32>::new(
                hue.unwrap_or(1.0),
                whiteness.unwrap_or(1.0),
                blackness.unwrap_or(1.0),
                alpha.unwrap_or(1.0),
            )
            .into_color(),
        )
    }

    fn from_lab(
        lightness: Option<f32>,
        a: Option<f32>,
        b: Option<f32>,
        alpha: Option<f32>,
    ) -> Self {
        Color::Lab(palette::Laba::new(
            lightness.unwrap_or(1.0),
            a.unwrap_or(1.0),
            b.unwrap_or(1.0),
            alpha.unwrap_or(1.0),
        ))
    }

    fn from_lch(
        lightness: Option<f32>,
        chroma: Option<f32>,
        hue: Option<f32>,
        alpha: Option<f32>,
    ) -> Self {
        Color::Lch(palette::Lcha::new(
            lightness.unwrap_or(1.0),
            chroma.unwrap_or(1.0),
            hue.unwrap_or(1.0),
            alpha.unwrap_or(1.0),
        ))
    }

    fn from_oklab(
        lightness: Option<f32>,
        a: Option<f32>,
        b: Option<f32>,
        alpha: Option<f32>,
    ) -> Self {
        Color::Oklab(palette::Oklaba::new(
            lightness.unwrap_or(1.0),
            a.unwrap_or(1.0),
            b.unwrap_or(1.0),
            alpha.unwrap_or(1.0),
        ))
    }

    fn from_oklch(
        lightness: Option<f32>,
        chroma: Option<f32>,
        hue: Option<f32>,
        alpha: Option<f32>,
    ) -> Self {
        Color::Oklch(palette::Oklcha::new(
            lightness.unwrap_or(1.0),
            chroma.unwrap_or(1.0),
            hue.unwrap_or(1.0),
            alpha.unwrap_or(1.0),
        ))
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn from_color_function(
        color_space: PredefinedColorSpace,
        c1: Option<f32>,
        c2: Option<f32>,
        c3: Option<f32>,
        alpha: Option<f32>,
    ) -> Self {
        let components = (
            c1.unwrap_or(1.0),
            c2.unwrap_or(1.0),
            c3.unwrap_or(1.0),
            alpha.unwrap_or(1.0),
        );
        match color_space {
            PredefinedColorSpace::SrgbLinear => {
                Color::LinSrgb(Rgba::<Linear<Srgb>, f32>::from_components(components))
            }
            PredefinedColorSpace::XyzD50 => {
                Color::XyzD50(Xyza::<D50, f32>::from_components(components))
            }
            PredefinedColorSpace::XyzD65 => {
                Color::XyzD65(Xyza::<D65, f32>::from_components(components))
            }
            _ => Color::LinSrgb(Rgba::<Srgb, f32>::from_components(components).into_linear()),
        }
    }
}

impl Into<Color32> for Color {
    fn into(self) -> Color32 {
        let color: PaletteColor32 = match self {
            Color::Hsl(hsl) => {
                let color: UnmultipliedPaletteColorF32 = hsl.into_color();
                color.premultiply()
            }
            Color::Hwb(hwb) => {
                let color: UnmultipliedPaletteColorF32 = hwb.into_color();
                color.premultiply()
            }
            Color::Lab(lab) => {
                let color: UnmultipliedPaletteColorF32 = lab.into_color();
                color.premultiply()
            }
            Color::Lch(lch) => {
                let color: UnmultipliedPaletteColorF32 = lch.into_color();
                color.premultiply()
            }
            Color::Oklab(oklab) => {
                let color: UnmultipliedPaletteColorF32 = oklab.into_color();
                color.premultiply()
            }
            Color::Oklch(oklch) => {
                let color: UnmultipliedPaletteColorF32 = oklch.into_color();
                color.premultiply()
            }
            Color::LinSrgb(srgb) => srgb.premultiply(),
            Color::XyzD50(xyz) => {
                let xyz: Alpha<Xyz<D65>, f32> = xyz.adapt_into();
                let color: UnmultipliedPaletteColorF32 = xyz.into_color();
                color.premultiply()
            }
            Color::XyzD65(xyz) => {
                let color: UnmultipliedPaletteColorF32 = xyz.into_color();
                color.premultiply()
            }
        };

        ecolor::Rgba::from_rgba_premultiplied(color.red, color.green, color.blue, color.alpha)
            .into()
    }
}

impl Into<[f32; 4]> for Color {
    fn into(self) -> [f32; 4] {
        match self {
            Color::Hsl(hsl) => hsl.into(),
            Color::Hwb(hwb) => hwb.into(),
            Color::Lab(lab) => lab.into(),
            Color::Lch(lch) => lch.into(),
            Color::Oklab(oklab) => oklab.into(),
            Color::Oklch(oklch) => oklch.into(),
            Color::LinSrgb(srgb) => srgb.into(),
            Color::XyzD50(xyz) => xyz.into(),
            Color::XyzD65(xyz) => xyz.into(),
        }
    }
}
