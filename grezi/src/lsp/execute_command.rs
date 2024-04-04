use std::{path::Path, process::Stdio};

use eframe::egui;
use helix_core::{tree_sitter::Parser, Rope};
use lsp_server::{Connection, Message, RequestId, Response};
use lsp_types::{
    notification::{Notification, PublishDiagnostics},
    ExecuteCommandParams, PublishDiagnosticsParams, Url,
};

use crate::MyEguiApp;

pub fn execute_command(
    app: &MyEguiApp,
    command: ExecuteCommandParams,
    rqid: RequestId,
    current_rope: &mut Rope,
    connection: &Connection,
    lsp_egui_ctx: &egui::Context,
    parser: &Parser,
    currently_open: &Url,
    current_document_version: u32,
) -> Response {
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

                Response::new_ok(rqid, None::<serde_json::Value>)
            } else {
                Response::new_err(rqid, 500, "graphviz is not installed".to_string())
            }
        }
        "full_reparse" => {
            let mut slide_show = app.slide_show.write();
            *current_rope =
                Rope::from_reader(std::fs::File::open(currently_open.path()).unwrap()).unwrap();
            let mut tree_info = app.tree_info.lock();
            let tree = parser
                .parse_with(
                    &mut |byte, _| {
                        if byte <= current_rope.len_bytes() {
                            let (chunk, start_byte, _, _) = current_rope.chunk_at_byte(byte);
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
                                        let diagnostic: lsp_types::Diagnostic = error.into();
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
            Response::new_ok(rqid, None::<serde_json::Value>)
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
                                let text = text.trim_matches(|c: char| c.is_ascii_punctuation());
                                if hunspell.check(text) == CheckResult::MissingInDictionary {
                                    warnings.push(super::parser::Error::SpellCheck(
                                        PointFromRange::new(
                                            query_match.captures[0].node.range(),
                                            &current_rope,
                                        ),
                                        hunspell.suggest(text),
                                    ));
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
                                    let diagnostic: lsp_types::Diagnostic = error.into();
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
