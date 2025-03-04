use std::{borrow::Cow, collections::HashMap, io, ops::DerefMut, sync::Arc};

use emath::{Pos2, Rect};
use grezi_layout::{Constraint, Direction, Layout};
use smallvec::SmallVec;
use smart_default::SmartDefault;
use tracing::instrument;
use tree_sitter_grz::NodeKind;

use crate::{
    parse::{
        cursor::{GrzCursor, GrzCursorGuard, GrzCursorGuardRaw},
        error::{ErrsWithSource, ParseError},
    },
    slide::{VbIdentifier, ViewboxRef, BASE_SIZE},
};

use super::registers::Registers;

#[derive(SmartDefault, Debug)]
pub struct Viewbox {
    pub split_on: ViewboxRef,
    #[default(Rect::ZERO)]
    pub split_rect: Rect,
    pub inner: ViewboxInner,
    pub margin: f32,
    pub margin_per: f32,
    #[cfg(feature = "parse")]
    // Used to remove viewboxes that are no longer in the syntax tree
    #[default = true]
    pub present: bool,
}

// Used for both Viewbox objects
// and inline viewboxes
#[derive(Default, Debug)]
pub struct ViewboxInner {
    pub direction: Direction,
    pub constraints: SmallVec<[Constraint; 5]>,
}

impl Viewbox {
    #[instrument(skip_all, fields(source = %cursor.parent_source()?, type = std::any::type_name::<Self>()))]
    pub fn parse(
        &mut self,
        mut cursor: GrzCursorGuard,
        viewboxes: &HashMap<
            smartstring::alias::String,
            (SmallVec<[Rect; 4]>, bool),
            ahash::RandomState,
        >,
        errors: Arc<ErrsWithSource>,
    ) -> io::Result<smartstring::alias::String> {
        let name = cursor.smartstring()?;
        cursor.goto_next_sibling()?;
        if let Some(vb_cursor) = cursor.goto_first_child()? {
            self.split_on
                .parse(vb_cursor, viewboxes, Arc::clone(&errors))?;
        } else {
            errors.append_error(
                ParseError::Missing(
                    cursor.char_range()?,
                    "Missing reference to viewbox which should be split on",
                ),
                cursor.error_info(),
            );
        }
        cursor.goto_next_sibling()?;
        if let Some(vb_cursor) = cursor.goto_first_child_raw()? {
            self.inner.parse(vb_cursor, Arc::clone(&errors))?;
        } else {
            errors.append_error(
                ParseError::Missing(
                    cursor.char_range()?,
                    "Missing inner details of viewbox object",
                ),
                cursor.error_info(),
            );
        }
        Ok(name)
    }

    fn get_split_rect(
        &self,
        viewboxes: &HashMap<
            smartstring::alias::String,
            (SmallVec<[Rect; 4]>, bool),
            ahash::RandomState,
        >,
    ) -> Rect {
        match &self.split_on.vb_name {
            // Should be validated before function is called
            VbIdentifier::Named(n) => {
                let Some(vb) = viewboxes.get(n) else {
                    return Rect::ZERO;
                };
                let Some(vb) = vb.0.get(self.split_on.subbox) else {
                    return Rect::ZERO;
                };
                *vb
            }
            VbIdentifier::Size => BASE_SIZE,
            VbIdentifier::Rect(r) => *r,
        }
    }

    /// Needs to regenerate split because registers
    /// changed
    pub fn damaged(
        &self,
        viewboxes: &HashMap<
            smartstring::alias::String,
            (SmallVec<[Rect; 4]>, bool),
            ahash::RandomState,
        >,
        registers: &Registers,
    ) -> bool {
        self.margin != registers.margin
            || self.margin_per != registers.margin_per
            || self.get_split_rect(viewboxes) != self.split_rect
    }

    pub fn split(
        &mut self,
        cursor: &GrzCursor,
        registers: Option<&Registers>,
        viewboxes: &HashMap<
            smartstring::alias::String,
            (SmallVec<[Rect; 4]>, bool),
            ahash::RandomState,
        >,
        errors: Arc<ErrsWithSource>,
    ) -> io::Result<SmallVec<[Rect; 4]>> {
        if let Some(registers) = registers {
            self.margin = registers.margin;
            self.margin_per = registers.margin_per;
        }
        let split_rect = self.get_split_rect(viewboxes);
        self.split_rect = split_rect;
        let layout = match Layout::new(self.inner.direction, &self.inner.constraints)
            .margin(self.margin)
            .try_split(self.split_rect)
        {
            Ok((layout, _)) => layout,
            Err(e) => {
                errors.append_error(
                    ParseError::Viewbox(cursor.char_range()?, format!("{:?}", e)),
                    cursor.error_info(),
                );
                SmallVec::new()
            }
        };

        Ok(layout)
    }
}

impl ViewboxInner {
    #[instrument(skip_all, fields(source = %cursor.parent_source()?, type = std::any::type_name::<Self>()))]
    pub fn parse(
        &mut self,
        mut cursor: GrzCursorGuardRaw,
        errors: Arc<ErrsWithSource>,
    ) -> io::Result<()> {
        if let Some(direction_cursor) = cursor.goto_first_child_raw()? {
            self.direction = parse_direction(direction_cursor, Arc::clone(&errors))?;
        }
        cursor.goto_next_sibling()?;
        loop {
            if let Some(vb_obj_cursor) = cursor.goto_first_child()? {
                let constraint = parse_constraint(vb_obj_cursor, Arc::clone(&errors))?;
                self.constraints.push(constraint);
            }

            if !cursor.goto_next_sibling()? {
                break;
            }
        }
        Ok(())
    }
}

#[instrument(skip_all, fields(source = %cursor.parent_source()?, type = "grezi_layout::Constraint"))]
pub fn parse_constraint(
    mut cursor: GrzCursorGuard,
    errors: Arc<ErrsWithSource>,
) -> io::Result<Constraint> {
    let numerator: Cow<'_, str> = cursor.rope_slice()?.into();
    let numerator: f64 = match numerator.parse() {
        Ok(n) => n,
        Err(_) => {
            errors.append_error(
                ParseError::Syntax(cursor.char_range()?, "Not a valid 64-bit float"),
                cursor.error_info(),
            );
            0.0
        }
    };
    cursor.goto_next_sibling_raw()?;
    match cursor
        .goto_first_child_raw()?
        .map(|c| NodeKind::from(c.node().kind_id()))
        .unwrap_or_else(|| NodeKind::from(cursor.node().kind_id()))
    {
        NodeKind::AnonSymPERCENT => Ok(Constraint::Percentage(numerator)),
        NodeKind::AnonSymPLUS => Ok(Constraint::Max(numerator)),
        NodeKind::AnonSymTILDE => Ok(Constraint::Length(numerator)),
        NodeKind::AnonSymDASH => Ok(Constraint::Min(numerator)),
        NodeKind::AnonSymPOUND => Ok(Constraint::Fill(numerator)),
        NodeKind::AnonSymCOLON => {
            cursor.goto_next_sibling()?;
            let denominator: Cow<'_, str> = cursor.rope_slice()?.into();
            let denominator: f64 = match denominator.parse() {
                Ok(n) => n,
                Err(_) => {
                    errors.append_error(
                        ParseError::Syntax(cursor.char_range()?, "Not a valid 64-bit float"),
                        cursor.error_info(),
                    );
                    0.0
                }
            };
            Ok(Constraint::Ratio(numerator, denominator))
        }
        n => {
            errors.append_error(
                ParseError::BadNode(
                    cursor.char_range()?,
                    n,
                    "Expected one of `%`, `+`, `~`, `-`, `#`, or `:`",
                ),
                cursor.error_info(),
            );
            Ok(Constraint::default())
        }
    }
}

impl ViewboxRef {
    #[instrument(skip_all, fields(source = %cursor.parent_source()?, type = std::any::type_name::<Self>()))]
    pub fn parse(
        &mut self,
        mut cursor: GrzCursorGuard,
        viewboxes: &HashMap<
            smartstring::alias::String,
            (SmallVec<[Rect; 4]>, bool),
            ahash::RandomState,
        >,
        errors: Arc<ErrsWithSource>,
    ) -> io::Result<bool> {
        let non_inherited_vb = self
            .vb_name
            .parse(cursor.deref_mut(), Arc::clone(&errors))?;
        if let VbIdentifier::Named(ref n) = self.vb_name {
            if !non_inherited_vb {
                cursor.goto_next_sibling()?;
                self.subbox = parse_index(cursor.deref_mut(), Arc::clone(&errors))?;
                return Ok(non_inherited_vb);
            }
            if let Some((vb, _)) = viewboxes.get(n) {
                cursor.goto_next_sibling()?;
                self.subbox = parse_index(cursor.deref_mut(), Arc::clone(&errors))?;
                if vb.len() < self.subbox {
                    errors.append_error(
                        ParseError::NotFound(cursor.char_range()?, "Cannot find that subbox"),
                        cursor.error_info(),
                    );
                }
            } else {
                errors.append_error(
                    ParseError::NotFound(cursor.char_range()?, "Cannot find that viewbox"),
                    cursor.error_info(),
                );
            }
        }
        Ok(non_inherited_vb)
    }
}

impl VbIdentifier {
    #[instrument(skip_all, fields(source = %cursor.rope_slice()?, type = std::any::type_name::<Self>()))]
    pub fn parse(
        &mut self,
        cursor: &mut GrzCursor,
        errors: Arc<ErrsWithSource>,
    ) -> io::Result<bool> {
        match NodeKind::from(cursor.node().kind_id()) {
            NodeKind::AnonSymSize => *self = Self::Size,
            NodeKind::AnonSymLPARENRPAREN => {
                *self = Self::Named("()".into());
                return Ok(false);
            }
            NodeKind::SymVbRect => {
                if let Some(mut vb_rect_cursor) = cursor.goto_first_child()? {
                    macro_rules! parse_pos2 {
                        ($cursor:expr) => {{
                            let x: Cow<'_, str> = $cursor.node_to_string_literal()?.into();
                            let x: f32 = match x.parse() {
                                Ok(n) => n,
                                Err(_) => {
                                    errors.append_error(
                                        ParseError::Syntax(
                                            $cursor.char_range()?,
                                            "Not a valid 32-bit float",
                                        ),
                                        $cursor.error_info(),
                                    );
                                    0.0
                                }
                            };
                            $cursor.goto_next_sibling()?;
                            let y: Cow<'_, str> = $cursor.node_to_string_literal()?.into();
                            let y: f32 = match y.parse() {
                                Ok(n) => n,
                                Err(_) => {
                                    errors.append_error(
                                        ParseError::Syntax(
                                            $cursor.char_range()?,
                                            "Not a valid 32-bit float",
                                        ),
                                        $cursor.error_info(),
                                    );
                                    0.0
                                }
                            };

                            Pos2::new(x, y)
                        }};
                    }
                    let min =
                        if let Some(mut rect_min_cursor) = vb_rect_cursor.goto_first_child()? {
                            parse_pos2!(rect_min_cursor)
                        } else {
                            errors.append_error(
                                ParseError::Missing(
                                    vb_rect_cursor.char_range()?,
                                    "Missing inner details of viewbox rect",
                                ),
                                vb_rect_cursor.error_info(),
                            );
                            Pos2::ZERO
                        };
                    vb_rect_cursor.goto_next_sibling()?;
                    let max =
                        if let Some(mut rect_max_cursor) = vb_rect_cursor.goto_first_child()? {
                            parse_pos2!(rect_max_cursor)
                        } else {
                            errors.append_error(
                                ParseError::Missing(
                                    vb_rect_cursor.char_range()?,
                                    "Missing inner details of viewbox rect",
                                ),
                                vb_rect_cursor.error_info(),
                            );
                            Pos2::ZERO
                        };
                    *self = Self::Rect(Rect::from_min_max(min, max));
                } else {
                    errors.append_error(
                        ParseError::Missing(
                            cursor.char_range()?,
                            "Missing inner details of viewbox rect",
                        ),
                        cursor.error_info(),
                    );
                }
            }
            NodeKind::SymIdentifier => *self = Self::Named(cursor.smartstring()?),
            node => errors.append_error(
                ParseError::BadNode(
                    cursor.char_range()?,
                    node,
                    "Expected one of `Size`, `()`, \
                    or some other identifier",
                ),
                cursor.error_info(),
            ),
        }
        Ok(true)
    }
}

#[instrument(skip_all, fields(source = %cursor.parent_source()?, type = "grezi_layout::Direction"))]
pub fn parse_direction(
    cursor: GrzCursorGuardRaw,
    errors: Arc<ErrsWithSource>,
) -> io::Result<Direction> {
    match NodeKind::from(cursor.node().kind_id()) {
        NodeKind::AnonSymCARET | NodeKind::AnonSym => Ok(Direction::Vertical),
        NodeKind::AnonSymGT | NodeKind::AnonSymLT => Ok(Direction::Horizontal),
        n => {
            errors.append_error(
                ParseError::BadNode(
                    cursor.char_range()?,
                    n,
                    "Expected one of `^`, `_`, `<`, or `>`",
                ),
                cursor.error_info(),
            );
            Ok(Direction::default())
        }
    }
}

#[instrument(skip_all, fields(source = %cursor.rope_slice()?))]
pub fn parse_index(cursor: &mut GrzCursor, errors: Arc<ErrsWithSource>) -> io::Result<usize> {
    if !cursor.first_child_exists() {
        errors.append_error(
            ParseError::Missing(cursor.char_range()?, "The index here is missing"),
            cursor.error_info(),
        );
    }
    if let Some(idx_cursor) = cursor.goto_first_child()? {
        let number: Cow<'_, str> = idx_cursor.rope_slice()?.into();
        match number.parse() {
            Ok(n) => Ok(n),
            Err(_) => {
                errors.append_error(
                    ParseError::Syntax(idx_cursor.char_range()?, "Not a valid array index"),
                    cursor.error_info(),
                );
                Ok(0)
            }
        }
    } else {
        Ok(0)
    }
}
