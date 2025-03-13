use std::{
    collections::{HashMap, HashSet},
    fs::File,
    hash::BuildHasherDefault,
    io::{self, ErrorKind, Read},
    ops::Sub,
    sync::Arc,
    time::Instant,
};

use cursor::GrzCursor;
use error::ErrsWithSource;
use miette::SourceSpan;
use prehash::Passthru;
use ropey::Rope;
use slideshow::viewbox::Viewbox;
use tracing::instrument;
use tree_sitter::{InputEdit, Point, Tree};
use tree_sitter_grz::NodeKind;

use crate::{slide::ObjState, GrzRoot};

pub mod cursor;
pub mod error;
pub mod helix_loader;
pub mod slideshow;

pub struct GrzFile {
    parser: tree_sitter::Parser,
    incremental_state: Option<IncrementalState>,
    pub tree: Option<Tree>,
    pub version: i32,
    pub error_free_tree: Option<Tree>,
    pub source: Rope,
    pub path_to_grz: String,
    pub slideshow: GrzRoot,
}

impl GrzFile {
    #[instrument(skip(file))]
    pub fn new<R: Read>(path_to_grz: String, file: R) -> io::Result<Self> {
        Ok(Self {
            source: Rope::from_reader(file)?,
            path_to_grz,
            ..Self::empty()?
        })
    }

    pub fn from_string(path_to_grz: String, file: &str) -> io::Result<Self> {
        Ok(Self {
            source: Rope::from_str(file),
            path_to_grz,
            ..Self::empty()?
        })
    }

    #[instrument(skip(slideshow))]
    pub fn wrap_root(path_to_grz: String, slideshow: GrzRoot) -> io::Result<Self> {
        Ok(Self {
            slideshow,
            path_to_grz,
            ..Self::empty()?
        })
    }

    pub fn empty() -> io::Result<Self> {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_grz::LANGUAGE.into())
            .map_err(|e| io::Error::new(ErrorKind::Unsupported, e))?;
        Ok(Self {
            source: Rope::new(),
            path_to_grz: String::new(),
            version: 0,
            tree: None,
            error_free_tree: None,
            parser,
            incremental_state: None,
            slideshow: GrzRoot::default(),
        })
    }

    pub fn clear_incremental_state(&mut self) {
        self.incremental_state = None;
        self.tree = None;
        self.error_free_tree = None;
    }

    #[instrument(skip_all)]
    pub fn update_file(&mut self) -> io::Result<Arc<ErrsWithSource>> {
        self.source = Rope::from_reader(File::open(&self.path_to_grz)?)?;
        self.tree = None;
        self.error_free_tree = None;
        self.incremental_state = None;
        self.slideshow = GrzRoot::default();
        self.parse(&[])
    }

    pub fn find_slide_index_for_edit(
        &self,
        edit: &InputEdit,
        current_index: usize,
    ) -> Option<usize> {
        let tree = self.tree.as_ref()?;
        let starting_node = tree
            .root_node()
            .first_named_child_for_byte(edit.new_end_byte)?;
        let mut tree_walker = GrzCursor::from_node(
            starting_node,
            tree,
            &self.source,
            Arc::new(ErrsWithSource::default()),
        );
        match NodeKind::from(starting_node.kind_id()) {
            NodeKind::SymSlide => {
                let node_id = tree_walker.id(2, NodeKind::SymSlide).ok();
                node_id.and_then(|nid| self.slideshow.slides.get_index_of(&nid))
            }
            NodeKind::SymViewbox => {
                let (_, current_slide) = self.slideshow.slides.get_index(current_index)?;
                let viewbox_name = tree_walker
                    .goto_first_child(NodeKind::SymViewbox)
                    .ok()??
                    .smartstring()
                    .ok()?;
                if current_slide.uses_viewbox(&viewbox_name) {
                    return Some(current_index);
                }
                self.slideshow
                    .slides
                    .iter()
                    .position(|(_, slide)| slide.uses_viewbox(&viewbox_name))
            }
            NodeKind::SymObj => {
                let (_, current_slide) = self.slideshow.slides.get_index(current_index)?;
                let object_name = tree_walker
                    .goto_first_child(NodeKind::SymObj)
                    .ok()??
                    .smartstring()
                    .ok()?;
                if let Some(obj) = current_slide.objects.get(&object_name) {
                    if matches!(obj.positions.state, ObjState::Entering | ObjState::OnScreen) {
                        return Some(current_index);
                    }
                }
                self.slideshow
                    .slides
                    .iter()
                    .position(|(_, slide)| slide.objects.contains_key(&object_name))
            }
            _ => None,
        }
    }

    #[instrument(skip_all)]
    pub fn parse(&mut self, edits: &[InputEdit]) -> io::Result<Arc<ErrsWithSource>> {
        let time = Instant::now();
        let tree = self
            .parser
            .parse_with_options(
                &mut |byte, _| {
                    if byte <= self.source.len_bytes() {
                        let (chunk, start_byte, _, _) = self.source.chunk_at_byte(byte);
                        &chunk.as_bytes()[byte - start_byte..]
                    } else {
                        // out of range
                        &[]
                    }
                },
                self.error_free_tree.as_ref(),
                // Some(tree_sitter::ParseOptions {
                //     progress_callback: Some(&mut |state| {
                //         dbg!(state.current_byte_offset());
                //         true
                //     }),
                // }),
                None,
            )
            .ok_or_else(|| {
                io::Error::new(
                    ErrorKind::TimedOut,
                    "Tree sitter parsing timed out or was cancelled",
                )
            })?;
        let tree_sitter_finished = time.elapsed();
        let mut damaged_node_map = HashSet::default();
        for edit in edits {
            let new_node = tree.root_node().first_child_for_byte(edit.new_end_byte);

            if let Some(new) = new_node {
                damaged_node_map.insert(new.id() as u64);
            }
        }
        let errors = self.update_slideshow(&tree, damaged_node_map)?;
        let total_time = time.elapsed();
        if !errors.has_errors() {
            self.error_free_tree = Some(tree.clone());
        }
        self.tree = Some(tree);
        tracing::warn!(
            ?tree_sitter_finished,
            parsing_finished = ?(total_time - tree_sitter_finished),
            ?total_time,
            "Parsing finished"
        );

        Ok(errors)
    }

    #[instrument(skip_all)]
    fn update_slideshow(
        &mut self,
        new_tree: &Tree,
        damaged_node_map: HashSet<u64, BuildHasherDefault<Passthru>>,
    ) -> io::Result<Arc<ErrsWithSource>> {
        let errors = Arc::new(ErrsWithSource::default());
        let mut new_tree_cursor = GrzCursor::new(new_tree, &self.source, Arc::clone(&errors));

        // Non-fatal errors are added to the ErrsWithSource struct
        // Fatal errors propogate through the parser as an io::Error
        match self.slideshow.parse(
            self.incremental_state.take(),
            damaged_node_map,
            new_tree_cursor
                .goto_first_child(NodeKind::SymSourceFile)?
                .ok_or_else(|| io::Error::new(ErrorKind::UnexpectedEof, "Source file is empty"))?,
            &self.path_to_grz,
            Arc::clone(&errors),
        ) {
            Ok(incremental_state) => self.incremental_state = Some(incremental_state),
            Err(e) => {
                errors.append_error(e.into(), new_tree_cursor.error_info());
            }
        }

        Ok(errors)
    }
}

#[derive(Debug, Default)]
pub struct IncrementalState {
    pub viewbox_nodes: HashMap<smartstring::alias::String, Viewbox>,
}

#[derive(Debug, Clone)]
pub struct CharRange {
    pub start_line: usize,
    pub start_character: usize,
    pub end_line: usize,
    pub end_character: usize,
    pub byte_range: std::ops::Range<usize>,
}

impl Sub<CharRange> for CharRange {
    type Output = CharRange;

    fn sub(mut self, rhs: CharRange) -> Self::Output {
        self.start_line -= rhs.start_line;
        self.end_line -= rhs.start_line;
        self.byte_range.start -= rhs.byte_range.start;
        self.byte_range.end -= rhs.byte_range.start;
        self
    }
}

impl From<CharRange> for SourceSpan {
    fn from(value: CharRange) -> Self {
        (value.byte_range.start, value.byte_range.len()).into()
    }
}

impl CharRange {
    pub fn new(byte_range: tree_sitter::Range, current_rope: &Rope) -> io::Result<CharRange> {
        let start = char_pos_from_byte_pos(byte_range.start_point, current_rope)?;
        let end = char_pos_from_byte_pos(byte_range.end_point, current_rope)?;
        Ok(CharRange {
            start_line: start.0,
            start_character: start.1,
            end_line: end.0,
            end_character: end.1,
            byte_range: byte_range.start_byte..byte_range.end_byte,
        })
    }
}

pub fn char_pos_from_byte_pos(byte_pos: Point, current_rope: &Rope) -> io::Result<(usize, usize)> {
    let line = current_rope.get_line(byte_pos.row).ok_or_else(|| {
        io::Error::new(
            ErrorKind::NotFound,
            format!("Line `{}` doesn't exist", byte_pos.row),
        )
    })?;
    Ok((
        byte_pos.row,
        line.try_byte_to_char(byte_pos.column)
            .map_err(|e| io::Error::new(ErrorKind::NotFound, e))?,
    ))
}

pub fn byte_pos_from_char_pos(char_pos: (usize, usize), current_rope: &Rope) -> io::Result<Point> {
    let line = current_rope.get_line(char_pos.0).ok_or_else(|| {
        io::Error::new(
            ErrorKind::NotFound,
            format!("Line `{}` doesn't exist", char_pos.0),
        )
    })?;
    Ok(Point {
        column: line
            .try_char_to_byte(char_pos.1)
            .map_err(|e| io::Error::new(ErrorKind::NotFound, e))?,
        row: char_pos.0,
    })
}
