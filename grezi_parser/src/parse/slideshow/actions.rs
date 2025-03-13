use std::{borrow::Cow, io, ops::DerefMut, sync::Arc};

use ecolor::Color32;
use emath::Align2;
use smallvec::SmallVec;
use tracing::instrument;
use tree_sitter_grz::NodeKind;

use crate::{
    actions::{DrawableAction, SlideParams},
    parse::{
        cursor::{GrzCursor, GrzCursorGuard},
        error::{ErrsWithSource, ParseError},
        slideshow::text::StringLiteral,
    },
};

use super::{object::parse_color, slide::parse_alignment_from_chars};

// Legacy
//
// pub const HIGHLIGHT_COLOR_DEFAULT: Color32 = {
//     let color = Color32::LIGHT_YELLOW;
//     Color32::from_rgba_premultiplied(
//         (color.r() as f32 * 0.5 + 0.5) as u8,
//         (color.g() as f32 * 0.5 + 0.5) as u8,
//         (color.b() as f32 * 0.5 + 0.5) as u8,
//         (color.a() as f32 * 0.5 + 0.5) as u8,
//     )
// };

pub const HIGHLIGHT_COLOR_DEFAULT: Color32 = Color32::from_rgba_premultiplied(61, 61, 53, 12);

macro_rules! goto_next_existing_sibling {
    ($cursor:ident, $errors:ident) => {
        if !$cursor.goto_next_sibling()? {
            $errors.append_error(
                ParseError::NotFound($cursor.char_range()?, "Action needs more arguments"),
                $cursor.error_info(),
            );
            return Ok(SmallVec::new());
        }
    };
}

macro_rules! parse_alignment {
    ($cursor:ident, $errors:ident) => {
        if let Some(mut alignment_cursor) = $cursor.goto_first_child(NodeKind::SymStringLiteral)? {
            if let Some(alignment) =
                parse_alignment_from_chars(alignment_cursor.deref_mut(), Arc::clone(&$errors))?.0
            {
                alignment
            } else {
                $errors.append_error(
                    ParseError::NotFound(
                        $cursor.char_range()?,
                        "Expecting a valid alignment that isn't `|`",
                    ),
                    $cursor.error_info(),
                );
                Align2::CENTER_CENTER
            }
        } else {
            $errors.append_error(
                ParseError::NotFound(
                    $cursor.char_range()?,
                    "Expecting a valid alignment string literal here",
                ),
                $cursor.error_info(),
            );
            Align2::CENTER_CENTER
        }
    };
}

impl SlideParams {
    #[instrument(skip_all, fields(source = %cursor.parent_source()?, type = std::any::type_name::<Self>()))]
    pub fn parse(
        &mut self,
        mut cursor: GrzCursorGuard,
        errors: Arc<ErrsWithSource>,
    ) -> io::Result<SmallVec<[DrawableAction; 2]>> {
        let mut drawable_actions = SmallVec::new();
        while let Some(mut action_cursor) = cursor.goto_first_child(NodeKind::SymSlideFunction)? {
            let action = action_cursor.rope_slice()?;

            match action {
                x if x == "highlight" => {
                    goto_next_existing_sibling!(action_cursor, errors);
                    let object_to_highlight = action_cursor.smartstring()?;
                    let mut locations = None;
                    if action_cursor.goto_next_sibling()? {
                        let first_locations = parse_highlight_locations(
                            action_cursor.deref_mut(),
                            Arc::clone(&errors),
                        )?;
                        if action_cursor.goto_next_sibling()? {
                            let second_locations = parse_highlight_locations(
                                action_cursor.deref_mut(),
                                Arc::clone(&errors),
                            )?;
                            locations = first_locations.and_then(|first_locations| {
                                Some([first_locations, second_locations?])
                            });
                        }
                    }
                    let mut color = HIGHLIGHT_COLOR_DEFAULT;
                    if action_cursor.goto_next_sibling()? {
                        let color_str = action_cursor.node_to_string_literal()?;
                        let color_str: Cow<'_, str> = color_str.into();
                        color = parse_color(
                            color_str.as_ref(),
                            action_cursor.char_range()?,
                            action_cursor.error_info(),
                            Arc::clone(&errors),
                        )?;
                    }
                    drawable_actions.push(DrawableAction::Highlight {
                        object: object_to_highlight,
                        locations,
                        color,
                    })
                }
                x if x == "line" => {
                    goto_next_existing_sibling!(action_cursor, errors);
                    let first_object = action_cursor.smartstring()?;
                    goto_next_existing_sibling!(action_cursor, errors);
                    let first_location = parse_alignment!(action_cursor, errors);
                    goto_next_existing_sibling!(action_cursor, errors);
                    let second_object = action_cursor.smartstring()?;
                    goto_next_existing_sibling!(action_cursor, errors);
                    let second_location = parse_alignment!(action_cursor, errors);
                    let mut color = Color32::WHITE;
                    if action_cursor.goto_next_sibling()? {
                        let color_str = action_cursor.node_to_string_literal()?;
                        let color_str: Cow<'_, str> = color_str.into();
                        color = parse_color(
                            color_str.as_ref(),
                            action_cursor.char_range()?,
                            action_cursor.error_info(),
                            Arc::clone(&errors),
                        )?;
                    }

                    drawable_actions.push(DrawableAction::Line {
                        objects: [first_object, second_object],
                        locations: [first_location, second_location],
                        color,
                    });
                }
                x if x == "speaker_notes" => {
                    goto_next_existing_sibling!(action_cursor, errors);
                    if let Some(notes_action_cursor) =
                        action_cursor.goto_first_child(NodeKind::SymStringLiteral)?
                    {
                        self.speaker_notes = Some(
                            StringLiteral::parse_from_cursor(
                                notes_action_cursor,
                                Arc::clone(&errors),
                            )?
                            .into(),
                        );
                    }
                }
                x if x == "next" => {
                    if action_cursor.goto_next_sibling()? {
                        let time_str: Cow<'_, str> = action_cursor.node_to_string_literal()?.into();
                        match time_str.parse() {
                            Ok(time) => self.next = Some(time),
                            Err(_) => errors.append_error(
                                ParseError::Syntax(action_cursor.char_range()?, "Not a integer"),
                                action_cursor.error_info(),
                            ),
                        }
                    } else {
                        self.next = Some(0.0);
                    }
                }
                x if x == "stagger" => {
                    goto_next_existing_sibling!(action_cursor, errors);
                    let stagger_time: Cow<'_, str> = action_cursor.node_to_string_literal()?.into();
                    match stagger_time.parse() {
                        Ok(stagger_time) => self.stagger = stagger_time,
                        Err(_) => errors.append_error(
                            ParseError::Syntax(action_cursor.char_range()?, "Not a integer"),
                            action_cursor.error_info(),
                        ),
                    }
                }
                x if x == "time" => {
                    goto_next_existing_sibling!(action_cursor, errors);
                    let time_str: Cow<'_, str> = action_cursor.node_to_string_literal()?.into();
                    match time_str.parse() {
                        Ok(time) => self.time = time,
                        Err(_) => errors.append_error(
                            ParseError::Syntax(action_cursor.char_range()?, "Not a integer"),
                            action_cursor.error_info(),
                        ),
                    }
                }
                _ => errors.append_error(
                    ParseError::NotFound(action_cursor.char_range()?, "That action does not exist"),
                    action_cursor.error_info(),
                ),
            }

            if !cursor.goto_next_sibling()? {
                break;
            }
        }

        Ok(drawable_actions)
    }
}

fn parse_highlight_locations(
    cursor: &mut GrzCursor,
    errors: Arc<ErrsWithSource>,
) -> io::Result<Option<[usize; 3]>> {
    let mut locations = [0; 3];
    let locations_str: Cow<'_, str> = cursor.node_to_string_literal()?.into();
    if locations_str.is_empty() {
        return Ok(None);
    }
    let mut locations_split = locations_str.split(':').rev();
    if let Some(offset) = locations_split.next() {
        match offset.parse() {
            Ok(offset) => locations[2] = offset,
            Err(_) => errors.append_error(
                ParseError::Syntax(cursor.char_range()?, "Not a integer"),
                cursor.error_info(),
            ),
        }
    }
    if let Some(line) = locations_split.next() {
        match line.parse() {
            Ok(line) => locations[1] = line,
            Err(_) => errors.append_error(
                ParseError::Syntax(cursor.char_range()?, "Not a integer"),
                cursor.error_info(),
            ),
        }
    }
    if let Some(paragraph) = locations_split.next() {
        match paragraph.parse() {
            Ok(paragraph) => locations[0] = paragraph,
            Err(_) => errors.append_error(
                ParseError::Syntax(cursor.char_range()?, "Not a integer"),
                cursor.error_info(),
            ),
        }
    }
    Ok(Some(locations))
}
