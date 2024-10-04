pub mod actions;
pub mod color;
use color::Color;
#[cfg(not(target_arch = "wasm32"))]
pub mod citations;
#[cfg(not(target_arch = "wasm32"))]
pub mod highlighting;
pub mod objects;
pub mod slides;
pub mod viewboxes;

use std::{
    borrow::Cow,
    collections::HashMap,
    fmt::{Debug, Display},
    hash::{BuildHasherDefault, Hasher},
    num::NonZeroU16,
    time::Duration,
};

#[cfg(not(target_arch = "wasm32"))]
use helix_core::tree_sitter::{Node, Tree, TreeCursor};
#[cfg(not(target_arch = "wasm32"))]
use helix_core::RopeSlice;
#[cfg(not(target_arch = "wasm32"))]
use miette::{Diagnostic, GraphicalReportHandler, SourceSpan};
use num_enum::FromPrimitive;
use serde::{Deserialize, Serialize};
#[cfg(not(target_arch = "wasm32"))]
use thiserror::Error;

include!(concat!(env!("OUT_DIR"), "/kinds.rs"));

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AstObject {
    Slide {
        objects: Vec<slides::SlideObj>,
        actions: Vec<actions::Actions>,
        bg: (Color, Option<(Duration, Color)>),
        max_time: f32,
        next: bool,
    },
    Action {
        actions: Vec<actions::Actions>,
        slide_in_ast: u64,
        next: bool,
    },
}

#[cfg(not(target_arch = "wasm32"))]
pub struct PointFromRange(lsp_types::Range, std::ops::Range<usize>);

#[cfg(not(target_arch = "wasm32"))]
impl From<&PointFromRange> for SourceSpan {
    fn from(value: &PointFromRange) -> Self {
        (value.1.start, value.1.len()).into()
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl PointFromRange {
    pub fn new(range: helix_core::tree_sitter::Range, rope: &helix_core::Rope) -> Self {
        use crate::lsp::formatter::char_range_from_byte_range;
        Self(
            char_range_from_byte_range(range, rope).unwrap(),
            range.start_byte..range.end_byte,
        )
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
        Cow<'static, str>,
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
    #[error("Color error")]
    ColorError(#[label("Error parsing color: {1}")] PointFromRange, String),
    #[error("Spell check")]
    #[diagnostic(severity(Warning))]
    SpellCheck(
        #[label("Try spelling that {1:?}")] PointFromRange,
        Vec<String>,
    ),
}

#[cfg(not(target_arch = "wasm32"))]
impl Error {
    pub fn range(&self) -> lsp_types::Range {
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
            Error::ColorError(range, _) => range.0,
            Error::SpellCheck(range, _) => range.0,
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl From<Error> for lsp_types::Diagnostic {
    fn from(error: Error) -> Self {
        use lsp_types::DiagnosticSeverity;
        use miette::Severity;
        let range = error.range();

        lsp_types::Diagnostic {
            range,
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
    rope: &'a helix_core::Rope,
}

#[cfg(not(target_arch = "wasm32"))]
impl<'a> GrzCursor<'a> {
    pub fn new(tree: &'a Tree, rope: &'a helix_core::Rope) -> GrzCursor<'a> {
        GrzCursor {
            tree_cursor: tree.walk(),
            rope,
        }
    }

    // TODO: Reduce allocations in here by reusing current tree_cursor
    pub fn fork<T>(&mut self, mut callback: impl FnMut(&mut Self) -> T) -> T {
        callback(&mut GrzCursor::from_node(
            self.tree_cursor.node(),
            self.rope,
        ))
    }

    fn check_for_error(&self, result: bool) -> Result<bool, Error> {
        if !result {
            return Ok(false);
        }

        if self.tree_cursor.node().is_error() {
            return Err(Error::Syntax(PointFromRange::new(
                self.tree_cursor.node().range(),
                self.rope,
            )));
        }

        if self.tree_cursor.node().is_missing() {
            return Err(Error::Missing(PointFromRange::new(
                self.tree_cursor.node().range(),
                self.rope,
            )));
        }

        Ok(result)
    }

    pub fn from_node(node: Node<'a>, rope: &'a helix_core::Rope) -> GrzCursor<'a> {
        GrzCursor {
            tree_cursor: node.walk(),
            rope,
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

    fn field_id(&self) -> Option<NonZeroU16> {
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
    use std::{collections::HashSet, ops::Deref, sync::Arc};

    use indexmap::IndexMap;

    use self::objects::ObjectState;

    let mut bg = (Color::default(), None);
    let mut errors_present = Vec::new();
    let hasher = ahash::RandomState::with_seeds(69, 420, 24, 96);
    let mut on_screen: HashMap<u64, (usize, bool), BuildHasherDefault<PassThroughHasher>> =
        HashMap::default();
    let mut last_slide: u64 = 0;
    let is_new = old_tree.is_none();
    let mut old_nodes: HashSet<u64, BuildHasherDefault<PassThroughHasher>> = HashSet::default();

    if let Some(old_tree) = old_tree {
        // {
        //     let mut nodes: HashSet<u64, BuildHasherDefault<PassThroughHasher>> = HashSet::default();
        //     let query =
        //         helix_core::tree_sitter::Query::new(&tree_sitter_grz::language(), "(_) @node")
        //             .unwrap();
        //     let mut cursor = helix_core::tree_sitter::QueryCursor::new();

        //     for query_match in cursor.matches(
        //         &query,
        //         old_tree.root_node(),
        //         helix_core::syntax::RopeProvider(source.slice(..)),
        //     ) {
        //         for capture in query_match.captures {
        //             nodes.insert(capture.node.id() as u64);
        //         }
        //     }

        //     for query_match in cursor.matches(
        //         &query,
        //         tree.root_node(),
        //         helix_core::syntax::RopeProvider(source.slice(..)),
        //     ) {
        //         for capture in query_match.captures {
        //             if nodes.contains(&(capture.node.id() as u64)) {
        //                 eprintln!("{}", source.byte_slice(capture.node.byte_range()));
        //             }
        //         }
        //     }
        // }

        let mut tree_cursor = GrzCursor::new(old_tree, source);
        if tree_cursor.goto_first_child().unwrap_or_default() {
            'parserloop: loop {
                let node = tree_cursor.node();
                match NodeKind::from(node.kind_id()) {
                    NodeKind::Slide => match slides::id(&mut tree_cursor) {
                        Ok(node_id) => {
                            old_nodes.insert(node_id);
                        }
                        _ => {}
                    },
                    NodeKind::Viewbox => match viewboxes::id(&mut tree_cursor) {
                        Ok(node_id) => {
                            old_nodes.insert(node_id);
                        }
                        _ => {}
                    },
                    NodeKind::Obj => match objects::id(&mut tree_cursor) {
                        Ok(node_id) => {
                            old_nodes.insert(node_id);
                        }
                        _ => {}
                    },
                    NodeKind::Register => match register_id(&mut tree_cursor) {
                        Ok(node_id) => {
                            old_nodes.insert(node_id);
                        }
                        _ => {}
                    },
                    NodeKind::SlideFunctions => match actions::id(&mut tree_cursor) {
                        Ok(node_id) => {
                            old_nodes.insert(node_id);
                        }
                        _ => {}
                    },
                    _ => {}
                }

                loop {
                    match tree_cursor.goto_next_sibling() {
                        Ok(false) => break 'parserloop,
                        Ok(true) => break,
                        Err(e) => errors_present.push(e),
                    }
                }
            }
        }
    }

    let mut tree_cursor = GrzCursor::new(tree, source);

    match tree_cursor.goto_first_child() {
        Ok(_) => {}
        Err(e) => return Err(vec![e]),
    }

    if is_new {
        slide_show.viewboxes.clear();
        slide_show.objects.clear();
    }

    let mut slides = IndexMap::with_capacity_and_hasher(
        slide_show.slide_show.capacity(),
        BuildHasherDefault::default(),
    );
    let mut last_slide_changed = false;
    let mut last_bg_changed = false;
    'parserloop: loop {
        let node = tree_cursor.node();
        match NodeKind::from(node.kind_id()) {
            NodeKind::Slide => match slides::id(&mut tree_cursor) {
                Ok(node_id) => {
                    let has_changes = !old_nodes.contains(&node_id);
                    last_slide = node_id;

                    let slide_on = slide_show.slide_show.get(&node_id);

                    if has_changes || slide_on.is_none() || is_new || last_slide_changed {
                        tree_cursor.fork(|cursor| {
                            match slides::parse_slides(
                                cursor,
                                &hasher,
                                &mut on_screen,
                                &mut slide_show.objects,
                                source,
                                &mut errors_present,
                                (bg.0, bg.1.take()),
                                &mut slide_show.viewboxes,
                            ) {
                                Ok((slide, color)) => {
                                    slides.insert(node_id, Arc::new(slide));

                                    if let Some(color) = color {
                                        bg.0 = color.1;
                                    }
                                }
                                Err(e) => errors_present.push(e),
                            }
                        });
                        last_slide_changed = has_changes || slide_on.is_none();
                    } else {
                        on_screen.clear();
                        if let Some(AstObject::Slide {
                            objects, bg: color, ..
                        }) = slide_on.map(|s| {
                            slides.insert(node_id, Arc::clone(s));
                            s.deref()
                        }) {
                            for (index, obj) in objects.iter().enumerate() {
                                on_screen
                                    .insert(obj.object, (index, obj.state == ObjectState::Exiting));
                                if let Some(object) = slide_show.objects.get_mut(&obj.object) {
                                    if obj.state == ObjectState::Exiting {
                                        object.viewbox = None;
                                        object.position = None;
                                    } else {
                                        object.viewbox = Some(obj.locations[1].1);
                                        object.position = Some(obj.locations[1].0);
                                    }
                                }
                            }
                            if let Some(color) = color.1 {
                                bg.0 = color.1;
                            }
                        }
                    }
                }
                Err(e) => errors_present.push(e),
            },
            NodeKind::Viewbox => match viewboxes::id(&mut tree_cursor) {
                Ok(node_id) => {
                    let has_changes = !old_nodes.contains(&node_id);
                    if has_changes || is_new {
                        tree_cursor.fork(|cursor| {
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
                Err(e) => errors_present.push(e),
            },
            NodeKind::Obj => match objects::id(&mut tree_cursor) {
                Ok(node_id) => {
                    let has_changes = !old_nodes.contains(&node_id);
                    if has_changes || is_new {
                        tree_cursor.fork(|cursor| {
                            match objects::parse_objects(
                                cursor,
                                source,
                                helix_cell,
                                &hasher,
                                egui_ctx,
                                &mut errors_present,
                                file_path,
                                |hash, object| {
                                    slide_show.objects.insert(hash, object);
                                },
                            ) {
                                Ok(_) => {}
                                Err(e) => errors_present.push(e),
                            }
                        })
                    }
                }
                Err(e) => errors_present.push(e),
            },
            NodeKind::Register => match register_id(&mut tree_cursor) {
                Ok(node_id) => {
                    let has_changes = !old_nodes.contains(&node_id);
                    if has_changes || is_new || last_bg_changed {
                        match parse_register(&mut tree_cursor, source) {
                            Ok((key, value)) => {
                                let value: std::borrow::Cow<'_, str> = value.into();
                                if key == "BACKGROUND" {
                                    match color::parse_color_with(
                                        &mut color::DefaultColorParser::new(Some(&mut bg.0)),
                                        &mut cssparser::Parser::new(
                                            &mut cssparser::ParserInput::new(value.as_ref()),
                                        ),
                                    ) {
                                        Ok(c) => bg.1 = Some(c),
                                        Err(e) => errors_present.push(Error::ColorError(
                                            PointFromRange::new(node.range(), source),
                                            format!("{:?}", e),
                                        )),
                                    }
                                }
                            }
                            Err(e) => errors_present.push(e),
                        }
                        last_slide_changed = has_changes || last_bg_changed;
                        last_bg_changed = has_changes;
                    }
                }
                Err(e) => errors_present.push(e),
            },
            NodeKind::SlideFunctions => match actions::id(&mut tree_cursor) {
                Ok(node_id) => {
                    let has_changes = !old_nodes.contains(&node_id);
                    let slide_on = slide_show.slide_show.get(&node_id);

                    if slide_on.is_none() || has_changes || is_new || last_slide_changed {
                        tree_cursor.fork(|cursor| {
                            match actions::parse_actions(
                                cursor,
                                source,
                                &hasher,
                                &on_screen,
                                last_slide,
                                &mut errors_present,
                            ) {
                                Ok(action) => {
                                    slides.insert(node_id, Arc::new(action));
                                }
                                Err(e) => errors_present.push(e),
                            }
                        });
                    } else {
                        slides.insert(node_id, slide_on.cloned().unwrap());
                    }
                }
                Err(e) => errors_present.push(e),
            },
            kind => errors_present.push(Error::BadNode(
                PointFromRange::new(tree_cursor.node().range(), source),
                kind,
            )),
        }

        loop {
            match tree_cursor.goto_next_sibling() {
                Ok(false) => break 'parserloop,
                Ok(true) => break,
                Err(e) => errors_present.push(e),
            }
        }
    }
    slide_show.slide_show = slides;
    if is_new {
        slide_show.viewboxes.shrink_to_fit();
        slide_show.objects.shrink_to_fit();
        citations::parse_citations(file_path, &hasher, slide_show).unwrap();
    }

    if errors_present.is_empty() {
        Ok(())
    } else {
        Err(errors_present)
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn parse_register<'a>(
    tree_cursor: &mut GrzCursor<'_>,
    source: &'a helix_core::ropey::Rope,
) -> Result<(RopeSlice<'a>, RopeSlice<'a>), Error> {
    tree_cursor.goto_first_child()?;
    tree_cursor.goto_first_child()?;
    let key = source.byte_slice(tree_cursor.node().byte_range());
    tree_cursor.goto_next_sibling()?;
    let value = match NodeKind::from(tree_cursor.node().kind_id()) {
        NodeKind::StringLiteral => {
            tree_cursor.goto_first_child()?;
            let value = source.byte_slice(tree_cursor.node().byte_range());
            tree_cursor.goto_parent();
            value
        }
        NodeKind::NumberLiteral | NodeKind::ObjOther => {
            source.byte_slice(tree_cursor.node().byte_range())
        }
        _ => todo!(),
    };
    tree_cursor.goto_parent();
    tree_cursor.goto_parent();
    Ok((key, value))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn register_id(walker: &mut GrzCursor<'_>) -> Result<u64, Error> {
    walker.goto_first_child_raw()?;
    let id = walker.node().id() as u64;
    walker.goto_parent();
    Ok(id)
}
