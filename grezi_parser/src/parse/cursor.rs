use std::{
    fmt::Debug,
    io::{self, ErrorKind},
    num::NonZeroU16,
    ops::{Deref, DerefMut},
    sync::Arc,
};

use ropey::{Rope, RopeSlice};
use tracing::instrument;
use tree_sitter::{Node, Tree, TreeCursor};
use tree_sitter_grz::NodeKind;

use super::{
    error::{ErrsWithSource, ParseError},
    slideshow::text::StringLiteral,
    CharRange,
};

trait IsNot {
    fn is_not(&self) -> bool;
}

impl IsNot for bool {
    #[inline(always)]
    fn is_not(&self) -> bool {
        !*self
    }
}

impl<T> IsNot for Option<T> {
    #[inline(always)]
    fn is_not(&self) -> bool {
        self.is_none()
    }
}

#[derive(derive_more::Debug)]
pub struct GrzCursor<'a> {
    #[debug(skip)]
    tree: &'a Tree,
    #[debug(skip)]
    tree_cursor: TreeCursor<'a>,
    #[debug(skip)]
    rope: &'a Rope,
    #[debug(skip)]
    errors: Arc<ErrsWithSource>,
}

impl<'a> GrzCursor<'a> {
    pub fn new(tree: &'a Tree, rope: &'a Rope, errors: Arc<ErrsWithSource>) -> GrzCursor<'a> {
        GrzCursor {
            tree,
            tree_cursor: tree.walk(),
            rope,
            errors,
        }
    }

    pub fn from_node(
        node: Node<'a>,
        tree: &'a Tree,
        rope: &'a Rope,
        errors: Arc<ErrsWithSource>,
    ) -> GrzCursor<'a> {
        GrzCursor {
            tree,
            tree_cursor: node.walk(),
            rope,
            errors,
        }
    }

    #[instrument(skip(self))]
    fn check_for_error<T: IsNot + Debug>(&self, result: T) -> io::Result<T> {
        if result.is_not() {
            return Ok(result);
        }

        if self.tree_cursor.node().is_error() {
            self.errors.append_error(
                ParseError::Syntax(self.char_range()?, "Something is wrong here"),
                self.error_info(),
            );

            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Syntax error"));
        }

        if self.tree_cursor.node().is_missing() {
            self.errors.append_error(
                ParseError::Missing(self.char_range()?, "Something is missing here"),
                self.error_info(),
            );

            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Missing error"));
        }

        Ok(result)
    }

    fn goto_first_impl(&mut self) -> io::Result<bool> {
        let result = self.tree_cursor.goto_first_child();
        self.check_for_error(result)
    }

    fn goto_next_impl(&mut self) -> io::Result<bool> {
        let result = self.tree_cursor.goto_next_sibling();
        self.check_for_error(result)
    }

    #[instrument(skip(self))]
    pub fn goto_first_child<'b>(
        &'b mut self,
        expected_node: NodeKind,
    ) -> io::Result<Option<GrzCursorGuard<'a, 'b>>> {
        let current_node_kind = NodeKind::from(self.node().kind_id());
        let mut new_cursor =
            GrzCursor::from_node(self.node(), self.tree, self.rope, Arc::clone(&self.errors));

        if !new_cursor.goto_first_impl()? {
            return new_cursor.check_for_error(None);
        }

        while !new_cursor.tree_cursor.node().is_named() || new_cursor.tree_cursor.node().is_extra()
        {
            if !new_cursor.goto_next_impl()? {
                return new_cursor.check_for_error(None);
            }
        }

        if current_node_kind != expected_node {
            self.errors.append_error(
                ParseError::BadNode(
                    self.char_range()?,
                    current_node_kind,
                    "A bug in the parser was caught here",
                ),
                self.error_info(),
            );

            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Bad node encountered in cursor",
            ));
        }

        match new_cursor.check_for_error(true) {
            Ok(_) => Ok(Some(GrzCursorGuard {
                hostage_cursor: self,
                new_cursor,
            })),
            Err(e) => Err(e),
        }
    }

    #[instrument(skip(self))]
    pub fn goto_first_child_raw<'b>(
        &'b mut self,
        expected_node: NodeKind,
    ) -> io::Result<Option<GrzCursorGuardRaw<'a, 'b>>> {
        let current_node_kind = NodeKind::from(self.node().kind_id());

        let mut new_cursor =
            GrzCursor::from_node(self.node(), self.tree, self.rope, Arc::clone(&self.errors));

        if !new_cursor.goto_first_impl()? {
            return new_cursor.check_for_error(None);
        }

        while new_cursor.tree_cursor.node().is_extra() {
            if !new_cursor.goto_next_impl()? {
                self.tree_cursor.goto_parent();
                return new_cursor.check_for_error(None);
            }
        }

        if current_node_kind != expected_node {
            self.errors.append_error(
                ParseError::BadNode(
                    self.char_range()?,
                    current_node_kind,
                    "A bug in the parser was caught here",
                ),
                self.error_info(),
            );

            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Bad node encountered in cursor",
            ));
        }

        match new_cursor.check_for_error(true) {
            Ok(_) => Ok(Some(GrzCursorGuardRaw {
                hostage_cursor: self,
                new_cursor,
            })),
            Err(e) => Err(e),
        }
    }

    pub fn first_child_exists(&self) -> bool {
        self.node().named_child_count() > 0
    }

    pub fn first_child_raw_exists(&self) -> bool {
        self.node().child_count() > 0
    }

    #[instrument(skip(self))]
    pub fn goto_next_sibling(&mut self) -> io::Result<bool> {
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

    #[instrument(skip(self))]
    pub fn goto_next_sibling_raw(&mut self) -> io::Result<bool> {
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

    #[instrument(skip_all)]
    pub fn node_to_string_literal(&mut self) -> io::Result<StringLiteral<'a>> {
        match NodeKind::from(self.node().kind_id()) {
            NodeKind::SymStringLiteral => {
                let errors = Arc::clone(&self.errors);
                if let Some(string_literal_cursor) =
                    self.goto_first_child(NodeKind::SymStringLiteral)?
                {
                    StringLiteral::parse_from_cursor(string_literal_cursor, errors)
                } else {
                    Ok(StringLiteral::Escaped(String::new()))
                }
            }
            NodeKind::SymNumberLiteral | NodeKind::SymObjOther | NodeKind::SymIdentifier => {
                Ok(StringLiteral::FullContent(self.rope_slice()?))
            }
            kind => {
                self.errors.append_error(
                    ParseError::BadNode(
                        self.char_range()?,
                        kind,
                        "Expected a string, a number, an identifier, or an `ObjOther` type",
                    ),
                    self.error_info(),
                );
                Ok(StringLiteral::Escaped(String::new()))
            }
        }
    }

    #[instrument(skip(self))]
    pub fn id(&mut self, depth: usize, expected_node_kind: NodeKind) -> io::Result<u64> {
        if let Some(mut child) = self.goto_first_child_raw(expected_node_kind)? {
            let mut id: usize = 0;
            for _ in 0..depth {
                id = id.wrapping_add(child.node().id());
                if !child.goto_next_sibling()? {
                    break;
                }
            }
            Ok(id as u64)
        } else {
            // A node ID of zero will never be inserted
            // into the map because there are no children
            // that a node would be parsed from
            Ok(0)
        }
    }

    pub fn field_id(&self) -> Option<NonZeroU16> {
        self.tree_cursor.field_id()
    }

    pub fn node(&self) -> Node<'a> {
        self.tree_cursor.node()
    }

    #[instrument(skip(self))]
    pub fn char_range(&self) -> io::Result<CharRange> {
        CharRange::new(self.node().range(), self.rope)
    }

    #[instrument(skip(self))]
    pub fn rope_slice(&self) -> io::Result<RopeSlice<'a>> {
        self.rope
            .get_byte_slice(self.tree_cursor.node().byte_range())
            .ok_or_else(|| {
                io::Error::new(
                    ErrorKind::NotFound,
                    "Byte slice that node refers to was not found",
                )
            })
    }

    pub fn smartstring(&self) -> io::Result<smartstring::alias::String> {
        let rope_slice = self.rope_slice()?;
        let mut result_string = smartstring::alias::String::new();
        for chunk in rope_slice.chunks() {
            result_string.push_str(chunk);
        }
        if !result_string.is_inline() {
            self.errors
                .append_error(ParseError::LongName(self.char_range()?), self.error_info());
        }
        Ok(result_string)
    }

    pub fn error_info(&self) -> ErrorInfo {
        ErrorInfo {
            source: self.rope,
            tree: self.tree,
        }
    }
}

pub struct ErrorInfo<'a> {
    pub source: &'a Rope,
    pub tree: &'a Tree,
}

#[derive(Debug)]
pub struct GrzCursorGuard<'a, 'b> {
    // Keeps you from advancing the parent cursor
    // while in a child subtree
    hostage_cursor: &'b mut GrzCursor<'a>,
    new_cursor: GrzCursor<'a>,
}

impl<'a> Deref for GrzCursorGuard<'a, '_> {
    type Target = GrzCursor<'a>;

    fn deref(&self) -> &Self::Target {
        &self.new_cursor
    }
}

impl DerefMut for GrzCursorGuard<'_, '_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.new_cursor
    }
}

impl<'a> GrzCursorGuard<'a, '_> {
    pub fn parent_source(&self) -> io::Result<RopeSlice<'a>> {
        self.hostage_cursor.rope_slice()
    }
}

#[derive(Debug)]
/// A separate type so that a type
/// can declare in it's function child
/// it's looking for the first child raw
pub struct GrzCursorGuardRaw<'a, 'b> {
    // Keeps you from advancing the parent cursor
    // while in a child subtree
    hostage_cursor: &'b mut GrzCursor<'a>,
    new_cursor: GrzCursor<'a>,
}

impl<'a> Deref for GrzCursorGuardRaw<'a, '_> {
    type Target = GrzCursor<'a>;

    fn deref(&self) -> &Self::Target {
        &self.new_cursor
    }
}

impl DerefMut for GrzCursorGuardRaw<'_, '_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.new_cursor
    }
}

impl<'a> GrzCursorGuardRaw<'a, '_> {
    pub fn parent_source(&self) -> io::Result<RopeSlice<'a>> {
        self.hostage_cursor.rope_slice()
    }
}
