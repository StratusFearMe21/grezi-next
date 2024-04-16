use std::{
    borrow::Cow,
    collections::HashMap,
    hash::Hash,
    hash::{BuildHasher, BuildHasherDefault, Hasher},
    path::Path,
    process::Stdio,
    sync::atomic::Ordering,
};

pub mod completion;
pub mod formatter;
pub mod inlay_hints;
pub mod rename;
pub mod semantic_tokens;
pub mod symbols;
pub mod you_can;

use crate::{
    parser::{viewboxes::ViewboxIn, AstObject, NodeKind, PointFromRange},
    MyEguiApp,
};
use helix_core::ropey::{Rope, RopeSlice};
use helix_core::syntax::RopeProvider;
use helix_core::tree_sitter::{Point, Query, QueryCursor, Tree};
use hunspell_rs::CheckResult;
use indexmap::IndexSet;
use lsp_server::{Connection, Message, Response};
use lsp_types::{
    notification::{
        DidChangeTextDocument, DidOpenTextDocument, DidSaveTextDocument, Notification,
        PublishDiagnostics, ShowMessage,
    },
    request::{
        ApplyWorkspaceEdit, Completion, DocumentSymbolRequest, ExecuteCommand, Formatting,
        GotoDeclaration, InlayHintRequest, PrepareRenameRequest, RangeFormatting, References,
        Rename, Request, SemanticTokensFullRequest,
    },
    ApplyWorkspaceEditParams, CompletionItem, CompletionItemKind, CompletionOptions,
    CompletionOptionsCompletionItem, CompletionParams, CompletionResponse, CompletionTextEdit,
    DeclarationCapability, DocumentChanges, DocumentFormattingParams,
    DocumentRangeFormattingParams, DocumentSymbolParams, ExecuteCommandOptions,
    ExecuteCommandParams, GotoDefinitionParams, Hover, InlayHintOptions, InlayHintParams,
    InlayHintServerCapabilities, InsertReplaceEdit, InsertTextFormat, MessageType, OneOf,
    OptionalVersionedTextDocumentIdentifier, Position, PositionEncodingKind,
    PublishDiagnosticsParams, ReferenceParams, RenameOptions, RenameParams, SaveOptions,
    SemanticTokenModifier, SemanticTokenType, SemanticTokensFullOptions, SemanticTokensLegend,
    SemanticTokensOptions, SemanticTokensParams, SemanticTokensServerCapabilities,
    ServerCapabilities, ShowMessageParams, TextDocumentEdit, TextDocumentPositionParams,
    TextDocumentSyncCapability, TextDocumentSyncKind, TextDocumentSyncOptions,
    TextDocumentSyncSaveOptions, TextEdit, Url, WorkDoneProgressOptions, WorkspaceEdit,
};

use self::formatter::char_range_from_byte_range;

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

    let mut query_cursor = QueryCursor::new();
    let tree_sitter_grz_lang = tree_sitter_grz::language();
    let rename_query =
        Query::new(&tree_sitter_grz_lang, include_str!("queries/rename.scm")).unwrap();
    let slide_complete_query = Query::new(
        &tree_sitter_grz_lang,
        include_str!("queries/slide_complete.scm"),
    )
    .unwrap();
    let slide_index_query = Query::new(
        &tree_sitter_grz_lang,
        include_str!("queries/slide_index.scm"),
    )
    .unwrap();
    let top_level_search_query = Query::new(
        &tree_sitter_grz_lang,
        include_str!("queries/top_level_search.scm"),
    )
    .unwrap();
    let inlay_edge_query = Query::new(
        &tree_sitter_grz_lang,
        include_str!("queries/inlay_edge.scm"),
    )
    .unwrap();
    let viewbox_name_query = Query::new(
        &tree_sitter_grz_lang,
        include_str!("queries/viewbox_name.scm"),
    )
    .unwrap();
    let object_name_query = Query::new(
        &tree_sitter_grz_lang,
        include_str!("queries/object_name.scm"),
    )
    .unwrap();
    let semantic_token_query = Query::new(
        &tree_sitter_grz_lang,
        include_str!("queries/semantic_tokens.scm"),
    )
    .unwrap();
    let vb_in_slide_query = Query::new(
        &tree_sitter_grz_lang,
        include_str!("queries/vb_in_slide.scm"),
    )
    .unwrap();
    let obj_in_slide_query = Query::new(
        &tree_sitter_grz_lang,
        include_str!("queries/obj_in_slide.scm"),
    )
    .unwrap();
    let strings_query =
        Query::new(&tree_sitter_grz_lang, include_str!("queries/strings.scm")).unwrap();

    let mut hunspell = None;
    let mut error_free_tree: Option<Tree> = None;
    let fonts: IndexSet<String, ahash::RandomState> = app
        .font_system
        .lock()
        .db()
        .faces()
        .map(|f| &f.families)
        .flatten()
        .map(|(f, _)| f.clone())
        .collect();

    // Run the server and wait for the two threads to end (typically by trigger LSP Exit event).
    let server_capabilities = serde_json::to_value(ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Options(
            TextDocumentSyncOptions {
                open_close: Some(true),
                change: Some(TextDocumentSyncKind::INCREMENTAL),
                save: Some(TextDocumentSyncSaveOptions::SaveOptions(SaveOptions {
                    include_text: Some(false),
                })),
                ..Default::default()
            },
        )),
        execute_command_provider: Some(ExecuteCommandOptions {
            commands: vec![
                "tree_to_dot".to_string(),
                "full_reparse".to_string(),
                "spell_check".to_string(),
            ],
            ..Default::default()
        }),
        rename_provider: Some(OneOf::Right(RenameOptions {
            prepare_provider: Some(true),
            work_done_progress_options: WorkDoneProgressOptions {
                work_done_progress: Some(false),
            },
        })),
        completion_provider: Some(CompletionOptions {
            resolve_provider: Some(false),
            trigger_characters: Some(vec![".".to_string()]),
            completion_item: Some(CompletionOptionsCompletionItem {
                label_details_support: Some(true),
            }),
            ..Default::default()
        }),
        declaration_provider: Some(DeclarationCapability::Simple(true)),
        references_provider: Some(OneOf::Left(true)),
        document_symbol_provider: Some(OneOf::Left(true)),
        document_range_formatting_provider: Some(OneOf::Left(true)),
        document_formatting_provider: Some(OneOf::Left(true)),
        semantic_tokens_provider: Some(SemanticTokensServerCapabilities::SemanticTokensOptions(
            SemanticTokensOptions {
                legend: SemanticTokensLegend {
                    token_types: semantic_token_query
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
                    token_modifiers: semantic_token_query
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
            },
        )),
        inlay_hint_provider: Some(OneOf::Right(InlayHintServerCapabilities::Options(
            InlayHintOptions {
                work_done_progress_options: WorkDoneProgressOptions {
                    work_done_progress: Some(false),
                },
                resolve_provider: Some(false),
            },
        ))),
        position_encoding: Some(PositionEncodingKind::UTF16),
        ..Default::default()
    })
    .unwrap();
    connection.initialize(server_capabilities).unwrap();

    let panic_hook = std::panic::take_hook();

    let hook_sender = connection.sender.clone();
    std::panic::set_hook(Box::new(move |panic_info| {
        hook_sender
            .send(Message::Notification(lsp_server::Notification::new(
                ShowMessage::METHOD.to_string(),
                ShowMessageParams {
                    typ: MessageType::ERROR,
                    message: format!("{:?}", panic_info),
                },
            )))
            .unwrap();
        (panic_hook)(panic_info)
    }));

    let mut current_rope = helix_core::ropey::Rope::new();
    let mut current_document_version = 0;
    let mut inlay_edge_map: HashMap<
        RopeSlice<'static>,
        RopeSlice<'static>,
        BuildHasherDefault<ahash::AHasher>,
    > = HashMap::default();
    let mut inlay_vb_map: HashMap<
        RopeSlice<'static>,
        Cow<'_, str>,
        BuildHasherDefault<ahash::AHasher>,
    > = HashMap::default();
    let mut last_inlay_len = 16;
    let mut currently_open = Url::parse("file:///dev/null").unwrap();
    for msg in &connection.receiver {
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req).unwrap() {
                    return;
                }

                match req.method.as_str() {
                    RangeFormatting::METHOD => {
                        if let Ok((rqid, _)) =
                            req.extract::<DocumentRangeFormattingParams>(RangeFormatting::METHOD)
                        {
                            let edits: Option<Vec<TextEdit>> =
                                formatter::format_code(&app, &current_rope).ok();

                            connection
                                .sender
                                .send(Message::Response(Response::new_ok(rqid, edits)))
                                .unwrap();
                        }
                    }
                    Formatting::METHOD => {
                        if let Ok((rqid, _)) =
                            req.extract::<DocumentFormattingParams>(Formatting::METHOD)
                        {
                            let edits: Option<Vec<TextEdit>> =
                                formatter::format_code(&app, &current_rope).ok();

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
                            connection
                                .sender
                                .send(Message::Response(Response::new_ok(
                                    rqid,
                                    rename::prepare_rename(&app, pos, &current_rope),
                                )))
                                .unwrap();
                        }
                    }
                    InlayHintRequest::METHOD => {
                        if let Ok((rqid, _)) =
                            req.extract::<InlayHintParams>(InlayHintRequest::METHOD)
                        {
                            let hints = inlay_hints::inlay_hints(
                                &app,
                                &inlay_edge_query,
                                &mut inlay_edge_map,
                                &mut inlay_vb_map,
                                &current_rope,
                                last_inlay_len,
                                &mut query_cursor,
                            );
                            last_inlay_len = hints.len();
                            connection
                                .sender
                                .send(Message::Response(Response::new_ok(rqid, Some(hints))))
                                .unwrap();
                        }
                    }
                    SemanticTokensFullRequest::METHOD => {
                        if let Ok((rqid, _)) =
                            req.extract::<SemanticTokensParams>(SemanticTokensFullRequest::METHOD)
                        {
                            connection
                                .sender
                                .send(Message::Response(Response::new_ok(
                                    rqid,
                                    Some(semantic_tokens::semantic_tokens(
                                        &app,
                                        &semantic_token_query,
                                        &current_rope,
                                        &mut query_cursor,
                                    )),
                                )))
                                .unwrap();
                        }
                    }
                    Rename::METHOD => {
                        if let Ok((rqid, rename_params)) =
                            req.extract::<RenameParams>(Rename::METHOD)
                        {
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
                                                edits: rename::rename(
                                                    &app,
                                                    rename_params,
                                                    &current_rope,
                                                    &rename_query,
                                                    &mut query_cursor,
                                                ),
                                            },
                                        ])),
                                        ..Default::default()
                                    }),
                                )))
                                .unwrap();
                        }
                    }
                    DocumentSymbolRequest::METHOD => {
                        #[allow(deprecated)]
                        if let Ok((rqid, _)) =
                            req.extract::<DocumentSymbolParams>(DocumentSymbolRequest::METHOD)
                        {
                            connection
                                .sender
                                .send(Message::Response(Response::new_ok(
                                    rqid,
                                    symbols::document_symbols(&app, &current_rope),
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
                            let completion_point = Point {
                                row: completion.text_document_position.position.line as usize,
                                column: (completion.text_document_position.position.character
                                    as usize)
                                    .saturating_sub(1),
                            };
                            let mut completion_node = tree_info
                                .root_node()
                                .descendant_for_point_range(completion_point, completion_point)
                                .unwrap();

                            while completion_node.is_error() || completion_node.is_extra() {
                                completion_node = completion_node.parent().unwrap();
                            }

                            match NodeKind::from(completion_node.kind_id()) {
                                NodeKind::Identifier => {
                                    let mut parent_object = completion_node.parent().unwrap();
                                    while parent_object.is_error() || parent_object.is_extra() {
                                        parent_object = parent_object.parent().unwrap();
                                    }
                                    match NodeKind::from(parent_object.kind_id()) {
                                        NodeKind::Viewbox
                                        | NodeKind::SlideObj
                                        | NodeKind::SlideObjects
                                            if completion_node.prev_named_sibling().is_some() =>
                                        {
                                            connection
                                                .sender
                                                .send(Message::Response(Response::new_ok(
                                                    rqid,
                                                    Some(CompletionResponse::Array(
                                                        completion::complete_viewbox(
                                                            &mut query_cursor,
                                                            &viewbox_name_query,
                                                            tree_info,
                                                            &current_rope,
                                                            completion_point,
                                                            completion_node,
                                                            completion,
                                                        ),
                                                    )),
                                                )))
                                                .unwrap();
                                        }
                                        NodeKind::SlideObjects | NodeKind::SlideObj => {
                                            connection
                                                .sender
                                                .send(Message::Response(Response::new_ok(
                                                    rqid,
                                                    Some(CompletionResponse::Array(
                                                        completion::complete_object(
                                                            &mut query_cursor,
                                                            &object_name_query,
                                                            tree_info,
                                                            &current_rope,
                                                            completion_point,
                                                            completion_node,
                                                            completion,
                                                        ),
                                                    )),
                                                )))
                                                .unwrap();
                                        }
                                        NodeKind::Obj
                                            if completion_node.prev_sibling().is_some() =>
                                        {
                                            connection
                                                .sender
                                                .send(Message::Response(Response::new_ok(
                                                    rqid,
                                                    Some(CompletionResponse::Array(
                                                        completion::complete_object_type(
                                                            &current_rope,
                                                            completion_node,
                                                            completion,
                                                        ),
                                                    )),
                                                )))
                                                .unwrap();
                                        }
                                        NodeKind::ObjInner => {
                                            connection
                                                .sender
                                                .send(Message::Response(Response::new_ok(
                                                    rqid,
                                                    Some(CompletionResponse::Array(
                                                        completion::complete_object_params(
                                                            &current_rope,
                                                            completion_node,
                                                            completion,
                                                        ),
                                                    )),
                                                )))
                                                .unwrap();
                                        }
                                        NodeKind::Completion => {
                                            connection
                                                .sender
                                                .send(Message::Response(Response::new_ok(
                                                    rqid,
                                                    Some(CompletionResponse::Array(
                                                        completion::complete_top_level_object(
                                                            &current_rope,
                                                            parent_object,
                                                            completion_node,
                                                            completion,
                                                        ),
                                                    )),
                                                )))
                                                .unwrap();
                                        }
                                        _ => {
                                            connection
                                                .sender
                                                .send(Message::Response(Response::new_ok(
                                                    rqid,
                                                    None::<CompletionResponse>,
                                                )))
                                                .unwrap();
                                        }
                                    }
                                }
                                NodeKind::SourceFile => {
                                    connection
                                        .sender
                                        .send(Message::Response(Response::new_ok(
                                            rqid,
                                            completion::complete_source_file(
                                                completion,
                                                &slide_complete_query,
                                                tree_info,
                                                &current_rope,
                                                &mut query_cursor,
                                            )
                                            .ok(),
                                        )))
                                        .unwrap();
                                }
                                NodeKind::StringContent => {
                                    let completion_range = completion_node.range();
                                    if let Some(parent) =
                                        completion_node.parent().and_then(|n| n.parent())
                                    {
                                        if parent.kind_id() == NodeKind::ObjParam as u16
                                            && parent
                                                .child(0)
                                                .map(|c| {
                                                    &current_rope.byte_slice(c.byte_range())
                                                        == "font_family"
                                                })
                                                .unwrap_or_default()
                                        {
                                            let fonts = fonts
                                                .iter()
                                                .map(|f| CompletionItem {
                                                    label: f.clone(),
                                                    kind: Some(CompletionItemKind::VARIABLE),
                                                    deprecated: Some(false),
                                                    preselect: Some(true),
                                                    insert_text_format: Some(
                                                        InsertTextFormat::PLAIN_TEXT,
                                                    ),
                                                    insert_text_mode: None,
                                                    text_edit: Some(
                                                        CompletionTextEdit::InsertAndReplace(
                                                            InsertReplaceEdit {
                                                                new_text: f.clone(),
                                                                insert: lsp_types::Range {
                                                                    start: completion
                                                                        .text_document_position
                                                                        .position,
                                                                    end: completion
                                                                        .text_document_position
                                                                        .position,
                                                                },
                                                                replace:
                                                                    char_range_from_byte_range(
                                                                        completion_range,
                                                                        &current_rope,
                                                                    )
                                                                    .unwrap(),
                                                            },
                                                        ),
                                                    ),
                                                    additional_text_edits: Some(Vec::new()),
                                                    ..Default::default()
                                                })
                                                .collect::<Vec<_>>();
                                            connection
                                                .sender
                                                .send(Message::Response(Response::new_ok(
                                                    rqid,
                                                    Some(CompletionResponse::Array(fonts)),
                                                )))
                                                .unwrap();
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    GotoDeclaration::METHOD => {
                        if let Ok((rqid, goto_params)) =
                            req.extract::<GotoDefinitionParams>(GotoDeclaration::METHOD)
                        {
                            connection
                                .sender
                                .send(Message::Response(Response::new_ok(
                                    rqid,
                                    symbols::goto_declaration(
                                        &app,
                                        goto_params,
                                        &top_level_search_query,
                                        &current_rope,
                                        currently_open.clone(),
                                        &mut query_cursor,
                                    ),
                                )))
                                .unwrap();
                        }
                    }
                    ExecuteCommand::METHOD => {
                        if let Ok((rqid, command)) =
                            req.extract::<ExecuteCommandParams>(ExecuteCommand::METHOD)
                        {
                            match command.command.as_str() {
                                "treerestaurant_to_dot" => {
                                    #[cfg(unix)]
                                    if let Ok(process) = std::process::Command::new("dot")
                                        .stdout(std::fs::File::create("out.dot").unwrap())
                                        .stdin(Stdio::piped())
                                        .spawn()
                                    {
                                        let tree_info = app.tree_info.lock();
                                        let tree_info = tree_info.as_ref().unwrap();

                                        tree_info.print_dot_graph(&process.stdin.unwrap());

                                        connection
                                            .sender
                                            .send(Message::Response(Response::new_ok(
                                                rqid,
                                                None::<serde_json::Value>,
                                            )))
                                            .unwrap();
                                    } else {
                                        connection
                                            .sender
                                            .send(Message::Response(Response::new_err(
                                                rqid,
                                                500,
                                                "graphviz is not installed".to_string(),
                                            )))
                                            .unwrap();
                                    }
                                }
                                "full_reparse" => {
                                    let mut slide_show = app.slide_show.write();
                                    current_rope = Rope::from_reader(
                                        std::fs::File::open(currently_open.path()).unwrap(),
                                    )
                                    .unwrap();
                                    let mut tree_info = app.tree_info.lock();
                                    let tree = parser
                                        .parse_with(
                                            &mut |byte, _| {
                                                if byte <= current_rope.len_bytes() {
                                                    let (chunk, start_byte, _, _) =
                                                        current_rope.chunk_at_byte(byte);
                                                    &chunk.as_bytes()[byte - start_byte..]
                                                } else {
                                                    // out of range
                                                    &[]
                                                }
                                            },
                                            None,
                                        )
                                        .unwrap();
                                    let ast = crate::parser::parse_file(
                                        &tree,
                                        None,
                                        &current_rope,
                                        &mut app.helix_cell,
                                        &mut slide_show,
                                        &lsp_egui_ctx,
                                        Path::new(currently_open.path()),
                                    );
                                    *tree_info = Some(tree.clone());
                                    *app.slide_show_file.lock() = current_rope.clone();
                                    match ast {
                                        Ok(_) => {
                                            connection
                                                .sender
                                                .send(Message::Notification(
                                                    lsp_server::Notification::new(
                                                        PublishDiagnostics::METHOD.to_string(),
                                                        PublishDiagnosticsParams {
                                                            uri: currently_open.clone(),
                                                            diagnostics: vec![],
                                                            version: Some(current_document_version),
                                                        },
                                                    ),
                                                ))
                                                .unwrap();
                                            error_free_tree = Some(tree);
                                        }
                                        Err(errors) => {
                                            connection
                                            .sender
                                            .send(Message::Notification(lsp_server::Notification::new(
                                                PublishDiagnostics::METHOD.to_string(),
                                                PublishDiagnosticsParams {
                                                    uri: currently_open.clone(),
                                                    diagnostics: errors
                                                        .into_iter()
                                                        .map(|error| {
                                                            let diagnostic: lsp_types::Diagnostic =
                                                                error.into();
                                                            diagnostic
                                                        })
                                                        .collect(),
                                                    version: Some(current_document_version),
                                                },
                                            )))
                                            .unwrap();
                                        }
                                    }

                                    app.resolved.store(None);
                                    app.restart_timer.store(true, Ordering::Relaxed);
                                    lsp_egui_ctx.request_repaint();
                                }
                                "spell_check" => {
                                    let hunspell = hunspell.get_or_insert_with(|| {
                                        hunspell_rs::Hunspell::new(
                                            "/usr/share/hunspell/en_US.aff",
                                            "/usr/share/hunspell/en_US.dic",
                                        )
                                    });
                                    query_cursor.set_point_range(
                                        Point { row: 0, column: 0 }..Point {
                                            row: usize::MAX,
                                            column: usize::MAX,
                                        },
                                    );
                                    let tree_info = app.tree_info.lock();
                                    let iter = query_cursor.matches(
                                        &strings_query,
                                        tree_info.as_ref().unwrap().root_node(),
                                        RopeProvider(current_rope.slice(..)),
                                    );

                                    let mut warnings = Vec::new();

                                    for query_match in iter {
                                        let source: Cow<'_, str> = current_rope
                                            .byte_slice(query_match.captures[0].node.byte_range())
                                            .into();
                                        let parser = jotdown::Parser::new(source.as_ref());

                                        for event in parser {
                                            match event {
                                                jotdown::Event::Str(t) => {
                                                    for text in t.split_whitespace() {
                                                        let text = text.trim_matches(|c: char| {
                                                            c.is_ascii_punctuation()
                                                        });
                                                        if hunspell.check(text)
                                                            == CheckResult::MissingInDictionary
                                                        {
                                                            warnings.push(
                                                                super::parser::Error::SpellCheck(
                                                                    PointFromRange::new(
                                                                        query_match.captures[0]
                                                                            .node
                                                                            .range(),
                                                                        &current_rope,
                                                                    ),
                                                                    hunspell.suggest(text),
                                                                ),
                                                            );
                                                        }
                                                    }
                                                }
                                                _ => {}
                                            }
                                        }
                                    }

                                    if !warnings.is_empty() {
                                        connection
                                        .sender
                                        .send(Message::Notification(lsp_server::Notification::new(
                                            PublishDiagnostics::METHOD.to_string(),
                                            PublishDiagnosticsParams {
                                                uri: currently_open.clone(),
                                                diagnostics: warnings
                                                    .into_iter()
                                                    .map(|error| {
                                                        let diagnostic: lsp_types::Diagnostic =
                                                            error.into();
                                                        diagnostic
                                                    })
                                                    .collect(),
                                                version: Some(current_document_version),
                                            },
                                        )))
                                        .unwrap();
                                    }

                                    connection
                                        .sender
                                        .send(Message::Response(Response::new_ok(
                                            rqid,
                                            None::<serde_json::Value>,
                                        )))
                                        .unwrap();
                                }
                                _ => {
                                    connection
                                        .sender
                                        .send(Message::Response(Response::new_ok(
                                            rqid,
                                            None::<serde_json::Value>,
                                        )))
                                        .unwrap();
                                }
                            }
                        }
                    }
                    References::METHOD => {
                        if let Ok((rqid, reference_params)) =
                            req.extract::<ReferenceParams>(References::METHOD)
                        {
                            connection
                                .sender
                                .send(Message::Response(Response::new_ok(
                                    rqid,
                                    symbols::references(
                                        &app,
                                        &rename_query,
                                        &current_rope,
                                        reference_params,
                                        &currently_open,
                                        &mut query_cursor,
                                    ),
                                )))
                                .unwrap();
                        }
                    }
                    _ => {}
                }

                // ...
            }
            Message::Response(_) => {}
            Message::Notification(not) => {
                match not.method.as_str() {
                    DidOpenTextDocument::METHOD => {
                        let doc: lsp_types::DidOpenTextDocumentParams =
                            serde_json::from_value(not.params).unwrap();
                        currently_open = doc.text_document.uri;
                        let mut slide_show = app.slide_show.write();
                        current_rope = helix_core::ropey::Rope::from_str(&doc.text_document.text);
                        if current_rope.len_lines() < 3 {
                            const HELLO_WORLD: &str = include_str!("hello_world.grz");
                            current_rope.insert(0, HELLO_WORLD);
                            connection
                                .sender
                                .send(Message::Request(lsp_server::Request::new(
                                    0.into(),
                                    ApplyWorkspaceEdit::METHOD.to_string(),
                                    ApplyWorkspaceEditParams {
                                        label: None,
                                        edit: WorkspaceEdit {
                                            document_changes: Some(DocumentChanges::Edits(vec![
                                                TextDocumentEdit {
                                                    edits: vec![OneOf::Left(TextEdit {
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
                                                        new_text: HELLO_WORLD.to_string(),
                                                    })],
                                                    text_document:
                                                        OptionalVersionedTextDocumentIdentifier {
                                                            uri: currently_open.clone(),
                                                            version: Some(current_document_version),
                                                        },
                                                },
                                            ])),
                                            ..Default::default()
                                        },
                                    },
                                )))
                                .unwrap();
                            current_document_version += 1;
                        }
                        let mut tree_info = app.tree_info.lock();
                        let tree = parser
                            .parse_with(
                                &mut |byte, _| {
                                    if byte <= current_rope.len_bytes() {
                                        let (chunk, start_byte, _, _) =
                                            current_rope.chunk_at_byte(byte);
                                        &chunk.as_bytes()[byte - start_byte..]
                                    } else {
                                        // out of range
                                        &[]
                                    }
                                },
                                None,
                            )
                            .unwrap();
                        let ast = crate::parser::parse_file(
                            &tree,
                            None,
                            &current_rope,
                            &mut app.helix_cell,
                            &mut slide_show,
                            &lsp_egui_ctx,
                            Path::new(currently_open.path()),
                        );
                        *tree_info = Some(tree.clone());
                        *app.slide_show_file.lock() = current_rope.clone();
                        match ast {
                            Ok(_) => {
                                connection
                                    .sender
                                    .send(Message::Notification(lsp_server::Notification::new(
                                        PublishDiagnostics::METHOD.to_string(),
                                        PublishDiagnosticsParams {
                                            uri: currently_open.clone(),
                                            diagnostics: vec![],
                                            version: Some(current_document_version),
                                        },
                                    )))
                                    .unwrap();
                                current_thread.unpark();
                                error_free_tree = Some(tree);
                            }
                            Err(errors) => {
                                connection
                                    .sender
                                    .send(Message::Notification(lsp_server::Notification::new(
                                        PublishDiagnostics::METHOD.to_string(),
                                        PublishDiagnosticsParams {
                                            uri: currently_open.clone(),
                                            diagnostics: errors
                                                .into_iter()
                                                .map(|error| {
                                                    let diagnostic: lsp_types::Diagnostic =
                                                        error.into();
                                                    diagnostic
                                                })
                                                .collect(),
                                            version: Some(current_document_version),
                                        },
                                    )))
                                    .unwrap();
                            }
                        }

                        app.resolved.store(None);
                    }
                    DidChangeTextDocument::METHOD => {
                        let changes: lsp_types::DidChangeTextDocumentParams =
                            serde_json::from_value(not.params).unwrap();

                        if current_document_version < changes.text_document.version {
                            current_document_version = changes.text_document.version;

                            let mut tree_info = app.tree_info.lock();
                            let tree_info = tree_info.as_mut().unwrap();

                            let changes_len = changes.content_changes.len();

                            let mut point = Point { row: 0, column: 0 };
                            for change in changes.content_changes {
                                let edit = lsp_types::TextEdit {
                                    range: change.range.unwrap(),
                                    new_text: change.text,
                                };
                                point = Point {
                                    row: edit.range.start.line as usize,
                                    column: edit.range.start.character as usize,
                                };

                                let transaction = helix_lsp::util::generate_transaction_from_edits(
                                    &current_rope,
                                    vec![edit],
                                    helix_lsp::OffsetEncoding::Utf16,
                                );

                                let edits =
                                    generate_edits(current_rope.slice(..), transaction.changes());
                                if transaction.apply(&mut current_rope) {
                                    let source = current_rope.slice(..);
                                    for edit in edits.iter().rev() {
                                        tree_info.edit(edit);
                                        if let Some(error_tree) = error_free_tree.as_mut() {
                                            error_tree.edit(edit);
                                        }
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
                                            Some(error_free_tree.as_ref().unwrap_or(&*tree_info)),
                                        )
                                        .unwrap();
                                    let mut slide_show = app.slide_show.write();

                                    match super::parser::parse_file(
                                        &tree,
                                        Some(error_free_tree.as_ref().unwrap_or(&*tree_info)),
                                        &current_rope,
                                        &mut app.helix_cell,
                                        &mut slide_show,
                                        &lsp_egui_ctx,
                                        Path::new(currently_open.path()),
                                    ) {
                                        Ok(_) => {
                                            connection
                                                .sender
                                                .send(Message::Notification(
                                                    lsp_server::Notification::new(
                                                        PublishDiagnostics::METHOD.to_string(),
                                                        PublishDiagnosticsParams {
                                                            uri: currently_open.clone(),
                                                            diagnostics: vec![],
                                                            version: Some(current_document_version),
                                                        },
                                                    ),
                                                ))
                                                .unwrap();
                                            app.resolved.store(None);
                                            lsp_egui_ctx.request_repaint();
                                        }
                                        Err(errors) => {
                                            connection
                                                    .sender
                                                    .send(Message::Notification(lsp_server::Notification::new(
                                                        PublishDiagnostics::METHOD.to_string(),
                                                        PublishDiagnosticsParams {
                                                            uri: currently_open.clone(),
                                                            diagnostics: errors
                                                                .into_iter()
                                                                .map(|error| {
                                                                    let diagnostic: lsp_types::Diagnostic =
                                                                        error.into();
                                                                    diagnostic
                                                                })
                                                                .collect(),
                                                            version: Some(current_document_version),
                                                        },
                                                    )))
                                                    .unwrap();
                                        }
                                    }
                                    if !tree.root_node().has_error() {
                                        error_free_tree = Some(tree.clone());
                                    }
                                    *tree_info = tree;
                                } else {
                                    panic!("Transaction could not be applied");
                                }
                            }

                            if changes_len == 1 {
                                hover(
                                    &app,
                                    &*tree_info,
                                    &mut query_cursor,
                                    &current_rope,
                                    &slide_index_query,
                                    &vb_in_slide_query,
                                    &obj_in_slide_query,
                                    &lsp_egui_ctx,
                                    point,
                                );
                            }
                        }
                    }
                    DidSaveTextDocument::METHOD => {
                        // let _: lsp_types::DidSaveTextDocumentParams =
                        //    serde_json::from_value(not.params).unwrap();
                        let mut tree_info = app.tree_info.lock();
                        if let Some(info) = tree_info.as_mut() {
                            let mut slide_show = app.slide_show.write();

                            let ast = super::parser::parse_file(
                                &*info,
                                None,
                                &current_rope,
                                &mut app.helix_cell,
                                &mut slide_show,
                                &lsp_egui_ctx,
                                Path::new(currently_open.path()),
                            );
                            match ast {
                                Ok(_) => {
                                    *app.slide_show_file.lock() = current_rope.clone();
                                    connection
                                        .sender
                                        .send(Message::Notification(lsp_server::Notification::new(
                                            PublishDiagnostics::METHOD.to_string(),
                                            PublishDiagnosticsParams {
                                                uri: currently_open.clone(),
                                                diagnostics: vec![],
                                                version: Some(current_document_version),
                                            },
                                        )))
                                        .unwrap();
                                    app.resolved.store(None);
                                    app.next.store(true, Ordering::Relaxed);
                                    app.restart_timer.store(true, Ordering::Relaxed);
                                    current_thread.unpark();
                                }
                                Err(errors) => {
                                    connection
                                        .sender
                                        .send(Message::Notification(lsp_server::Notification::new(
                                            PublishDiagnostics::METHOD.to_string(),
                                            PublishDiagnosticsParams {
                                                uri: currently_open.clone(),
                                                diagnostics: errors
                                                    .into_iter()
                                                    .map(|error| {
                                                        let diagnostic: lsp_types::Diagnostic =
                                                            error.into();
                                                        diagnostic
                                                    })
                                                    .collect(),
                                                version: Some(current_document_version),
                                            },
                                        )))
                                        .unwrap();
                                }
                            }
                        }

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

pub fn hover(
    app: &MyEguiApp,
    tree_info: &Tree,
    query_cursor: &mut QueryCursor,
    current_rope: &Rope,
    slide_index_query: &Query,
    vb_in_slide_query: &Query,
    obj_in_slide_query: &Query,
    lsp_egui_ctx: &eframe::egui::Context,
    point: Point,
) -> Option<Hover> {
    let changed_point = tree_info
        .root_node()
        .descendant_for_point_range(point, point);

    if let Some(mut node) = changed_point {
        if node.kind_id() == NodeKind::EdgeParser as u16 {
            app.restart_timer.store(true, Ordering::Relaxed);
        }

        while node.kind_id() != NodeKind::Slide as u16
            && node.kind_id() != NodeKind::Viewbox as u16
            && node.kind_id() != NodeKind::Obj as u16
            && node.kind_id() != NodeKind::SlideVb as u16
        {
            if let Some(parent) = node.parent() {
                node = parent;
            } else {
                if app.vb_dbg.swap(0, Ordering::Relaxed) != 0
                    || app.obj_dbg.swap(0, Ordering::Relaxed) != 0
                {
                    lsp_egui_ctx.request_repaint();
                }
                return None;
            }
        }

        if node.parent().map(|n| NodeKind::from(n.kind_id()) as u16) == Some(NodeKind::Slide as u16)
        {
            node = node.parent().unwrap();
        }

        match NodeKind::from(node.kind_id()) {
            nk @ NodeKind::Slide | nk @ NodeKind::SlideFunctions => {
                query_cursor.set_point_range(Point { row: 0, column: 0 }..point);

                let mut iter = query_cursor.matches(
                    slide_index_query,
                    tree_info.root_node(),
                    RopeProvider(current_rope.slice(..)),
                );

                let slide_num = iter
                    .position(|n| n.captures[0].node.id() == node.id())
                    .unwrap_or_default();

                if app.index.swap(slide_num, Ordering::Relaxed) != slide_num {
                    app.next.store(false, Ordering::Relaxed);
                    app.vb_dbg.store(0, Ordering::Relaxed);
                    app.obj_dbg.store(0, Ordering::Relaxed);
                    app.resolved.store(None);
                    lsp_egui_ctx.request_repaint();
                }
                if matches!(nk, NodeKind::SlideFunctions) {
                    app.restart_timer.store(true, Ordering::Relaxed);
                    lsp_egui_ctx.request_repaint();
                }
            }
            NodeKind::Viewbox => {
                if let Some(name_node) = node.named_child(0) {
                    let vb_name = current_rope.byte_slice(name_node.byte_range());
                    let hasher = ahash::RandomState::with_seeds(69, 420, 24, 96);

                    let hashed_vb = hasher.hash_one(vb_name);
                    if app.vb_dbg.swap(hashed_vb, Ordering::Relaxed) != hashed_vb {
                        let mut already_on_slide = false;
                        match &app.slide_show.read().slide_show[app.index.load(Ordering::Relaxed)] {
                            AstObject::Slide { objects, .. } => {
                                already_on_slide = objects
                                    .iter()
                                    .flat_map(|o| &o.locations)
                                    .any(|(_, vb)| matches!(vb, ViewboxIn::Custom(vb, _) if *vb == hashed_vb))
                            }
                            _ => {}
                        }

                        if !already_on_slide {
                            query_cursor.set_point_range(
                                name_node.range().start_point..Point {
                                    row: usize::MAX,
                                    column: usize::MAX,
                                },
                            );

                            let mut iter = query_cursor.matches(
                                vb_in_slide_query,
                                tree_info.root_node(),
                                RopeProvider(current_rope.slice(..)),
                            );

                            let on_slide = iter.find(|query_match| {
                                current_rope.byte_slice(query_match.captures[0].node.byte_range())
                                    == vb_name
                            });

                            if let Some(mut on_slide) =
                                on_slide.map(|q_match| q_match.captures[0].node)
                            {
                                while on_slide.kind_id() != NodeKind::Slide as u16 {
                                    if let Some(parent) = on_slide.parent() {
                                        on_slide = parent;
                                    } else {
                                        return None;
                                    }
                                }
                                let range = on_slide.range();

                                query_cursor
                                    .set_point_range(Point { row: 0, column: 0 }..range.end_point);

                                let mut iter = query_cursor.matches(
                                    slide_index_query,
                                    tree_info.root_node(),
                                    RopeProvider(current_rope.slice(..)),
                                );

                                let slide_num = iter
                                    .position(|n| n.captures[0].node.id() == on_slide.id())
                                    .unwrap_or_default();

                                app.index.store(slide_num, Ordering::Relaxed);
                            }
                            app.next.store(false, Ordering::Relaxed);
                            app.obj_dbg.store(0, Ordering::Relaxed);

                            app.resolved.store(None);
                            lsp_egui_ctx.request_repaint();
                        }
                    }
                }
            }
            NodeKind::SlideVb => {
                if current_rope
                    .byte_slice(node.byte_range())
                    .chunks()
                    .next()
                    .map(|c| c.starts_with('|'))
                    .unwrap_or_default()
                {
                    if let Some(name_node) = node.parent().and_then(|n| n.named_child(0)) {
                        let vb_name = current_rope.byte_slice(name_node.byte_range());
                        let hasher = ahash::RandomState::with_seeds(69, 420, 24, 96);
                        let mut hasher = hasher.build_hasher();
                        vb_name.hash(&mut hasher);
                        node.range().hash(&mut hasher);
                        let hashed_vb = hasher.finish();
                        if app.vb_dbg.swap(hashed_vb, Ordering::Relaxed) != hashed_vb {
                            while node.kind_id() != NodeKind::Slide as u16 {
                                if let Some(parent) = node.parent() {
                                    node = parent;
                                } else {
                                    return None;
                                }
                            }
                            query_cursor.set_point_range(Point { row: 0, column: 0 }..point);

                            let mut iter = query_cursor.matches(
                                slide_index_query,
                                tree_info.root_node(),
                                RopeProvider(current_rope.slice(..)),
                            );

                            let slide_num = iter
                                .position(|n| n.captures[0].node.id() == node.id())
                                .unwrap_or_default();

                            if app.index.swap(slide_num, Ordering::Relaxed) != slide_num {
                                app.next.store(false, Ordering::Relaxed);
                                app.resolved.store(None);
                                lsp_egui_ctx.request_repaint();
                            }
                        }
                    }
                }
            }
            NodeKind::Obj => {
                if let Some(name_node) = node.named_child(0) {
                    let obj_name = current_rope.byte_slice(name_node.byte_range());
                    let hasher = ahash::RandomState::with_seeds(69, 420, 24, 96);

                    let hashed_obj = hasher.hash_one(obj_name);
                    if app.obj_dbg.swap(hashed_obj, Ordering::Relaxed) != hashed_obj {
                        query_cursor.set_point_range(
                            name_node.range().start_point..Point {
                                row: usize::MAX,
                                column: usize::MAX,
                            },
                        );

                        let mut iter = query_cursor.matches(
                            obj_in_slide_query,
                            tree_info.root_node(),
                            RopeProvider(current_rope.slice(..)),
                        );

                        let on_slide = iter.find(|query_match| {
                            current_rope.byte_slice(query_match.captures[0].node.byte_range())
                                == obj_name
                        });

                        if let Some(mut on_slide) = on_slide.map(|q_match| q_match.captures[0].node)
                        {
                            while on_slide.kind_id() != NodeKind::Slide as u16 {
                                if let Some(parent) = on_slide.parent() {
                                    on_slide = parent;
                                } else {
                                    return None;
                                }
                            }
                            let range = on_slide.range();

                            query_cursor
                                .set_point_range(Point { row: 0, column: 0 }..range.end_point);

                            let mut iter = query_cursor.matches(
                                slide_index_query,
                                tree_info.root_node(),
                                RopeProvider(current_rope.slice(..)),
                            );

                            let slide_num = iter
                                .position(|n| n.captures[0].node.id() == on_slide.id())
                                .unwrap_or_default();

                            app.index.store(slide_num, Ordering::Relaxed);
                            app.next.store(false, Ordering::Relaxed);
                            app.vb_dbg.store(0, Ordering::Relaxed);

                            app.resolved.store(None);
                            lsp_egui_ctx.request_repaint();
                        }
                    }
                }
            }
            _ => unreachable!(),
        }
    }
    None
}

#[cfg(not(target_arch = "wasm32"))]
pub fn generate_edits(
    old_text: helix_core::ropey::RopeSlice<'_>,
    changeset: &helix_core::ChangeSet,
) -> Vec<helix_core::tree_sitter::InputEdit> {
    use helix_core::{chars::char_is_line_ending, Operation::*, Tendril};

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
                edits.push(helix_core::tree_sitter::InputEdit {
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
                    edits.push(helix_core::tree_sitter::InputEdit {
                        start_byte,                                    // old_pos to byte
                        old_end_byte,                                  // old_end to byte
                        new_end_byte: start_byte + s.len(),            // old_pos to byte + s.len()
                        start_position,                                // old pos to coords
                        old_end_position,                              // old_end to coords
                        new_end_position: traverse(start_position, s), // old pos + chars, newlines matter too (iter over)
                    });
                } else {
                    // insert
                    edits.push(helix_core::tree_sitter::InputEdit {
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
