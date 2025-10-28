use helix_lsp_types::{
    DocumentChanges, DocumentFormattingParams, DocumentSymbolParams, DocumentSymbolResponse,
    FoldingRangeParams, GotoDefinitionParams, Location, OneOf,
    OptionalVersionedTextDocumentIdentifier, ReferenceParams, RenameParams, SemanticTokensParams,
    TextDocumentEdit, TextDocumentPositionParams, TextEdit, WorkspaceEdit, WorkspaceSymbol,
    WorkspaceSymbolParams, WorkspaceSymbolResponse,
    request::{
        DocumentSymbolRequest, FoldingRangeRequest, Formatting, GotoDeclaration,
        PrepareRenameRequest, References, Rename, Request, SemanticTokensFullRequest,
        WorkspaceSymbolRequest,
    },
};
use lsp_server::{Connection, Message, Response};
use nucleo_matcher::{
    Matcher,
    pattern::{AtomKind, CaseMatching, Normalization, Pattern},
};

use crate::GrzLsp;

mod folding_range;
mod formatter;
mod rename;
mod semantic_tokens;
mod symbols;

impl GrzLsp {
    pub fn handle_request_message(
        &mut self,
        connection: &Connection,
        req: lsp_server::Request,
    ) -> bool {
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
                                doc.tree.as_ref().map(|tree| tree.root_node()),
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
                    let symbols =
                        symbols::document_symbols(&doc.source, &doc.tree, &mut symbols, |s| s)
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

                    pub struct MatchableWorkspaceSymbol(WorkspaceSymbol);

                    impl AsRef<str> for MatchableWorkspaceSymbol {
                        fn as_ref(&self) -> &str {
                            self.0.name.as_str()
                        }
                    }

                    for (uri, doc) in self.grz_files.iter() {
                        symbols::document_symbols(&doc.source, &doc.tree, &mut symbols, |s| {
                            MatchableWorkspaceSymbol(WorkspaceSymbol {
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
                        });
                    }
                    let mut matcher = Matcher::new(nucleo_matcher::Config::DEFAULT);
                    let matches = Pattern::new(
                        &params.query,
                        CaseMatching::Ignore,
                        Normalization::Smart,
                        AtomKind::Fuzzy,
                    )
                    .match_list(symbols, &mut matcher);
                    let symbols = matches.into_iter().map(|s| s.0.0).collect::<Vec<_>>();
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
                                &doc.tree,
                            ),
                        )))
                        .unwrap();
                }
            }
            Rename::METHOD => {
                if let Ok((rqid, rename_params)) = req.extract::<RenameParams>(Rename::METHOD) {
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
                                        text_document: OptionalVersionedTextDocumentIdentifier {
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

        false
    }
}
