use std::{collections::HashMap, ops::DerefMut, str::FromStr, sync::Arc};

use crossbeam_channel::Receiver;
use egui::Modifiers;
use grezi_egui::GrzResolvedSlide;
use grezi_file_owner::{AppHandle, FileOwnerMessage};
use grezi_parser::parse::{GrzFile, error::ErrsWithSource};
use helix_core::tree_sitter::{Grammar, Query, query::InvalidPredicateError};
use helix_lsp::Url;
use helix_lsp_types::{
    self as lsp_types, DeclarationCapability, SemanticTokenModifier, SemanticTokenType,
    SemanticTokensFullOptions, SemanticTokensLegend, SemanticTokensOptions,
    SemanticTokensServerCapabilities,
};
use lsp_server::{Connection, Message};
use lsp_types::{
    DidChangeWatchedFilesRegistrationOptions, FileSystemWatcher, GlobPattern, OneOf,
    PositionEncodingKind, PublishDiagnosticsParams, Registration, RegistrationParams,
    RenameOptions, ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind,
    TextDocumentSyncOptions, WorkDoneProgressOptions,
    notification::{DidChangeWatchedFiles, Notification, PublishDiagnostics},
    request::{RegisterCapability, Request},
};
use miette::Diagnostic;

mod notification;
mod request;

pub struct GrzLsp {
    slide_index: usize,
    last_edited_uri: Url,
    grz_files: HashMap<Url, GrzFile>,
    shared_data: AppHandle,
    owner_receiver: Receiver<FileOwnerMessage>,
    // Queries
    rename_query: Query,
    semantic_tokens_query: Query,
    folding_range_query: Query,
    top_level_search_query: Query,
}

impl GrzLsp {
    pub fn new(shared_data: AppHandle, owner_receiver: Receiver<FileOwnerMessage>) -> Self {
        let tree_sitter_grz_lang: Grammar = tree_sitter_grz::LANGUAGE.try_into().unwrap();
        let rename_query = Query::new(
            tree_sitter_grz_lang,
            include_str!("queries/rename.scm"),
            |_, predicate| Err(InvalidPredicateError::unknown(predicate)),
        )
        .unwrap();
        let semantic_tokens_query = Query::new(
            tree_sitter_grz_lang,
            include_str!("queries/semantic_tokens.scm"),
            |_, predicate| Err(InvalidPredicateError::unknown(predicate)),
        )
        .unwrap();
        let folding_range_query = Query::new(
            tree_sitter_grz_lang,
            include_str!("queries/folding_range.scm"),
            |_, predicate| Err(InvalidPredicateError::unknown(predicate)),
        )
        .unwrap();
        let top_level_search_query = Query::new(
            tree_sitter_grz_lang,
            include_str!("queries/top_level_search.scm"),
            |_, predicate| Err(InvalidPredicateError::unknown(predicate)),
        )
        .unwrap();

        GrzLsp {
            slide_index: 0,
            last_edited_uri: Url::from_str("file:///dev/null").unwrap(),
            grz_files: HashMap::new(),
            shared_data,
            owner_receiver,
            rename_query,
            semantic_tokens_query,
            folding_range_query,
            top_level_search_query,
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
                            include_text: Some(true),
                        },
                    )),
                    ..Default::default()
                },
            )),
            rename_provider: Some(OneOf::Right(RenameOptions {
                prepare_provider: Some(true),
                work_done_progress_options: WorkDoneProgressOptions {
                    work_done_progress: Some(false),
                },
            })),
            semantic_tokens_provider: Some(
                SemanticTokensServerCapabilities::SemanticTokensOptions(SemanticTokensOptions {
                    legend: SemanticTokensLegend {
                        token_types: self
                            .semantic_tokens_query
                            .captures()
                            .map(|(_, name)| {
                                SemanticTokenType::new(unsafe {
                                    std::mem::transmute::<&str, &'static str>(
                                        name.split_once('.').map(|split| split.0).unwrap_or(name),
                                    )
                                })
                            })
                            .collect(),
                        token_modifiers: self
                            .semantic_tokens_query
                            .captures()
                            .map(|(_, name)| unsafe {
                                std::mem::transmute::<&str, &'static str>(name)
                            })
                            .filter_map(|name| {
                                // Safe because string exists for lifetime of LSP
                                name.split_once('.')
                                    .map(|name| SemanticTokenModifier::new(name.1))
                            })
                            .collect(),
                    },
                    range: Some(false),
                    full: Some(SemanticTokensFullOptions::Delta { delta: Some(false) }),
                    ..Default::default()
                }),
            ),
            declaration_provider: Some(DeclarationCapability::Simple(true)),
            references_provider: Some(OneOf::Left(true)),
            document_symbol_provider: Some(OneOf::Left(true)),
            workspace_symbol_provider: Some(OneOf::Left(true)),
            document_formatting_provider: Some(OneOf::Left(true)),
            folding_range_provider: Some(lsp_types::FoldingRangeProviderCapability::Simple(true)),
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
            Message::Request(req) => self.handle_request_message(connection, req),
            Message::Response(_) => false,
            Message::Notification(not) => self.handle_notification_message(connection, not),
        }
    }

    fn report_error_messages(
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
            FileOwnerMessage::Next(trigger_was_action) => {
                self.slide_index += 1;
                reset_time = true;
                if trigger_was_action
                    && self
                        .grz_files
                        .get(&self.last_edited_uri)
                        .map(|s| s.slideshow.slides.len())
                        .unwrap_or_default()
                        <= self.slide_index
                {
                    self.slide_index = 0;
                }
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
