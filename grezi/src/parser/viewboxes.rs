use std::{
    hash::{BuildHasher, Hasher},
    str::FromStr,
};

use crate::layout::{Constraint, UnresolvedLayout};

use super::{GrzCursor, NodeKind};
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

pub fn parse_viewbox(
    tree_cursor: &mut GrzCursor<'_>,
    source: &str,
    hasher: &ahash::RandomState,
) -> (u64, UnresolvedLayout) {
    tree_cursor.goto_first_child();
    let name = &source[tree_cursor.node().byte_range()];
    tree_cursor.goto_next_sibling();
    let attached_box = &source[tree_cursor.node().byte_range()];
    let node_kind = NodeKind::from(tree_cursor.node().kind_id());
    tree_cursor.goto_next_sibling();
    tree_cursor.goto_first_child();
    let attached_box = match node_kind {
        NodeKind::Size => ViewboxIn::Size,
        NodeKind::Identifier => {
            let mut hasher = hasher.build_hasher();
            std::hash::Hash::hash(attached_box, &mut hasher);
            ViewboxIn::Custom(
                hasher.finish(),
                source[tree_cursor.node().byte_range()].parse().unwrap(),
            )
        }
        _ => todo!(),
    };
    tree_cursor.goto_parent();
    tree_cursor.goto_next_sibling();
    tree_cursor.goto_first_child();
    let direction = Direction::from_str(&source[tree_cursor.node().byte_range()]).unwrap();
    tree_cursor.goto_next_sibling();
    let mut constraints = Vec::new();
    while tree_cursor.node().kind_id() == NodeKind::ViewboxObj as u16 {
        tree_cursor.goto_first_child();
        let numerator: f32 = source[tree_cursor.node().byte_range()].parse().unwrap();
        // We want a char literal here
        tree_cursor.tree_cursor.goto_next_sibling();
        match &source[tree_cursor.node().byte_range()] {
            "%" => constraints.push(Constraint::Percentage(numerator)),
            "-" => constraints.push(Constraint::Min(numerator)),
            "+" => constraints.push(Constraint::Max(numerator)),
            "~" => constraints.push(Constraint::Length(numerator)),
            ":" => {
                tree_cursor.goto_next_sibling();
                constraints.push(Constraint::Ratio(
                    numerator,
                    source[tree_cursor.node().byte_range()].parse().unwrap(),
                ));
            }
            _ => todo!(),
        }
        tree_cursor.goto_parent();
        tree_cursor.goto_next_sibling();
    }
    tree_cursor.goto_parent();
    tree_cursor.goto_parent();
    let mut hasher = hasher.build_hasher();
    std::hash::Hash::hash(name, &mut hasher);
    (
        hasher.finish(),
        UnresolvedLayout {
            direction: crate::layout::Direction::from(direction),
            margin: 15.0,
            constraints,
            expand_to_fill: true,
            split_on: attached_box,
        },
    )
}
