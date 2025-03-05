use std::{collections::HashMap, ops::DerefMut, str::FromStr, sync::Arc};

use crossbeam_channel::Receiver;
use egui::Modifiers;
use grezi_egui::GrzResolvedSlide;
use grezi_file_owner::{AppHandle, FileOwnerMessage};
use grezi_parser::parse::{error::ErrsWithSource, GrzFile};
use helix_core::syntax::generate_edits;
use helix_lsp::Url;
use helix_lsp_types::{self as lsp_types, notification::DidSaveTextDocument};
use lsp_server::{Connection, Message};
use lsp_types::{
    notification::{
        DidChangeTextDocument, DidChangeWatchedFiles, DidCloseTextDocument, DidOpenTextDocument,
        Notification, PublishDiagnostics,
    },
    request::{RegisterCapability, Request},
    DidChangeWatchedFilesRegistrationOptions, FileSystemWatcher, GlobPattern, PositionEncodingKind,
    PublishDiagnosticsParams, Registration, RegistrationParams, ServerCapabilities,
    TextDocumentSyncCapability, TextDocumentSyncKind, TextDocumentSyncOptions,
};
use miette::Diagnostic;

pub struct GrzLsp {
    slide_index: usize,
    last_edited_uri: Url,
    grz_files: HashMap<Url, GrzFile>,
    shared_data: AppHandle,
    owner_receiver: Receiver<FileOwnerMessage>,
}

impl GrzLsp {
    pub fn new(shared_data: AppHandle, owner_receiver: Receiver<FileOwnerMessage>) -> Self {
        GrzLsp {
            slide_index: 0,
            last_edited_uri: Url::from_str("file:///dev/null").unwrap(),
            grz_files: HashMap::new(),
            shared_data,
            owner_receiver,
        }
    }
}

impl GrzLsp {
    pub fn run(self) {
        let (connection, io_threads) = Connection::stdio();

        let server_capabilities = serde_json::to_value(&ServerCapabilities {
            text_document_sync: Some(TextDocumentSyncCapability::Options(
                TextDocumentSyncOptions {
                    open_close: Some(true),
                    change: Some(TextDocumentSyncKind::INCREMENTAL),
                    save: Some(lsp_types::TextDocumentSyncSaveOptions::SaveOptions(
                        lsp_types::SaveOptions {
                            include_text: Some(false),
                        },
                    )),
                    ..Default::default()
                },
            )),
            position_encoding: Some(PositionEncodingKind::UTF16),
            ..Default::default()
        })
        .unwrap();

        let initialization_params = connection.initialize(server_capabilities).unwrap();

        connection
            .sender
            .send(Message::Request(lsp_server::Request::new(
                69420.into(),
                RegisterCapability::METHOD.to_string(),
                RegistrationParams {
                    registrations: vec![Registration {
                        id: "GRZ File watching".to_string(),
                        method: DidChangeWatchedFiles::METHOD.to_string(),
                        register_options: Some(
                            serde_json::to_value(DidChangeWatchedFilesRegistrationOptions {
                                watchers: vec![FileSystemWatcher {
                                    glob_pattern: GlobPattern::String("**/*.grz".to_string()),
                                    kind: None,
                                }],
                            })
                            .unwrap(),
                        ),
                    }],
                },
            )))
            .unwrap();

        self.main_loop(connection, initialization_params);
        io_threads.join().unwrap();
    }

    fn main_loop(mut self, connection: Connection, _params: serde_json::Value) {
        loop {
            crossbeam_channel::select! {
                recv(connection.receiver) -> message => {
                    if self.handle_lsp_message(&connection, message.unwrap()) {
                        self.shared_data.egui_ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        self.shared_data.egui_ctx.request_repaint();
                        break;
                    }
                },
                recv(self.owner_receiver) -> message => self.handle_file_owner_message(message.unwrap()),
            }
        }
    }

    fn handle_lsp_message(
        &mut self,
        connection: &Connection,
        message: lsp_server::Message,
    ) -> bool {
        match message {
            Message::Request(req) => {
                if connection.handle_shutdown(&req).unwrap() {
                    return true;
                }
            }
            Message::Response(_) => {}
            Message::Notification(not) => match not.method.as_str() {
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

                    if let Ok(parse_result) = new_file.parse(Vec::new()) {
                        self.report_error_messages(
                            connection,
                            Arc::clone(&parse_result),
                            doc.text_document.uri.clone(),
                            new_file.version,
                        );
                        self.last_edited_uri = doc.text_document.uri.clone();
                        self.grz_files.insert(doc.text_document.uri, new_file);
                        if !parse_result.has_errors() {
                            self.handle_file_owner_message(FileOwnerMessage::Index {
                                index: 0,
                                reset_time: true,
                            });
                        }
                    }
                }
                DidCloseTextDocument::METHOD => {
                    let doc: lsp_types::DidCloseTextDocumentParams =
                        serde_json::from_value(not.params).unwrap();

                    self.grz_files.remove(&doc.text_document.uri);
                }
                DidSaveTextDocument::METHOD | DidChangeWatchedFiles::METHOD => {
                    let saved_doc_uri = match not.method.as_str() {
                        DidSaveTextDocument::METHOD => {
                            serde_json::from_value::<lsp_types::DidSaveTextDocumentParams>(
                                not.params,
                            )
                            .unwrap()
                            .text_document
                            .uri
                        }
                        DidChangeWatchedFiles::METHOD => {
                            serde_json::from_value::<lsp_types::DidChangeWatchedFilesParams>(
                                not.params,
                            )
                            .unwrap()
                            .changes
                            .remove(0)
                            .uri
                        }
                        _ => unreachable!(),
                    };

                    self.shared_data.egui_ctx.forget_all_images();
                    let doc = self.grz_files.get_mut(&saved_doc_uri).unwrap();
                    doc.clear_incremental_state();

                    let version = doc.version;
                    if let Ok(parse_result) = doc.parse(Vec::new()) {
                        self.report_error_messages(
                            connection,
                            Arc::clone(&parse_result),
                            saved_doc_uri.clone(),
                            version,
                        );

                        self.last_edited_uri = saved_doc_uri;

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

                    if doc.version < changes.text_document.version {
                        doc.version = changes.text_document.version;

                        let transaction = helix_lsp::util::generate_transaction_from_edits(
                            &doc.source,
                            changes
                                .content_changes
                                .iter()
                                .map(|change| lsp_types::TextEdit {
                                    range: change.range.unwrap(),
                                    new_text: change.text.clone(),
                                })
                                .collect(),
                            helix_lsp::OffsetEncoding::Utf16,
                        );

                        let edits = generate_edits(doc.source.slice(..), transaction.changes());
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

                        if let Ok(parse_result) = doc.parse(edits) {
                            self.report_error_messages(
                                connection,
                                Arc::clone(&parse_result),
                                changes.text_document.uri.clone(),
                                changes.text_document.version,
                            );

                            self.last_edited_uri = changes.text_document.uri.clone();

                            if !parse_result.has_errors() {
                                self.handle_file_owner_message(FileOwnerMessage::Index {
                                    index: self.slide_index,
                                    reset_time: false,
                                });
                            }
                        }
                    }
                }
                _ => {}
            },
        }

        false
    }

    fn report_error_messages(
        &self,
        connection: &Connection,
        errors: Arc<ErrsWithSource>,
        uri: Url,
        version: i32,
    ) {
        let mut diagnostics = Vec::with_capacity(errors.errors.count());
        for (error, label) in errors
            .errors
            .iter()
            .filter_map(|(_, (error, _))| Some((error, error.labels()?.next()?)))
        {
            if let Some(range) = error.char_range() {
                let range = lsp_types::Range {
                    start: lsp_types::Position {
                        line: range.start_line as u32,
                        character: range.start_character as u32,
                    },
                    end: lsp_types::Position {
                        line: range.end_line as u32,
                        character: range.end_character as u32,
                    },
                };

                diagnostics.push(lsp_types::Diagnostic {
                    range,
                    severity: Some(
                        error
                            .severity()
                            .map(|s| match s {
                                miette::Severity::Advice => lsp_types::DiagnosticSeverity::HINT,
                                miette::Severity::Warning => lsp_types::DiagnosticSeverity::WARNING,
                                miette::Severity::Error => lsp_types::DiagnosticSeverity::ERROR,
                            })
                            .unwrap_or(lsp_types::DiagnosticSeverity::ERROR),
                    ),
                    source: Some("Grezi LSP".to_owned()),
                    message: format!("{}", label.label().unwrap_or("Error here (unknown)")),
                    ..Default::default()
                });
            }
        }
        connection
            .sender
            .send(Message::Notification(lsp_server::Notification::new(
                PublishDiagnostics::METHOD.to_string(),
                PublishDiagnosticsParams {
                    uri,
                    diagnostics,
                    version: Some(version),
                },
            )))
            .unwrap();
    }

    fn handle_file_owner_message(&mut self, message: FileOwnerMessage) {
        let reset_time;
        match message {
            FileOwnerMessage::Index {
                index,
                reset_time: rt,
            } => {
                self.slide_index = index;
                reset_time = rt;
            }
            FileOwnerMessage::Next => {
                self.slide_index += 1;
                reset_time = true;
            }
            FileOwnerMessage::Previous => {
                self.slide_index = self.slide_index.saturating_sub(1);
                reset_time = false;
            }
            FileOwnerMessage::ResetFile => {
                if let Some(grz_file) = self.grz_files.get_mut(&self.last_edited_uri) {
                    let parse_result = grz_file.update_file().unwrap();
                    if parse_result.has_errors() {
                        return;
                    }
                    reset_time = true;
                } else {
                    return;
                }
            }
        }

        let mut new_slide = None;
        if let Some(grz_file) = self.grz_files.get(&self.last_edited_uri) {
            loop {
                new_slide = GrzResolvedSlide::resolve_slide(
                    &grz_file.slideshow,
                    self.shared_data.font_system.lock().deref_mut(),
                    &self.shared_data.egui_ctx,
                    self.slide_index,
                );

                if new_slide.is_none() {
                    self.slide_index = self.slide_index.saturating_sub(1);
                    if self.slide_index == 0 {
                        break;
                    }
                } else {
                    break;
                }
            }
        }
        self.shared_data.resolved.store(new_slide.map(Arc::new));
        if reset_time {
            self.shared_data
                .custom_key_sender
                .send(egui::Event::Key {
                    key: egui::Key::R,
                    physical_key: None,
                    pressed: true,
                    repeat: false,
                    modifiers: Modifiers::NONE,
                })
                .unwrap();
        }
        self.shared_data.egui_ctx.request_repaint();
    }
}
