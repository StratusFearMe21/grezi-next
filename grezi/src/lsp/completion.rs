use helix_core::{
    syntax::RopeProvider,
    tree_sitter::{Node, Point, Query, QueryCursor, Tree},
    Rope,
};
use lsp_types::{
    CompletionItem, CompletionItemKind, CompletionItemLabelDetails, CompletionParams,
    CompletionResponse, CompletionTextEdit, Documentation, InsertTextFormat, MarkupContent,
    MarkupKind, Position, TextEdit,
};

use crate::parser::{GrzCursor, NodeKind};

use super::formatter::char_range_from_byte_range;

pub fn complete_source_file(
    completion: CompletionParams,
    slide_complete_query: &Query,
    tree_info: &Tree,
    current_rope: &Rope,
    query_cursor: &mut QueryCursor,
) -> Result<CompletionResponse, crate::parser::Error> {
    let new_slide_range = lsp_types::Range {
        start: Position {
            line: completion.text_document_position.position.line,
            character: 0,
        },
        end: Position {
            line: completion.text_document_position.position.line,
            character: 0,
        },
    };
    let new_slide_item = CompletionItem {
        label: "new".into(),
        label_details: Some(CompletionItemLabelDetails {
            description: Some("Create a new slide".to_string()),
            detail: None,
        }),
        deprecated: Some(false),
        insert_text_format: Some(InsertTextFormat::SNIPPET),
        sort_text: Some("ffffffef".to_string()),
        filter_text: Some("new".into()),
        kind: Some(CompletionItemKind::SNIPPET),
        preselect: Some(true),
        text_edit: Some(CompletionTextEdit::Edit(TextEdit {
            new_text: "{$0}[]".to_string(),
            range: new_slide_range,
        })),
        additional_text_edits: Some(Vec::new()),
        ..Default::default()
    };

    let mut new_text = String::from("{\n");
    query_cursor.set_point_range(
        Point { row: 0, column: 0 }..Point {
            row: usize::MAX,
            column: usize::MAX,
        },
    );
    let iter = query_cursor.matches(
        slide_complete_query,
        tree_info.root_node(),
        RopeProvider(current_rope.slice(..)),
    );
    let mut node = None;
    for query_match in iter {
        if query_match.captures[0].node.range().end_point.row
            > completion.text_document_position.position.line as usize
        {
            break;
        }
        node = Some(query_match.captures[0].node);
    }
    let mut walker = if let Some(n) = node {
        GrzCursor::from_node(n, current_rope)
    } else {
        return Ok(CompletionResponse::Array(vec![new_slide_item]));
    };

    walker.goto_first_child()?;
    walker.goto_first_child()?;

    let mut line = String::new();
    loop {
        line.clear();
        walker.goto_first_child_raw()?;
        line.push_str("    ");
        'outer: {
            loop {
                match NodeKind::from(walker.node().kind_id()) {
                    NodeKind::EdgeParser => {
                        if current_rope
                            .byte_slice(walker.node().byte_range())
                            .chunks()
                            .any(|c| c.contains('|'))
                        {
                            break 'outer;
                        }
                    }
                    NodeKind::Identifier => {
                        current_rope
                            .byte_slice(walker.node().byte_range())
                            .chunks()
                            .for_each(|c| line.push_str(c));
                        line.push(',');
                    }
                    _ => {}
                }

                if !walker.goto_next_sibling_raw()? {
                    break;
                }
            }
            new_text.push_str(&line);
            new_text.push('\n');
        }

        walker.goto_parent();

        walker.goto_next_sibling_raw()?;
        if !walker.goto_next_sibling_raw()? {
            break;
        }
        if !matches!(NodeKind::from(walker.node().kind_id()), NodeKind::SlideObj) {
            break;
        }
    }
    new_text.push_str("}[]");
    let continue_slide_range = lsp_types::Range {
        start: Position {
            line: completion.text_document_position.position.line,
            character: 0,
        },
        end: Position {
            line: completion.text_document_position.position.line,
            character: current_rope
                .line(completion.text_document_position.position.line as usize)
                .len_chars() as u32,
        },
    };
    let continue_slide_item = CompletionItem {
        label: "continue".into(),
        label_details: Some(CompletionItemLabelDetails {
            description: Some("Copy the previous slide here".to_string()),
            detail: None,
        }),
        kind: Some(CompletionItemKind::SNIPPET),
        preselect: Some(true),
        deprecated: Some(false),
        insert_text_format: Some(InsertTextFormat::SNIPPET),
        sort_text: Some("ffffffef".to_string()),
        filter_text: Some("continue".into()),
        text_edit: Some(CompletionTextEdit::Edit(TextEdit {
            new_text,
            range: continue_slide_range,
        })),
        additional_text_edits: Some(Vec::new()),
        ..Default::default()
    };

    Ok(CompletionResponse::Array(vec![
        continue_slide_item,
        new_slide_item,
    ]))
}

pub fn complete_viewbox(
    query_cursor: &mut QueryCursor,
    viewbox_name_query: &Query,
    tree_info: &Tree,
    current_rope: &Rope,
    completion_point: Point,
    completion_node: Node<'_>,
) -> Vec<CompletionItem> {
    query_cursor.set_point_range(Point { row: 0, column: 0 }..completion_point);
    let iter = query_cursor.matches(
        &viewbox_name_query,
        tree_info.root_node(),
        RopeProvider(current_rope.slice(..)),
    );

    let completion_range = completion_node.range();

    let mut completions = vec![CompletionItem {
        label: "Size".to_string(),
        kind: Some(CompletionItemKind::VARIABLE),
        deprecated: Some(false),
        preselect: Some(true),
        insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
        insert_text_mode: None,
        text_edit: Some(CompletionTextEdit::Edit(TextEdit {
            new_text: "Size".to_string(),
            range: char_range_from_byte_range(completion_range, &current_rope).unwrap(),
        })),
        additional_text_edits: Some(Vec::new()),
        ..Default::default()
    }];
    completions.extend(iter.map(|query_match| {
        let vb_byte_range = query_match.captures[0].node.byte_range();
        let byte_range = query_match.captures[1].node.byte_range();
        let label = current_rope.byte_slice(byte_range).to_string();

        CompletionItem {
            label: label.clone(),
            kind: Some(CompletionItemKind::VARIABLE),
            deprecated: Some(false),
            preselect: Some(true),
            insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
            insert_text_mode: None,
            text_edit: Some(CompletionTextEdit::Edit(TextEdit {
                new_text: label,
                range: char_range_from_byte_range(completion_range, &current_rope).unwrap(),
            })),
            additional_text_edits: Some(Vec::new()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: format!("```grz\n{}\n```", current_rope.byte_slice(vb_byte_range)),
            })),
            ..Default::default()
        }
    }));

    completions
}

pub fn complete_object(
    query_cursor: &mut QueryCursor,
    object_name_query: &Query,
    tree_info: &Tree,
    current_rope: &Rope,
    completion_point: Point,
    completion_node: Node<'_>,
) -> Vec<CompletionItem> {
    query_cursor.set_point_range(Point { row: 0, column: 0 }..completion_point);
    let iter = query_cursor.matches(
        &object_name_query,
        tree_info.root_node(),
        RopeProvider(current_rope.slice(..)),
    );

    iter.map(|query_match| {
        let byte_range = query_match.captures[0].node.byte_range();
        let label = current_rope.byte_slice(byte_range).to_string();
        let completion_range = completion_node.range();
        CompletionItem {
            kind: Some(CompletionItemKind::VARIABLE),
            deprecated: Some(false),
            preselect: Some(true),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            insert_text_mode: None,
            text_edit: Some(CompletionTextEdit::Edit(TextEdit {
                new_text: if current_rope
                    .line(completion_range.end_point.row)
                    .char(completion_range.end_point.column)
                    == ':'
                {
                    format!("{}$0", label)
                } else {
                    format!("{}$0,", label)
                },
                range: char_range_from_byte_range(completion_range, &current_rope).unwrap(),
            })),
            label,
            additional_text_edits: Some(Vec::new()),
            ..Default::default()
        }
    })
    .collect()
}

pub fn complete_object_type(current_rope: &Rope, completion_node: Node<'_>) -> Vec<CompletionItem> {
    let completion_range = completion_node.range();

    vec![
        CompletionItem {
            label: "Paragraph".to_string(),
            kind: Some(CompletionItemKind::TYPE_PARAMETER),
            deprecated: Some(false),
            preselect: Some(true),
            insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
            insert_text_mode: None,
            text_edit: Some(CompletionTextEdit::Edit(TextEdit {
                new_text: "Paragraph".to_string(),
                range: char_range_from_byte_range(completion_range, &current_rope).unwrap(),
            })),
            additional_text_edits: Some(Vec::new()),
            ..Default::default()
        },
        CompletionItem {
            label: "Header".to_string(),
            kind: Some(CompletionItemKind::TYPE_PARAMETER),
            deprecated: Some(false),
            preselect: Some(true),
            insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
            insert_text_mode: None,
            text_edit: Some(CompletionTextEdit::Edit(TextEdit {
                new_text: "Header".to_string(),
                range: char_range_from_byte_range(completion_range, &current_rope).unwrap(),
            })),
            additional_text_edits: Some(Vec::new()),
            ..Default::default()
        },
        CompletionItem {
            label: "Rect".to_string(),
            kind: Some(CompletionItemKind::TYPE_PARAMETER),
            deprecated: Some(false),
            preselect: Some(true),
            insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
            insert_text_mode: None,
            text_edit: Some(CompletionTextEdit::Edit(TextEdit {
                new_text: "Rect".to_string(),
                range: char_range_from_byte_range(completion_range, &current_rope).unwrap(),
            })),
            additional_text_edits: Some(Vec::new()),
            ..Default::default()
        },
        CompletionItem {
            label: "Image".to_string(),
            kind: Some(CompletionItemKind::TYPE_PARAMETER),
            deprecated: Some(false),
            preselect: Some(true),
            insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
            insert_text_mode: None,
            text_edit: Some(CompletionTextEdit::Edit(TextEdit {
                new_text: "Image".to_string(),
                range: char_range_from_byte_range(completion_range, &current_rope).unwrap(),
            })),
            additional_text_edits: Some(Vec::new()),
            ..Default::default()
        },
    ]
}

pub fn complete_object_params(
    current_rope: &Rope,
    completion_node: Node<'_>,
) -> Vec<CompletionItem> {
    let completion_range = completion_node.range();

    vec![
        CompletionItem {
            label: "value".to_string(),
            kind: Some(CompletionItemKind::TYPE_PARAMETER),
            deprecated: Some(false),
            preselect: Some(true),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            insert_text_mode: None,
            text_edit: Some(CompletionTextEdit::Edit(TextEdit {
                new_text: "value: \"$0\",".to_string(),
                range: char_range_from_byte_range(completion_range, &current_rope).unwrap(),
            })),
            additional_text_edits: Some(Vec::new()),
            ..Default::default()
        },
        CompletionItem {
            label: "height".to_string(),
            kind: Some(CompletionItemKind::TYPE_PARAMETER),
            deprecated: Some(false),
            preselect: Some(true),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            insert_text_mode: None,
            text_edit: Some(CompletionTextEdit::Edit(TextEdit {
                new_text: "height: \"$0\",".to_string(),
                range: char_range_from_byte_range(completion_range, &current_rope).unwrap(),
            })),
            additional_text_edits: Some(Vec::new()),
            ..Default::default()
        },
        CompletionItem {
            label: "code".to_string(),
            kind: Some(CompletionItemKind::TYPE_PARAMETER),
            deprecated: Some(false),
            preselect: Some(true),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            insert_text_mode: None,
            text_edit: Some(CompletionTextEdit::Edit(TextEdit {
                new_text: "code: r#\"$0\"#,".to_string(),
                range: char_range_from_byte_range(completion_range, &current_rope).unwrap(),
            })),
            additional_text_edits: Some(Vec::new()),
            ..Default::default()
        },
        CompletionItem {
            label: "tint".to_string(),
            kind: Some(CompletionItemKind::TYPE_PARAMETER),
            deprecated: Some(false),
            preselect: Some(true),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            insert_text_mode: None,
            text_edit: Some(CompletionTextEdit::Edit(TextEdit {
                new_text: "tint: $0,".to_string(),
                range: char_range_from_byte_range(completion_range, &current_rope).unwrap(),
            })),
            additional_text_edits: Some(Vec::new()),
            ..Default::default()
        },
        CompletionItem {
            label: "color".to_string(),
            kind: Some(CompletionItemKind::TYPE_PARAMETER),
            deprecated: Some(false),
            preselect: Some(true),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            insert_text_mode: None,
            text_edit: Some(CompletionTextEdit::Edit(TextEdit {
                new_text: "color: $0,".to_string(),
                range: char_range_from_byte_range(completion_range, &current_rope).unwrap(),
            })),
            additional_text_edits: Some(Vec::new()),
            ..Default::default()
        },
        CompletionItem {
            label: "background".to_string(),
            kind: Some(CompletionItemKind::TYPE_PARAMETER),
            deprecated: Some(false),
            preselect: Some(true),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            insert_text_mode: None,
            text_edit: Some(CompletionTextEdit::Edit(TextEdit {
                new_text: "background: $0,".to_string(),
                range: char_range_from_byte_range(completion_range, &current_rope).unwrap(),
            })),
            additional_text_edits: Some(Vec::new()),
            ..Default::default()
        },
        CompletionItem {
            label: "scale".to_string(),
            kind: Some(CompletionItemKind::TYPE_PARAMETER),
            deprecated: Some(false),
            preselect: Some(true),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            insert_text_mode: None,
            text_edit: Some(CompletionTextEdit::Edit(TextEdit {
                new_text: "scale: \"$0\",".to_string(),
                range: char_range_from_byte_range(completion_range, &current_rope).unwrap(),
            })),
            additional_text_edits: Some(Vec::new()),
            ..Default::default()
        },
        CompletionItem {
            label: "font_family".to_string(),
            kind: Some(CompletionItemKind::TYPE_PARAMETER),
            deprecated: Some(false),
            preselect: Some(true),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            insert_text_mode: None,
            text_edit: Some(CompletionTextEdit::Edit(TextEdit {
                new_text: "font_family: \"$0\",".to_string(),
                range: char_range_from_byte_range(completion_range, &current_rope).unwrap(),
            })),
            additional_text_edits: Some(Vec::new()),
            ..Default::default()
        },
        CompletionItem {
            label: "line_height".to_string(),
            kind: Some(CompletionItemKind::TYPE_PARAMETER),
            deprecated: Some(false),
            preselect: Some(true),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            insert_text_mode: None,
            text_edit: Some(CompletionTextEdit::Edit(TextEdit {
                new_text: "line_height: $0,".to_string(),
                range: char_range_from_byte_range(completion_range, &current_rope).unwrap(),
            })),
            additional_text_edits: Some(Vec::new()),
            ..Default::default()
        },
        CompletionItem {
            label: "source".to_string(),
            kind: Some(CompletionItemKind::TYPE_PARAMETER),
            deprecated: Some(false),
            preselect: Some(true),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            insert_text_mode: None,
            text_edit: Some(CompletionTextEdit::Edit(TextEdit {
                new_text: "source: \"$0\",".to_string(),
                range: char_range_from_byte_range(completion_range, &current_rope).unwrap(),
            })),
            additional_text_edits: Some(Vec::new()),
            ..Default::default()
        },
        CompletionItem {
            label: "font_size".to_string(),
            kind: Some(CompletionItemKind::TYPE_PARAMETER),
            deprecated: Some(false),
            preselect: Some(true),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            insert_text_mode: None,
            text_edit: Some(CompletionTextEdit::Edit(TextEdit {
                new_text: "font_size: $0,".to_string(),
                range: char_range_from_byte_range(completion_range, &current_rope).unwrap(),
            })),
            additional_text_edits: Some(Vec::new()),
            ..Default::default()
        },
        CompletionItem {
            label: "language".to_string(),
            kind: Some(CompletionItemKind::TYPE_PARAMETER),
            deprecated: Some(false),
            preselect: Some(true),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            insert_text_mode: None,
            text_edit: Some(CompletionTextEdit::Edit(TextEdit {
                new_text: "language: \"$0\",".to_string(),
                range: char_range_from_byte_range(completion_range, &current_rope).unwrap(),
            })),
            additional_text_edits: Some(Vec::new()),
            ..Default::default()
        },
        CompletionItem {
            label: "align".to_string(),
            kind: Some(CompletionItemKind::TYPE_PARAMETER),
            deprecated: Some(false),
            preselect: Some(true),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            insert_text_mode: None,
            text_edit: Some(CompletionTextEdit::Edit(TextEdit {
                new_text: "align: \"$0\",".to_string(),
                range: char_range_from_byte_range(completion_range, &current_rope).unwrap(),
            })),
            additional_text_edits: Some(Vec::new()),
            ..Default::default()
        },
    ]
}

pub fn complete_top_level_object(
    current_rope: &Rope,
    parent_object: Node<'_>,
    completion_node: Node<'_>,
) -> Vec<CompletionItem> {
    let completion_range = parent_object.range();

    vec![
        CompletionItem {
            label: "viewbox".to_string(),
            kind: Some(CompletionItemKind::SNIPPET),
            deprecated: Some(false),
            preselect: Some(true),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            insert_text_mode: None,
            text_edit: Some(CompletionTextEdit::Edit(TextEdit {
                new_text: format!(
                    "{}: ${{1:Size}}[0] >$0]",
                    current_rope
                        .byte_slice(completion_node.prev_named_sibling().unwrap().byte_range())
                ),
                range: char_range_from_byte_range(completion_range, &current_rope).unwrap(),
            })),
            additional_text_edits: Some(Vec::new()),
            ..Default::default()
        },
        CompletionItem {
            label: "object".to_string(),
            kind: Some(CompletionItemKind::SNIPPET),
            deprecated: Some(false),
            preselect: Some(true),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            insert_text_mode: None,
            text_edit: Some(CompletionTextEdit::Edit(TextEdit {
                new_text: format!(
                    "{}: ${{1:Paragraph}}($0)",
                    current_rope
                        .byte_slice(completion_node.prev_named_sibling().unwrap().byte_range())
                ),
                range: char_range_from_byte_range(completion_range, &current_rope).unwrap(),
            })),
            additional_text_edits: Some(Vec::new()),
            ..Default::default()
        },
    ]
}
