#![allow(unreachable_patterns)]

pub mod actions;
#[cfg(not(target_arch = "wasm32"))]
pub mod highlighting;
pub mod objects;
pub mod slides;
pub mod viewboxes;

use std::{
    borrow::Cow,
    collections::HashMap,
    fmt::Debug,
    hash::{BuildHasherDefault, Hasher},
};

use miette::{Diagnostic, GraphicalReportHandler, SourceSpan};
use num_enum::FromPrimitive;
use serde::{Deserialize, Serialize};
use thiserror::Error;
#[cfg(not(target_arch = "wasm32"))]
use tree_sitter::{Node, Tree, TreeCursor};

include!(concat!(env!("OUT_DIR"), "/kinds.rs"));

#[macro_export]
/// A macro for converting a lineup value, a viewbox and an object to a pair of x, y coordinates.
macro_rules! get_pos {
    ($line_up:expr, $vbx:expr, $obj:expr) => {
        match $line_up {
            LineUp::TopLeft => [$vbx.min.x, $vbx.min.y],
            LineUp::TopRight => [$vbx.max.x - $obj.width(), $vbx.min.y],
            LineUp::BottomLeft => [$vbx.min.x, $vbx.max.y - $obj.height()],
            LineUp::BottomRight => [$vbx.max.x - $obj.width(), $vbx.max.y - $obj.height()],
            LineUp::CenterTop => [
                ($vbx.min.x + $vbx.max.x) / 2.0 - ($obj.width() / 2.0),
                $vbx.min.y,
            ],
            LineUp::CenterBottom => [
                ($vbx.min.x + $vbx.max.x) / 2.0 - ($obj.width() / 2.0),
                $vbx.max.y - $obj.height(),
            ],
            LineUp::CenterLeft => [
                $vbx.min.x,
                ($vbx.min.y + $vbx.max.y) / 2.0 - ($obj.max.y / 2.0),
            ],
            LineUp::CenterRight => [
                $vbx.max.x - $obj.width(),
                ($vbx.min.y + $vbx.max.y) / 2.0 - ($obj.height() / 2.0),
            ],
            LineUp::CenterCenter => [
                ($vbx.min.x + $vbx.max.x) / 2.0 - ($obj.width() / 2.0),
                ($vbx.min.y + $vbx.max.y) / 2.0 - ($obj.height() / 2.0),
            ],
        }
    };
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AstObject {
    Slide {
        objects: Vec<slides::SlideObj>,
        actions: Vec<actions::Actions>,
        max_time: f32,
    },
    Action {
        actions: Vec<actions::Actions>,
        slide_in_ast: usize,
    },
}

#[derive(Error, Diagnostic)]
pub enum Error<'a> {
    #[error("Bad Node")]
    #[diagnostic(code(parser::parse_file))]
    BadNode(
        #[label("here")] SourceSpan,
        #[source_code] Cow<'a, str>,
        #[help("Node is of kind: {}")] &'a str,
    ),
    #[error("Object is not on screen")]
    #[diagnostic(code(parser::slide::parse_slide))]
    BadExit(
        #[label("Object cannot exit, as it is not currently on screen")] SourceSpan,
        #[source_code] Cow<'a, str>,
    ),
    #[error("Object could not be found")]
    #[diagnostic(code(parser::slide::parse_slide))]
    NotFound(
        #[label("Object not found")] SourceSpan,
        #[source_code] Cow<'a, str>,
    ),
}

impl<'a> Debug for Error<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        GraphicalReportHandler::new().render_report(f, self)
    }
}

/// Returns the last 8 bytes of the data, as a u64.
#[derive(Default)]
pub struct PassThroughHasher(u64);

impl Hasher for PassThroughHasher {
    #[inline]
    fn finish(&self) -> u64 {
        self.0
    }

    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes.iter() {
            self.0 = self.0.wrapping_shl(8) + (*byte as u64);
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub struct GrzCursor<'a> {
    tree_cursor: TreeCursor<'a>,
}

#[cfg(not(target_arch = "wasm32"))]
impl<'a> GrzCursor<'a> {
    fn new(tree: &'a Tree) -> GrzCursor<'a> {
        GrzCursor {
            tree_cursor: tree.walk(),
        }
    }

    fn from_node(node: &'a Node<'a>) -> GrzCursor<'a> {
        GrzCursor {
            tree_cursor: node.walk(),
        }
    }

    fn goto_first_child(&mut self) -> bool {
        if !self.tree_cursor.goto_first_child() {
            return false;
        }

        while !self.tree_cursor.node().is_named() || self.tree_cursor.node().is_extra() {
            if !self.tree_cursor.goto_next_sibling() {
                return false;
            }
        }
        true
    }

    fn goto_next_sibling(&mut self) -> bool {
        if !self.tree_cursor.goto_next_sibling() {
            return false;
        }

        while !self.tree_cursor.node().is_named() || self.tree_cursor.node().is_extra() {
            if !self.tree_cursor.goto_next_sibling() {
                return false;
            }
        }
        true
    }

    fn goto_parent(&mut self) -> bool {
        self.tree_cursor.goto_parent()
    }

    fn field_id(&self) -> Option<u16> {
        self.tree_cursor.field_id()
    }

    fn node(&self) -> Node<'a> {
        self.tree_cursor.node()
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn parse_file<'a>(
    source: &'a ropey::Rope,
    tree: &Tree,
    helix_cell: &mut Option<highlighting::HelixCell>,
    slide_show: &mut crate::SlideShow,
) -> Result<Vec<AstObject>, Error<'a>> {
    // let yoke = Yoke::<ParseError<'static>, String>::attach_to_cart(file, |source| {
    // let mut registers: AHashMap<&str, &str> = AHashMap::default();

    let hasher = ahash::RandomState::with_seeds(69, 420, 24, 96);
    let mut on_screen: HashMap<u64, usize, BuildHasherDefault<PassThroughHasher>> =
        HashMap::default();
    let mut last_slide: usize = 0;
    let mut tree_cursor = GrzCursor::new(&tree);

    tree_cursor.goto_first_child();
    let mut ast = Vec::new();
    loop {
        let node = tree_cursor.node();
        match NodeKind::from(node.kind_id()) {
            NodeKind::Slide => {
                last_slide = ast.len();
                ast.push(slides::parse_slides(
                    &mut tree_cursor,
                    &hasher,
                    &mut on_screen,
                    &mut slide_show.objects,
                    &source,
                )?);
            }
            NodeKind::Viewbox => {
                let layout = viewboxes::parse_viewbox(&mut tree_cursor, &source, &hasher);

                slide_show.viewboxes.insert(layout.0, layout.1);
            }
            NodeKind::Obj => {
                let object = objects::parse_objects(&mut tree_cursor, &source, helix_cell, &hasher);

                slide_show.objects.insert(object.0, object.1);
            }
            NodeKind::Register => {
                tree_cursor.goto_first_child();
                let key = source.byte_slice(tree_cursor.node().byte_range());
                tree_cursor.goto_next_sibling();
                let value = match NodeKind::from(tree_cursor.node().kind_id()) {
                    NodeKind::StringLiteral => {
                        tree_cursor.goto_first_child();
                        tree_cursor.goto_next_sibling();
                        let value = source.byte_slice(tree_cursor.node().byte_range());
                        tree_cursor.goto_parent();
                        value
                    }
                    NodeKind::NumberLiteral => source.byte_slice(tree_cursor.node().byte_range()),
                    _ => todo!(),
                };
                tree_cursor.goto_parent();
                // ast.push(AstObject::Register { key, value });
            }
            NodeKind::Action => {
                ast.push(actions::parse_actions(
                    &mut tree_cursor,
                    &source,
                    &hasher,
                    &on_screen,
                    last_slide,
                ));
            }
            _ => {
                let range = tree_cursor.node().byte_range();
                let kind: &'static str = node.kind();
                drop(tree_cursor);
                return Err(Error::BadNode(
                    SourceSpan::new(range.start.into(), (range.end - range.start).into()),
                    source.into(),
                    kind,
                ));
            }
        }

        if !tree_cursor.goto_next_sibling() {
            break;
        }
    }
    drop(tree_cursor);
    Ok(ast)
}
