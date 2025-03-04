use std::{
    collections::HashMap,
    io,
    ops::{Deref, DerefMut},
    sync::Arc,
};

use emath::{Align2, Rect};
use smallvec::SmallVec;
use tracing::instrument;
use tree_sitter_grz::NodeKind;

use crate::{
    parse::{
        cursor::{GrzCursor, GrzCursorGuard, GrzCursorGuardRaw},
        error::{ErrsWithSource, ParseError},
    },
    slide::{ObjPositions, ObjState, Slide, SlideObj, SlideVb, ViewboxRef},
};

use super::{
    registers::Registers,
    viewbox::{parse_index, Viewbox, ViewboxInner},
};

impl Slide {
    #[instrument(skip_all, fields(source = %cursor.parent_source()?, type = std::any::type_name::<Self>()))]
    pub fn parse(
        &mut self,
        mut cursor: GrzCursorGuard,
        viewboxes: &HashMap<
            smartstring::alias::String,
            (SmallVec<[Rect; 4]>, bool),
            ahash::RandomState,
        >,
        last_slide: &Self,
        errors: Arc<ErrsWithSource>,
    ) -> io::Result<()> {
        if cursor.node().kind_id() == NodeKind::SymSlideObjects as u16 {
            if let Some(mut slide_objs_cursor) = cursor.goto_first_child()? {
                loop {
                    if let Some(slide_obj_cursor) = slide_objs_cursor.goto_first_child()? {
                        let mut slide_obj = SlideObj::default();
                        let name = slide_obj.parse(
                            slide_obj_cursor,
                            viewboxes,
                            last_slide,
                            self.objects.last().map(|l| l.1),
                            self.create_edges,
                            Arc::clone(&errors),
                        )?;
                        self.objects.insert(name, slide_obj);
                    }

                    if !slide_objs_cursor.goto_next_sibling()? {
                        break;
                    }
                }
            }
            cursor.goto_next_sibling()?;
        }

        if let Some(actions_block_cursor) = cursor.goto_first_child()? {
            let drawable_actions = self
                .slide_params
                .parse(actions_block_cursor, Arc::clone(&errors))?;

            self.actions = drawable_actions;
        }

        Ok(())
    }

    pub fn apply_registers(&mut self, registers: &Registers) {
        self.create_edges = registers.create_edges;
        self.bg = registers.bg;
    }

    pub fn make_action(&mut self) {
        self.objects
            .retain(|_, o| matches!(o.positions.state, ObjState::Entering | ObjState::OnScreen));
        for object in self.objects.values_mut() {
            object.positions.state = ObjState::OnScreen;
            object.vb_from = object.viewbox.clone();
            object.positions.from_alignment = object.positions.to_alignment;
        }
    }
}

impl SlideObj {
    #[instrument(skip_all, fields(source = %cursor.parent_source()?, type = std::any::type_name::<Self>()))]
    pub fn parse(
        &mut self,
        mut cursor: GrzCursorGuard,
        viewboxes: &HashMap<
            smartstring::alias::String,
            (SmallVec<[Rect; 4]>, bool),
            ahash::RandomState,
        >,
        last_slide: &Slide,
        last_obj: Option<&SlideObj>,
        create_edges: bool,
        errors: Arc<ErrsWithSource>,
    ) -> io::Result<smartstring::alias::String> {
        let name = cursor.smartstring()?;
        cursor.goto_next_sibling()?;
        self.viewbox = None;
        if cursor.node().kind_id() == NodeKind::SymSlideVb as u16 {
            if let Some(vb_cursor) = cursor.goto_first_child_raw()? {
                let mut slide_vb = SlideVb::default();
                if slide_vb.parse(vb_cursor, viewboxes, last_obj, Arc::clone(&errors))? {
                    self.viewbox = Some(slide_vb);
                }
            }
            cursor.goto_next_sibling()?;
        }
        if cursor.node().kind_id() == NodeKind::SymVbRef as u16 {
            if !cursor.first_child_raw_exists() {
                errors.append_error(
                    ParseError::Missing(
                        cursor.char_range()?,
                        "The viewbox where the object should come from is missing",
                    ),
                    cursor.error_info(),
                );
                return Ok(name);
            }

            if let Some(from_cursor) = cursor.goto_first_child()? {
                let mut vb_from = ViewboxRef::default();
                if vb_from.parse(from_cursor, viewboxes, Arc::clone(&errors))? {
                    self.vb_from = Some(SlideVb::Viewbox(vb_from));
                } else {
                    self.vb_from = last_obj.and_then(|obj| {
                        let mut vb = obj.viewbox.clone()?;
                        vb.set_subbox(vb_from.subbox);
                        Some(vb)
                    });
                }
            }

            cursor.goto_next_sibling()?;
        }
        self.positions.from_alignment = None;
        self.positions.to_alignment = None;
        if cursor.node().kind_id() == NodeKind::SymEdgeParser as u16 {
            if self
                .positions
                .parse(cursor.deref_mut(), Arc::clone(&errors))?
            {
                self.positions.state = ObjState::Exiting;
            }
        }

        let mut last_slide_obj = last_slide.objects.get(&name);
        if last_slide_obj.is_none()
            || matches!(
                last_slide_obj.map(|o| o.positions.state),
                Some(ObjState::Exiting)
            )
        {
            self.positions.state = ObjState::Entering;
            last_slide_obj = None;
        }
        if self.positions.to_alignment.is_none() {
            self.positions.to_alignment = if matches!(self.positions.state, ObjState::Exiting) {
                self.positions.from_alignment
            } else {
                last_slide_obj.and_then(|o| o.positions.to_alignment)
            };
        }
        if self.positions.from_alignment.is_none()
            || matches!(self.positions.state, ObjState::Exiting)
        {
            self.positions.from_alignment = last_slide_obj.and_then(|o| o.positions.to_alignment);
        }
        if self.viewbox.is_none() {
            self.viewbox = last_slide_obj.and_then(|o| o.viewbox.clone());
        }
        if self.vb_from.is_none() {
            match self.positions.state {
                ObjState::Entering => self.vb_from = self.viewbox.clone(),
                ObjState::OnScreen | ObjState::Exiting => {
                    self.vb_from = last_slide_obj.and_then(|o| o.viewbox.clone())
                }
            }
        }
        if !create_edges
            && (self.positions.from_alignment.is_none()
                || self.positions.to_alignment.is_none()
                || self.vb_from.is_none()
                || self.viewbox.is_none())
        {
            errors.append_error(
                ParseError::NotFound(
                    cursor.char_range()?,
                    "Implicit elements could not be resolved",
                ),
                cursor.error_info(),
            );
        }
        Ok(name)
    }
}

impl SlideVb {
    #[instrument(skip_all, fields(source = %cursor.parent_source()?, type = std::any::type_name::<Self>()))]
    pub fn parse(
        &mut self,
        mut cursor: GrzCursorGuardRaw,
        viewboxes: &HashMap<
            smartstring::alias::String,
            (SmallVec<[Rect; 4]>, bool),
            ahash::RandomState,
        >,
        last_obj: Option<&SlideObj>,
        errors: Arc<ErrsWithSource>,
    ) -> io::Result<bool> {
        let token = NodeKind::from(cursor.node().kind_id());
        match token {
            NodeKind::AnonSymCOLON => {
                cursor.goto_next_sibling()?;
                let mut vb_ref = ViewboxRef::default();
                if let Some(vb_cursor) = cursor.goto_first_child()? {
                    if vb_ref.parse(vb_cursor, viewboxes, Arc::clone(&errors))? {
                        *self = Self::Viewbox(vb_ref);
                    } else {
                        *self = last_obj
                            .and_then(|obj| {
                                let mut vb = obj.viewbox.clone()?;
                                vb.set_subbox(vb_ref.subbox);
                                Some(vb)
                            })
                            .unwrap_or_default();
                    }
                } else {
                    errors.append_error(
                        ParseError::Missing(
                            cursor.char_range()?,
                            "The reference to the viewbox where \
                            the object should go to is missing",
                        ),
                        cursor.error_info(),
                    );
                }
                Ok(true)
            }
            NodeKind::AnonSymPIPE => {
                cursor.goto_next_sibling()?;
                let mut split_on = ViewboxRef::default();
                if let Some(vb_cursor) = cursor.goto_first_child()? {
                    split_on.parse(vb_cursor, viewboxes, Arc::clone(&errors))?;
                } else {
                    errors.append_error(
                        ParseError::Missing(
                            cursor.char_range()?,
                            "The viewbox which the inline \
                            viewbox should split on is missing",
                        ),
                        cursor.error_info(),
                    );
                }
                cursor.goto_next_sibling()?;
                let mut viewbox = ViewboxInner::default();
                if let Some(vb_cursor) = cursor.goto_first_child_raw()? {
                    viewbox.parse(vb_cursor, Arc::clone(&errors))?;
                } else {
                    errors.append_error(
                        ParseError::Missing(
                            cursor.char_range()?,
                            "The inner details of the \
                            inline viewbox are missing",
                        ),
                        cursor.error_info(),
                    );
                }
                cursor.goto_next_sibling()?;
                let subbox = parse_index(cursor.deref_mut(), Arc::clone(&errors))?;

                let mut viewbox = Viewbox {
                    split_on,
                    inner: viewbox,
                    ..Default::default()
                };

                let split = viewbox.split(cursor.deref(), None, viewboxes, Arc::clone(&errors))?;

                *self = Self::InnerVb { split, subbox };
                Ok(true)
            }
            NodeKind::AnonSymTILDE => Ok(false),
            n => {
                errors.append_error(
                    ParseError::BadNode(
                        cursor.char_range()?,
                        n,
                        "Expected one of `:`, `|`, or `~`",
                    ),
                    cursor.error_info(),
                );
                Ok(true)
            }
        }
    }
}

impl ObjPositions {
    #[instrument(skip_all, fields(source = %cursor.rope_slice()?, type = std::any::type_name::<Self>()))]
    pub fn parse(
        &mut self,
        cursor: &mut GrzCursor,
        errors: Arc<ErrsWithSource>,
    ) -> io::Result<bool> {
        let mut exiting = false;
        if let Some(mut edge_parser) = cursor.goto_first_child()? {
            if let Some(first_alignment_cursor) = edge_parser.goto_first_child_raw()? {
                let (first_alignment, obj_exiting) =
                    parse_alignment(first_alignment_cursor, Arc::clone(&errors))?;
                self.to_alignment = first_alignment;

                if obj_exiting {
                    exiting = obj_exiting;
                }

                if !edge_parser.goto_next_sibling()? {
                    return Ok(exiting);
                }

                if let Some(second_alignment_cursor) = edge_parser.goto_first_child_raw()? {
                    self.from_alignment = first_alignment;
                    let (second_alignment, obj_exiting) =
                        parse_alignment(second_alignment_cursor, Arc::clone(&errors))?;
                    self.to_alignment = second_alignment;
                    if obj_exiting {
                        exiting = obj_exiting;
                    }
                }
            }
        }

        Ok(exiting)
    }
}

#[instrument(skip_all, fields(source = %cursor.rope_slice()?))]
pub fn parse_alignment_from_chars(
    cursor: &mut GrzCursor,
    errors: Arc<ErrsWithSource>,
) -> io::Result<(Option<Align2>, bool)> {
    let mut chars = cursor.rope_slice()?.chars();
    let align = match chars.next() {
        Some('|') => {
            return Ok((None, true));
        }
        // ^
        Some('^') => match chars.next() {
            // ^^
            Some('^') => Align2::CENTER_TOP,
            // ^_
            Some('_') => Align2::CENTER_CENTER,
            // ^<
            Some('<') => Align2::LEFT_TOP,
            // ^>
            Some('>') => Align2::RIGHT_TOP,
            // ^.
            Some('.') => Align2::CENTER_TOP,
            _ => {
                errors.append_error(
                    ParseError::Syntax(
                        cursor.char_range()?,
                        "Expected one of `^`, `_`, `<`, `>`, or `.`",
                    ),
                    cursor.error_info(),
                );
                return Ok((None, false));
            }
        },
        // _
        Some('_') => match chars.next() {
            // _^
            Some('^') => Align2::CENTER_CENTER,
            // __
            Some('_') => Align2::CENTER_BOTTOM,
            // _<
            Some('<') => Align2::LEFT_BOTTOM,
            // _>
            Some('>') => Align2::RIGHT_BOTTOM,
            // _.
            Some('.') => Align2::CENTER_BOTTOM,
            _ => {
                errors.append_error(
                    ParseError::Syntax(
                        cursor.char_range()?,
                        "Expected one of `^`, `_`, `<`, `>`, or `.`",
                    ),
                    cursor.error_info(),
                );
                return Ok((None, false));
            }
        },
        // <
        Some('<') => match chars.next() {
            // <^
            Some('^') => Align2::LEFT_TOP,
            // <_
            Some('_') => Align2::LEFT_BOTTOM,
            // <<
            Some('<') => Align2::LEFT_CENTER,
            // <>
            Some('>') => Align2::CENTER_CENTER,
            // <.
            Some('.') => Align2::LEFT_CENTER,
            _ => {
                errors.append_error(
                    ParseError::Syntax(
                        cursor.char_range()?,
                        "Expected one of `^`, `_`, `<`, `>`, or `.`",
                    ),
                    cursor.error_info(),
                );
                return Ok((None, false));
            }
        },
        // >
        Some('>') => match chars.next() {
            // >^
            Some('^') => Align2::RIGHT_TOP,
            // >_
            Some('_') => Align2::RIGHT_BOTTOM,
            // ><
            Some('<') => Align2::CENTER_CENTER,
            // >>
            Some('>') => Align2::RIGHT_CENTER,
            // >.
            Some('.') => Align2::RIGHT_CENTER,
            _ => {
                errors.append_error(
                    ParseError::Syntax(
                        cursor.char_range()?,
                        "Expected one of `^`, `_`, `<`, `>`, or `.`",
                    ),
                    cursor.error_info(),
                );
                return Ok((None, false));
            }
        },
        // .
        Some('.') => match chars.next() {
            // .^
            Some('^') => Align2::CENTER_TOP,
            // ._
            Some('_') => Align2::CENTER_BOTTOM,
            // .<
            Some('<') => Align2::LEFT_CENTER,
            // .>
            Some('>') => Align2::RIGHT_CENTER,
            // ..
            Some('.') => Align2::CENTER_CENTER,
            _ => {
                errors.append_error(
                    ParseError::Syntax(
                        cursor.char_range()?,
                        "Expected one of `^`, `_`, `<`, `>`, or `.`",
                    ),
                    cursor.error_info(),
                );
                return Ok((None, false));
            }
        },
        _ => {
            errors.append_error(
                ParseError::Syntax(
                    cursor.char_range()?,
                    "Expected one of `|`, `^`, `_`, `<`, `>`, or `.`",
                ),
                cursor.error_info(),
            );
            return Ok((None, false));
        }
    };

    Ok((Some(align), false))
}

#[instrument(skip_all, fields(source = %cursor.rope_slice()?))]
pub fn parse_alignment(
    mut cursor: GrzCursorGuardRaw,
    errors: Arc<ErrsWithSource>,
) -> io::Result<(Option<Align2>, bool)> {
    let align = match NodeKind::from(cursor.node().kind_id()) {
        NodeKind::AnonSymPIPE => {
            return Ok((None, true));
        }
        NodeKind::SymDirection => {
            match NodeKind::from(cursor.goto_first_child_raw()?.unwrap().node().kind_id()) {
                // ^
                NodeKind::AnonSymCARET => {
                    let next_node = {
                        cursor.goto_next_sibling()?;
                        NodeKind::from(cursor.goto_first_child_raw()?.unwrap().node().kind_id())
                    };
                    match next_node {
                        // ^^
                        NodeKind::AnonSymCARET => Align2::CENTER_TOP,
                        // ^_
                        NodeKind::AnonSym => Align2::CENTER_CENTER,
                        // ^<
                        NodeKind::AnonSymLT => Align2::LEFT_TOP,
                        // ^>
                        NodeKind::AnonSymGT => Align2::RIGHT_TOP,
                        // ^.
                        NodeKind::AnonSymDOT => Align2::CENTER_TOP,
                        n => {
                            errors.append_error(
                                ParseError::BadNode(
                                    cursor.char_range()?,
                                    n,
                                    "Expected one of `^`, `_`, `<`, `>`, or `.`",
                                ),
                                cursor.error_info(),
                            );
                            return Ok((None, false));
                        }
                    }
                }
                // _
                NodeKind::AnonSym => {
                    let next_node = {
                        cursor.goto_next_sibling()?;
                        NodeKind::from(cursor.goto_first_child_raw()?.unwrap().node().kind_id())
                    };
                    match next_node {
                        // _^
                        NodeKind::AnonSymCARET => Align2::CENTER_CENTER,
                        // __
                        NodeKind::AnonSym => Align2::CENTER_BOTTOM,
                        // _<
                        NodeKind::AnonSymLT => Align2::LEFT_BOTTOM,
                        // _>
                        NodeKind::AnonSymGT => Align2::RIGHT_BOTTOM,
                        // _.
                        NodeKind::AnonSymDOT => Align2::CENTER_BOTTOM,
                        n => {
                            errors.append_error(
                                ParseError::BadNode(
                                    cursor.char_range()?,
                                    n,
                                    "Expected one of `^`, `_`, `<`, `>`, or `.`",
                                ),
                                cursor.error_info(),
                            );
                            return Ok((None, false));
                        }
                    }
                }
                // <
                NodeKind::AnonSymLT => {
                    let next_node = {
                        cursor.goto_next_sibling()?;
                        NodeKind::from(cursor.goto_first_child_raw()?.unwrap().node().kind_id())
                    };
                    match next_node {
                        // <^
                        NodeKind::AnonSymCARET => Align2::LEFT_TOP,
                        // <_
                        NodeKind::AnonSym => Align2::LEFT_BOTTOM,
                        // <<
                        NodeKind::AnonSymLT => Align2::LEFT_CENTER,
                        // <>
                        NodeKind::AnonSymGT => Align2::CENTER_CENTER,
                        // <.
                        NodeKind::AnonSymDOT => Align2::LEFT_CENTER,
                        n => {
                            errors.append_error(
                                ParseError::BadNode(
                                    cursor.char_range()?,
                                    n,
                                    "Expected one of `^`, `_`, `<`, `>`, or `.`",
                                ),
                                cursor.error_info(),
                            );
                            return Ok((None, false));
                        }
                    }
                }
                // >
                NodeKind::AnonSymGT => {
                    let next_node = {
                        cursor.goto_next_sibling()?;
                        NodeKind::from(cursor.goto_first_child_raw()?.unwrap().node().kind_id())
                    };
                    match next_node {
                        // >^
                        NodeKind::AnonSymCARET => Align2::RIGHT_TOP,
                        // >_
                        NodeKind::AnonSym => Align2::RIGHT_BOTTOM,
                        // ><
                        NodeKind::AnonSymLT => Align2::CENTER_CENTER,
                        // >>
                        NodeKind::AnonSymGT => Align2::RIGHT_CENTER,
                        // >.
                        NodeKind::AnonSymDOT => Align2::RIGHT_CENTER,
                        n => {
                            errors.append_error(
                                ParseError::BadNode(
                                    cursor.char_range()?,
                                    n,
                                    "Expected one of `^`, `_`, `<`, `>`, or `.`",
                                ),
                                cursor.error_info(),
                            );
                            return Ok((None, false));
                        }
                    }
                }
                // .
                NodeKind::AnonSymDOT => {
                    let next_node = {
                        cursor.goto_next_sibling()?;
                        NodeKind::from(cursor.goto_first_child_raw()?.unwrap().node().kind_id())
                    };
                    match next_node {
                        // .^
                        NodeKind::AnonSymCARET => Align2::CENTER_TOP,
                        // ._
                        NodeKind::AnonSym => Align2::CENTER_BOTTOM,
                        // .<
                        NodeKind::AnonSymLT => Align2::LEFT_CENTER,
                        // .>
                        NodeKind::AnonSymGT => Align2::RIGHT_CENTER,
                        // ..
                        NodeKind::AnonSymDOT => Align2::CENTER_CENTER,
                        n => {
                            errors.append_error(
                                ParseError::BadNode(
                                    cursor.char_range()?,
                                    n,
                                    "Expected one of `^`, `_`, `<`, `>`, or `.`",
                                ),
                                cursor.error_info(),
                            );
                            return Ok((None, false));
                        }
                    }
                }
                n => {
                    errors.append_error(
                        ParseError::BadNode(
                            cursor.char_range()?,
                            n,
                            "Expected one of `^`, `_`, `<`, `>`, or `.`",
                        ),
                        cursor.error_info(),
                    );
                    return Ok((None, false));
                }
            }
        }

        n => {
            errors.append_error(
                ParseError::BadNode(
                    cursor.char_range()?,
                    n,
                    "Expected one of `|`, `^`, `_`, `<`, `>`, or `.`",
                ),
                cursor.error_info(),
            );
            return Ok((None, false));
        }
    };

    Ok((Some(align), false))
}
