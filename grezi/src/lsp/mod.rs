use std::{
    collections::HashMap,
    hash::Hash,
    hash::{BuildHasher, BuildHasherDefault, Hasher},
    process::Stdio,
    sync::atomic::Ordering,
};

mod you_can;

use crate::{
    parser::{Error, FieldName, GrzCursor, NodeKind},
    MyEguiApp, SlideShow,
};
use helix_core::ropey::{Rope, RopeSlice};
use helix_core::syntax::RopeProvider;
use helix_core::tree_sitter::{Node, Point, Query, QueryCursor, Tree, TreeCursor};
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

    // Run the server and wait for the two threads to end (typically by trigger LSP Exit event).
    let server_capabilities = serde_json::to_value(&ServerCapabilities {
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
            commands: vec!["tree_to_dot".to_string()],
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
        position_encoding: Some(PositionEncodingKind::UTF8),
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
        RopeSlice<'_>,
        RopeSlice<'_>,
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
                                format_code(&app, &current_rope).ok();

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
                                format_code(&app, &current_rope).ok();

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
                                &slide_complete_query,
                                &inlay_edge_query,
                                &mut inlay_edge_map,
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
                                .0
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
                                                tree_info.0.root_node(),
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
                                        NodeKind::SlideObjects
                                        | NodeKind::SlideObj
                                        | NodeKind::Action => {
                                            query_cursor.set_point_range(
                                                Point { row: 0, column: 0 }..completion_point,
                                            );
                                            let iter = query_cursor.matches(
                                                &object_name_query,
                                                tree_info.0.root_node(),
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
                                                                        new_text: "value: \"$0\"".to_string(),
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
                                                                    new_text: "font_family: \"$0\"".to_string(),
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
                                                                    new_text: "language: \"$0\"".to_string(),
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
                                                                    new_text: "align: \"$0\"".to_string(),
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
                                                &tree_info.0,
                                                &current_rope,
                                                &mut query_cursor,
                                            )
                                            .ok(),
                                        )))
                                        .unwrap();
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
                                "tree_to_dot" => {
                                    if let Ok(process) = std::process::Command::new("dot")
                                        .stdout(std::fs::File::create("out.dot").unwrap())
                                        .stdin(Stdio::piped())
                                        .spawn()
                                    {
                                        let tree_info = app.tree_info.lock();
                                        let tree_info = tree_info.as_ref().unwrap();

                                        tree_info.0.print_dot_graph(&process.stdin.unwrap());

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
                        );
                        *tree_info = Some((tree, Rope::new()));
                        *app.slide_show_file.lock() = current_rope.clone();
                        match ast {
                            Ok(_) => {
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

                        app.clear_resolved.store(false, Ordering::Relaxed);
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
                                    helix_lsp::OffsetEncoding::Utf8,
                                );

                                let edits =
                                    generate_edits(current_rope.slice(..), transaction.changes());
                                if transaction.apply(&mut current_rope) {
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
                                    if !tree.root_node().has_error() {
                                        edit_slideshow(
                                            &tree_info.0,
                                            &tree,
                                            &mut app.slide_show.write(),
                                        );

                                        tree_info.1 = current_rope.clone();

                                        let mut slide_show = app.slide_show.write();

                                        if super::parser::parse_file(
                                            &tree,
                                            Some(&tree_info.0),
                                            &tree_info.1,
                                            &mut app.helix_cell,
                                            &mut slide_show,
                                        )
                                        .is_ok()
                                        {
                                            app.clear_resolved.store(true, Ordering::Relaxed);
                                            lsp_egui_ctx.request_repaint();
                                        }
                                    }
                                    tree_info.0 = tree;
                                } else {
                                    panic!("Transaction could not be applied");
                                }
                            }

                            if changes_len == 1 {
                                hover(
                                    &app,
                                    &tree_info.0,
                                    &mut query_cursor,
                                    &current_rope,
                                    &slide_complete_query,
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
                            info.1 = current_rope.clone();

                            let mut slide_show = app.slide_show.write();

                            let ast = super::parser::parse_file(
                                &info.0,
                                None,
                                &info.1,
                                &mut app.helix_cell,
                                &mut slide_show,
                            );
                            match ast {
                                Ok(_) => {
                                    *app.slide_show_file.lock() = info.1.clone();
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
        .0
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
        .0
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

    workspace_edit
}

fn inlay_hints(
    app: &MyEguiApp,
    slide_complete_query: &Query,
    inlay_edge_query: &Query,
    inlay_edge_map: &mut HashMap<RopeSlice<'_>, RopeSlice<'_>, BuildHasherDefault<ahash::AHasher>>,
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
    let slide_iter = query_cursor.matches(
        slide_complete_query,
        tree_info.0.root_node(),
        RopeProvider(current_rope.slice(..)),
    );

    for (slide_num, query_match) in slide_iter.enumerate() {
        let range = query_match.captures[0].node.range();
        hints.push(InlayHint {
            position: Position {
                line: range.start_point.row as u32,
                character: range.start_point.column as u32,
            },
            label: InlayHintLabel::String(format!("Slide {}:", slide_num + 1)),
            kind: Some(InlayHintKind::PARAMETER),
            text_edits: None,
            tooltip: None,
            padding_right: Some(true),
            padding_left: Some(false),
            data: None,
        });
    }

    let mut edge_iter = query_cursor
        .matches(
            inlay_edge_query,
            tree_info.0.root_node(),
            RopeProvider(current_rope.slice(..)),
        )
        .peekable();

    // SAFETY:
    // The hashmap where the byte slices are stored is cleared after inlay hints are computed
    // The slices are only used when converting them into Strings
    //
    // I'm pretty sure that running `clear()` on a hashmap *drops* all values within it,
    // if not, oops, memory leak
    while let Some(query_match) = edge_iter.next() {
        let query_node = query_match.captures[0].node;
        let query_slice = unsafe {
            you_can::borrow_unchecked(
                current_rope.byte_slice(query_match.captures[0].node.byte_range()),
            )
        };

        match edge_iter.peek() {
            Some(edge_match)
                if matches!(
                    NodeKind::from(edge_match.captures[0].node.kind_id()),
                    NodeKind::EdgeParser
                ) =>
            {
                let edge_slice = unsafe {
                    you_can::borrow_unchecked(
                        current_rope.byte_slice(edge_match.captures[0].node.byte_range()),
                    )
                };
                if edge_slice.len_chars() == 2 {
                    if let Some(edge) = inlay_edge_map.get(&query_slice) {
                        let range = edge_match.captures[0].node.range();

                        hints.push(InlayHint {
                            position: Position {
                                line: range.start_point.row as u32,
                                character: range.start_point.column as u32,
                            },
                            // This parameter must NEVER be the actual borrowed slice
                            label: InlayHintLabel::String(format!("{}", edge)),
                            kind: Some(InlayHintKind::PARAMETER),
                            text_edits: None,
                            tooltip: None,
                            padding_right: Some(false),
                            padding_left: Some(false),
                            data: None,
                        });
                    }
                    inlay_edge_map.insert(query_slice, edge_slice);
                } else {
                    inlay_edge_map.insert(query_slice, edge_slice.slice(2..));
                }

                edge_iter.next();
            }
            _ => {
                if let Some(edge) = inlay_edge_map.get(&query_slice) {
                    let mut walker = query_node.parent().unwrap().walk();
                    while walker.goto_next_sibling() {}
                    let range = walker.node().range();

                    hints.push(InlayHint {
                        position: Position {
                            line: range.end_point.row as u32,
                            character: range.end_point.column as u32,
                        },
                        // This parameter must NEVER be the actual borrowed slice
                        label: InlayHintLabel::String(format!("{}{}", edge, edge)),
                        kind: Some(InlayHintKind::PARAMETER),
                        text_edits: None,
                        tooltip: None,
                        padding_right: Some(false),
                        padding_left: Some(false),
                        data: None,
                    });
                }
            }
        }
    }

    // ABSOLUTELY DO NOT REMOVE
    inlay_edge_map.clear();

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

    let start_node = tree_info.0.root_node();

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
    let mut walker = if let Some(n) = node.as_ref() {
        GrzCursor::from_node(n)
    } else {
        return Ok(CompletionResponse::Array(vec![new_slide_item]));
    };

    walker.goto_first_child_raw()?;
    walker.goto_first_child_raw()?;
    walker.goto_next_sibling_raw()?;
    let mut cursor_counter = 0;

    loop {
        let mut line = String::new();
        walker.goto_first_child_raw()?;
        line.push_str("    ");
        loop {
            match NodeKind::from(walker.node().kind_id()) {
                NodeKind::EdgeParser => {
                    std::fmt::Write::write_fmt(&mut line, format_args!("${},\n", cursor_counter))
                        .unwrap();
                    cursor_counter += 1;
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
                    if !walker.goto_next_sibling_raw()? {
                        std::fmt::Write::write_fmt(
                            &mut line,
                            format_args!("${},\n", cursor_counter),
                        )
                        .unwrap();
                        cursor_counter += 1;
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

                    if !walker.goto_next_sibling_raw()? {
                        std::fmt::Write::write_fmt(
                            &mut line,
                            format_args!("${},\n", cursor_counter),
                        )
                        .unwrap();
                        cursor_counter += 1;
                        new_text.push_str(&line);
                        break;
                    }
                }
            }
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
        .0
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
        tree_info.0.root_node(),
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
        .0
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
        tree_info.0.root_node(),
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

    let mut tree_cursor = GrzCursor::new(&tree_info.0);
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
            NodeKind::Action => {
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

pub fn edit_slideshow(old_tree: &Tree, new_tree: &Tree, slideshow: &mut SlideShow) {
    for range in old_tree.changed_ranges(&new_tree) {
        let mut node = old_tree
            .root_node()
            .descendant_for_point_range(range.start_point, range.start_point);
        if let Some(mut node) = node {
            while node.is_extra() {
                if let Some(next_sibling) = node.next_named_sibling() {
                    node = next_sibling;
                } else {
                    break;
                }
            }
        }
    }
    eprintln!();
}

pub fn hover(
    app: &MyEguiApp,
    tree_info: &Tree,
    query_cursor: &mut QueryCursor,
    current_rope: &Rope,
    slide_complete_query: &Query,
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

        match NodeKind::from(node.kind_id()) {
            NodeKind::Slide => {
                query_cursor.set_point_range(Point { row: 0, column: 0 }..point);

                let iter = query_cursor.matches(
                    &slide_complete_query,
                    tree_info.root_node(),
                    RopeProvider(current_rope.slice(..)),
                );

                let slide_num = iter.count().saturating_sub(1);

                if app.index.swap(slide_num, Ordering::Relaxed) != slide_num {
                    app.vb_dbg.store(0, Ordering::Relaxed);
                    app.obj_dbg.store(0, Ordering::Relaxed);
                    app.clear_resolved.store(true, Ordering::Relaxed);
                    lsp_egui_ctx.request_repaint();
                }
            }
            NodeKind::Viewbox => {
                if let Some(name_node) = node.named_child(0) {
                    let vb_name = current_rope.byte_slice(name_node.byte_range());
                    let hasher = ahash::RandomState::with_seeds(69, 420, 24, 96);
                    let mut hasher = hasher.build_hasher();
                    vb_name.hash(&mut hasher);
                    let hashed_vb = hasher.finish();
                    if app.vb_dbg.swap(hashed_vb, Ordering::Relaxed) != hashed_vb {
                        query_cursor.set_point_range(
                            name_node.range().start_point..Point {
                                row: usize::MAX,
                                column: usize::MAX,
                            },
                        );

                        let mut iter = query_cursor.matches(
                            &vb_in_slide_query,
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

                            let iter = query_cursor.matches(
                                &slide_complete_query,
                                tree_info.root_node(),
                                RopeProvider(current_rope.slice(..)),
                            );

                            let slide_num = iter.count() - 1;

                            app.index.store(slide_num, Ordering::Relaxed);
                            app.obj_dbg.store(0, Ordering::Relaxed);

                            app.clear_resolved.store(true, Ordering::Relaxed);
                            lsp_egui_ctx.request_repaint();
                        }
                    }
                }
            }
            NodeKind::Obj => {
                if let Some(name_node) = node.named_child(0) {
                    let obj_name = current_rope.byte_slice(name_node.byte_range());
                    let hasher = ahash::RandomState::with_seeds(69, 420, 24, 96);
                    let mut hasher = hasher.build_hasher();
                    obj_name.hash(&mut hasher);
                    let hashed_obj = hasher.finish();
                    if app.obj_dbg.swap(hashed_obj, Ordering::Relaxed) != hashed_obj {
                        query_cursor.set_point_range(
                            name_node.range().start_point..Point {
                                row: usize::MAX,
                                column: usize::MAX,
                            },
                        );

                        let mut iter = query_cursor.matches(
                            &obj_in_slide_query,
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

                            let iter = query_cursor.matches(
                                &slide_complete_query,
                                tree_info.root_node(),
                                RopeProvider(current_rope.slice(..)),
                            );

                            let slide_num = iter.count() - 1;

                            app.index.store(slide_num, Ordering::Relaxed);
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

pub fn format_code(app: &MyEguiApp, current_rope: &Rope) -> Result<Vec<TextEdit>, Error> {
    let tree_info = app.tree_info.lock();
    let tree_info = tree_info.as_ref().unwrap();

    let mut formatting_cursor = FormattingCursor::new(&tree_info.0);

    formatting_cursor.goto_first_child(WhitespaceEdit::Delete, current_rope)?;

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
                        formatting_cursor
                            .goto_next_sibling(WhitespaceEdit::Assert(" "), current_rope)?;
                        formatting_cursor
                            .goto_next_sibling(WhitespaceEdit::Delete, current_rope)?;
                        formatting_cursor.goto_first_child(WhitespaceEdit::Delete, current_rope)?;
                        formatting_cursor
                            .goto_next_sibling(WhitespaceEdit::Delete, current_rope)?;
                        formatting_cursor
                            .goto_next_sibling(WhitespaceEdit::Delete, current_rope)?;
                        formatting_cursor.goto_parent();
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
                formatting_cursor.goto_next_sibling(WhitespaceEdit::Assert(" "), current_rope)?;
                formatting_cursor.goto_next_sibling(WhitespaceEdit::Delete, current_rope)?;
                formatting_cursor.goto_first_child(WhitespaceEdit::Delete, current_rope)?;
                formatting_cursor.goto_next_sibling(WhitespaceEdit::Delete, current_rope)?;
                formatting_cursor.goto_next_sibling(WhitespaceEdit::Delete, current_rope)?;
                formatting_cursor.goto_parent();
                formatting_cursor.goto_next_sibling(WhitespaceEdit::Assert(" "), current_rope)?;
                formatting_cursor.goto_first_child(WhitespaceEdit::Delete, current_rope)?;
                formatting_cursor
                    .goto_next_sibling(WhitespaceEdit::Assert("\n    "), current_rope)?;
                while formatting_cursor.node().kind_id() == NodeKind::ViewboxObj as u16 {
                    formatting_cursor.goto_first_child(WhitespaceEdit::Delete, current_rope)?;
                    formatting_cursor.goto_next_sibling(WhitespaceEdit::Delete, current_rope)?;
                    formatting_cursor.goto_next_sibling(WhitespaceEdit::Delete, current_rope)?;
                    formatting_cursor.goto_parent();
                    formatting_cursor
                        .goto_next_sibling(WhitespaceEdit::Trailing(","), current_rope)?;
                    formatting_cursor
                        .goto_next_sibling(WhitespaceEdit::Assert("\n    "), current_rope)?;
                }
                formatting_cursor.revisit(WhitespaceEdit::Assert("\n"), current_rope);
                formatting_cursor.goto_parent();
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
                while formatting_cursor.tree_cursor.field_id() == Some(FieldName::Parameters as u16)
                {
                    formatting_cursor.goto_next_sibling(WhitespaceEdit::Delete, current_rope)?;
                    formatting_cursor
                        .goto_next_sibling(WhitespaceEdit::Assert(" "), current_rope)?;
                    formatting_cursor
                        .goto_next_sibling(WhitespaceEdit::Trailing(","), current_rope)?;
                    formatting_cursor
                        .goto_next_sibling(WhitespaceEdit::Assert("\n    "), current_rope)?;
                }
                formatting_cursor.revisit(WhitespaceEdit::Assert("\n"), current_rope);
                formatting_cursor.goto_parent();
                formatting_cursor.goto_parent();
            }
            NodeKind::Register => {
                formatting_cursor.goto_first_child(WhitespaceEdit::Delete, current_rope)?;
                formatting_cursor.goto_next_sibling(WhitespaceEdit::Delete, current_rope)?;
                formatting_cursor.goto_next_sibling(WhitespaceEdit::Assert(" "), current_rope)?;
                formatting_cursor.goto_parent();
            }
            NodeKind::Action => {
                formatting_cursor.goto_first_child(WhitespaceEdit::Delete, current_rope)?;
                formatting_cursor
                    .goto_next_sibling(WhitespaceEdit::Assert("\n    "), current_rope)?;
                if formatting_cursor.node().kind_id() == NodeKind::ActionObj as u16 {
                    while formatting_cursor.node().kind_id() == NodeKind::ActionObj as u16 {
                        formatting_cursor.goto_first_child(WhitespaceEdit::Delete, current_rope)?;
                        formatting_cursor
                            .goto_next_sibling(WhitespaceEdit::Delete, current_rope)?;
                        formatting_cursor
                            .goto_next_sibling(WhitespaceEdit::Assert(" "), current_rope)?;

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
