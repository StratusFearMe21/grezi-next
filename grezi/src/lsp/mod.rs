use std::sync::atomic::Ordering;

use crate::parser::NodeKind;
use helix_core::syntax::RopeProvider;
use lsp_server::{Connection, Message, Response};
use lsp_types::{
    request::{Completion, PrepareRenameRequest, Rename, Request},
    AnnotatedTextEdit, CompletionItem, CompletionItemKind, CompletionOptions,
    CompletionOptionsCompletionItem, CompletionParams, CompletionResponse, CompletionTextEdit,
    DocumentChanges, OneOf, OptionalVersionedTextDocumentIdentifier, Position,
    PrepareRenameResponse, RenameOptions, RenameParams, SaveOptions, ServerCapabilities,
    TextDocumentEdit, TextDocumentPositionParams, TextDocumentSyncCapability, TextDocumentSyncKind,
    TextDocumentSyncOptions, TextDocumentSyncSaveOptions, TextEdit, Url, WorkDoneProgressOptions,
    WorkspaceEdit,
};
use tree_sitter::{Point, Query, QueryCursor};

pub fn start_lsp(
    mut app: crate::MyEguiApp,
    current_thread: std::thread::Thread,
    lsp_egui_ctx: eframe::egui::Context,
) {
    // Only the lsp will use the parser in lsp mode
    let mut parser = app.parser.lock();
    // Create the transport. Includes the stdio (stdin and stdout) versions but this could
    // also be implemented to use sockets or HTTP.
    let (connection, io_threads) = Connection::stdio();

    // Run the server and wait for the two threads to end (typically by trigger LSP Exit event).
    let server_capabilities = serde_json::to_value(&ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Options(
            TextDocumentSyncOptions {
                open_close: Some(true),
                change: Some(TextDocumentSyncKind::INCREMENTAL),
                save: Some(TextDocumentSyncSaveOptions::SaveOptions(SaveOptions {
                    include_text: Some(true),
                })),
                ..Default::default()
            },
        )),
        rename_provider: Some(OneOf::Right(RenameOptions {
            prepare_provider: Some(true),
            work_done_progress_options: WorkDoneProgressOptions {
                work_done_progress: Some(false),
            },
        })),
        completion_provider: Some(CompletionOptions {
            resolve_provider: Some(false),
            trigger_characters: Some(vec!["{".to_string()]),
            completion_item: Some(CompletionOptionsCompletionItem {
                label_details_support: Some(false),
            }),
            ..Default::default()
        }),
        ..Default::default()
    })
    .unwrap();
    connection.initialize(server_capabilities).unwrap();
    let mut current_rope = ropey::Rope::new();
    let mut current_document_version = 0;
    let mut currently_open = Url::parse("file:///dev/null").unwrap();
    let rename_query = Query::new(tree_sitter_grz::language(), "(identifier) @rename").unwrap();
    let slide_complete_query = Query::new(tree_sitter_grz::language(), "[(slide)] @find").unwrap();

    let mut query_cursor = QueryCursor::new();
    'lsploop: for msg in &connection.receiver {
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req).unwrap() {
                    return;
                }

                match req.method.as_str() {
                    PrepareRenameRequest::METHOD => {
                        if let Ok((rqid, pos)) =
                            req.extract::<TextDocumentPositionParams>(PrepareRenameRequest::METHOD)
                        {
                            let tree_info = app.tree_info.lock();
                            let tree_info = tree_info.as_ref().unwrap();
                            let point = Point {
                                row: pos.position.line as usize,
                                column: pos.position.character as usize,
                            };

                            connection
                                .sender
                                .send(Message::Response(Response::new_ok(
                                    rqid,
                                    tree_info
                                        .0
                                        .root_node()
                                        .descendant_for_point_range(point, point)
                                        .and_then(|f| {
                                            if matches!(
                                                NodeKind::from(f.kind_id()),
                                                NodeKind::Identifier
                                            ) {
                                                let node_range = f.range();
                                                Some(PrepareRenameResponse::Range(
                                                    lsp_types::Range {
                                                        start: Position {
                                                            line: node_range.start_point.row as u32,
                                                            character: node_range.start_point.column
                                                                as u32,
                                                        },
                                                        end: Position {
                                                            line: node_range.end_point.row as u32,
                                                            character: node_range.end_point.column
                                                                as u32,
                                                        },
                                                    },
                                                ))
                                            } else {
                                                None
                                            }
                                        }),
                                )))
                                .unwrap();
                        }
                    }
                    Rename::METHOD => {
                        if let Ok((rqid, rename)) = req.extract::<RenameParams>(Rename::METHOD) {
                            let tree_info = app.tree_info.lock();
                            let tree_info = tree_info.as_ref().unwrap();
                            let mut workspace_edit: Vec<OneOf<TextEdit, AnnotatedTextEdit>> =
                                Vec::new();
                            let point = Point {
                                row: rename.text_document_position.position.line as usize,
                                column: rename.text_document_position.position.character as usize,
                            };

                            let rename_node = tree_info
                                .0
                                .root_node()
                                .descendant_for_point_range(point, point)
                                .unwrap();

                            // identifiers cannot have new lines, so this should work
                            let rename_name = current_rope.byte_slice(rename_node.byte_range());

                            let iter = query_cursor.matches(
                                &rename_query,
                                tree_info.0.root_node(),
                                RopeProvider(current_rope.slice(..)),
                            );

                            for query_match in iter {
                                let node = query_match.captures[0].node;
                                if current_rope.byte_slice(node.byte_range()).eq(&rename_name) {
                                    let range = node.range();

                                    workspace_edit.push(OneOf::Left(TextEdit {
                                        range: lsp_types::Range {
                                            start: Position {
                                                line: range.start_point.row as u32,
                                                character: range.start_point.column as u32,
                                            },
                                            end: Position {
                                                line: range.end_point.row as u32,
                                                character: range.end_point.column as u32,
                                            },
                                        },
                                        new_text: rename.new_name.clone(),
                                    }));
                                }
                            }

                            connection
                                .sender
                                .send(Message::Response(Response::new_ok(
                                    rqid,
                                    Some(WorkspaceEdit {
                                        document_changes: Some(DocumentChanges::Edits(vec![
                                            TextDocumentEdit {
                                                text_document:
                                                    OptionalVersionedTextDocumentIdentifier {
                                                        uri: currently_open.clone(),
                                                        version: Some(current_document_version),
                                                    },
                                                edits: workspace_edit,
                                            },
                                        ])),
                                        ..Default::default()
                                    }),
                                )))
                                .unwrap();
                        }
                    }
                    Completion::METHOD => {
                        if let Ok((rqid, completion)) =
                            req.extract::<CompletionParams>(Completion::METHOD)
                        {
                            let tree_info = app.tree_info.lock();
                            let tree_info = tree_info.as_ref().unwrap();
                            let mut new_text = String::from("{\n");
                            let iter = query_cursor.matches(
                                &slide_complete_query,
                                tree_info.0.root_node(),
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
                                n.walk()
                            } else {
                                connection
                                    .sender
                                    .send(Message::Response(Response::new_ok(
                                        rqid,
                                        None::<CompletionResponse>,
                                    )))
                                    .unwrap();
                                continue 'lsploop;
                            };

                            walker.goto_first_child();
                            walker.goto_first_child();
                            walker.goto_next_sibling();
                            loop {
                                let mut line = String::new();
                                walker.goto_first_child();
                                line.push_str("    ");
                                loop {
                                    match NodeKind::from(walker.node().kind_id()) {
                                        NodeKind::EdgeParser => {
                                            line.push_str(",\n");
                                            if !current_rope
                                                .byte_slice(walker.node().byte_range())
                                                .chunks()
                                                .any(|c| c.contains('|'))
                                            {
                                                new_text.push_str(&line);
                                            }
                                            break;
                                        }
                                        NodeKind::SlideFrom => {
                                            if !walker.goto_next_sibling() {
                                                line.push_str(",\n");
                                                new_text.push_str(&line);
                                                break;
                                            }
                                        }
                                        _ => {
                                            current_rope
                                                .byte_slice(walker.node().byte_range())
                                                .chunks()
                                                .for_each(|c| {
                                                    line.push_str(c);
                                                    if c == ":" {
                                                        line.push(' ');
                                                    }
                                                });

                                            if !walker.goto_next_sibling() {
                                                line.push_str(",\n");
                                                new_text.push_str(&line);
                                                break;
                                            }
                                        }
                                    }
                                }
                                walker.goto_parent();

                                walker.goto_next_sibling();
                                if !walker.goto_next_sibling() {
                                    break;
                                }
                                if !matches!(
                                    NodeKind::from(walker.node().kind_id()),
                                    NodeKind::SlideObj
                                ) {
                                    break;
                                }
                            }
                            new_text.push_str("}[]");
                            let item = CompletionItem {
                                label: "Continue Slide".into(),
                                kind: Some(CompletionItemKind::SNIPPET),
                                preselect: Some(true),
                                text_edit: Some(CompletionTextEdit::Edit(TextEdit {
                                    range: lsp_types::Range {
                                        start: Position {
                                            line: completion.text_document_position.position.line,
                                            character: 0,
                                        },
                                        end: Position {
                                            line: completion.text_document_position.position.line,
                                            character: 10,
                                        },
                                    },
                                    new_text,
                                })),
                                ..Default::default()
                            };
                            connection
                                .sender
                                .send(Message::Response(Response::new_ok(
                                    rqid,
                                    Some(CompletionResponse::Array(vec![item])),
                                )))
                                .unwrap();
                        }
                    }
                    _ => {}
                }

                // ...
            }
            Message::Response(resp) => {}
            Message::Notification(not) => {
                match not.method.as_str() {
                    "textDocument/didOpen" => {
                        let doc: lsp_types::DidOpenTextDocumentParams =
                            serde_json::from_value(not.params).unwrap();
                        currently_open = doc.text_document.uri;
                        let mut slide_show = app.slide_show.write();
                        current_rope = ropey::Rope::from_str(&doc.text_document.text);
                        let mut tree_info = app.tree_info.lock();
                        let tree = parser.parse(&doc.text_document.text, None).unwrap();
                        let ast = crate::parser::parse_file(
                            &doc.text_document.text,
                            &tree,
                            &mut app.helix_cell,
                            &mut *slide_show,
                        );
                        match ast {
                            Ok(ast) => {
                                *tree_info = Some((tree, String::new()));
                                *app.slide_show_file.lock() = doc.text_document.text;
                                slide_show.slide_show = ast;
                            }
                            Err(e) => {
                                println!("{:?}", e);
                                std::process::exit(1);
                            }
                        }

                        app.new_file.store(false, Ordering::Relaxed);
                        current_thread.unpark();
                    }
                    "textDocument/didChange" => {
                        let changes: lsp_types::DidChangeTextDocumentParams =
                            serde_json::from_value(not.params).unwrap();

                        if current_document_version < changes.text_document.version {
                            current_document_version = changes.text_document.version;

                            let mut tree_info = app.tree_info.lock();
                            let tree_info = tree_info.as_mut().unwrap();
                            let changes = changes
                                .content_changes
                                .into_iter()
                                .map(|change| lsp_types::TextEdit {
                                    range: change.range.unwrap(),
                                    new_text: change.text,
                                })
                                .collect();

                            let transaction = helix_lsp::util::generate_transaction_from_edits(
                                &current_rope,
                                changes,
                                helix_lsp::OffsetEncoding::Utf8,
                            );
                            let edits =
                                generate_edits(current_rope.slice(..), transaction.changes());
                            transaction.apply(&mut current_rope);
                            let source = current_rope.slice(..);
                            for edit in edits.iter().rev() {
                                tree_info.0.edit(edit);
                            }

                            // unsafe { syntax.parser.set_cancellation_flag(cancellation_flag) };
                            let tree = parser
                                .parse_with(
                                    &mut |byte, _| {
                                        if byte <= source.len_bytes() {
                                            let (chunk, start_byte, _, _) =
                                                source.chunk_at_byte(byte);
                                            &chunk.as_bytes()[byte - start_byte..]
                                        } else {
                                            // out of range
                                            &[]
                                        }
                                    },
                                    Some(&tree_info.0),
                                )
                                .unwrap();
                            tree_info.0 = tree;
                        }
                    }
                    "textDocument/didSave" => {
                        let save: lsp_types::DidSaveTextDocumentParams =
                            serde_json::from_value(not.params).unwrap();
                        let mut tree_info = app.tree_info.lock();
                        if let Some(info) = tree_info.as_mut() {
                            let text = save.text.unwrap();
                            info.1 = text;
                        }

                        app.new_file.store(true, Ordering::Relaxed);
                        lsp_egui_ctx.request_repaint();
                    }
                    _ => {}
                }
            }
        }
    }
    io_threads.join().unwrap();

    // Shut down gracefully.
}

#[cfg(not(target_arch = "wasm32"))]
pub fn generate_edits(
    old_text: ropey::RopeSlice<'_>,
    changeset: &helix_core::ChangeSet,
) -> Vec<tree_sitter::InputEdit> {
    use helix_core::{chars::char_is_line_ending, Operation::*, Tendril};
    use ropey::RopeSlice;
    let mut old_pos = 0;

    let mut edits = Vec::new();

    if changeset.changes().is_empty() {
        return edits;
    }

    let mut iter = changeset.changes().iter().peekable();

    // TODO; this is a lot easier with Change instead of Operation.

    fn point_at_pos(text: RopeSlice<'_>, pos: usize) -> (usize, Point) {
        let byte = text.char_to_byte(pos); // <- attempted to index past end
        let line = text.char_to_line(pos);
        let line_start_byte = text.line_to_byte(line);
        let col = byte - line_start_byte;

        (byte, Point::new(line, col))
    }

    fn traverse(point: Point, text: &Tendril) -> Point {
        let Point {
            mut row,
            mut column,
        } = point;

        // TODO: there should be a better way here.
        let mut chars = text.chars().peekable();
        while let Some(ch) = chars.next() {
            if char_is_line_ending(ch) && !(ch == '\r' && chars.peek() == Some(&'\n')) {
                row += 1;
                column = 0;
            } else {
                column += 1;
            }
        }
        Point { row, column }
    }

    while let Some(change) = iter.next() {
        let len = match change {
            Delete(i) | Retain(i) => *i,
            Insert(_) => 0,
        };
        let mut old_end = old_pos + len;

        match change {
            Retain(_) => {}
            Delete(_) => {
                let (start_byte, start_position) = point_at_pos(old_text, old_pos);
                let (old_end_byte, old_end_position) = point_at_pos(old_text, old_end);

                // deletion
                edits.push(tree_sitter::InputEdit {
                    start_byte,                       // old_pos to byte
                    old_end_byte,                     // old_end to byte
                    new_end_byte: start_byte,         // old_pos to byte
                    start_position,                   // old pos to coords
                    old_end_position,                 // old_end to coords
                    new_end_position: start_position, // old pos to coords
                });
            }
            Insert(s) => {
                let (start_byte, start_position) = point_at_pos(old_text, old_pos);

                // a subsequent delete means a replace, consume it
                if let Some(Delete(len)) = iter.peek() {
                    old_end = old_pos + len;
                    let (old_end_byte, old_end_position) = point_at_pos(old_text, old_end);

                    iter.next();

                    // replacement
                    edits.push(tree_sitter::InputEdit {
                        start_byte,                                    // old_pos to byte
                        old_end_byte,                                  // old_end to byte
                        new_end_byte: start_byte + s.len(),            // old_pos to byte + s.len()
                        start_position,                                // old pos to coords
                        old_end_position,                              // old_end to coords
                        new_end_position: traverse(start_position, s), // old pos + chars, newlines matter too (iter over)
                    });
                } else {
                    // insert
                    edits.push(tree_sitter::InputEdit {
                        start_byte,                                    // old_pos to byte
                        old_end_byte: start_byte,                      // same
                        new_end_byte: start_byte + s.len(),            // old_pos + s.len()
                        start_position,                                // old pos to coords
                        old_end_position: start_position,              // same
                        new_end_position: traverse(start_position, s), // old pos + chars, newlines matter too (iter over)
                    });
                }
            }
        }
        old_pos = old_end;
    }
    edits
}