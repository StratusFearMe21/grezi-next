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
use helix_core::tree_sitter::{Node, Tree, TreeCursor};
#[cfg(not(target_arch = "wasm32"))]
use miette::{Diagnostic, GraphicalReportHandler, SourceSpan};
use num_enum::FromPrimitive;
use serde::{Deserialize, Serialize};
#[cfg(not(target_arch = "wasm32"))]
use thiserror::Error;

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
        next: bool,
    },
    Action {
        actions: Vec<actions::Actions>,
        slide_in_ast: usize,
    },
}

#[cfg(not(target_arch = "wasm32"))]
pub struct PointFromRange(helix_core::tree_sitter::Range);

#[cfg(not(target_arch = "wasm32"))]
impl From<&PointFromRange> for SourceSpan {
    fn from(value: &PointFromRange) -> Self {
        (value.0.start_byte, value.0.end_byte - value.0.start_byte).into()
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl From<helix_core::tree_sitter::Range> for PointFromRange {
    fn from(value: helix_core::tree_sitter::Range) -> Self {
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
    Syntax(#[label("Something is wrong here")] PointFromRange),
    #[error("Missing error")]
    Missing(#[label("Something is missing here")] PointFromRange),
    #[error("Missing error")]
    KnownMissing(
        #[label("a {1} is missing here")] PointFromRange,
        &'static str,
    ),
    #[error("Bad node")]
    BadNode(
        #[label("Invalid Node kind: {1:?} here")] PointFromRange,
        NodeKind,
    ),
    #[error("Image error")]
    ImageError(
        #[label("Error loading image: {1}")] PointFromRange,
        eframe::egui::load::LoadError,
    ),
}

#[cfg(not(target_arch = "wasm32"))]
impl Error {
    pub fn range(&self) -> helix_core::tree_sitter::Range {
        match self {
            Error::BadExit(range) => range.0,
            Error::ImplicitEdge(range) => range.0,
            Error::ActionNotFound(range) => range.0,
            Error::KnownMissing(range, _) => range.0,
            Error::Missing(range) => range.0,
            Error::NotFound(range) => range.0,
            Error::Syntax(range) => range.0,
            Error::InvalidParameter(range) => range.0,
            Error::BadNode(range, _) => range.0,
            Error::ImageError(range, _) => range.0,
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

    // TODO: Reduce allocations in here by reusing current tree_cursor
    pub fn fork<T>(&mut self, mut callback: impl FnMut(&mut Self) -> T) -> T {
        callback(&mut GrzCursor::from_node(self.tree_cursor.node()))
    }

    fn check_for_error(&self, result: bool) -> Result<bool, Error> {
        if !result {
            return Ok(false);
        }

        if self.tree_cursor.node().is_error() {
            return Err(Error::Syntax(self.tree_cursor.node().range().into()));
        }

        if self.tree_cursor.node().is_missing() {
            return Err(Error::Missing(self.tree_cursor.node().range().into()));
        }

        Ok(result)
    }

    pub fn from_node(node: Node<'a>) -> GrzCursor<'a> {
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

    /*
    pub fn reset(&mut self, node: Node<'a>) {
        self.tree_cursor.reset(node);
    }
    */
}

#[cfg(not(target_arch = "wasm32"))]
pub fn parse_file(
    tree: &Tree,
    old_tree: Option<&Tree>,
    source: &helix_core::ropey::Rope,
    helix_cell: &mut Option<highlighting::HelixCell>,
    slide_show: &mut crate::SlideShow,
    egui_ctx: &eframe::egui::Context,
    file_path: &std::path::Path,
) -> Result<(), Vec<Error>> {
    let mut errors_present = Vec::new();
    let hasher = ahash::RandomState::with_seeds(69, 420, 24, 96);
    let mut on_screen: HashMap<u64, usize, BuildHasherDefault<PassThroughHasher>> =
        HashMap::default();
    let mut last_slide: usize = 0;
    let mut is_new = old_tree.is_none();

    if !is_new {
        let old_tree = old_tree.unwrap();
        for range in old_tree.changed_ranges(tree) {
            let node = tree
                .root_node()
                .descendant_for_point_range(range.start_point, range.end_point);

            if let Some(mut n) = node {
                if n.is_extra()
                    && n.parent().map(|n| n.kind_id()) == Some(NodeKind::SourceFile as u16)
                {
                    continue;
                }

                while n.is_extra() {
                    if let Some(parent) = n.parent() {
                        n = parent;
                    } else {
                        break;
                    }
                }

                if matches!(
                    NodeKind::from(n.kind_id()),
                    NodeKind::Slide | NodeKind::SlideFunctions
                ) {
                    is_new = true;
                }
            }
        }
    }

    let mut old_tree_cursor = GrzCursor::new(if let Some(t) = old_tree { t } else { tree });
    let mut new_tree_cursor = GrzCursor::new(tree);

    match (
        old_tree_cursor.goto_first_child(),
        new_tree_cursor.goto_first_child(),
    ) {
        (Ok(_), Ok(_)) => {}
        (Err(e), _) | (_, Err(e)) => return Err(vec![e]),
    }

    if is_new {
        slide_show.slide_show.clear();
        slide_show.viewboxes.clear();
        slide_show.objects.clear();
    }

    let mut ast_object_at = 0;
    let mut last_slide_changed = false;
    'parserloop: loop {
        let node = old_tree_cursor.node();
        match NodeKind::from(new_tree_cursor.node().kind_id()) {
            NodeKind::Slide => {
                last_slide = ast_object_at;

                if node.has_changes() || is_new || last_slide_changed {
                    new_tree_cursor.fork(|cursor| {
                        match slides::parse_slides(
                            cursor,
                            &hasher,
                            &mut on_screen,
                            &mut slide_show.objects,
                            source,
                            &mut errors_present,
                            &slide_show.viewboxes,
                        ) {
                            Ok(slide) => {
                                if is_new {
                                    slide_show.slide_show.push(slide)
                                } else {
                                    slide_show.slide_show[ast_object_at] = slide;
                                }
                            }
                            Err(e) => errors_present.push(e),
                        }
                    });
                    last_slide_changed = node.has_changes();
                } else {
                    on_screen.clear();
                    if let Some(AstObject::Slide { objects, .. }) =
                        slide_show.slide_show.get_mut(ast_object_at)
                    {
                        for (index, obj) in objects.iter_mut().enumerate() {
                            on_screen.insert(obj.object, index);
                            if let Some(object) = slide_show.objects.get_mut(&obj.object) {
                                object.viewbox = Some(obj.locations[1].1);
                                object.position = Some(obj.locations[1].0);
                            }
                        }
                    }
                }
                ast_object_at += 1;
            }
            NodeKind::Viewbox => {
                if node.has_changes() || is_new {
                    new_tree_cursor.fork(|cursor| {
                        match viewboxes::parse_viewbox(
                            cursor,
                            source,
                            &hasher,
                            &slide_show.viewboxes,
                        ) {
                            Ok(layout) => {
                                slide_show.viewboxes.insert(layout.0, layout.1);
                            }
                            Err(e) => errors_present.push(e),
                        }
                    })
                }
            }
            NodeKind::Obj => {
                if node.has_changes() || is_new {
                    new_tree_cursor.fork(|cursor| {
                        match objects::parse_objects(
                            cursor,
                            source,
                            helix_cell,
                            &hasher,
                            egui_ctx,
                            &mut errors_present,
                            file_path,
                        ) {
                            Ok(object) => {
                                slide_show.objects.insert(object.0, object.1);
                            }
                            Err(e) => errors_present.push(e),
                        }
                    })
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
            NodeKind::SlideFunctions => {
                if node.has_changes() || is_new || last_slide_changed {
                    new_tree_cursor.fork(|cursor| {
                        match actions::parse_actions(
                            cursor,
                            source,
                            &hasher,
                            &on_screen,
                            last_slide,
                            &mut errors_present,
                        ) {
                            Ok(action) => {
                                if is_new {
                                    slide_show.slide_show.push(action)
                                } else {
                                    slide_show.slide_show[ast_object_at] = action;
                                }
                            }
                            Err(e) => errors_present.push(e),
                        }
                    });
                }
                ast_object_at += 1;
            }
            kind => {
                errors_present.push(Error::BadNode(new_tree_cursor.node().range().into(), kind))
            }
        }

        loop {
            let _ = old_tree_cursor.goto_next_sibling();
            match new_tree_cursor.goto_next_sibling() {
                Ok(false) => break 'parserloop,
                Ok(true) => break,
                Err(e) => errors_present.push(e),
            }
        }
    }
    if is_new {
        slide_show.viewboxes.shrink_to_fit();
        slide_show.objects.shrink_to_fit();
    }
    drop(old_tree_cursor);
    drop(new_tree_cursor);

    if errors_present.is_empty() {
        Ok(())
    } else {
        Err(errors_present)
    }
}
