use std::{
    borrow::Cow,
    collections::HashMap,
    hash::Hash,
    hash::{BuildHasher, BuildHasherDefault, Hasher},
    path::Path,
    process::Stdio,
    sync::atomic::Ordering,
};

pub mod formatter;
pub mod you_can;

use crate::{
    parser::{Error, GrzCursor, NodeKind},
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
    AnnotatedTextEdit, ApplyWorkspaceEditParams, CompletionItem, CompletionItemKind,
    CompletionItemLabelDetails, CompletionOptions, CompletionOptionsCompletionItem,
    CompletionParams, CompletionResponse, CompletionTextEdit, DeclarationCapability,
    DocumentChanges, DocumentFormattingParams, DocumentRangeFormattingParams, DocumentSymbol,
    DocumentSymbolParams, DocumentSymbolResponse, ExecuteCommandOptions, ExecuteCommandParams,
    GotoDefinitionParams, GotoDefinitionResponse, Hover, InlayHint, InlayHintKind, InlayHintLabel,
    InlayHintOptions, InlayHintParams, InlayHintServerCapabilities, InsertReplaceEdit,
    InsertTextFormat, Location, MessageType, OneOf, OptionalVersionedTextDocumentIdentifier,
    Position, PositionEncodingKind, PrepareRenameResponse, PublishDiagnosticsParams,
    ReferenceParams, RenameOptions, RenameParams, SaveOptions, SemanticToken,
    SemanticTokenModifier, SemanticTokenType, SemanticTokens, SemanticTokensFullOptions,
    SemanticTokensLegend, SemanticTokensOptions, SemanticTokensParams, SemanticTokensResult,
    SemanticTokensServerCapabilities, ServerCapabilities, ShowMessageParams, SymbolKind,
    TextDocumentEdit, TextDocumentPositionParams, TextDocumentSyncCapability, TextDocumentSyncKind,
    TextDocumentSyncOptions, TextDocumentSyncSaveOptions, TextEdit, Url, WorkDoneProgressOptions,
    WorkspaceEdit,
};

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
    let rename_query = Query::new(
        tree_sitter_grz::language(),
        include_str!("queries/rename.scm"),
    )
    .unwrap();
    let slide_complete_query = Query::new(
        tree_sitter_grz::language(),
        include_str!("queries/slide_complete.scm"),
    )
    .unwrap();
    let slide_index_query = Query::new(
        tree_sitter_grz::language(),
        include_str!("queries/slide_index.scm"),
    )
    .unwrap();
    let top_level_search_query = Query::new(
        tree_sitter_grz::language(),
        include_str!("queries/top_level_search.scm"),
    )
    .unwrap();
    let inlay_edge_query = Query::new(
        tree_sitter_grz::language(),
        include_str!("queries/inlay_edge.scm"),
    )
    .unwrap();
    let viewbox_name_query = Query::new(
        tree_sitter_grz::language(),
        include_str!("queries/viewbox_name.scm"),
    )
    .unwrap();
    let object_name_query = Query::new(
        tree_sitter_grz::language(),
        include_str!("queries/object_name.scm"),
    )
    .unwrap();
    let semantic_token_query = Query::new(
        tree_sitter_grz::language(),
        include_str!("queries/semantic_tokens.scm"),
    )
    .unwrap();
    let vb_in_slide_query = Query::new(
        tree_sitter_grz::language(),
        include_str!("queries/vb_in_slide.scm"),
    )
    .unwrap();
    let obj_in_slide_query = Query::new(
        tree_sitter_grz::language(),
        include_str!("queries/obj_in_slide.scm"),
    )
    .unwrap();
    let strings_query = Query::new(
        tree_sitter_grz::language(),
        include_str!("queries/strings.scm"),
    )
    .unwrap();
    let font_strings = font_loader::system_fonts::query_all()
        .into_iter()
        .collect::<IndexSet<_, ahash::RandomState>>();

    let mut hunspell = None;

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
                        .filter_map(|name| {
                            // Safe because string exists for lifetime of LSP
                            unsafe { std::mem::transmute::<&String, &'static String>(name) }
                                .split_once('.')
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
    let mut error_tree = None;
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

                            match edits.clone() {
                                Some(edits) if !edits.is_empty() => {
                                    current_document_version += 1;

                                    let mut tree_info = app.tree_info.lock();
                                    let tree_info = tree_info.as_mut().unwrap();

                                    let transaction =
                                        helix_lsp::util::generate_transaction_from_edits(
                                            &current_rope,
                                            edits,
                                            helix_lsp::OffsetEncoding::Utf16,
                                        );

                                    let edits = generate_edits(
                                        current_rope.slice(..),
                                        transaction.changes(),
                                    );
                                    if transaction.apply(&mut current_rope) {
                                        let source = current_rope.slice(..);
                                        for edit in edits.iter().rev() {
                                            tree_info.edit(edit);
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
                                                Some(tree_info),
                                            )
                                            .unwrap();

                                        *tree_info = tree;
                                    } else {
                                        panic!("Transaction could not be applied");
                                    }
                                }
                                _ => {}
                            }

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
                                    prepare_rename(&app, pos),
                                )))
                                .unwrap();
                        }
                    }
                    InlayHintRequest::METHOD => {
                        if let Ok((rqid, _)) =
                            req.extract::<InlayHintParams>(InlayHintRequest::METHOD)
                        {
                            let hints = inlay_hints(
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
                                    Some(semantic_tokens(
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
                                                edits: rename(
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
                                    document_symbols(&app, &current_rope),
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
                                            query_cursor.set_point_range(
                                                Point { row: 0, column: 0 }..completion_point,
                                            );
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
                                                insert_text_format: Some(
                                                    InsertTextFormat::PLAIN_TEXT,
                                                ),
                                                insert_text_mode: None,
                                                text_edit: Some(
                                                    CompletionTextEdit::InsertAndReplace(
                                                        InsertReplaceEdit {
                                                            new_text: "Size".to_string(),
                                                            insert: lsp_types::Range {
                                                                start: completion
                                                                    .text_document_position
                                                                    .position,
                                                                end: completion
                                                                    .text_document_position
                                                                    .position,
                                                            },
                                                            replace: lsp_types::Range {
                                                                start: Position {
                                                                    line: completion_range
                                                                        .start_point
                                                                        .row
                                                                        as u32,
                                                                    character: completion_range
                                                                        .start_point
                                                                        .column
                                                                        as u32,
                                                                },
                                                                end: Position {
                                                                    line: completion_range
                                                                        .end_point
                                                                        .row
                                                                        as u32,
                                                                    character: completion_range
                                                                        .end_point
                                                                        .column
                                                                        as u32,
                                                                },
                                                            },
                                                        },
                                                    ),
                                                ),
                                                additional_text_edits: Some(Vec::new()),
                                                ..Default::default()
                                            }];
                                            completions.extend(iter.map(|query_match| {
                                                let byte_range =
                                                    query_match.captures[0].node.byte_range();
                                                let label =
                                                    current_rope.byte_slice(byte_range).to_string();

                                                CompletionItem {
                                                    label: label.clone(),
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
                                                                new_text: label,
                                                                insert: lsp_types::Range {
                                                                    start: completion
                                                                        .text_document_position
                                                                        .position,
                                                                    end: completion
                                                                        .text_document_position
                                                                        .position,
                                                                },
                                                                replace: lsp_types::Range {
                                                                    start: Position {
                                                                        line: completion_range
                                                                            .start_point
                                                                            .row
                                                                            as u32,
                                                                        character: completion_range
                                                                            .start_point
                                                                            .column
                                                                            as u32,
                                                                    },
                                                                    end: Position {
                                                                        line: completion_range
                                                                            .end_point
                                                                            .row
                                                                            as u32,
                                                                        character: completion_range
                                                                            .end_point
                                                                            .column
                                                                            as u32,
                                                                    },
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    additional_text_edits: Some(Vec::new()),
                                                    ..Default::default()
                                                }
                                            }));

                                            connection
                                                .sender
                                                .send(Message::Response(Response::new_ok(
                                                    rqid,
                                                    Some(CompletionResponse::Array(completions)),
                                                )))
                                                .unwrap();
                                        }
                                        NodeKind::SlideObjects | NodeKind::SlideObj => {
                                            query_cursor.set_point_range(
                                                Point { row: 0, column: 0 }..completion_point,
                                            );
                                            let iter = query_cursor.matches(
                                                &object_name_query,
                                                tree_info.root_node(),
                                                RopeProvider(current_rope.slice(..)),
                                            );

                                            let completions: Vec<CompletionItem> = iter
                                                .map(|query_match| {
                                                    let byte_range =
                                                        query_match.captures[0].node.byte_range();
                                                    let label = current_rope
                                                        .byte_slice(byte_range)
                                                        .to_string();
                                                    let completion_range = completion_node.range();
                                                    CompletionItem {
                                                        kind: Some(CompletionItemKind::VARIABLE),
                                                        deprecated: Some(false),
                                                        preselect: Some(true),
                                                        insert_text_format: Some(
                                                            InsertTextFormat::SNIPPET,
                                                        ),
                                                        insert_text_mode: None,
                                                        text_edit: Some(
                                                            CompletionTextEdit::InsertAndReplace(
                                                                InsertReplaceEdit {
                                                                    new_text: format!(
                                                                        "{}$0,",
                                                                        label
                                                                    ),
                                                                    insert: lsp_types::Range {
                                                                        start: completion
                                                                            .text_document_position
                                                                            .position,
                                                                        end: completion
                                                                            .text_document_position
                                                                            .position,
                                                                    },
                                                                    replace: lsp_types::Range {
                                                                        start: Position {
                                                                            line: completion_range
                                                                                .start_point
                                                                                .row
                                                                                as u32,
                                                                            character:
                                                                                completion_range
                                                                                    .start_point
                                                                                    .column
                                                                                    as u32,
                                                                        },
                                                                        end: Position {
                                                                            line: completion_range
                                                                                .end_point
                                                                                .row
                                                                                as u32,
                                                                            character:
                                                                                completion_range
                                                                                    .end_point
                                                                                    .column
                                                                                    as u32,
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                        label,
                                                        additional_text_edits: Some(Vec::new()),
                                                        ..Default::default()
                                                    }
                                                })
                                                .collect();

                                            connection
                                                .sender
                                                .send(Message::Response(Response::new_ok(
                                                    rqid,
                                                    Some(CompletionResponse::Array(completions)),
                                                )))
                                                .unwrap();
                                        }
                                        NodeKind::Obj
                                            if completion_node.prev_sibling().is_some() =>
                                        {
                                            let completion_range = completion_node.range();
                                            connection
                                                .sender
                                                .send(Message::Response(Response::new_ok(
                                                    rqid,
                                                    Some(CompletionResponse::Array(vec![
                                        CompletionItem {
                                        label: "Paragraph".to_string(),
                                        kind: Some(CompletionItemKind::TYPE_PARAMETER),
                                        deprecated: Some(false),
                                        preselect: Some(true),
                                        insert_text_format: Some(
                                        InsertTextFormat::PLAIN_TEXT,
                                        ),
                                        insert_text_mode: None,
                                        text_edit: Some(
                                        CompletionTextEdit::InsertAndReplace(
                                        InsertReplaceEdit {
                                        new_text: "Paragraph".to_string(),
                                        insert: lsp_types::Range {
                                        start: completion
                                        .text_document_position
                                        .position,
                                        end: completion
                                        .text_document_position
                                        .position,
                                        },
                                        replace: lsp_types::Range {
                                        start: Position {
                                        line: completion_range
                                        .start_point
                                        .row
                                        as u32,
                                        character:
                                        completion_range
                                        .start_point
                                        .column
                                        as u32,
                                        },
                                        end: Position {
                                        line: completion_range
                                        .end_point
                                        .row
                                        as u32,
                                        character:
                                        completion_range
                                        .end_point
                                        .column
                                        as u32,
                                        },
                                        },
                                        },
                                        ),
                                        ),
                                        additional_text_edits: Some(Vec::new()),
                                        ..Default::default()
                                        },
                                        CompletionItem {
                                        label: "Header".to_string(),
                                        kind: Some(CompletionItemKind::TYPE_PARAMETER),
                                        deprecated: Some(false),
                                        preselect: Some(true),
                                        insert_text_format: Some(
                                        InsertTextFormat::PLAIN_TEXT,
                                        ),
                                        insert_text_mode: None,
                                        text_edit: Some(
                                        CompletionTextEdit::InsertAndReplace(
                                        InsertReplaceEdit {
                                        new_text: "Header".to_string(),
                                        insert: lsp_types::Range {
                                        start: completion
                                        .text_document_position
                                        .position,
                                        end: completion
                                        .text_document_position
                                        .position,
                                        },
                                        replace: lsp_types::Range {
                                        start: Position {
                                        line: completion_range
                                        .start_point
                                        .row
                                        as u32,
                                        character:
                                        completion_range
                                        .start_point
                                        .column
                                        as u32,
                                        },
                                        end: Position {
                                        line: completion_range
                                        .end_point
                                        .row
                                        as u32,
                                        character:
                                        completion_range
                                        .end_point
                                        .column
                                        as u32,
                                        },
                                        },
                                        },
                                        ),
                                        ),
                                        additional_text_edits: Some(Vec::new()),
                                        ..Default::default()
                                        },
                                        CompletionItem {
                                        label: "Rect".to_string(),
                                        kind: Some(CompletionItemKind::TYPE_PARAMETER),
                                        deprecated: Some(false),
                                        preselect: Some(true),
                                        insert_text_format: Some(
                                        InsertTextFormat::PLAIN_TEXT,
                                        ),
                                        insert_text_mode: None,
                                        text_edit: Some(
                                        CompletionTextEdit::InsertAndReplace(
                                        InsertReplaceEdit {
                                        new_text: "Rect".to_string(),
                                        insert: lsp_types::Range {
                                        start: completion
                                        .text_document_position
                                        .position,
                                        end: completion
                                        .text_document_position
                                        .position,
                                        },
                                        replace: lsp_types::Range {
                                        start: Position {
                                        line: completion_range
                                        .start_point
                                        .row
                                        as u32,
                                        character:
                                        completion_range
                                        .start_point
                                        .column
                                        as u32,
                                        },
                                        end: Position {
                                        line: completion_range
                                        .end_point
                                        .row
                                        as u32,
                                        character:
                                        completion_range
                                        .end_point
                                        .column
                                        as u32,
                                        },
                                        },
                                        },
                                        ),
                                        ),
                                        additional_text_edits: Some(Vec::new()),
                                        ..Default::default()
                                        },
                                        CompletionItem {
                                        label: "Image".to_string(),
                                        kind: Some(CompletionItemKind::TYPE_PARAMETER),
                                        deprecated: Some(false),
                                        preselect: Some(true),
                                        insert_text_format: Some(
                                        InsertTextFormat::PLAIN_TEXT,
                                        ),
                                        insert_text_mode: None,
                                        text_edit: Some(
                                        CompletionTextEdit::InsertAndReplace(
                                        InsertReplaceEdit {
                                        new_text: "Image".to_string(),
                                        insert: lsp_types::Range {
                                        start: completion
                                        .text_document_position
                                        .position,
                                        end: completion
                                        .text_document_position
                                        .position,
                                        },
                                        replace: lsp_types::Range {
                                        start: Position {
                                        line: completion_range
                                        .start_point
                                        .row
                                        as u32,
                                        character:
                                        completion_range
                                        .start_point
                                        .column
                                        as u32,
                                        },
                                        end: Position {
                                        line: completion_range
                                        .end_point
                                        .row
                                        as u32,
                                        character:
                                        completion_range
                                        .end_point
                                        .column
                                        as u32,
                                        },
                                        },
                                        },
                                        ),
                                        ),
                                        additional_text_edits: Some(Vec::new()),
                                        ..Default::default()
                                        }])),
                                                )))
                                                .unwrap();
                                        }
                                        NodeKind::ObjInner => {
                                            let completion_range = completion_node.range();
                                            connection
                                                .sender
                                                .send(Message::Response(Response::new_ok(
                                                    rqid,
                                                    Some(CompletionResponse::Array(vec![
                                                        CompletionItem {
                                                            label: "value".to_string(),
                                                            kind: Some(CompletionItemKind::TYPE_PARAMETER),
                                                            deprecated: Some(false),
                                                            preselect: Some(true),
                                                            insert_text_format: Some(
                                                                InsertTextFormat::SNIPPET,
                                                            ),
                                                            insert_text_mode: None,
                                                            text_edit: Some(
                                                                CompletionTextEdit::InsertAndReplace(
                                                                    InsertReplaceEdit {
                                                                        new_text: "value: \"$0\",".to_string(),
                                                                        insert: lsp_types::Range {
                                                                            start: completion
                                                                                .text_document_position
                                                                                .position,
                                                                            end: completion
                                                                                .text_document_position
                                                                                .position,
                                                                        },
                                                                        replace: lsp_types::Range {
                                                                            start: Position {
                                                                                line: completion_range
                                                                                    .start_point
                                                                                    .row
                                                                                    as u32,
                                                                                character:
                                                                                    completion_range
                                                                                        .start_point
                                                                                        .column
                                                                                        as u32,
                                                                            },
                                                                            end: Position {
                                                                                line: completion_range
                                                                                    .end_point
                                                                                    .row
                                                                                    as u32,
                                                                                character:
                                                                                    completion_range
                                                                                        .end_point
                                                                                        .column
                                                                                        as u32,
                                                                            },
                                                                        },
                                                                    },
                                                                ),
                                                            ),
                                                            additional_text_edits: Some(Vec::new()),
                                                            ..Default::default()
                                                        },
                                                        CompletionItem {
                                                            label: "height".to_string(),
                                                            kind: Some(CompletionItemKind::TYPE_PARAMETER),
                                                            deprecated: Some(false),
                                                            preselect: Some(true),
                                                            insert_text_format: Some(
                                                                InsertTextFormat::SNIPPET,
                                                            ),
                                                            insert_text_mode: None,
                                                            text_edit: Some(
                                                                CompletionTextEdit::InsertAndReplace(
                                                                    InsertReplaceEdit {
                                                                        new_text: "height: \"$0\",".to_string(),
                                                                        insert: lsp_types::Range {
                                                                            start: completion
                                                                                .text_document_position
                                                                                .position,
                                                                            end: completion
                                                                                .text_document_position
                                                                                .position,
                                                                        },
                                                                        replace: lsp_types::Range {
                                                                            start: Position {
                                                                                line: completion_range
                                                                                    .start_point
                                                                                    .row
                                                                                    as u32,
                                                                                character:
                                                                                    completion_range
                                                                                        .start_point
                                                                                        .column
                                                                                        as u32,
                                                                            },
                                                                            end: Position {
                                                                                line: completion_range
                                                                                    .end_point
                                                                                    .row
                                                                                    as u32,
                                                                                character:
                                                                                    completion_range
                                                                                        .end_point
                                                                                        .column
                                                                                        as u32,
                                                                            },
                                                                        },
                                                                    },
                                                                ),
                                                            ),
                                                            additional_text_edits: Some(Vec::new()),
                                                            ..Default::default()
                                                        },
                                                        CompletionItem {
                                                            label: "code".to_string(),
                                                            kind: Some(CompletionItemKind::TYPE_PARAMETER),
                                                            deprecated: Some(false),
                                                            preselect: Some(true),
                                                            insert_text_format: Some(
                                                                InsertTextFormat::SNIPPET,
                                                            ),
                                                            insert_text_mode: None,
                                                            text_edit: Some(
                                                                CompletionTextEdit::InsertAndReplace(
                                                                    InsertReplaceEdit {
                                                                        new_text: "code: r#\"$0\"#,".to_string(),
                                                                        insert: lsp_types::Range {
                                                                            start: completion
                                                                                .text_document_position
                                                                                .position,
                                                                            end: completion
                                                                                .text_document_position
                                                                                .position,
                                                                        },
                                                                        replace: lsp_types::Range {
                                                                            start: Position {
                                                                                line: completion_range
                                                                                    .start_point
                                                                                    .row
                                                                                    as u32,
                                                                                character:
                                                                                    completion_range
                                                                                        .start_point
                                                                                        .column
                                                                                        as u32,
                                                                            },
                                                                            end: Position {
                                                                                line: completion_range
                                                                                    .end_point
                                                                                    .row
                                                                                    as u32,
                                                                                character:
                                                                                    completion_range
                                                                                        .end_point
                                                                                        .column
                                                                                        as u32,
                                                                            },
                                                                        },
                                                                    },
                                                                ),
                                                            ),
                                                            additional_text_edits: Some(Vec::new()),
                                                            ..Default::default()
                                                        },
                                                        CompletionItem {
                                                            label: "tint".to_string(),
                                                            kind: Some(CompletionItemKind::TYPE_PARAMETER),
                                                            deprecated: Some(false),
                                                            preselect: Some(true),
                                                            insert_text_format: Some(
                                                                InsertTextFormat::SNIPPET,
                                                            ),
                                                            insert_text_mode: None,
                                                            text_edit: Some(
                                                                CompletionTextEdit::InsertAndReplace(
                                                                    InsertReplaceEdit {
                                                                        new_text: "tint: $0,".to_string(),
                                                                        insert: lsp_types::Range {
                                                                            start: completion
                                                                                .text_document_position
                                                                                .position,
                                                                            end: completion
                                                                                .text_document_position
                                                                                .position,
                                                                        },
                                                                        replace: lsp_types::Range {
                                                                            start: Position {
                                                                                line: completion_range
                                                                                    .start_point
                                                                                    .row
                                                                                    as u32,
                                                                                character:
                                                                                    completion_range
                                                                                        .start_point
                                                                                        .column
                                                                                        as u32,
                                                                            },
                                                                            end: Position {
                                                                                line: completion_range
                                                                                    .end_point
                                                                                    .row
                                                                                    as u32,
                                                                                character:
                                                                                    completion_range
                                                                                        .end_point
                                                                                        .column
                                                                                        as u32,
                                                                            },
                                                                        },
                                                                    },
                                                                ),
                                                            ),
                                                            additional_text_edits: Some(Vec::new()),
                                                            ..Default::default()
                                                        },
                                                        CompletionItem {
                                                            label: "color".to_string(),
                                                            kind: Some(CompletionItemKind::TYPE_PARAMETER),
                                                            deprecated: Some(false),
                                                            preselect: Some(true),
                                                            insert_text_format: Some(
                                                                InsertTextFormat::SNIPPET,
                                                            ),
                                                            insert_text_mode: None,
                                                            text_edit: Some(
                                                                CompletionTextEdit::InsertAndReplace(
                                                                    InsertReplaceEdit {
                                                                        new_text: "color: $0,".to_string(),
                                                                        insert: lsp_types::Range {
                                                                            start: completion
                                                                                .text_document_position
                                                                                .position,
                                                                            end: completion
                                                                                .text_document_position
                                                                                .position,
                                                                        },
                                                                        replace: lsp_types::Range {
                                                                            start: Position {
                                                                                line: completion_range
                                                                                    .start_point
                                                                                    .row
                                                                                    as u32,
                                                                                character:
                                                                                    completion_range
                                                                                        .start_point
                                                                                        .column
                                                                                        as u32,
                                                                            },
                                                                            end: Position {
                                                                                line: completion_range
                                                                                    .end_point
                                                                                    .row
                                                                                    as u32,
                                                                                character:
                                                                                    completion_range
                                                                                        .end_point
                                                                                        .column
                                                                                        as u32,
                                                                            },
                                                                        },
                                                                    },
                                                                ),
                                                            ),
                                                            additional_text_edits: Some(Vec::new()),
                                                            ..Default::default()
                                                        },
                                                        CompletionItem {
                                                            label: "background".to_string(),
                                                            kind: Some(CompletionItemKind::TYPE_PARAMETER),
                                                            deprecated: Some(false),
                                                            preselect: Some(true),
                                                            insert_text_format: Some(
                                                                InsertTextFormat::SNIPPET,
                                                            ),
                                                            insert_text_mode: None,
                                                            text_edit: Some(
                                                                CompletionTextEdit::InsertAndReplace(
                                                                    InsertReplaceEdit {
                                                                        new_text: "background: $0,".to_string(),
                                                                        insert: lsp_types::Range {
                                                                            start: completion
                                                                                .text_document_position
                                                                                .position,
                                                                            end: completion
                                                                                .text_document_position
                                                                                .position,
                                                                        },
                                                                        replace: lsp_types::Range {
                                                                            start: Position {
                                                                                line: completion_range
                                                                                    .start_point
                                                                                    .row
                                                                                    as u32,
                                                                                character:
                                                                                    completion_range
                                                                                        .start_point
                                                                                        .column
                                                                                        as u32,
                                                                            },
                                                                            end: Position {
                                                                                line: completion_range
                                                                                    .end_point
                                                                                    .row
                                                                                    as u32,
                                                                                character:
                                                                                    completion_range
                                                                                        .end_point
                                                                                        .column
                                                                                        as u32,
                                                                            },
                                                                        },
                                                                    },
                                                                ),
                                                            ),
                                                            additional_text_edits: Some(Vec::new()),
                                                            ..Default::default()
                                                        },
                                                        CompletionItem {
                                                            label: "scale".to_string(),
                                                            kind: Some(CompletionItemKind::TYPE_PARAMETER),
                                                            deprecated: Some(false),
                                                            preselect: Some(true),
                                                            insert_text_format: Some(
                                                                InsertTextFormat::SNIPPET,
                                                            ),
                                                            insert_text_mode: None,
                                                            text_edit: Some(
                                                                CompletionTextEdit::InsertAndReplace(
                                                                    InsertReplaceEdit {
                                                                        new_text: "scale: \"$0\",".to_string(),
                                                                        insert: lsp_types::Range {
                                                                            start: completion
                                                                                .text_document_position
                                                                                .position,
                                                                            end: completion
                                                                                .text_document_position
                                                                                .position,
                                                                        },
                                                                        replace: lsp_types::Range {
                                                                            start: Position {
                                                                                line: completion_range
                                                                                    .start_point
                                                                                    .row
                                                                                    as u32,
                                                                                character:
                                                                                    completion_range
                                                                                        .start_point
                                                                                        .column
                                                                                        as u32,
                                                                            },
                                                                            end: Position {
                                                                                line: completion_range
                                                                                    .end_point
                                                                                    .row
                                                                                    as u32,
                                                                                character:
                                                                                    completion_range
                                                                                        .end_point
                                                                                        .column
                                                                                        as u32,
                                                                            },
                                                                        },
                                                                    },
                                                                ),
                                                            ),
                                                            additional_text_edits: Some(Vec::new()),
                                                            ..Default::default()
                                                        },
                                                    CompletionItem {
                                                        label: "font_family".to_string(),
                                                        kind: Some(CompletionItemKind::TYPE_PARAMETER),
                                                        deprecated: Some(false),
                                                        preselect: Some(true),
                                                        insert_text_format: Some(
                                                            InsertTextFormat::SNIPPET,
                                                        ),
                                                        insert_text_mode: None,
                                                        text_edit: Some(
                                                            CompletionTextEdit::InsertAndReplace(
                                                                InsertReplaceEdit {
                                                                    new_text: "font_family: \"$0\",".to_string(),
                                                                    insert: lsp_types::Range {
                                                                        start: completion
                                                                            .text_document_position
                                                                            .position,
                                                                        end: completion
                                                                            .text_document_position
                                                                            .position,
                                                                    },
                                                                    replace: lsp_types::Range {
                                                                        start: Position {
                                                                            line: completion_range
                                                                                .start_point
                                                                                .row
                                                                                as u32,
                                                                            character:
                                                                                completion_range
                                                                                    .start_point
                                                                                    .column
                                                                                    as u32,
                                                                        },
                                                                        end: Position {
                                                                            line: completion_range
                                                                                .end_point
                                                                                .row
                                                                                as u32,
                                                                            character:
                                                                                completion_range
                                                                                    .end_point
                                                                                    .column
                                                                                    as u32,
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                        additional_text_edits: Some(Vec::new()),
                                                        ..Default::default()
                                                    },
                                                    CompletionItem {
                                                        label: "source".to_string(),
                                                        kind: Some(CompletionItemKind::TYPE_PARAMETER),
                                                        deprecated: Some(false),
                                                        preselect: Some(true),
                                                        insert_text_format: Some(
                                                            InsertTextFormat::SNIPPET,
                                                        ),
                                                        insert_text_mode: None,
                                                        text_edit: Some(
                                                            CompletionTextEdit::InsertAndReplace(
                                                                InsertReplaceEdit {
                                                                    new_text: "source: \"$0\",".to_string(),
                                                                    insert: lsp_types::Range {
                                                                        start: completion
                                                                            .text_document_position
                                                                            .position,
                                                                        end: completion
                                                                            .text_document_position
                                                                            .position,
                                                                    },
                                                                    replace: lsp_types::Range {
                                                                        start: Position {
                                                                            line: completion_range
                                                                                .start_point
                                                                                .row
                                                                                as u32,
                                                                            character:
                                                                                completion_range
                                                                                    .start_point
                                                                                    .column
                                                                                    as u32,
                                                                        },
                                                                        end: Position {
                                                                            line: completion_range
                                                                                .end_point
                                                                                .row
                                                                                as u32,
                                                                            character:
                                                                                completion_range
                                                                                    .end_point
                                                                                    .column
                                                                                    as u32,
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                        additional_text_edits: Some(Vec::new()),
                                                        ..Default::default()
                                                    },
                                                    CompletionItem {
                                                        label: "font_size".to_string(),
                                                        kind: Some(CompletionItemKind::TYPE_PARAMETER),
                                                        deprecated: Some(false),
                                                        preselect: Some(true),
                                                        insert_text_format: Some(
                                                            InsertTextFormat::SNIPPET,
                                                        ),
                                                        insert_text_mode: None,
                                                        text_edit: Some(
                                                            CompletionTextEdit::InsertAndReplace(
                                                                InsertReplaceEdit {
                                                                    new_text: "font_size: $0,".to_string(),
                                                                    insert: lsp_types::Range {
                                                                        start: completion
                                                                            .text_document_position
                                                                            .position,
                                                                        end: completion
                                                                            .text_document_position
                                                                            .position,
                                                                    },
                                                                    replace: lsp_types::Range {
                                                                        start: Position {
                                                                            line: completion_range
                                                                                .start_point
                                                                                .row
                                                                                as u32,
                                                                            character:
                                                                                completion_range
                                                                                    .start_point
                                                                                    .column
                                                                                    as u32,
                                                                        },
                                                                        end: Position {
                                                                            line: completion_range
                                                                                .end_point
                                                                                .row
                                                                                as u32,
                                                                            character:
                                                                                completion_range
                                                                                    .end_point
                                                                                    .column
                                                                                    as u32,
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                        additional_text_edits: Some(Vec::new()),
                                                        ..Default::default()
                                                    },
                                                    CompletionItem {
                                                        label: "language".to_string(),
                                                        kind: Some(CompletionItemKind::TYPE_PARAMETER),
                                                        deprecated: Some(false),
                                                        preselect: Some(true),
                                                        insert_text_format: Some(
                                                            InsertTextFormat::SNIPPET,
                                                        ),
                                                        insert_text_mode: None,
                                                        text_edit: Some(
                                                            CompletionTextEdit::InsertAndReplace(
                                                                InsertReplaceEdit {
                                                                    new_text: "language: \"$0\",".to_string(),
                                                                    insert: lsp_types::Range {
                                                                        start: completion
                                                                            .text_document_position
                                                                            .position,
                                                                        end: completion
                                                                            .text_document_position
                                                                            .position,
                                                                    },
                                                                    replace: lsp_types::Range {
                                                                        start: Position {
                                                                            line: completion_range
                                                                                .start_point
                                                                                .row
                                                                                as u32,
                                                                            character:
                                                                                completion_range
                                                                                    .start_point
                                                                                    .column
                                                                                    as u32,
                                                                        },
                                                                        end: Position {
                                                                            line: completion_range
                                                                                .end_point
                                                                                .row
                                                                                as u32,
                                                                            character:
                                                                                completion_range
                                                                                    .end_point
                                                                                    .column
                                                                                    as u32,
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                        additional_text_edits: Some(Vec::new()),
                                                        ..Default::default()
                                                    },
                                                    CompletionItem {
                                                        label: "align".to_string(),
                                                        kind: Some(CompletionItemKind::TYPE_PARAMETER),
                                                        deprecated: Some(false),
                                                        preselect: Some(true),
                                                        insert_text_format: Some(
                                                            InsertTextFormat::SNIPPET,
                                                        ),
                                                        insert_text_mode: None,
                                                        text_edit: Some(
                                                            CompletionTextEdit::InsertAndReplace(
                                                                InsertReplaceEdit {
                                                                    new_text: "align: \"$0\",".to_string(),
                                                                    insert: lsp_types::Range {
                                                                        start: completion
                                                                            .text_document_position
                                                                            .position,
                                                                        end: completion
                                                                            .text_document_position
                                                                            .position,
                                                                    },
                                                                    replace: lsp_types::Range {
                                                                        start: Position {
                                                                            line: completion_range
                                                                                .start_point
                                                                                .row
                                                                                as u32,
                                                                            character:
                                                                                completion_range
                                                                                    .start_point
                                                                                    .column
                                                                                    as u32,
                                                                        },
                                                                        end: Position {
                                                                            line: completion_range
                                                                                .end_point
                                                                                .row
                                                                                as u32,
                                                                            character:
                                                                                completion_range
                                                                                    .end_point
                                                                                    .column
                                                                                    as u32,
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                        additional_text_edits: Some(Vec::new()),
                                                        ..Default::default()
                                                    }]))),
                                                ))
                                                .unwrap();
                                        }
                                        NodeKind::Completion => {
                                            let completion_range = parent_object.range();
                                            connection
                                                .sender
                                                .send(Message::Response(Response::new_ok(
                                                    rqid,
                                                    Some(CompletionResponse::Array(vec![
                                                        CompletionItem {
                                                            label: "viewbox".to_string(),
                                                            kind: Some(CompletionItemKind::SNIPPET),
                                                            deprecated: Some(false),
                                                            preselect: Some(true),
                                                            insert_text_format: Some(
                                                                InsertTextFormat::SNIPPET,
                                                            ),
                                                            insert_text_mode: None,
                                                            text_edit: Some(
                                                                CompletionTextEdit::InsertAndReplace(
                                                                    InsertReplaceEdit {
                                                                        new_text: format!("{}: ${{1:Size}}[0] >$0]", current_rope.byte_slice(completion_node.prev_named_sibling().unwrap().byte_range())),
                                                                        insert: lsp_types::Range {
                                                                            start: completion
                                                                                .text_document_position
                                                                                .position,
                                                                            end: completion
                                                                                .text_document_position
                                                                                .position,
                                                                        },
                                                                        replace: lsp_types::Range {
                                                                            start: Position {
                                                                                line: completion_range
                                                                                    .start_point
                                                                                    .row
                                                                                    as u32,
                                                                                character:
                                                                                    completion_range
                                                                                        .start_point
                                                                                        .column
                                                                                        as u32,
                                                                            },
                                                                            end: Position {
                                                                                line: completion_range
                                                                                    .end_point
                                                                                    .row
                                                                                    as u32,
                                                                                character:
                                                                                    completion_range
                                                                                        .end_point
                                                                                        .column
                                                                                        as u32,
                                                                            },
                                                                        },
                                                                    },
                                                                ),
                                                            ),
                                                            additional_text_edits: Some(Vec::new()),
                                                            ..Default::default()
                                                        },
                                                    CompletionItem {
                                                        label: "object".to_string(),
                                                        kind: Some(CompletionItemKind::SNIPPET),
                                                        deprecated: Some(false),
                                                        preselect: Some(true),
                                                        insert_text_format: Some(
                                                            InsertTextFormat::SNIPPET,
                                                        ),
                                                        insert_text_mode: None,
                                                        text_edit: Some(
                                                            CompletionTextEdit::InsertAndReplace(
                                                                InsertReplaceEdit {
                                                                    new_text: format!("{}: ${{1:Paragraph}}($0)", current_rope.byte_slice(completion_node.prev_named_sibling().unwrap().byte_range())),
                                                                    insert: lsp_types::Range {
                                                                        start: completion
                                                                            .text_document_position
                                                                            .position,
                                                                        end: completion
                                                                            .text_document_position
                                                                            .position,
                                                                    },
                                                                    replace: lsp_types::Range {
                                                                        start: Position {
                                                                            line: completion_range
                                                                                .start_point
                                                                                .row
                                                                                as u32,
                                                                            character:
                                                                                completion_range
                                                                                    .start_point
                                                                                    .column
                                                                                    as u32,
                                                                        },
                                                                        end: Position {
                                                                            line: completion_range
                                                                                .end_point
                                                                                .row
                                                                                as u32,
                                                                            character:
                                                                                completion_range
                                                                                    .end_point
                                                                                    .column
                                                                                    as u32,
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                        additional_text_edits: Some(Vec::new()),
                                                        ..Default::default()
                                                    }]))),
                                                ))
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
                                            complete_source_file(
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
                                                    &current_rope.slice(c.byte_range())
                                                        == "font_family"
                                                })
                                                .unwrap_or_default()
                                        {
                                            let fonts = font_strings
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
                                                                replace: lsp_types::Range {
                                                                    start: Position {
                                                                        line: completion_range
                                                                            .start_point
                                                                            .row
                                                                            as u32,
                                                                        character: completion_range
                                                                            .start_point
                                                                            .column
                                                                            as u32,
                                                                    },
                                                                    end: Position {
                                                                        line: completion_range
                                                                            .end_point
                                                                            .row
                                                                            as u32,
                                                                        character: completion_range
                                                                            .end_point
                                                                            .column
                                                                            as u32,
                                                                    },
                                                                },
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
                                    goto_declaration(
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
                                        &font_strings,
                                        &mut app.sources,
                                        &mut app.fonts,
                                        &lsp_egui_ctx,
                                        Path::new(currently_open.path()),
                                    );
                                    *tree_info = Some(tree);
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

                                    lsp_egui_ctx.set_fonts(app.fonts.clone());
                                    app.clear_resolved.store(true, Ordering::Relaxed);
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
                                        let parser = pulldown_cmark::Parser::new(source.as_ref());

                                        for event in parser {
                                            match event {
                                                pulldown_cmark::Event::Text(t) => {
                                                    for text in t.split_whitespace() {
                                                        let text = text.trim_matches(|c: char| {
                                                            c.is_ascii_punctuation()
                                                        });
                                                        if hunspell.check(text)
                                                            == CheckResult::MissingInDictionary
                                                        {
                                                            warnings.push(
                                                                super::parser::Error::SpellCheck(
                                                                    query_match.captures[0]
                                                                        .node
                                                                        .range()
                                                                        .into(),
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
                                    references(
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
                            &font_strings,
                            &mut app.sources,
                            &mut app.fonts,
                            &lsp_egui_ctx,
                            Path::new(currently_open.path()),
                        );
                        *tree_info = Some(tree);
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

                        lsp_egui_ctx.set_fonts(app.fonts.clone());
                        app.clear_resolved.store(true, Ordering::Relaxed);
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
                                            Some(&*tree_info),
                                        )
                                        .unwrap();
                                    if !tree.root_node().has_error() {
                                        let mut slide_show = app.slide_show.write();

                                        match super::parser::parse_file(
                                            &tree,
                                            Some(error_tree.take().as_ref().unwrap_or(&*tree_info)),
                                            &current_rope,
                                            &mut app.helix_cell,
                                            &mut slide_show,
                                            &font_strings,
                                            &mut app.sources,
                                            &mut app.fonts,
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
                                                                version: Some(
                                                                    current_document_version,
                                                                ),
                                                            },
                                                        ),
                                                    ))
                                                    .unwrap();
                                                app.clear_resolved.store(true, Ordering::Relaxed);
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
                                    } else if error_tree.is_none() {
                                        error_tree = Some(tree.clone());
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
                                &font_strings,
                                &mut app.sources,
                                &mut app.fonts,
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
                                    app.clear_resolved.store(true, Ordering::Relaxed);
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

fn prepare_rename(
    app: &MyEguiApp,
    pos: TextDocumentPositionParams,
) -> Option<PrepareRenameResponse> {
    let tree_info = app.tree_info.lock();
    let tree_info = tree_info.as_ref().unwrap();
    let point = Point {
        row: pos.position.line as usize,
        column: pos.position.character as usize,
    };

    tree_info
        .root_node()
        .descendant_for_point_range(point, point)
        .and_then(|f| {
            if matches!(NodeKind::from(f.kind_id()), NodeKind::Identifier) {
                let node_range = f.range();
                Some(PrepareRenameResponse::Range(lsp_types::Range {
                    start: Position {
                        line: node_range.start_point.row as u32,
                        character: node_range.start_point.column as u32,
                    },
                    end: Position {
                        line: node_range.end_point.row as u32,
                        character: node_range.end_point.column as u32,
                    },
                }))
            } else {
                None
            }
        })
}

fn rename(
    app: &MyEguiApp,
    rename: RenameParams,
    current_rope: &Rope,
    rename_query: &Query,
    query_cursor: &mut QueryCursor,
) -> Vec<OneOf<TextEdit, AnnotatedTextEdit>> {
    let tree_info = app.tree_info.lock();
    let tree_info = tree_info.as_ref().unwrap();
    let mut workspace_edit: Vec<OneOf<TextEdit, AnnotatedTextEdit>> = Vec::new();
    let point = Point {
        row: rename.text_document_position.position.line as usize,
        column: rename.text_document_position.position.character as usize,
    };

    let rename_node = tree_info
        .root_node()
        .descendant_for_point_range(point, point)
        .unwrap();

    // identifiers cannot have new lines, so this should work
    let rename_name = current_rope.byte_slice(rename_node.byte_range());

    query_cursor.set_point_range(
        Point { row: 0, column: 0 }..Point {
            row: usize::MAX,
            column: usize::MAX,
        },
    );
    let iter = query_cursor.matches(
        rename_query,
        tree_info.root_node(),
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

    workspace_edit
}

fn inlay_hints(
    app: &MyEguiApp,
    inlay_edge_query: &Query,
    inlay_edge_map: &mut HashMap<
        RopeSlice<'static>,
        RopeSlice<'static>,
        BuildHasherDefault<ahash::AHasher>,
    >,
    inlay_vb_map: &mut HashMap<
        RopeSlice<'static>,
        Cow<'_, str>,
        BuildHasherDefault<ahash::AHasher>,
    >,
    current_rope: &Rope,
    last_inlay_len: usize,
    query_cursor: &mut QueryCursor,
) -> Vec<InlayHint> {
    let tree_info = app.tree_info.lock();
    let tree_info = tree_info.as_ref().unwrap();

    let mut hints = Vec::with_capacity(last_inlay_len);

    query_cursor.set_point_range(
        Point { row: 0, column: 0 }..Point {
            row: usize::MAX,
            column: usize::MAX,
        },
    );

    let mut slide_num = 0;

    let edge_iter = query_cursor.matches(
        inlay_edge_query,
        tree_info.root_node(),
        RopeProvider(current_rope.slice(..)),
    );

    for query_match in edge_iter {
        match query_match.pattern_index {
            0 => {
                let query_node = query_match.captures[0].node;
                let query_slice = unsafe {
                    you_can::borrow_unchecked(
                        current_rope.byte_slice(query_match.captures[0].node.byte_range()),
                    )
                };

                let mut vb = None;
                let mut edge = query_match.captures.get(2).map(|capture| capture.node);

                if let Some(v) = query_match.captures.get(1) {
                    match NodeKind::from(v.node.kind_id()) {
                        NodeKind::SlideVb => vb = Some(v.node),
                        NodeKind::EdgeParser => edge = Some(v.node),
                        _ => unreachable!(),
                    }
                }

                let mut walker = query_node.parent().unwrap().walk();
                while walker.goto_next_sibling() {}
                let range = walker.node().range();
                let mut position = Position {
                    line: range.end_point.row as u32,
                    character: range.end_point.column as u32,
                };
                let mut hint = String::new();

                if let Some(vb) = vb {
                    let mut vb: Cow<'_, str> = unsafe {
                        you_can::borrow_unchecked(current_rope.byte_slice(vb.byte_range()))
                    }
                    .into();
                    if vb.starts_with('|') {
                        vb = Cow::Borrowed(": InlineVb[_]");
                        inlay_vb_map.insert(query_slice, vb);
                    } else if vb.starts_with('~') {
                        if let Some(vb) = inlay_vb_map.get(&query_slice) {
                            std::fmt::Write::write_fmt(&mut hint, format_args!("{}", &vb[1..]))
                                .unwrap();
                        }
                    } else {
                        inlay_vb_map.insert(query_slice, vb);
                    }
                } else {
                    let entry = inlay_vb_map
                        .entry(query_slice)
                        .or_insert(Cow::Borrowed(": Unknown[_]"));

                    std::fmt::Write::write_fmt(&mut hint, format_args!("{}", entry)).unwrap();
                }

                if let Some(edge) = edge {
                    let slice = unsafe {
                        you_can::borrow_unchecked(current_rope.byte_slice(edge.byte_range()))
                    };
                    let entry = inlay_edge_map
                        .entry(query_slice)
                        .or_insert_with(|| {
                            if edge.byte_range().len() == 4 {
                                slice.byte_slice(2..)
                            } else {
                                slice
                            }
                        });

                    let range = edge.range();
                    position = Position {
                        line: range.start_point.row as u32,
                        character: range.start_point.column as u32,
                    };

                    if slice.len_chars() < 3 {
                        std::fmt::Write::write_fmt(&mut hint, format_args!("{}", entry)).unwrap();
                        *entry = slice;
                    } else {
                        *entry = slice.byte_slice(2..);
                    }
                } else {
                    let entry = inlay_edge_map.entry(query_slice);
                    let entry = entry.or_insert_with(|| RopeSlice::from(""));

                    std::fmt::Write::write_fmt(&mut hint, format_args!("{}{}", entry, entry))
                        .unwrap();
                }

                if !hint.is_empty() {
                    hints.push(InlayHint {
                        position,
                        // This parameter must NEVER be the actual borrowed slice
                        label: InlayHintLabel::String(hint),
                        kind: Some(InlayHintKind::PARAMETER),
                        text_edits: None,
                        tooltip: None,
                        padding_right: Some(false),
                        padding_left: Some(false),
                        data: None,
                    });
                }
            }
            1 => {
                slide_num += 1;
                let range = query_match.captures[0].node.range();
                hints.push(InlayHint {
                    position: Position {
                        line: range.start_point.row as u32,
                        character: range.start_point.column as u32,
                    },
                    label: InlayHintLabel::String(format!("Slide {}:", slide_num)),
                    kind: Some(InlayHintKind::PARAMETER),
                    text_edits: None,
                    tooltip: None,
                    padding_right: Some(true),
                    padding_left: Some(false),
                    data: None,
                });
            }
            _ => unreachable!(),
        }
    }

    {
        inlay_edge_map.clear();
        inlay_vb_map.clear();
    }

    hints
}

fn semantic_tokens(
    app: &MyEguiApp,
    semantic_token_query: &Query,
    current_rope: &Rope,
    query_cursor: &mut QueryCursor,
) -> SemanticTokensResult {
    let tree_info = app.tree_info.lock();
    let tree_info = tree_info.as_ref().unwrap();

    let start_node = tree_info.root_node();

    query_cursor.set_point_range(
        Point { row: 0, column: 0 }..Point {
            row: usize::MAX,
            column: usize::MAX,
        },
    );
    let iter = query_cursor.matches(
        semantic_token_query,
        start_node,
        RopeProvider(current_rope.slice(..)),
    );

    let mut tokens = Vec::new();
    let mut last_range = helix_core::tree_sitter::Range {
        start_byte: 0,
        end_byte: 0,
        start_point: Point { row: 0, column: 0 },
        end_point: Point { row: 0, column: 0 },
    };
    for query_match in iter {
        let capture = query_match.captures.last().unwrap();
        let range = capture.node.range();

        if last_range != range {
            let mut delta_line = (range.start_point.row - last_range.end_point.row) as u32;
            let mut multiline = false;
            for line in range.start_point.row..=range.end_point.row {
                tokens.push(SemanticToken {
                    delta_line,
                    delta_start: if multiline {
                        0
                    } else if delta_line == 0 {
                        (range.start_point.column - last_range.start_point.column) as u32
                    } else {
                        range.start_point.column as u32
                    },
                    length: if line == range.start_point.row {
                        if range.end_point.row - range.start_point.row > 0 {
                            current_rope
                                .line(line)
                                .slice(range.start_point.column..)
                                .len_chars() as u32
                        } else {
                            (range.end_point.column - range.start_point.column) as u32
                        }
                    } else if line == range.end_point.row {
                        current_rope
                            .line(line)
                            .slice(..range.end_point.column)
                            .len_chars() as u32
                    } else {
                        current_rope.line(line).len_chars() as u32
                    },
                    token_type: capture.index,
                    token_modifiers_bitset: if semantic_token_query.capture_names()
                        [capture.index as usize]
                        .contains('.')
                    {
                        0b00000001
                    } else {
                        0
                    },
                });
                delta_line = 1;
                multiline = true;
            }

            last_range = range;
        }
    }

    SemanticTokensResult::Tokens(SemanticTokens {
        data: tokens,
        ..Default::default()
    })
}

pub fn complete_source_file(
    completion: CompletionParams,
    slide_complete_query: &Query,
    tree_info: &Tree,
    current_rope: &Rope,
    query_cursor: &mut QueryCursor,
) -> Result<CompletionResponse, Error> {
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
        text_edit: Some(CompletionTextEdit::InsertAndReplace(InsertReplaceEdit {
            new_text: "{$0}[]".to_string(),
            insert: new_slide_range,
            replace: new_slide_range,
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
        GrzCursor::from_node(n)
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
        text_edit: Some(CompletionTextEdit::InsertAndReplace(InsertReplaceEdit {
            new_text,
            insert: continue_slide_range,
            replace: continue_slide_range,
        })),
        additional_text_edits: Some(Vec::new()),
        ..Default::default()
    };

    Ok(CompletionResponse::Array(vec![
        continue_slide_item,
        new_slide_item,
    ]))
}

pub fn references(
    app: &MyEguiApp,
    rename_query: &Query,
    current_rope: &Rope,
    references: ReferenceParams,
    currently_open: &Url,
    query_cursor: &mut QueryCursor,
) -> Option<Vec<Location>> {
    let tree_info = app.tree_info.lock();
    let tree_info = tree_info.as_ref().unwrap();
    let point = Point {
        row: references.text_document_position.position.line as usize,
        column: references.text_document_position.position.character as usize,
    };

    let reference_node = tree_info
        .root_node()
        .descendant_for_point_range(point, point)
        .unwrap();

    query_cursor.set_point_range(
        Point { row: 0, column: 0 }..Point {
            row: usize::MAX,
            column: usize::MAX,
        },
    );
    let iter = query_cursor.matches(
        rename_query,
        tree_info.root_node(),
        RopeProvider(current_rope.slice(..)),
    );

    let mut locations = Vec::new();

    for query_match in iter {
        if current_rope.byte_slice(query_match.captures[0].node.byte_range())
            == current_rope.byte_slice(reference_node.byte_range())
        {
            let range = query_match.captures[0].node.range();
            locations.push(Location {
                uri: currently_open.clone(),
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
            });
        }
    }

    Some(locations)
}

pub fn goto_declaration(
    app: &MyEguiApp,
    goto_declaration: GotoDefinitionParams,
    top_level_search_query: &Query,
    current_rope: &Rope,
    currently_open: Url,
    query_cursor: &mut QueryCursor,
) -> Option<GotoDefinitionResponse> {
    let tree_info = app.tree_info.lock();
    let tree_info = tree_info.as_ref().unwrap();
    let point = Point {
        row: goto_declaration.text_document_position_params.position.line as usize,
        column: goto_declaration
            .text_document_position_params
            .position
            .character as usize,
    };

    let usage_node = tree_info
        .root_node()
        .descendant_for_point_range(point, point)
        .unwrap();

    query_cursor.set_point_range(
        Point { row: 0, column: 0 }..Point {
            row: usize::MAX,
            column: usize::MAX,
        },
    );
    let iter = query_cursor.matches(
        top_level_search_query,
        tree_info.root_node(),
        RopeProvider(current_rope.slice(..)),
    );

    for query_match in iter {
        if current_rope.byte_slice(query_match.captures[0].node.byte_range())
            == current_rope.byte_slice(usage_node.byte_range())
        {
            let range = query_match.captures[0].node.range();
            return Some(GotoDefinitionResponse::Scalar(Location {
                uri: currently_open,
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
            }));
        }
    }

    None
}

#[allow(deprecated)]
pub fn document_symbols(app: &MyEguiApp, current_rope: &Rope) -> Option<DocumentSymbolResponse> {
    let tree_info = app.tree_info.lock();
    let tree_info = tree_info.as_ref().unwrap();

    let mut tree_cursor = GrzCursor::new(tree_info);
    let mut symbols = Vec::new();

    let _ = tree_cursor.goto_first_child();
    let mut slide_num = 0;
    'parserloop: loop {
        let node = tree_cursor.node();
        let range = node.range();

        match NodeKind::from(node.kind_id()) {
            NodeKind::Slide => {
                slide_num += 1;

                let _ = tree_cursor.goto_first_child();
                let _ = tree_cursor.goto_first_child();
                let selection_range = tree_cursor.node().range();
                tree_cursor.goto_parent();
                tree_cursor.goto_parent();

                symbols.push(DocumentSymbol {
                    name: format!("Slide {}", slide_num),
                    kind: SymbolKind::FUNCTION,
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
                    detail: None,
                    selection_range: lsp_types::Range {
                        start: Position {
                            line: selection_range.start_point.row as u32,
                            character: selection_range.start_point.column as u32,
                        },
                        end: Position {
                            line: selection_range.end_point.row as u32,
                            character: selection_range.end_point.column as u32,
                        },
                    },
                    tags: None,
                    deprecated: None,
                    children: None,
                })
            }
            NodeKind::Viewbox => {
                let _ = tree_cursor.goto_first_child();
                let selection_range = tree_cursor.node().range();
                let byte_range = tree_cursor.node().byte_range();
                let _ = tree_cursor.goto_next_sibling();
                let name_range = tree_cursor.node().byte_range();
                let _ = tree_cursor.goto_next_sibling();
                let index_range = tree_cursor.node().byte_range();
                tree_cursor.goto_parent();

                symbols.push(DocumentSymbol {
                    name: current_rope.byte_slice(byte_range).to_string(),
                    kind: SymbolKind::VARIABLE,
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
                    detail: Some(format!(
                        "{}{}",
                        current_rope.slice(name_range),
                        current_rope.slice(index_range)
                    )),
                    selection_range: lsp_types::Range {
                        start: Position {
                            line: selection_range.start_point.row as u32,
                            character: selection_range.start_point.column as u32,
                        },
                        end: Position {
                            line: selection_range.end_point.row as u32,
                            character: selection_range.end_point.column as u32,
                        },
                    },
                    tags: None,
                    deprecated: None,
                    children: None,
                })
            }
            NodeKind::Obj => {
                let _ = tree_cursor.goto_first_child();
                let selection_range = tree_cursor.node().range();
                let byte_range = tree_cursor.node().byte_range();
                let _ = tree_cursor.goto_next_sibling();
                let name_range = tree_cursor.node().byte_range();
                tree_cursor.goto_parent();

                symbols.push(DocumentSymbol {
                    name: current_rope.byte_slice(byte_range).to_string(),
                    kind: SymbolKind::OBJECT,
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
                    detail: Some(current_rope.slice(name_range).to_string()),
                    selection_range: lsp_types::Range {
                        start: Position {
                            line: selection_range.start_point.row as u32,
                            character: selection_range.start_point.column as u32,
                        },
                        end: Position {
                            line: selection_range.end_point.row as u32,
                            character: selection_range.end_point.column as u32,
                        },
                    },
                    tags: None,
                    deprecated: None,
                    children: None,
                })
            }
            NodeKind::Register => { /* todo */ }
            NodeKind::SlideFunctions => {
                let _ = tree_cursor.goto_first_child();
                let selection_range = tree_cursor.node().range();
                tree_cursor.goto_parent();

                symbols.push(DocumentSymbol {
                    name: "Actions".to_string(),
                    kind: SymbolKind::ARRAY,
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
                    detail: None,
                    selection_range: lsp_types::Range {
                        start: Position {
                            line: selection_range.start_point.row as u32,
                            character: selection_range.start_point.column as u32,
                        },
                        end: Position {
                            line: selection_range.end_point.row as u32,
                            character: selection_range.end_point.column as u32,
                        },
                    },
                    tags: None,
                    deprecated: None,
                    children: None,
                })
            }
            _ => {}
        }

        loop {
            match tree_cursor.goto_next_sibling() {
                Ok(false) => break 'parserloop,
                Ok(true) => break,
                Err(_) => {}
            }
        }
    }

    Some(DocumentSymbolResponse::Nested(symbols))
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
                    app.clear_resolved.store(true, Ordering::Relaxed);
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
                            app.obj_dbg.store(0, Ordering::Relaxed);

                            app.clear_resolved.store(true, Ordering::Relaxed);
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
                        "__viewbox__".hash(&mut hasher);
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
                                app.clear_resolved.store(true, Ordering::Relaxed);
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

                            app.clear_resolved.store(true, Ordering::Relaxed);
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
