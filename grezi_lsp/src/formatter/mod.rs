use helix_core::tree_sitter::{self, Node, Tree, TreeCursor};
use helix_lsp_types as lsp_types;
use lsp_types::{Position, TextEdit};
use ropey::Rope;
use tracing::instrument;

use grezi_parser::parse::{byte_pos_from_char_pos, char_pos_from_byte_pos};
use tree_sitter_grz::NodeKind;

mod actions;
mod object;
mod registers;
mod slide;
mod viewbox;

pub fn char_range_from_byte_range(
    range: tree_sitter::Range,
    current_rope: &Rope,
) -> Result<lsp_types::Range, ()> {
    let start = char_pos_from_byte_pos(range.start_point, current_rope).or(Err(()))?;
    let end = char_pos_from_byte_pos(range.end_point, current_rope).or(Err(()))?;
    Ok(lsp_types::Range {
        start: Position {
            line: start.0 as u32,
            character: start.1 as u32,
        },
        end: Position {
            line: end.0 as u32,
            character: end.1 as u32,
        },
    })
}

pub fn byte_range_from_char_range(
    range: lsp_types::Range,
    current_rope: &Rope,
) -> Result<tree_sitter::Range, ()> {
    let start_point = byte_pos_from_char_pos(
        (range.start.line as usize, range.start.character as usize),
        current_rope,
    )
    .or(Err(()))?;
    let end_point = byte_pos_from_char_pos(
        (range.end.line as usize, range.end.character as usize),
        current_rope,
    )
    .or(Err(()))?;
    let start_byte = current_rope
        .try_line_to_byte(start_point.row as usize)
        .or(Err(()))? as u32
        + start_point.col;
    let end_byte = current_rope
        .try_line_to_byte(end_point.row as usize)
        .or(Err(()))? as u32
        + end_point.col;
    Ok(tree_sitter::Range {
        start_point,
        end_point,
        start_byte,
        end_byte,
    })
}

#[instrument(skip_all)]
pub fn format_code(current_rope: &Rope, tree: &Tree) -> Result<Vec<TextEdit>, ()> {
    let mut formatting_cursor = FormattingCursor::new(tree);

    formatting_cursor.goto_first_child(
        WhitespaceEdit::Delete,
        NodeKind::SymSourceFile,
        current_rope,
    )?;

    loop {
        let node = formatting_cursor.node();
        let node_kind = NodeKind::from(node.kind_id());
        match node_kind {
            NodeKind::SymObj => object::format_object(current_rope, &mut formatting_cursor)?,
            NodeKind::SymRegister => {
                registers::format_registers(current_rope, &mut formatting_cursor)?
            }
            NodeKind::SymViewbox => viewbox::format_viewbox(current_rope, &mut formatting_cursor)?,
            NodeKind::SymSlide => slide::format_slide(current_rope, &mut formatting_cursor)?,
            // NodeKind::SymSlide => {}
            NodeKind::SymActions => actions::format_actions(current_rope, &mut formatting_cursor)?,
            _ => return Err(()),
        }

        let result =
            formatting_cursor.goto_next_sibling(WhitespaceEdit::Assert("\n\n"), current_rope)?;

        if !result {
            if formatting_cursor.edited {
                formatting_cursor.edits.pop();
            }
            let range = formatting_cursor.node().byte_range();
            if current_rope.byte_slice(range.start as usize..range.end as usize) != "\n" {
                let mut edit = TextEdit {
                    range: lsp_types::Range {
                        start: Position {
                            line: 0,
                            character: 0,
                        },
                        end: Position {
                            line: 0,
                            character: 0,
                        },
                    },
                    new_text: "\n".to_string(),
                };

                let pos =
                    char_pos_from_byte_pos(formatting_cursor.last_range.end_point, current_rope)
                        .or(Err(()))?;
                edit.range.end.line = pos.0 as u32;
                edit.range.end.character = pos.1 as u32;
                edit.range.start.line = pos.0 as u32;
                edit.range.start.character = pos.1 as u32;

                if formatting_cursor.node().kind_id() == NodeKind::SymWhitespace as u16 {
                    let pos = char_pos_from_byte_pos(
                        formatting_cursor.last_range.start_point,
                        current_rope,
                    )
                    .or(Err(()))?;
                    edit.range.start.line = pos.0 as u32;
                    edit.range.start.character = pos.1 as u32;
                }

                formatting_cursor.edits.push(edit);
            }
            break;
        }
    }

    Ok(formatting_cursor.edits)
}

#[derive(Debug, Clone, Copy)]
pub enum WhitespaceEdit {
    Delete,
    Trailing(&'static str),
    Assert(&'static str),
}

pub struct FormattingCursor<'a> {
    tree_cursor: TreeCursor<'a>,
    pub edits: Vec<TextEdit>,
    pub last_range: helix_core::tree_sitter::Range,
    pub edited: bool,
}

impl<'a> FormattingCursor<'a> {
    pub fn new(tree: &'a Tree) -> FormattingCursor<'a> {
        FormattingCursor {
            tree_cursor: tree.walk(),
            edits: Vec::new(),
            last_range: tree.root_node().range(),
            edited: false,
        }
    }

    fn check_for_error(&self, result: bool) -> Result<bool, ()> {
        if !result {
            return Ok(false);
        }

        if self.tree_cursor.node().is_error() || self.tree_cursor.node().is_missing() {
            return Err(());
        }

        Ok(result)
    }

    fn goto_first_impl(&mut self) -> Result<bool, ()> {
        let result = self.tree_cursor.goto_first_child();
        self.check_for_error(result)
    }

    fn goto_next_impl(&mut self) -> Result<bool, ()> {
        let result = self.tree_cursor.goto_next_sibling();
        self.check_for_error(result)
    }

    pub fn goto_first_child(
        &mut self,
        whitespace_rule: WhitespaceEdit,
        expected_node_kind: NodeKind,
        current_rope: &Rope,
    ) -> Result<bool, ()> {
        assert_eq!(expected_node_kind, NodeKind::from(self.node().kind_id()));
        let result = self.goto_first_impl()?;

        if !self.navigate_and_format(whitespace_rule, current_rope)? {
            return self.check_for_error(false);
        }

        self.check_for_error(result)
    }

    pub fn goto_next_sibling(
        &mut self,
        whitespace_rule: WhitespaceEdit,
        current_rope: &Rope,
    ) -> Result<bool, ()> {
        let result = self.goto_next_impl()?;

        if !self.navigate_and_format(whitespace_rule, current_rope)? {
            return self.check_for_error(false);
        }

        self.check_for_error(result)
    }

    pub fn navigate_and_format(
        &mut self,
        whitespace_rule: WhitespaceEdit,
        current_rope: &Rope,
    ) -> Result<bool, ()> {
        let mut edit = TextEdit {
            range: lsp_types::Range {
                start: Position {
                    line: 0,
                    character: 0,
                },
                end: Position {
                    line: 0,
                    character: 0,
                },
            },
            new_text: String::new(),
        };

        self.edited = false;
        self.last_range = self.tree_cursor.node().range();

        edit.range = char_range_from_byte_range(self.last_range, current_rope).or(Err(()))?;
        if self.tree_cursor.node().kind_id() == NodeKind::SymWhitespace as u16 {
            let next;
            match whitespace_rule {
                WhitespaceEdit::Delete => {
                    self.edits.push(edit);
                    tracing::warn!("deletion");
                    self.edited = true;
                    next = self.goto_next_impl()?;
                }
                WhitespaceEdit::Assert(assertion) => {
                    if current_rope.byte_slice(
                        self.last_range.start_byte as usize..self.last_range.end_byte as usize,
                    ) != assertion
                    {
                        tracing::warn!(assertion, got = %current_rope.byte_slice(self.last_range.start_byte as usize..self.last_range.end_byte as usize), whitespace = true, "Fix assertion");
                        edit.new_text = assertion.to_owned();
                        self.edits.push(edit.clone());
                        self.edited = true;
                    }
                    next = self.goto_next_impl()?;
                }
                WhitespaceEdit::Trailing(trailing) => {
                    next = self.goto_next_impl()?;
                    let range = self.tree_cursor.node().byte_range();
                    if current_rope.byte_slice(range.start as usize..range.end as usize) != trailing
                    {
                        let range = self.tree_cursor.node().byte_range();
                        tracing::warn!(trailing, got = %current_rope.byte_slice(range.start as usize..range.end as usize), whitespace = true, "Fix trailing");
                        edit.new_text = trailing.to_owned();
                    }
                    self.edits.push(edit.clone());
                    self.edited = true;
                }
            }

            if !next {
                return self.check_for_error(false);
            }
        } else {
            match whitespace_rule {
                WhitespaceEdit::Delete => {
                    // Lets assume the `last_range` is a `whitespace` node
                    // with a length of zero, that was just before the last_range
                    // we had before.
                    self.last_range.end_point = self.last_range.start_point;
                    self.last_range.end_byte = self.last_range.start_byte;
                }
                WhitespaceEdit::Assert(assertion) => {
                    tracing::warn!(assertion, got = %current_rope.byte_slice(self.last_range.start_byte as usize..self.last_range.end_byte as usize), whitespace = false, "Fix assertion");
                    edit.new_text = assertion.to_owned();
                    edit.range.end = edit.range.start;
                    self.edits.push(edit);
                    self.edited = true;
                }
                WhitespaceEdit::Trailing(trailing) => {
                    let range = self.tree_cursor.node().byte_range();
                    if current_rope.byte_slice(range.start as usize..range.end as usize) != trailing
                    {
                        let range = self.tree_cursor.node().byte_range();
                        tracing::warn!(trailing, got = %current_rope.byte_slice(range.start as usize..range.end as usize), whitespace = false, "Fix trailing");
                        edit.new_text = trailing.to_owned();
                        edit.range.end = edit.range.start;
                        self.edits.push(edit);
                        self.edited = true;
                    }
                }
            }
        }

        if self.node().kind_id() == NodeKind::SymComment as u16
            && matches!(whitespace_rule, WhitespaceEdit::Assert(_))
            && self.goto_next_impl()?
        {
            self.navigate_and_format(whitespace_rule, current_rope)
        } else {
            Ok(true)
        }
    }

    pub fn revisit(
        &mut self,
        whitespace_rule: WhitespaceEdit,
        current_rope: &Rope,
    ) -> Result<(), ()> {
        match whitespace_rule {
            WhitespaceEdit::Assert(assertion) => {
                if current_rope.byte_slice(
                    self.last_range.start_byte as usize..self.last_range.end_byte as usize,
                ) != assertion
                {
                    tracing::warn!(assertion, got = %current_rope.byte_slice(self.last_range.start_byte as usize..self.last_range.end_byte as usize), "Revisit and fix assertion");
                    if self.edited {
                        if let Some(edit) = self.edits.last_mut() {
                            edit.new_text = assertion.to_owned();
                        }
                    } else {
                        self.edits.push(TextEdit {
                            range: char_range_from_byte_range(self.last_range, current_rope)
                                .or(Err(()))?,
                            new_text: assertion.to_owned(),
                        });
                    }
                } else {
                    tracing::warn!("Popped assertion edit");
                    self.edits.pop();
                }
            }
            WhitespaceEdit::Delete => {
                if self.edited {
                    if let Some(edit) = self.edits.last_mut() {
                        if edit.range.start != edit.range.end {
                            tracing::warn!(popped = false, "Revisit with deletion");
                            edit.new_text.clear();
                        } else {
                            tracing::warn!(popped = true, "Revisit with deletion");
                            self.edits.pop();
                        }
                    }
                }
            }
            _ => unimplemented!(),
        }
        Ok(())
    }

    pub fn goto_parent(&mut self) -> bool {
        self.tree_cursor.goto_parent()
    }

    pub fn node(&self) -> Node<'a> {
        self.tree_cursor.node()
    }
}
