use std::{io, path::Path, sync::Arc};

use registers::Registers;
use tracing::instrument;
use tree_sitter_grz::NodeKind;
use viewbox::Viewbox;

use crate::{object::Object, parse::error::ErrsWithSource, slide::Slide, GrzRoot};

use super::{cursor::GrzCursorGuard, error::ParseError, IncrementalState};

pub mod actions;
pub mod object;
pub mod registers;
pub mod slide;
pub mod text;
pub mod viewbox;

impl GrzRoot {
    #[instrument(skip_all, fields(type = std::any::type_name::<Self>()))]
    pub fn parse(
        &mut self,
        incremental_state: Option<IncrementalState>,
        mut cursor: GrzCursorGuard,
        path_to_grz: &Path,
        errors: Arc<ErrsWithSource>,
    ) -> io::Result<IncrementalState> {
        let mut incremental_state = incremental_state.unwrap_or_default();
        let mut registers = Registers::default();

        // If slide isn't present, it gets deleted
        for (_, slide) in self.slides.iter_mut() {
            slide.present = false;
        }
        // If viewbox isn't present, it gets deleted
        for (_, viewbox) in self.viewboxes.iter_mut() {
            viewbox.1 = false;
        }
        // If object isn't present, it gets deleted
        for (_, object) in self.objects.iter_mut() {
            object.present = false;
        }

        let default_slide = Slide::default();
        let mut last_slide = 0;
        let mut last_slide_changed = true;

        loop {
            let node = cursor.node();
            match NodeKind::from(node.kind_id()) {
                NodeKind::SymSlide => {
                    let id = cursor.node().id() as u64;
                    if let Some(slide) = self.slides.get_mut(&id).and_then(|s| {
                        if last_slide_changed {
                            None
                        } else {
                            Some(s)
                        }
                    }) {
                        slide.present = true;
                        slide.apply_registers(&registers);
                    } else if let Some(slide_tree_cursor) = cursor.goto_first_child()? {
                        let mut slide = Slide::default();
                        slide.apply_registers(&registers);
                        slide.parse(
                            slide_tree_cursor,
                            &self.viewboxes,
                            self.slides.get(&last_slide).unwrap_or(&default_slide),
                            Arc::clone(&errors),
                        )?;
                        last_slide_changed = self.slides.insert(id, slide).is_none();
                    }
                    last_slide = id;
                }
                NodeKind::SymViewbox => {
                    let id = cursor.node().id() as u64;
                    if let Some((vb, viewbox)) = incremental_state.viewbox_nodes.get_mut(&id) {
                        let split = if viewbox.damaged(&self.viewboxes, &registers) {
                            Some(viewbox.split(
                                &cursor,
                                Some(&registers),
                                &self.viewboxes,
                                Arc::clone(&errors),
                            )?)
                        } else {
                            None
                        };
                        if let Some(vb) = self.viewboxes.get_mut(vb) {
                            if let Some(split) = split {
                                vb.0 = split;
                            }
                            vb.1 = true;
                        }
                    } else if let Some(viewbox_tree_cursor) = cursor.goto_first_child()? {
                        let mut viewbox = Viewbox::default();
                        let viewbox_name = viewbox.parse(
                            viewbox_tree_cursor,
                            &self.viewboxes,
                            Arc::clone(&errors),
                        )?;
                        let split = viewbox.split(
                            &cursor,
                            Some(&registers),
                            &self.viewboxes,
                            Arc::clone(&errors),
                        )?;
                        incremental_state
                            .viewbox_nodes
                            .insert(id, (viewbox_name.clone(), viewbox));
                        self.viewboxes.insert(viewbox_name, (split, true));
                    }
                }
                NodeKind::SymObj => {
                    let id = cursor.node().id() as u64;
                    if let Some(object) = incremental_state
                        .object_nodes
                        .get(&id)
                        .and_then(|obj_name| self.objects.get_mut(obj_name))
                    {
                        object.present = true;
                        object.apply_registers(&registers);
                    } else if let Some(object_tree_cursor) = cursor.goto_first_child()? {
                        let mut object = Object::default();
                        object.apply_registers(&registers);
                        let object_name =
                            object.parse(object_tree_cursor, path_to_grz, Arc::clone(&errors))?;
                        incremental_state
                            .object_nodes
                            .insert(id, object_name.clone());
                        self.objects.insert(object_name, object);
                    }
                }
                NodeKind::SymRegister => {
                    if let Some(register_cursor) = cursor.goto_first_child_raw()? {
                        registers.parse(register_cursor, Arc::clone(&errors))?;
                    }
                }
                NodeKind::SymActions => {
                    let id = cursor.node().id() as u64;
                    // Edge case: Slide followed by many many action blocks
                    // may cause incremental parsing to take performance hit
                    if let Some(action) = self.slides.get_mut(&id).and_then(|s| {
                        if last_slide_changed {
                            None
                        } else {
                            Some(s)
                        }
                    }) {
                        action.present = true;
                        action.apply_registers(&registers);
                    } else if let Some(action_tree_cursor) = cursor.goto_first_child()? {
                        // Actions draw the objects from the previous
                        // slide
                        let mut action = Slide::default();
                        action.apply_registers(&registers);
                        let last_slide = self.slides.get(&last_slide).unwrap_or(&default_slide);
                        // TODO: Borrow instead of clone
                        action.objects = last_slide.objects.clone();
                        action.parse(
                            action_tree_cursor,
                            &self.viewboxes,
                            last_slide,
                            Arc::clone(&errors),
                        )?;
                        action.make_action();
                        action.actions.extend(last_slide.actions.iter().cloned());
                        self.slides.insert(id, action);
                    }
                }
                kind => errors.append_error(
                    ParseError::BadNode(cursor.char_range()?, kind, "Expected a top level object"),
                    cursor.error_info(),
                ),
            }

            if !cursor.goto_next_sibling()? {
                break;
            }
        }

        self.slides.retain(|_, slide| slide.present);
        self.objects.retain(|_, object| object.present);
        self.viewboxes.retain(|_, viewbox| viewbox.1);

        Ok(incremental_state)
    }
}
