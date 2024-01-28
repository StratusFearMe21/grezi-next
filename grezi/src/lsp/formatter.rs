use helix_core::{
    tree_sitter::{Node, Tree, TreeCursor},
    Rope,
};
use lsp_types::{Position, TextEdit};

use crate::{
    parser::{Error, NodeKind},
    MyEguiApp,
};

pub fn format_code(app: &MyEguiApp, current_rope: &Rope) -> Result<Vec<TextEdit>, Error> {
    let tree_info = app.tree_info.lock();
    let tree_info = tree_info.as_ref().unwrap();

    let mut formatting_cursor = FormattingCursor::new(&*tree_info);

    formatting_cursor.goto_first_child(WhitespaceEdit::Delete, current_rope)?;

    macro_rules! format_vb_inner {
        ($tab:expr, $tab_two:expr) => {
            formatting_cursor.goto_next_sibling(WhitespaceEdit::Assert(" "), current_rope)?;
            formatting_cursor.goto_next_sibling(WhitespaceEdit::Delete, current_rope)?;
            formatting_cursor.goto_first_child(WhitespaceEdit::Delete, current_rope)?;
            formatting_cursor.goto_next_sibling(WhitespaceEdit::Delete, current_rope)?;
            formatting_cursor.goto_next_sibling(WhitespaceEdit::Delete, current_rope)?;
            formatting_cursor.goto_parent();
            formatting_cursor.goto_next_sibling(WhitespaceEdit::Assert(" "), current_rope)?;
            formatting_cursor.goto_first_child(WhitespaceEdit::Delete, current_rope)?;
            formatting_cursor
                .goto_next_sibling(WhitespaceEdit::Assert(concat!("\n", $tab)), current_rope)?;
            while formatting_cursor.node().kind_id() == NodeKind::ViewboxObj as u16 {
                formatting_cursor.goto_first_child(WhitespaceEdit::Delete, current_rope)?;
                formatting_cursor.goto_next_sibling(WhitespaceEdit::Delete, current_rope)?;
                formatting_cursor.goto_next_sibling(WhitespaceEdit::Delete, current_rope)?;
                formatting_cursor.goto_parent();
                formatting_cursor.goto_next_sibling(WhitespaceEdit::Trailing(","), current_rope)?;
                formatting_cursor
                    .goto_next_sibling(WhitespaceEdit::Assert(concat!("\n", $tab)), current_rope)?;
            }
            formatting_cursor.revisit(
                WhitespaceEdit::Assert(concat!("\n", $tab_two)),
                current_rope,
            );
            formatting_cursor.goto_parent();
        };
    }

    loop {
        let node = formatting_cursor.node();
        match NodeKind::from(node.kind_id()) {
            NodeKind::Slide => {
                formatting_cursor.goto_first_child(WhitespaceEdit::Delete, current_rope)?;
                formatting_cursor.goto_first_child(WhitespaceEdit::Delete, current_rope)?;
                formatting_cursor
                    .goto_next_sibling(WhitespaceEdit::Assert("\n    "), current_rope)?;
                if formatting_cursor.node().kind_id() == NodeKind::SlideObj as u16 {
                    while formatting_cursor.node().kind_id() == NodeKind::SlideObj as u16 {
                        formatting_cursor.goto_first_child(WhitespaceEdit::Delete, current_rope)?;
                        formatting_cursor
                            .goto_next_sibling(WhitespaceEdit::Delete, current_rope)?;
                        if formatting_cursor.node().kind_id() == NodeKind::SlideVb as u16 {
                            formatting_cursor
                                .goto_first_child(WhitespaceEdit::Delete, current_rope)?;
                            let current_char =
                                current_rope.byte_slice(formatting_cursor.node().byte_range());
                            if current_char == ":" {
                                formatting_cursor
                                    .goto_next_sibling(WhitespaceEdit::Assert(" "), current_rope)?;
                                formatting_cursor
                                    .goto_next_sibling(WhitespaceEdit::Delete, current_rope)?;
                                if formatting_cursor.node().kind_id()
                                    == NodeKind::IndexParser as u16
                                {
                                    formatting_cursor
                                        .goto_first_child(WhitespaceEdit::Delete, current_rope)?;
                                    formatting_cursor
                                        .goto_next_sibling(WhitespaceEdit::Delete, current_rope)?;
                                    formatting_cursor
                                        .goto_next_sibling(WhitespaceEdit::Delete, current_rope)?;
                                    formatting_cursor.goto_parent();
                                }
                            } else if current_char == "|" {
                                format_vb_inner!("        ", "    ");
                                formatting_cursor
                                    .goto_next_sibling(WhitespaceEdit::Delete, current_rope)?;
                                formatting_cursor
                                    .goto_first_child(WhitespaceEdit::Delete, current_rope)?;
                                formatting_cursor
                                    .goto_next_sibling(WhitespaceEdit::Delete, current_rope)?;
                                formatting_cursor
                                    .goto_next_sibling(WhitespaceEdit::Delete, current_rope)?;
                                formatting_cursor.goto_parent();
                            }
                            formatting_cursor.goto_parent();
                        }
                        formatting_cursor
                            .goto_next_sibling(WhitespaceEdit::Delete, current_rope)?;
                        formatting_cursor
                            .goto_next_sibling(WhitespaceEdit::Delete, current_rope)?;
                        formatting_cursor.goto_parent();
                        formatting_cursor
                            .goto_next_sibling(WhitespaceEdit::Trailing(","), current_rope)?;
                        formatting_cursor
                            .goto_next_sibling(WhitespaceEdit::Assert("\n    "), current_rope)?;
                    }
                    formatting_cursor.revisit(WhitespaceEdit::Assert("\n"), current_rope);
                } else {
                    formatting_cursor.revisit(WhitespaceEdit::Delete, current_rope);
                }

                formatting_cursor.goto_parent();
                formatting_cursor.goto_next_sibling(WhitespaceEdit::Delete, current_rope)?;
                formatting_cursor.goto_first_child(WhitespaceEdit::Delete, current_rope)?;
                formatting_cursor
                    .goto_next_sibling(WhitespaceEdit::Assert("\n    "), current_rope)?;
                if formatting_cursor.node().kind() != "]" {
                    while formatting_cursor.node().kind_id() == NodeKind::SlideFunction as u16 {
                        formatting_cursor.goto_first_child(WhitespaceEdit::Delete, current_rope)?;
                        formatting_cursor
                            .goto_next_sibling(WhitespaceEdit::Delete, current_rope)?;
                        formatting_cursor
                            .goto_next_sibling(WhitespaceEdit::Delete, current_rope)?;
                        loop {
                            if formatting_cursor.goto_next_impl()? {
                                formatting_cursor
                                    .navigate_and_format(WhitespaceEdit::Delete, current_rope)?;
                                formatting_cursor
                                    .goto_next_sibling(WhitespaceEdit::Assert(" "), current_rope)?;
                            } else {
                                formatting_cursor.revisit(WhitespaceEdit::Delete, current_rope);
                                break;
                            }
                        }

                        formatting_cursor.goto_parent();
                        formatting_cursor
                            .goto_next_sibling(WhitespaceEdit::Trailing(","), current_rope)?;
                        formatting_cursor
                            .goto_next_sibling(WhitespaceEdit::Assert("\n    "), current_rope)?;
                    }
                    formatting_cursor.revisit(WhitespaceEdit::Assert("\n"), current_rope);
                } else {
                    formatting_cursor.revisit(WhitespaceEdit::Delete, current_rope);
                }

                formatting_cursor.goto_parent();
                formatting_cursor.goto_parent();
            }
            NodeKind::Viewbox => {
                formatting_cursor.goto_first_child(WhitespaceEdit::Delete, current_rope)?;
                formatting_cursor.goto_next_sibling(WhitespaceEdit::Delete, current_rope)?;
                format_vb_inner!("    ", "");
                formatting_cursor.goto_parent();
            }
            NodeKind::Obj => {
                formatting_cursor.goto_first_child(WhitespaceEdit::Delete, current_rope)?;
                formatting_cursor.goto_next_sibling(WhitespaceEdit::Delete, current_rope)?;
                formatting_cursor.goto_next_sibling(WhitespaceEdit::Assert(" "), current_rope)?;
                formatting_cursor.goto_next_sibling(WhitespaceEdit::Delete, current_rope)?;
                formatting_cursor.goto_first_child(WhitespaceEdit::Delete, current_rope)?;
                formatting_cursor
                    .goto_next_sibling(WhitespaceEdit::Assert("\n    "), current_rope)?;
                let mut value_location = None;
                let mut code_location = None;
                let mut language_location = false;
                while formatting_cursor.tree_cursor.node().kind_id() == NodeKind::ObjParam as u16 {
                    formatting_cursor.goto_first_child(WhitespaceEdit::Delete, current_rope)?;
                    let value_rope =
                        current_rope.byte_slice(formatting_cursor.tree_cursor.node().byte_range());
                    if value_rope == "value" {
                        if language_location {
                            language_location = false;
                            let location = formatting_cursor.tree_cursor.node().range();
                            formatting_cursor.edits.push(TextEdit {
                                range: lsp_types::Range {
                                    start: Position {
                                        line: location.start_point.row as u32,
                                        character: location.start_point.column as u32,
                                    },
                                    end: Position {
                                        line: location.end_point.row as u32,
                                        character: location.end_point.column as u32,
                                    },
                                },
                                new_text: "code".to_owned(),
                            });
                        } else {
                            value_location = Some(formatting_cursor.tree_cursor.node().range())
                        }
                    } else if value_rope == "code" {
                        code_location = Some(formatting_cursor.edits.len());
                        let location = formatting_cursor.tree_cursor.node().range();
                        formatting_cursor.edits.push(TextEdit {
                            range: lsp_types::Range {
                                start: Position {
                                    line: location.start_point.row as u32,
                                    character: location.start_point.column as u32,
                                },
                                end: Position {
                                    line: location.end_point.row as u32,
                                    character: location.end_point.column as u32,
                                },
                            },
                            new_text: "value".to_owned(),
                        });
                    } else if value_rope == "language" {
                        if let Some(value_location) = value_location.take() {
                            formatting_cursor.edits.push(TextEdit {
                                range: lsp_types::Range {
                                    start: Position {
                                        line: value_location.start_point.row as u32,
                                        character: value_location.start_point.column as u32,
                                    },
                                    end: Position {
                                        line: value_location.end_point.row as u32,
                                        character: value_location.end_point.column as u32,
                                    },
                                },
                                new_text: "code".to_owned(),
                            });
                        } else {
                            language_location = true;
                        }
                    }
                    formatting_cursor.goto_next_sibling(WhitespaceEdit::Delete, current_rope)?;
                    formatting_cursor
                        .goto_next_sibling(WhitespaceEdit::Assert(" "), current_rope)?;
                    formatting_cursor.goto_parent();
                    formatting_cursor
                        .goto_next_sibling(WhitespaceEdit::Trailing(","), current_rope)?;
                    formatting_cursor
                        .goto_next_sibling(WhitespaceEdit::Assert("\n    "), current_rope)?;
                }
                if let Some(code_location) = code_location.take() {
                    if language_location {
                        formatting_cursor.edits.remove(code_location);
                    }
                }
                formatting_cursor.revisit(WhitespaceEdit::Assert("\n"), current_rope);
                formatting_cursor.goto_parent();
                formatting_cursor.goto_parent();
            }
            NodeKind::Register => {
                formatting_cursor.goto_first_child(WhitespaceEdit::Delete, current_rope)?;
                formatting_cursor.goto_next_sibling(WhitespaceEdit::Delete, current_rope)?;
                formatting_cursor.goto_first_child(WhitespaceEdit::Delete, current_rope)?;
                formatting_cursor.goto_next_sibling(WhitespaceEdit::Delete, current_rope)?;
                formatting_cursor.goto_next_sibling(WhitespaceEdit::Assert(" "), current_rope)?;
                formatting_cursor.goto_parent();
                formatting_cursor.goto_next_sibling(WhitespaceEdit::Delete, current_rope)?;
                formatting_cursor.goto_parent();
            }
            NodeKind::SlideFunction => {
                formatting_cursor.goto_first_child(WhitespaceEdit::Delete, current_rope)?;
                formatting_cursor
                    .goto_next_sibling(WhitespaceEdit::Assert("\n    "), current_rope)?;
                if formatting_cursor.node().kind() != "]" {
                    while formatting_cursor.node().kind_id() == NodeKind::SlideFunction as u16 {
                        formatting_cursor.goto_first_child(WhitespaceEdit::Delete, current_rope)?;
                        formatting_cursor
                            .goto_next_sibling(WhitespaceEdit::Delete, current_rope)?;
                        formatting_cursor
                            .goto_next_sibling(WhitespaceEdit::Delete, current_rope)?;
                        loop {
                            if formatting_cursor.goto_next_impl()? {
                                formatting_cursor
                                    .navigate_and_format(WhitespaceEdit::Delete, current_rope)?;
                                formatting_cursor
                                    .goto_next_sibling(WhitespaceEdit::Assert(" "), current_rope)?;
                            } else {
                                formatting_cursor.revisit(WhitespaceEdit::Delete, current_rope);
                                break;
                            }
                        }

                        formatting_cursor.goto_parent();
                        formatting_cursor
                            .goto_next_sibling(WhitespaceEdit::Trailing(","), current_rope)?;
                        formatting_cursor
                            .goto_next_sibling(WhitespaceEdit::Assert("\n    "), current_rope)?;
                    }
                    formatting_cursor.revisit(WhitespaceEdit::Assert("\n"), current_rope);
                } else {
                    formatting_cursor.revisit(WhitespaceEdit::Delete, current_rope);
                }

                formatting_cursor.goto_parent();
            }
            kind => {
                return Err(Error::BadNode(
                    formatting_cursor.node().range().into(),
                    kind,
                ))
            }
        }

        if !formatting_cursor.goto_next_sibling(WhitespaceEdit::Assert("\n\n"), current_rope)? {
            if formatting_cursor.edited {
                formatting_cursor.edits.pop();
            }
            if current_rope.byte_slice(formatting_cursor.node().byte_range()) != "\n" {
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

                let pos = Position {
                    line: formatting_cursor.last_range.end_point.row as u32,
                    character: formatting_cursor.last_range.end_point.column as u32,
                };
                edit.range.end = pos;
                edit.range.start = pos;

                if formatting_cursor.node().kind_id() == NodeKind::Whitespace as u16 {
                    let pos = Position {
                        line: formatting_cursor.last_range.start_point.row as u32,
                        character: formatting_cursor.last_range.start_point.column as u32,
                    };
                    edit.range.start = pos;
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

    fn goto_first_impl(&mut self) -> Result<bool, Error> {
        let result = self.tree_cursor.goto_first_child();
        self.check_for_error(result)
    }

    fn goto_next_impl(&mut self) -> Result<bool, Error> {
        let result = self.tree_cursor.goto_next_sibling();
        self.check_for_error(result)
    }

    pub fn goto_first_child(
        &mut self,
        whitespace_rule: WhitespaceEdit,
        current_rope: &Rope,
    ) -> Result<bool, Error> {
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
    ) -> Result<bool, Error> {
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
    ) -> Result<bool, Error> {
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

        let pos = Position {
            line: self.last_range.start_point.row as u32,
            character: self.last_range.start_point.column as u32,
        };
        edit.range.start = pos;

        let pos = Position {
            line: self.last_range.end_point.row as u32,
            character: self.last_range.end_point.column as u32,
        };
        edit.range.end = pos;
        if self.tree_cursor.node().kind_id() == NodeKind::Whitespace as u16 {
            let next;
            match whitespace_rule {
                WhitespaceEdit::Delete => {
                    self.edits.push(edit);
                    self.edited = true;
                    next = self.goto_next_impl()?;
                }
                WhitespaceEdit::Assert(assertion) => {
                    if current_rope.byte_slice(self.last_range.start_byte..self.last_range.end_byte)
                        != assertion
                    {
                        edit.new_text = assertion.to_owned();
                        self.edits.push(edit.clone());
                        self.edited = true;
                    }
                    next = self.goto_next_impl()?;
                }
                WhitespaceEdit::Trailing(trailing) => {
                    next = self.goto_next_impl()?;
                    if current_rope.byte_slice(self.tree_cursor.node().byte_range()) != trailing {
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
                WhitespaceEdit::Assert(assertion) => {
                    edit.new_text = assertion.to_owned();
                    edit.range.end = edit.range.start;
                    self.edits.push(edit);
                    self.edited = true;
                }
                WhitespaceEdit::Trailing(trailing) => {
                    if current_rope.byte_slice(self.tree_cursor.node().byte_range()) != trailing {
                        edit.new_text = trailing.to_owned();
                        edit.range.end = edit.range.start;
                        self.edits.push(edit);
                        self.edited = true;
                    }
                }
                _ => {}
            }
        }

        Ok(true)
    }

    pub fn revisit(&mut self, whitespace_rule: WhitespaceEdit, current_rope: &Rope) {
        match whitespace_rule {
            WhitespaceEdit::Assert(assertion) => {
                if current_rope.byte_slice(self.last_range.start_byte..self.last_range.end_byte)
                    != assertion
                {
                    if self.edited {
                        if let Some(edit) = self.edits.last_mut() {
                            edit.new_text = assertion.to_owned();
                        }
                    } else {
                        self.edits.push(TextEdit {
                            range: lsp_types::Range {
                                start: Position {
                                    line: self.last_range.start_point.row as u32,
                                    character: self.last_range.start_point.column as u32,
                                },
                                end: Position {
                                    line: self.last_range.end_point.row as u32,
                                    character: self.last_range.end_point.column as u32,
                                },
                            },
                            new_text: assertion.to_owned(),
                        });
                    }
                } else {
                    self.edits.pop();
                }
            }
            WhitespaceEdit::Delete => {
                if self.edited {
                    if let Some(edit) = self.edits.last_mut() {
                        if edit.range.start != edit.range.end {
                            edit.new_text.clear();
                        } else {
                            self.edits.pop();
                        }
                    }
                }
            }
            _ => unimplemented!(),
        }
    }

    pub fn goto_parent(&mut self) -> bool {
        self.tree_cursor.goto_parent()
    }

    pub fn node(&self) -> Node<'a> {
        self.tree_cursor.node()
    }
}
