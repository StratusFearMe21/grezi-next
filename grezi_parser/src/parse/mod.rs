use std::{
    collections::HashMap,
    fs::File,
    hash::BuildHasherDefault,
    io::{self, ErrorKind, Read},
    ops::Sub,
    path::PathBuf,
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
use tree_sitter::{Point, Tree};

use crate::GrzRoot;

pub mod cursor;
pub mod error;
pub mod helix_loader;
pub mod slideshow;

pub struct GrzFile {
    parser: tree_sitter::Parser,
    incremental_state: Option<IncrementalState>,
    pub tree: Option<Tree>,
    pub error_free_tree: Option<Tree>,
    pub source: Rope,
    pub path_to_grz: PathBuf,
    pub slideshow: GrzRoot,
}

impl GrzFile {
    #[instrument(skip(file))]
    pub fn new<R: Read>(path_to_grz: PathBuf, file: R) -> io::Result<Self> {
        let source = Rope::from_reader(file)?;
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_grz::LANGUAGE.into())
            .map_err(|e| io::Error::new(ErrorKind::Unsupported, e))?;
        Ok(Self {
            source,
            path_to_grz,
            tree: None,
            error_free_tree: None,
            parser,
            incremental_state: None,
            slideshow: GrzRoot::default(),
        })
    }

    pub fn update_file(&mut self) -> io::Result<Arc<ErrsWithSource>> {
        self.source = Rope::from_reader(File::open(self.path_to_grz.as_path())?)?;
        self.tree = None;
        self.error_free_tree = None;
        self.incremental_state = None;
        self.parse()
    }

    #[instrument(skip(slideshow))]
    pub fn wrap_root(path_to_grz: PathBuf, slideshow: GrzRoot) -> io::Result<Self> {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_grz::LANGUAGE.into())
            .map_err(|e| io::Error::new(ErrorKind::Unsupported, e))?;
        Ok(Self {
            source: Rope::new(),
            path_to_grz,
            tree: None,
            error_free_tree: None,
            parser,
            incremental_state: None,
            slideshow,
        })
    }

    #[instrument(skip_all)]
    pub fn parse(&mut self) -> io::Result<Arc<ErrsWithSource>> {
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
                None,
            )
            .ok_or_else(|| {
                io::Error::new(
                    ErrorKind::TimedOut,
                    "Tree sitter parsing timed out or was cancelled",
                )
            })?;
        let tree_sitter_finished = time.elapsed();
        let errors = self.update_slideshow(&tree)?;
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
    fn update_slideshow(&mut self, new_tree: &Tree) -> io::Result<Arc<ErrsWithSource>> {
        let errors = Arc::new(ErrsWithSource::default());
        let mut new_tree_cursor = GrzCursor::new(new_tree, &self.source, Arc::clone(&errors));

        // Non-fatal errors are added to the ErrsWithSource struct
        // Fatal errors propogate through the parser as an io::Error
        match self.slideshow.parse(
            self.incremental_state.take(),
            new_tree_cursor
                .goto_first_child()?
                .ok_or_else(|| io::Error::new(ErrorKind::UnexpectedEof, "Source file is empty"))?,
            self.path_to_grz.as_path(),
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
    pub viewbox_nodes:
        HashMap<u64, (smartstring::alias::String, Viewbox), BuildHasherDefault<Passthru>>,
    pub object_nodes: HashMap<u64, smartstring::alias::String, BuildHasherDefault<Passthru>>,
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
