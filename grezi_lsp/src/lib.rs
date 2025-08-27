use std::{collections::HashMap, ops::DerefMut, str::FromStr, sync::Arc};

use crossbeam_channel::Receiver;
use egui::Modifiers;
use grezi_egui::GrzResolvedSlide;
use grezi_file_owner::{AppHandle, FileOwnerMessage};
use grezi_parser::parse::{error::ErrsWithSource, GrzFile};
use helix_core::syntax::generate_edits;
use helix_lsp::{Position, Url};
use helix_lsp_types::{
    self as lsp_types,
    request::{
        DocumentSymbolRequest, FoldingRangeRequest, GotoDeclaration, References,
        SemanticTokensFullRequest, WorkspaceSymbolRequest,
    },
    DeclarationCapability, DocumentSymbolParams, DocumentSymbolResponse, FoldingRangeParams,
    GotoDefinitionParams, Location, ReferenceParams, RenameParams, SemanticTokenModifier,
    SemanticTokenType, SemanticTokensFullOptions, SemanticTokensLegend, SemanticTokensOptions,
    SemanticTokensParams, SemanticTokensServerCapabilities, TextDocumentPositionParams,
    WorkspaceSymbolParams, WorkspaceSymbolResponse,
};
use lsp_server::{Connection, Message, Response};
use lsp_types::{
    notification::DidSaveTextDocument,
    notification::{
        DidChangeTextDocument, DidChangeWatchedFiles, DidCloseTextDocument, DidOpenTextDocument,
        Notification, PublishDiagnostics,
    },
    request::{ApplyWorkspaceEdit, Formatting, PrepareRenameRequest, Rename},
    request::{RegisterCapability, Request},
    ApplyWorkspaceEditParams, DidChangeWatchedFilesRegistrationOptions, DocumentChanges,
    DocumentFormattingParams, FileSystemWatcher, GlobPattern, OneOf,
    OptionalVersionedTextDocumentIdentifier, PositionEncodingKind, PublishDiagnosticsParams,
    Registration, RegistrationParams, RenameOptions, ServerCapabilities,
    TextDocumentContentChangeEvent, TextDocumentEdit, TextDocumentSyncCapability,
    TextDocumentSyncKind, TextDocumentSyncOptions, TextEdit, VersionedTextDocumentIdentifier,
    WorkDoneProgressOptions, WorkspaceEdit,
};
use miette::Diagnostic;
use nucleo_matcher::{
    pattern::{AtomKind, CaseMatching, Normalization, Pattern},
    Matcher,
};
use ropey::Rope;
use tree_sitter::{Query, QueryCursor};
use tree_sitter_grz::NodeKind;

mod folding_range;
mod formatter;
mod rename;
mod semantic_tokens;
mod symbols;

pub struct GrzLsp {
    slide_index: usize,
    last_edited_uri: Url,
    grz_files: HashMap<Url, GrzFile>,
    shared_data: AppHandle,
    owner_receiver: Receiver<FileOwnerMessage>,
    query_cursor: QueryCursor,
    // Queries
    rename_query: Query,
    semantic_tokens_query: Query,
    folding_range_query: Query,
    top_level_search_query: Query,
}

impl GrzLsp {
    pub fn new(shared_data: AppHandle, owner_receiver: Receiver<FileOwnerMessage>) -> Self {
        let tree_sitter_grz_lang = tree_sitter_grz::LANGUAGE.into();
        let rename_query =
            Query::new(&tree_sitter_grz_lang, include_str!("queries/rename.scm")).unwrap();
        let semantic_tokens_query = Query::new(
            &tree_sitter_grz_lang,
            include_str!("queries/semantic_tokens.scm"),
        )
        .unwrap();
        let folding_range_query = Query::new(
            &tree_sitter_grz_lang,
            include_str!("queries/folding_range.scm"),
        )
        .unwrap();
        let top_level_search_query = Query::new(
            &tree_sitter_grz_lang,
            include_str!("queries/top_level_search.scm"),
        )
        .unwrap();

        GrzLsp {
            slide_index: 0,
            last_edited_uri: Url::from_str("file:///dev/null").unwrap(),
            grz_files: HashMap::new(),
            shared_data,
            owner_receiver,
            query_cursor: QueryCursor::new(),
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
                            .capture_names()
                            .iter()
                            .map(|name| {
                                SemanticTokenType::new(unsafe {
                                    std::mem::transmute::<&str, &'static str>(
                                        name.split_once('.').map(|split| split.0).unwrap_or(name),
                                    )
                                })
                            })
                            .collect(),
                        token_modifiers: self
                            .semantic_tokens_query
                            .capture_names()
                            .iter()
                            .map(|name| unsafe { std::mem::transmute::<&str, &'static str>(name) })
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
            Message::Request(req) => {
                if connection.handle_shutdown(&req).unwrap() {
                    return true;
                }

                match req.method.as_str() {
                    FoldingRangeRequest::METHOD => {
                        if let Ok((rqid, params)) =
                            req.extract::<FoldingRangeParams>(FoldingRangeRequest::METHOD)
                        {
                            let doc = self.grz_files.get(&params.text_document.uri).unwrap();
                            connection
                                .sender
                                .send(Message::Response(Response::new_ok(
                                    rqid,
                                    folding_range::folding_ranges(
                                        &doc.source,
                                        &doc.tree,
                                        &mut self.query_cursor,
                                        &self.folding_range_query,
                                    ),
                                )))
                                .unwrap();
                        }
                    }
                    Formatting::METHOD => {
                        if let Ok((rqid, params)) =
                            req.extract::<DocumentFormattingParams>(Formatting::METHOD)
                        {
                            let doc = self.grz_files.get(&params.text_document.uri).unwrap();
                            let edits: Option<Vec<TextEdit>> = doc
                                .tree
                                .as_ref()
                                .and_then(|tree| formatter::format_code(&doc.source, tree).ok());

                            connection
                                .sender
                                .send(Message::Response(Response::new_ok(rqid, edits)))
                                .unwrap();
                        }
                    }
                    PrepareRenameRequest::METHOD => {
                        if let Ok((rqid, pos)) =
                            req.extract::<TextDocumentPositionParams>(PrepareRenameRequest::METHOD)
                        {
                            let doc = self.grz_files.get(&pos.text_document.uri).unwrap();
                            connection
                                .sender
                                .send(Message::Response(Response::new_ok(
                                    rqid,
                                    rename::prepare_rename(pos, &doc.source, &doc.tree),
                                )))
                                .unwrap();
                        }
                    }
                    DocumentSymbolRequest::METHOD => {
                        if let Ok((rqid, params)) =
                            req.extract::<DocumentSymbolParams>(DocumentSymbolRequest::METHOD)
                        {
                            let doc = self.grz_files.get(&params.text_document.uri).unwrap();
                            let mut symbols = Vec::new();
                            let symbols = symbols::document_symbols(
                                &doc.source,
                                &doc.tree,
                                &mut symbols,
                                |s| s,
                            )
                            .map(|_| DocumentSymbolResponse::Nested(symbols));
                            connection
                                .sender
                                .send(Message::Response(Response::new_ok(rqid, symbols)))
                                .unwrap();
                        }
                    }
                    WorkspaceSymbolRequest::METHOD => {
                        if let Ok((rqid, params)) =
                            req.extract::<WorkspaceSymbolParams>(WorkspaceSymbolRequest::METHOD)
                        {
                            let mut symbols = Vec::new();

                            pub struct MatchableWorkspaceSymbol(lsp_types::WorkspaceSymbol);

                            impl AsRef<str> for MatchableWorkspaceSymbol {
                                fn as_ref(&self) -> &str {
                                    self.0.name.as_str()
                                }
                            }

                            for (uri, doc) in self.grz_files.iter() {
                                symbols::document_symbols(
                                    &doc.source,
                                    &doc.tree,
                                    &mut symbols,
                                    |s| {
                                        MatchableWorkspaceSymbol(lsp_types::WorkspaceSymbol {
                                            name: s.name,
                                            kind: s.kind,
                                            tags: s.tags,
                                            container_name: s.detail,
                                            location: OneOf::Left(Location {
                                                uri: uri.clone(),
                                                range: s.range,
                                            }),
                                            data: None,
                                        })
                                    },
                                );
                            }
                            let mut matcher = Matcher::new(nucleo_matcher::Config::DEFAULT);
                            let matches = Pattern::new(
                                &params.query,
                                CaseMatching::Ignore,
                                Normalization::Smart,
                                AtomKind::Fuzzy,
                            )
                            .match_list(symbols, &mut matcher);
                            let symbols = matches.into_iter().map(|s| s.0 .0).collect::<Vec<_>>();
                            connection
                                .sender
                                .send(Message::Response(Response::new_ok(
                                    rqid,
                                    if symbols.is_empty() {
                                        None
                                    } else {
                                        Some(WorkspaceSymbolResponse::Nested(symbols))
                                    },
                                )))
                                .unwrap();
                        }
                    }
                    GotoDeclaration::METHOD => {
                        if let Ok((rqid, goto_params)) =
                            req.extract::<GotoDefinitionParams>(GotoDeclaration::METHOD)
                        {
                            let doc = self
                                .grz_files
                                .get(&goto_params.text_document_position_params.text_document.uri)
                                .unwrap();
                            connection
                                .sender
                                .send(Message::Response(Response::new_ok(
                                    rqid,
                                    symbols::goto_declaration(
                                        &self.top_level_search_query,
                                        &doc.source,
                                        goto_params
                                            .text_document_position_params
                                            .text_document
                                            .uri
                                            .clone(),
                                        goto_params,
                                        &mut self.query_cursor,
                                        &doc.tree,
                                    ),
                                )))
                                .unwrap();
                        }
                    }
                    References::METHOD => {
                        if let Ok((rqid, reference_params)) =
                            req.extract::<ReferenceParams>(References::METHOD)
                        {
                            let doc = self
                                .grz_files
                                .get(&reference_params.text_document_position.text_document.uri)
                                .unwrap();
                            connection
                                .sender
                                .send(Message::Response(Response::new_ok(
                                    rqid,
                                    symbols::references(
                                        &self.rename_query,
                                        &doc.source,
                                        reference_params,
                                        &mut self.query_cursor,
                                        &doc.tree,
                                    ),
                                )))
                                .unwrap();
                        }
                    }
                    SemanticTokensFullRequest::METHOD => {
                        if let Ok((rqid, params)) =
                            req.extract::<SemanticTokensParams>(SemanticTokensFullRequest::METHOD)
                        {
                            let doc = self.grz_files.get(&params.text_document.uri).unwrap();
                            connection
                                .sender
                                .send(Message::Response(Response::new_ok(
                                    rqid,
                                    semantic_tokens::semantic_tokens(
                                        &self.semantic_tokens_query,
                                        &doc.source,
                                        &mut self.query_cursor,
                                        &doc.tree,
                                    ),
                                )))
                                .unwrap();
                        }
                    }
                    Rename::METHOD => {
                        if let Ok((rqid, rename_params)) =
                            req.extract::<RenameParams>(Rename::METHOD)
                        {
                            let doc = self
                                .grz_files
                                .get(&rename_params.text_document_position.text_document.uri)
                                .unwrap();
                            connection
                                .sender
                                .send(Message::Response(Response::new_ok(
                                    rqid,
                                    Some(WorkspaceEdit {
                                        document_changes: Some(DocumentChanges::Edits(vec![
                                            TextDocumentEdit {
                                                text_document:
                                                    OptionalVersionedTextDocumentIdentifier {
                                                        uri: rename_params
                                                            .text_document_position
                                                            .text_document
                                                            .uri
                                                            .clone(),
                                                        version: Some(doc.version),
                                                    },
                                                edits: rename::rename(
                                                    rename_params,
                                                    &doc.source,
                                                    &self.rename_query,
                                                    &mut self.query_cursor,
                                                    &doc.tree,
                                                ),
                                            },
                                        ])),
                                        ..Default::default()
                                    }),
                                )))
                                .unwrap();
                        }
                    }
                    _ => {}
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
                    new_file.version = doc.text_document.version;

                    let parse_result = new_file.parse(&[]);

                    if let Some(parse_result) = parse_result.as_ref().ok().map(|pr| Arc::clone(pr))
                    {
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
                            let params = serde_json::from_value::<
                                lsp_types::DidSaveTextDocumentParams,
                            >(not.params)
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
                                            && n.start_position().row
                                                != edit_range?.start_position.row,
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
            },
        }

        false
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
