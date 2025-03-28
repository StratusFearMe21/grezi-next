use std::{
    collections::HashSet,
    hash::BuildHasherDefault,
    io,
    sync::Arc,
    time::{Duration, Instant},
};

use indexmap::IndexMap;
use prehash::Passthru;
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
        damaged_node_map: HashSet<u64, BuildHasherDefault<Passthru>>,
        mut cursor: GrzCursorGuard,
        path_to_grz: &str,
        errors: Arc<ErrsWithSource>,
    ) -> io::Result<IncrementalState> {
        if incremental_state.is_none() {
            self.slides.clear();
        }
        let mut incremental_state = incremental_state.unwrap_or_default();
        let mut registers = Registers::default();
        let mut current_slides: IndexMap<u64, Slide, BuildHasherDefault<Passthru>> =
            IndexMap::with_capacity_and_hasher(
                self.slides.capacity(),
                BuildHasherDefault::default(),
            );

        // If viewbox isn't present, it gets deleted
        for (_, viewbox) in self.viewboxes.iter_mut() {
            viewbox.1 = false;
        }
        // If object isn't present, it gets deleted
        for (_, object) in self.objects.iter_mut() {
            object.present = false;
        }

        let mut reused_slides = 0;
        let mut slide_time = Duration::from_secs(0);
        let mut viewbox_time = Duration::from_secs(0);
        let mut object_time = Duration::from_secs(0);
        let mut register_time = Duration::from_secs(0);

        let default_slide = Slide::default();
        let mut last_slide_changed = false;

        loop {
            let node = cursor.node();
            match NodeKind::from(node.kind_id()) {
                NodeKind::SymSlide => {
                    let slide_instant = Instant::now();
                    let id = cursor.id(2, NodeKind::SymSlide)?;
                    if let Some(mut slide) = self.slides.swap_remove(&id) {
                        reused_slides += 1;
                        slide.apply_registers(&registers);
                        current_slides.insert(id, slide);
                        last_slide_changed = false;
                    } else if let Some(slide_tree_cursor) =
                        cursor.goto_first_child(NodeKind::SymSlide)?
                    {
                        let mut slide = Slide::default();
                        slide.apply_registers(&registers);
                        slide.parse(
                            slide_tree_cursor,
                            &self.viewboxes,
                            current_slides.last().map(|s| s.1).unwrap_or(&default_slide),
                            Arc::clone(&errors),
                        )?;
                        current_slides.insert(id, slide);
                        last_slide_changed = true;
                    }
                    slide_time += slide_instant.elapsed();
                }
                NodeKind::SymViewbox => {
                    let viewbox_instant = Instant::now();
                    let id = cursor.node().id() as u64;
                    if let Some(mut viewbox_tree_cursor) =
                        cursor.goto_first_child(NodeKind::SymViewbox)?
                    {
                        let viewbox_name = viewbox_tree_cursor.smartstring()?;
                        viewbox_tree_cursor.goto_next_sibling()?;
                        if let Some(viewbox) =
                            incremental_state.viewbox_nodes.get_mut(&viewbox_name)
                        {
                            let node_damaged = damaged_node_map.contains(&id);
                            if node_damaged {
                                *viewbox = Viewbox::default();
                                viewbox.parse(
                                    viewbox_tree_cursor,
                                    &self.viewboxes,
                                    Arc::clone(&errors),
                                )?;
                            }
                            let split =
                                if node_damaged || viewbox.damaged(&self.viewboxes, &registers) {
                                    Some(viewbox.split(
                                        &cursor,
                                        Some(&registers),
                                        &self.viewboxes,
                                        Arc::clone(&errors),
                                    )?)
                                } else {
                                    None
                                };
                            if let Some(vb) = self.viewboxes.get_mut(&viewbox_name) {
                                if let Some(split) = split {
                                    vb.0 = split;
                                }
                                vb.1 = true;
                            }
                        } else {
                            let mut viewbox = Viewbox::default();
                            viewbox.parse(
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
                                .insert(viewbox_name.clone(), viewbox);
                            self.viewboxes.insert(viewbox_name, (split, true));
                        }
                    }
                    viewbox_time += viewbox_instant.elapsed();
                }
                NodeKind::SymObj => {
                    let object_instant = Instant::now();
                    let id = cursor.node().id() as u64;
                    if let Some(mut object_tree_cursor) =
                        cursor.goto_first_child(NodeKind::SymObj)?
                    {
                        let object_name = object_tree_cursor.smartstring()?;
                        object_tree_cursor.goto_next_sibling()?;
                        if let Some(object) = self.objects.get_mut(&object_name) {
                            if damaged_node_map.contains(&id) {
                                *object = Object::default();
                                object.apply_registers(&registers);
                                object.parse(
                                    object_tree_cursor,
                                    path_to_grz,
                                    Arc::clone(&errors),
                                )?;
                            } else {
                                object.present = true;
                                object.apply_registers(&registers);
                            }
                        } else {
                            let mut object = Object::default();
                            object.apply_registers(&registers);
                            object.parse(object_tree_cursor, path_to_grz, Arc::clone(&errors))?;
                            self.objects.insert(object_name, object);
                        }
                    }
                    object_time += object_instant.elapsed();
                }
                NodeKind::SymRegister => {
                    let register_instant = Instant::now();
                    if let Some(register_cursor) =
                        cursor.goto_first_child_raw(NodeKind::SymRegister)?
                    {
                        registers.parse(register_cursor, Arc::clone(&errors))?;
                    }
                    register_time += register_instant.elapsed();
                }
                NodeKind::SymActions => {
                    let action_instant = Instant::now();
                    let id = cursor.id(1, NodeKind::SymActions)?;
                    if let Some(mut action) = self.slides.swap_remove(&id).and_then(|action| {
                        if last_slide_changed {
                            None
                        } else {
                            Some(action)
                        }
                    }) {
                        reused_slides += 1;
                        action.apply_registers(&registers);
                        current_slides.insert(id, action);
                    } else if let Some(action_tree_cursor) =
                        cursor.goto_first_child(NodeKind::SymActions)?
                    {
                        // Actions draw the objects from the previous
                        // slide
                        let mut action = Slide::default();
                        action.apply_registers(&registers);
                        // TODO: Borrow instead of clone
                        let last_slide =
                            current_slides.last().map(|s| s.1).unwrap_or(&default_slide);
                        action.objects = last_slide.objects.clone();
                        action.parse(
                            action_tree_cursor,
                            &self.viewboxes,
                            last_slide,
                            Arc::clone(&errors),
                        )?;
                        action.make_action();
                        action.actions.extend(last_slide.actions.iter().cloned());
                        current_slides.insert(id, action);
                    }
                    slide_time += action_instant.elapsed();
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

        tracing::warn!(
            ?slide_time,
            ?register_time,
            ?object_time,
            ?viewbox_time,
            reused_slides
        );

        if !errors.has_errors() {
            self.objects.retain(|_, object| object.present);
            self.viewboxes.retain(|_, viewbox| viewbox.1);
            self.slides = current_slides;
        }

        Ok(incremental_state)
    }
}
