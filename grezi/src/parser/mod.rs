pub mod actions;
#[cfg(not(target_arch = "wasm32"))]
pub mod highlighting;
pub mod objects;
pub mod slides;
pub mod viewboxes;

use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    hash::{BuildHasherDefault, Hasher},
};

#[cfg(not(target_arch = "wasm32"))]
use miette::{Diagnostic, GraphicalReportHandler, SourceSpan};
use num_enum::FromPrimitive;
use serde::{Deserialize, Serialize};
#[cfg(not(target_arch = "wasm32"))]
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

#[cfg(not(target_arch = "wasm32"))]
pub struct PointFromRange(tree_sitter::Range);

#[cfg(not(target_arch = "wasm32"))]
impl From<&PointFromRange> for SourceSpan {
    fn from(value: &PointFromRange) -> Self {
        (value.0.start_byte, value.0.end_byte - value.0.start_byte).into()
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl From<tree_sitter::Range> for PointFromRange {
    fn from(value: tree_sitter::Range) -> Self {
        Self(value)
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Error, Diagnostic)]
pub enum Error {
    #[error("Object is not on screen")]
    BadExit(#[label("Object cannot exit, as it is not currently on screen")] PointFromRange),
    #[error("Object is not on screen")]
    ImplicitEdge(#[label("Implicit edge could not be resolved")] PointFromRange),
    #[error("Action not found")]
    ActionNotFound(#[label("Action not found")] PointFromRange),
    #[error("Object could not be found")]
    NotFound(#[label("Object not found")] PointFromRange),
    #[error("Invalid parameter")]
    InvalidParameter(#[label("This parameter is not valid for this action")] PointFromRange),
    #[error("Syntax error")]
    SyntaxError(#[label("Something is wrong here")] PointFromRange),
    #[error("Missing error")]
    MissingError(#[label("Something is missing here")] PointFromRange),
    #[error("Missing error")]
    KnownMissingError(
        #[label("a {1} is missing here")] PointFromRange,
        &'static str,
    ),
    #[error("Bad node")]
    BadNode(
        #[label("Invalid Node kind: {1:?} here")] PointFromRange,
        NodeKind,
    ),
}

#[cfg(not(target_arch = "wasm32"))]
impl Error {
    pub fn range(&self) -> tree_sitter::Range {
        match self {
            Error::BadExit(range) => range.0,
            Error::ImplicitEdge(range) => range.0,
            Error::ActionNotFound(range) => range.0,
            Error::KnownMissingError(range, _) => range.0,
            Error::MissingError(range) => range.0,
            Error::NotFound(range) => range.0,
            Error::SyntaxError(range) => range.0,
            Error::InvalidParameter(range) => range.0,
            Error::BadNode(range, _) => range.0,
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl From<Error> for lsp_types::Diagnostic {
    fn from(error: Error) -> Self {
        use lsp_types::{DiagnosticSeverity, Position, Range};
        use miette::Severity;
        let range = error.range();

        lsp_types::Diagnostic {
            range: Range {
                start: Position {
                    line: range.start_point.row as u32,
                    character: range.start_point.column as u32,
                },
                end: Position {
                    line: range.end_point.row as u32,
                    character: range.end_point.column as u32,
                },
            },
            severity: error.severity().map(|s| match s {
                Severity::Warning => DiagnosticSeverity::WARNING,
                Severity::Advice => DiagnosticSeverity::HINT,
                Severity::Error => DiagnosticSeverity::ERROR,
            }),
            message: error
                .labels()
                .unwrap()
                .fold(String::new(), |mut message, label| {
                    message.push_str(label.label().unwrap());
                    message.push('\n');
                    message
                }),
            ..Default::default()
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        GraphicalReportHandler::new().render_report(f, self)
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub struct ErrWithSource {
    pub error: Error,
    pub source_code: String,
}

#[cfg(not(target_arch = "wasm32"))]
impl Diagnostic for ErrWithSource {
    fn code<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        self.error.code()
    }

    fn diagnostic_source(&self) -> Option<&dyn Diagnostic> {
        self.error.diagnostic_source()
    }

    fn help<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        self.error.help()
    }

    fn labels(&self) -> Option<Box<dyn Iterator<Item = miette::LabeledSpan> + '_>> {
        self.error.labels()
    }

    fn related<'a>(&'a self) -> Option<Box<dyn Iterator<Item = &'a dyn Diagnostic> + 'a>> {
        self.error.related()
    }

    fn severity(&self) -> Option<miette::Severity> {
        self.error.severity()
    }

    fn source_code(&self) -> Option<&dyn miette::SourceCode> {
        Some(&self.source_code)
    }

    fn url<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        self.error.url()
    }
}

#[allow(deprecated)]
#[cfg(not(target_arch = "wasm32"))]
impl std::error::Error for ErrWithSource {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.error.cause()
    }

    fn description(&self) -> &str {
        self.error.description()
    }

    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.error.source()
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Debug for ErrWithSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        GraphicalReportHandler::new().render_report(f, self)
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Display for ErrWithSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
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
    pub fn new(tree: &'a Tree) -> GrzCursor<'a> {
        GrzCursor {
            tree_cursor: tree.walk(),
        }
    }

    pub fn fork<'b>(&'b self) -> GrzCursor<'b> {
        GrzCursor {
            tree_cursor: self.tree_cursor.node().walk(),
        }
    }

    fn check_for_error(&self, result: bool) -> Result<bool, Error> {
        if !result {
            return Ok(false);
        }

        if self.tree_cursor.node().is_error() {
            return Err(Error::SyntaxError(self.tree_cursor.node().range().into()));
        }

        if self.tree_cursor.node().is_missing() {
            return Err(Error::MissingError(self.tree_cursor.node().range().into()));
        }

        Ok(result)
    }

    pub fn from_node(node: &'a Node<'a>) -> GrzCursor<'a> {
        GrzCursor {
            tree_cursor: node.walk(),
        }
    }

    fn goto_first_impl(&mut self) -> Result<bool, Error> {
        let result = self.tree_cursor.goto_first_child();
        self.check_for_error(result)
    }

    fn goto_next_impl(&mut self) -> Result<bool, Error> {
        let result = self.tree_cursor.goto_next_sibling();
        self.check_for_error(result)
    }

    pub fn goto_first_child(&mut self) -> Result<bool, Error> {
        if !self.goto_first_impl()? {
            return self.check_for_error(false);
        }

        while !self.tree_cursor.node().is_named() || self.tree_cursor.node().is_extra() {
            if !self.goto_next_impl()? {
                return self.check_for_error(false);
            }
        }

        self.check_for_error(true)
    }

    pub fn goto_first_child_raw(&mut self) -> Result<bool, Error> {
        if !self.goto_first_impl()? {
            return self.check_for_error(false);
        }

        while self.tree_cursor.node().is_extra() {
            if !self.goto_next_impl()? {
                return self.check_for_error(false);
            }
        }

        self.check_for_error(true)
    }

    pub fn goto_next_sibling(&mut self) -> Result<bool, Error> {
        if !self.goto_next_impl()? {
            return self.check_for_error(false);
        }

        while !self.tree_cursor.node().is_named() || self.tree_cursor.node().is_extra() {
            if !self.goto_next_impl()? {
                return self.check_for_error(false);
            }
        }

        self.check_for_error(true)
    }

    pub fn goto_next_sibling_raw(&mut self) -> Result<bool, Error> {
        if !self.goto_next_impl()? {
            return self.check_for_error(false);
        }

        while self.tree_cursor.node().is_extra() {
            if !self.goto_next_impl()? {
                return self.check_for_error(false);
            }
        }

        self.check_for_error(true)
    }

    pub fn goto_parent(&mut self) -> bool {
        self.tree_cursor.goto_parent()
    }

    fn field_id(&self) -> Option<u16> {
        self.tree_cursor.field_id()
    }

    pub fn node(&self) -> Node<'a> {
        self.tree_cursor.node()
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn parse_file(
    tree: &Tree,
    source: &ropey::Rope,
    helix_cell: &mut Option<highlighting::HelixCell>,
    slide_show: &mut crate::SlideShow,
) -> Result<Vec<AstObject>, Vec<Error>> {
    // let yoke = Yoke::<ParseError<'static>, String>::attach_to_cart(file, |source| {
    // let mut registers: AHashMap<&str, &str> = AHashMap::default();

    let mut errors_present = Vec::new();
    let hasher = ahash::RandomState::with_seeds(69, 420, 24, 96);
    let mut on_screen: HashMap<u64, usize, BuildHasherDefault<PassThroughHasher>> =
        HashMap::default();
    let mut last_slide: usize = 0;
    let mut tree_cursor = GrzCursor::new(&tree);

    match tree_cursor.goto_first_child() {
        Ok(_) => {}
        Err(e) => return Err(vec![e]),
    }
    let mut ast = Vec::new();
    'parserloop: loop {
        let node = tree_cursor.node();
        match NodeKind::from(node.kind_id()) {
            NodeKind::Slide => {
                last_slide = ast.len();
                match slides::parse_slides(
                    tree_cursor.fork(),
                    &hasher,
                    &mut on_screen,
                    &mut slide_show.objects,
                    &source,
                    &mut errors_present,
                    &slide_show.viewboxes,
                ) {
                    Ok(slide) => ast.push(slide),
                    Err(e) => errors_present.push(e),
                }
            }
            NodeKind::Viewbox => {
                match viewboxes::parse_viewbox(
                    tree_cursor.fork(),
                    &source,
                    &hasher,
                    &slide_show.viewboxes,
                ) {
                    Ok(layout) => {
                        slide_show.viewboxes.insert(layout.0, layout.1);
                    }
                    Err(e) => errors_present.push(e),
                }
            }
            NodeKind::Obj => {
                match objects::parse_objects(
                    tree_cursor.fork(),
                    &source,
                    helix_cell,
                    &hasher,
                    &mut errors_present,
                ) {
                    Ok(object) => {
                        slide_show.objects.insert(object.0, object.1);
                    }
                    Err(e) => errors_present.push(e),
                }
            }
            NodeKind::Register => {
                /*
                tree_cursor.goto_first_child()?;
                let key = source.byte_slice(tree_cursor.node().byte_range());
                tree_cursor.goto_next_sibling()?;
                let value = match NodeKind::from(tree_cursor.node().kind_id()) {
                    NodeKind::StringLiteral => {
                        tree_cursor.goto_first_child()?;
                        tree_cursor.goto_next_sibling()?;
                        let value = source.byte_slice(tree_cursor.node().byte_range());
                        tree_cursor.goto_parent();
                        value
                    }
                    NodeKind::NumberLiteral => source.byte_slice(tree_cursor.node().byte_range()),
                    _ => todo!(),
                };
                tree_cursor.goto_parent();
                ast.push(AstObject::Register { key, value });
                */
            }
            NodeKind::Action => {
                match actions::parse_actions(
                    tree_cursor.fork(),
                    &source,
                    &hasher,
                    &on_screen,
                    last_slide,
                    &mut errors_present,
                ) {
                    Ok(action) => ast.push(action),
                    Err(e) => errors_present.push(e),
                }
            }
            kind => errors_present.push(Error::BadNode(tree_cursor.node().range().into(), kind)),
        }

        loop {
            match tree_cursor.goto_next_sibling() {
                Ok(false) => break 'parserloop,
                Ok(true) => break,
                Err(e) => errors_present.push(e),
            }
        }
    }
    drop(tree_cursor);
    if errors_present.is_empty() {
        Ok(ast)
    } else {
        Err(errors_present)
    }
}
