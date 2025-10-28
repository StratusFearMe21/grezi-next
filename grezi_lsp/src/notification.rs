use std::sync::Arc;

use grezi_file_owner::FileOwnerMessage;
use grezi_parser::parse::GrzFile;
use helix_core::syntax::generate_edits;
use helix_lsp::Position;
use helix_lsp_types::{
    self as lsp_types, ApplyWorkspaceEditParams, DocumentChanges, OneOf,
    OptionalVersionedTextDocumentIdentifier, TextDocumentContentChangeEvent, TextDocumentEdit,
    TextEdit, VersionedTextDocumentIdentifier, WorkspaceEdit,
    notification::{
        DidChangeTextDocument, DidChangeWatchedFiles, DidCloseTextDocument, DidOpenTextDocument,
        DidSaveTextDocument, Notification,
    },
    request::{ApplyWorkspaceEdit, Request},
};
use lsp_server::{Connection, Message};
use ropey::Rope;
use tree_sitter_grz::NodeKind;

use crate::GrzLsp;

impl GrzLsp {
    pub fn handle_notification_message(
        &mut self,
        connection: &Connection,
        not: lsp_server::Notification,
    ) -> bool {
        match not.method.as_str() {
            DidOpenTextDocument::METHOD => {
                let doc: lsp_types::DidOpenTextDocumentParams =
                    serde_json::from_value(not.params).unwrap();
                let mut new_file = GrzFile::from_string(
                    percent_encoding::percent_decode_str(doc.text_document.uri.path())
                        .decode_utf8()
                        .unwrap()
                        .into_owned(),
                    &doc.text_document.text,
                )
                .unwrap();
                new_file.version = doc.text_document.version;

                let parse_result = new_file.parse(&[]);

                if let Some(parse_result) = parse_result.as_ref().ok().map(|pr| Arc::clone(pr)) {
                    Self::report_error_messages(
                        connection,
                        Arc::clone(&parse_result),
                        doc.text_document.uri.clone(),
                        new_file.version,
                    );
                }

                self.last_edited_uri = doc.text_document.uri.clone();
                self.grz_files.insert(doc.text_document.uri, new_file);

                if !parse_result.ok().map(|pr| pr.has_errors()).unwrap_or(true) {
                    self.handle_file_owner_message(FileOwnerMessage::Index {
                        index: 0,
                        reset_time: true,
                    });
                }
            }
            DidCloseTextDocument::METHOD => {
                let doc: lsp_types::DidCloseTextDocumentParams =
                    serde_json::from_value(not.params).unwrap();

                self.grz_files.remove(&doc.text_document.uri);
            }
            DidSaveTextDocument::METHOD | DidChangeWatchedFiles::METHOD => {
                let saved_doc_uri;
                let text;

                match not.method.as_str() {
                    DidSaveTextDocument::METHOD => {
                        let params =
                            serde_json::from_value::<lsp_types::DidSaveTextDocumentParams>(
                                not.params,
                            )
                            .unwrap();
                        saved_doc_uri = params.text_document.uri;
                        text = params.text.map(|s| Rope::from_str(&s));
                    }
                    DidChangeWatchedFiles::METHOD => {
                        let mut params = serde_json::from_value::<
                            lsp_types::DidChangeWatchedFilesParams,
                        >(not.params)
                        .unwrap();
                        saved_doc_uri = params.changes.remove(0).uri;
                        text = None;
                    }
                    _ => unreachable!(),
                }

                self.shared_data.egui_ctx.forget_all_images();
                let doc = self.grz_files.get_mut(&saved_doc_uri).unwrap();
                doc.clear_incremental_state();

                if let Some(text) = text {
                    doc.source = text;
                }

                let version = doc.version;
                self.last_edited_uri = saved_doc_uri.clone();
                if let Ok(parse_result) = doc.parse(&[]) {
                    Self::report_error_messages(
                        connection,
                        Arc::clone(&parse_result),
                        saved_doc_uri,
                        version,
                    );

                    if !parse_result.has_errors() {
                        self.handle_file_owner_message(FileOwnerMessage::Index {
                            index: self.slide_index,
                            reset_time: true,
                        });
                    }
                }
            }
            DidChangeTextDocument::METHOD => {
                let changes: lsp_types::DidChangeTextDocumentParams =
                    serde_json::from_value(not.params).unwrap();

                let doc = self.grz_files.get_mut(&changes.text_document.uri).unwrap();

                let mut has_errors = false;
                if doc.version < changes.text_document.version {
                    doc.version = changes.text_document.version;

                    for change in changes.content_changes {
                        let transaction = helix_lsp::util::generate_transaction_from_edits(
                            &doc.source,
                            vec![lsp_types::TextEdit {
                                range: change.range.unwrap(),
                                new_text: change.text.clone(),
                            }],
                            helix_lsp::OffsetEncoding::Utf16,
                        );

                        let edits = generate_edits(doc.source.slice(..), transaction.changes());

                        let edit_range = edits.first();
                        if doc
                            .tree
                            .as_ref()
                            .and_then(|t| {
                                t.root_node().first_child_for_byte(edit_range?.old_end_byte)
                            })
                            .and_then(|n| {
                                Some(
                                    n.kind_id() == NodeKind::SymWhitespace as u16
                                        && n.start_position().row != edit_range?.start_point.row,
                                )
                            })
                            .unwrap_or(true)
                        {
                            expand_change(
                                &change,
                                "{}",
                                "{\n    ..,\n}[]",
                                &changes.text_document,
                                connection,
                            );
                            expand_change(
                                &change,
                                "()",
                                "Object: Paragraph(\n)",
                                &changes.text_document,
                                connection,
                            );
                            expand_change(
                                &change,
                                "^",
                                "ViewBox: Size[0] ^\n]",
                                &changes.text_document,
                                connection,
                            );
                            expand_change(
                                &change,
                                ">",
                                "ViewBox: Size[0] >\n]",
                                &changes.text_document,
                                connection,
                            );
                            // expand_change(
                            //     &change,
                            //     "<",
                            //     "<REGISTER: value>",
                            //     &changes.text_document,
                            //     connection,
                            // );
                        }

                        if transaction.apply(&mut doc.source) {
                            for edit in edits.iter().rev() {
                                if let Some(ref mut tree) = doc.tree {
                                    tree.edit(edit);
                                }
                                if let Some(ref mut tree) = doc.error_free_tree {
                                    tree.edit(edit);
                                }
                            }
                        } else {
                            panic!("Transaction could not be applied");
                        }

                        if let Ok(parse_result) = doc.parse(&edits) {
                            Self::report_error_messages(
                                connection,
                                Arc::clone(&parse_result),
                                changes.text_document.uri.clone(),
                                changes.text_document.version,
                            );

                            self.last_edited_uri = changes.text_document.uri.clone();
                            if let Some(edited_index) = edits.first().and_then(|edit| {
                                doc.find_slide_index_for_edit(edit, self.slide_index)
                            }) {
                                self.slide_index = edited_index;
                            }
                            if parse_result.has_errors() {
                                has_errors = true;
                            }
                        }
                    }
                }
                if !has_errors {
                    self.handle_file_owner_message(FileOwnerMessage::Index {
                        index: self.slide_index,
                        reset_time: false,
                    });
                }
            }
            _ => {}
        }

        false
    }
}

fn expand_change(
    change: &TextDocumentContentChangeEvent,
    text: &str,
    expanded: &str,
    text_document: &VersionedTextDocumentIdentifier,
    connection: &Connection,
) {
    if change.text.trim() == text {
        let range = change.range.unwrap();
        let start_character =
            range.start.character + (change.text.trim_end().len() - text.len()) as u32;
        connection
            .sender
            .send(Message::Request(lsp_server::Request::new(
                0.into(),
                ApplyWorkspaceEdit::METHOD.to_string(),
                ApplyWorkspaceEditParams {
                    label: None,
                    edit: WorkspaceEdit {
                        document_changes: Some(DocumentChanges::Edits(vec![TextDocumentEdit {
                            edits: vec![OneOf::Left(TextEdit {
                                range: lsp_types::Range {
                                    start: Position {
                                        line: range.start.line,
                                        character: start_character,
                                    },
                                    end: Position {
                                        line: range.end.line,
                                        character: start_character + text.len() as u32,
                                    },
                                },
                                new_text: String::from(expanded),
                            })],
                            text_document: OptionalVersionedTextDocumentIdentifier {
                                uri: text_document.uri.clone(),
                                version: Some(text_document.version),
                            },
                        }])),
                        ..Default::default()
                    },
                },
            )))
            .unwrap();
    }
}
