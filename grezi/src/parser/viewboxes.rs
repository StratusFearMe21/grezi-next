use std::{
    hash::{BuildHasher, Hasher},
    str::FromStr,
};

use crate::layout::{Constraint, UnresolvedLayout};

#[cfg(not(target_arch = "wasm32"))]
use super::GrzCursor;
use super::NodeKind;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum LineUp {
    /// .. or <> or ^_
    CenterCenter,
    /// ^< or <^
    TopLeft,
    /// << or .< or <.
    CenterLeft,
    /// _< or <_
    BottomLeft,
    /// __ or ._ or _.
    CenterBottom,
    /// _< or <_
    BottomRight,
    /// << or .< or <.
    CenterRight,
    /// ^< or <^
    TopRight,
    /// ^^ or .^ or ^.
    CenterTop,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum ViewboxIn {
    Size,
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

impl FromStr for LineUp {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Direction::from_str(&s[..1])? {
            Direction::Up => match Direction::from_str(&s[1..2])? {
                Direction::Up => Ok(Self::CenterTop),
                Direction::Down => Ok(Self::CenterCenter),
                Direction::Left => Ok(Self::TopLeft),
                Direction::Right => Ok(Self::TopRight),
                Direction::Center => Ok(Self::CenterTop),
            },
            Direction::Down => match Direction::from_str(&s[1..2])? {
                Direction::Up => Ok(Self::CenterCenter),
                Direction::Down => Ok(Self::CenterBottom),
                Direction::Left => Ok(Self::BottomLeft),
                Direction::Right => Ok(Self::BottomRight),
                Direction::Center => Ok(Self::CenterBottom),
            },
            Direction::Left => match Direction::from_str(&s[1..2])? {
                Direction::Up => Ok(Self::TopLeft),
                Direction::Down => Ok(Self::BottomLeft),
                Direction::Left => Ok(Self::CenterLeft),
                Direction::Right => Ok(Self::CenterCenter),
                Direction::Center => Ok(Self::CenterLeft),
            },
            Direction::Right => match Direction::from_str(&s[1..2])? {
                Direction::Up => Ok(Self::TopRight),
                Direction::Down => Ok(Self::BottomRight),
                Direction::Left => Ok(Self::CenterCenter),
                Direction::Right => Ok(Self::CenterRight),
                Direction::Center => Ok(Self::CenterRight),
            },
            Direction::Center => match Direction::from_str(&s[1..2])? {
                Direction::Up => Ok(Self::CenterTop),
                Direction::Down => Ok(Self::CenterBottom),
                Direction::Left => Ok(Self::CenterLeft),
                Direction::Right => Ok(Self::CenterRight),
                Direction::Center => Ok(Self::CenterCenter),
            },
        }
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
    let attached_box = parse_viewbox_ident(source, tree_cursor, hasher, viewboxes)?;
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
    tree_cursor.goto_parent();
    let mut hasher = hasher.build_hasher();
    std::hash::Hash::hash(&name, &mut hasher);
    Ok((
        hasher.finish(),
        UnresolvedLayout {
            direction: crate::layout::Direction::from(direction),
            margin: 15.0,
            constraints,
            expand_to_fill: true,
            split_on: attached_box,
        },
    ))
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
