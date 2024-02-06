use std::{
    hash::{BuildHasher, Hasher},
    str::FromStr,
};

use crate::layout::{Constraint, UnresolvedLayout};

#[cfg(not(target_arch = "wasm32"))]
use super::GrzCursor;
use super::NodeKind;
use eframe::emath::Align2;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
pub enum ViewboxIn {
    Size,
    Inherit(Option<usize>),
    Custom(u64, usize),
}

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Center,
}

pub fn align_from_str(s: &str) -> Result<Align2, ()> {
    match Direction::from_str(&s[..1])? {
        Direction::Up => match Direction::from_str(&s[1..2])? {
            Direction::Up => Ok(Align2::CENTER_TOP),
            Direction::Down => Ok(Align2::CENTER_CENTER),
            Direction::Left => Ok(Align2::LEFT_TOP),
            Direction::Right => Ok(Align2::RIGHT_TOP),
            Direction::Center => Ok(Align2::CENTER_TOP),
        },
        Direction::Down => match Direction::from_str(&s[1..2])? {
            Direction::Up => Ok(Align2::CENTER_CENTER),
            Direction::Down => Ok(Align2::CENTER_BOTTOM),
            Direction::Left => Ok(Align2::LEFT_BOTTOM),
            Direction::Right => Ok(Align2::RIGHT_BOTTOM),
            Direction::Center => Ok(Align2::CENTER_BOTTOM),
        },
        Direction::Left => match Direction::from_str(&s[1..2])? {
            Direction::Up => Ok(Align2::LEFT_TOP),
            Direction::Down => Ok(Align2::LEFT_BOTTOM),
            Direction::Left => Ok(Align2::LEFT_CENTER),
            Direction::Right => Ok(Align2::CENTER_CENTER),
            Direction::Center => Ok(Align2::LEFT_CENTER),
        },
        Direction::Right => match Direction::from_str(&s[1..2])? {
            Direction::Up => Ok(Align2::RIGHT_TOP),
            Direction::Down => Ok(Align2::RIGHT_BOTTOM),
            Direction::Left => Ok(Align2::CENTER_CENTER),
            Direction::Right => Ok(Align2::RIGHT_CENTER),
            Direction::Center => Ok(Align2::RIGHT_CENTER),
        },
        Direction::Center => match Direction::from_str(&s[1..2])? {
            Direction::Up => Ok(Align2::CENTER_TOP),
            Direction::Down => Ok(Align2::CENTER_BOTTOM),
            Direction::Left => Ok(Align2::LEFT_CENTER),
            Direction::Right => Ok(Align2::RIGHT_CENTER),
            Direction::Center => Ok(Align2::CENTER_CENTER),
        },
    }
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "^" => Ok(Direction::Up),
            "_" => Ok(Direction::Down),
            ">" => Ok(Direction::Right),
            "<" => Ok(Direction::Left),
            "." => Ok(Direction::Center),
            _ => Err(()),
        }
    }
}

use std::{borrow::Cow, collections::HashMap, hash::BuildHasherDefault};

use super::PassThroughHasher;
#[cfg(not(target_arch = "wasm32"))]
pub fn parse_viewbox(
    tree_cursor: &mut GrzCursor<'_>,
    source: &helix_core::ropey::Rope,
    hasher: &ahash::RandomState,
    viewboxes: &HashMap<u64, UnresolvedLayout, BuildHasherDefault<PassThroughHasher>>,
) -> Result<(u64, UnresolvedLayout), super::Error> {
    tree_cursor.goto_first_child()?;
    let name = source.byte_slice(tree_cursor.node().byte_range());
    tree_cursor.goto_next_sibling()?;
    let mut name_hash = hasher.build_hasher();
    std::hash::Hash::hash(&name, &mut name_hash);
    let name_hash = name_hash.finish();
    let vb = parse_viewbox_inner(tree_cursor, source, hasher, viewboxes)?;
    tree_cursor.goto_parent();
    Ok((name_hash, vb))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn parse_viewbox_inner(
    tree_cursor: &mut GrzCursor<'_>,
    source: &helix_core::ropey::Rope,
    hasher: &ahash::RandomState,
    viewboxes: &HashMap<u64, UnresolvedLayout, BuildHasherDefault<PassThroughHasher>>,
) -> Result<UnresolvedLayout, super::Error> {
    let attached_box = parse_viewbox_ident(source, tree_cursor, hasher, viewboxes)?;
    if matches!(attached_box, ViewboxIn::Inherit(_)) {
        return Err(super::Error::InvalidParameter(
            tree_cursor.node().range().into(),
        ));
    }
    tree_cursor.goto_next_sibling()?;
    tree_cursor.goto_first_child()?;
    let direction: Cow<'_, str> = source.byte_slice(tree_cursor.node().byte_range()).into();
    let direction = Direction::from_str(direction.as_ref())
        .map_err(|_| super::Error::InvalidParameter(tree_cursor.node().range().into()))?;
    tree_cursor.goto_next_sibling()?;
    let mut constraints = Vec::new();
    while tree_cursor.node().kind_id() == NodeKind::ViewboxObj as u16 {
        tree_cursor.goto_first_child()?;
        let numerator: Cow<'_, str> = source.byte_slice(tree_cursor.node().byte_range()).into();
        let numerator: f32 = numerator.parse().unwrap();
        // We want a char literal here
        tree_cursor.goto_next_sibling_raw()?;
        let op: Cow<'_, str> = source.byte_slice(tree_cursor.node().byte_range()).into();
        match op.as_ref() {
            "%" => constraints.push(Constraint::Percentage(numerator)),
            "-" => constraints.push(Constraint::Min(numerator)),
            "+" => constraints.push(Constraint::Max(numerator)),
            "~" => constraints.push(Constraint::Length(numerator)),
            ":" => {
                tree_cursor.goto_next_sibling()?;
                let denominator: Cow<'_, str> =
                    source.byte_slice(tree_cursor.node().byte_range()).into();
                constraints.push(Constraint::Ratio(numerator, denominator.parse().unwrap()));
            }
            _ => {
                return Err(super::Error::InvalidParameter(
                    tree_cursor.node().range().into(),
                ))
            }
        }
        tree_cursor.goto_parent();
        tree_cursor.goto_next_sibling()?;
    }
    tree_cursor.goto_parent();
    Ok(UnresolvedLayout {
        direction: crate::layout::Direction::from(direction),
        margin: 15.0,
        constraints,
        expand_to_fill: true,
        split_on: attached_box,
    })
}

#[cfg(not(target_arch = "wasm32"))]
pub fn parse_viewbox_ident(
    source: &helix_core::ropey::Rope,
    tree_cursor: &mut GrzCursor<'_>,
    hasher: &ahash::RandomState,
    viewboxes: &HashMap<u64, UnresolvedLayout, BuildHasherDefault<PassThroughHasher>>,
) -> Result<ViewboxIn, super::Error> {
    let viewbox = source.byte_slice(tree_cursor.node().byte_range());
    let viewbox_node = NodeKind::from(tree_cursor.node().kind_id());
    let viewbox_range = tree_cursor.node().range();

    match viewbox_node {
        NodeKind::Size => {
            tree_cursor.goto_next_sibling()?;
            Ok(ViewboxIn::Size)
        }
        NodeKind::Inherit => {
            tree_cursor.goto_next_sibling()?;
            let mut vb_index_res: Option<usize> = None;
            if tree_cursor.node().kind_id() == NodeKind::IndexParser as u16 {
                tree_cursor.goto_first_child()?;
                let vb_index: Cow<'_, str> =
                    source.byte_slice(tree_cursor.node().byte_range()).into();
                vb_index_res = Some(vb_index.parse().unwrap());
                tree_cursor.goto_parent();
            }
            Ok(ViewboxIn::Inherit(vb_index_res))
        }
        NodeKind::Identifier => {
            let mut hasher = hasher.build_hasher();
            std::hash::Hash::hash(&viewbox, &mut hasher);
            let name = hasher.finish();
            if let Some(vb) = viewboxes.get(&name) {
                tree_cursor.goto_next_sibling()?;
                tree_cursor.goto_first_child()?;
                let vb_index: Cow<'_, str> =
                    source.byte_slice(tree_cursor.node().byte_range()).into();
                let vb_index: usize = vb_index.parse().unwrap();
                tree_cursor.goto_parent();

                if vb.constraints.get(vb_index).is_none() {
                    return Err(super::Error::NotFound(tree_cursor.node().range().into()));
                }

                Ok(ViewboxIn::Custom(name, vb_index))
            } else {
                return Err(super::Error::NotFound(tree_cursor.node().range().into()));
            }
        }
        kind => Err(super::Error::BadNode(viewbox_range.into(), kind)),
    }
}
