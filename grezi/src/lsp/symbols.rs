use helix_core::{
    syntax::RopeProvider,
    tree_sitter::{Point, Query, QueryCursor, Tree},
    Rope,
};
use lsp_types::{
    DocumentSymbol, GotoDefinitionParams, GotoDefinitionResponse, Hover, HoverParams, Location,
    ReferenceParams, SymbolKind, Url,
};

use crate::{
    parser::{GrzCursor, NodeKind},
    MyEguiApp, SlideShow,
};

use super::formatter::char_range_from_byte_range;

pub fn references(
    rename_query: &Query,
    current_rope: &Rope,
    references: ReferenceParams,
    query_cursor: &mut QueryCursor,
    tree: &Tree,
) -> Option<Vec<Location>> {
    let point = Point {
        row: references.text_document_position.position.line as usize,
        column: references.text_document_position.position.character as usize,
    };

    let reference_node = tree
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
        tree.root_node(),
        RopeProvider(current_rope.slice(..)),
    );

    let mut locations = Vec::new();

    for query_match in iter {
        if current_rope.byte_slice(query_match.captures[0].node.byte_range())
            == current_rope.byte_slice(reference_node.byte_range())
        {
            let range = query_match.captures[0].node.range();
            locations.push(Location {
                uri: references.text_document_position.text_document.uri.clone(),
                range: char_range_from_byte_range(range, current_rope).ok()?,
            });
        }
    }

    Some(locations)
}

pub fn goto_declaration(
    top_level_search_query: &Query,
    current_rope: &Rope,
    currently_open: Url,
    goto_declaration: GotoDefinitionParams,
    query_cursor: &mut QueryCursor,
    tree: &Tree,
) -> Option<GotoDefinitionResponse> {
    let point = Point {
        row: goto_declaration.text_document_position_params.position.line as usize,
        column: goto_declaration
            .text_document_position_params
            .position
            .character as usize,
    };

    let usage_node = tree
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
        tree.root_node(),
        RopeProvider(current_rope.slice(..)),
    );

    for query_match in iter {
        if current_rope.byte_slice(query_match.captures[1].node.byte_range())
            == current_rope.byte_slice(usage_node.byte_range())
        {
            let range = query_match.captures[1].node.range();
            return Some(GotoDefinitionResponse::Scalar(Location {
                uri: currently_open,
                range: char_range_from_byte_range(range, current_rope).ok()?,
            }));
        }
    }

    None
}

#[allow(deprecated)]
pub fn document_symbols<F, S>(
    current_rope: &Rope,
    tree: &Tree,
    symbols: &mut Vec<S>,
    callback: F,
) -> Option<()>
where
    F: Fn(DocumentSymbol) -> S,
{
    let mut tree_cursor = GrzCursor::new(tree, current_rope);

    let _ = tree_cursor.goto_first_child();
    let mut slide_num = 0;
    'parserloop: loop {
        let node = tree_cursor.node();
        let range = node.range();

        match NodeKind::from(node.kind_id()) {
            NodeKind::Slide => {
                slide_num += 1;

                let selection_range = tree_cursor.node().range();

                symbols.push(callback(DocumentSymbol {
                    name: format!("Slide {}", slide_num),
                    kind: SymbolKind::FUNCTION,
                    range: char_range_from_byte_range(range, current_rope).ok()?,
                    detail: None,
                    selection_range: char_range_from_byte_range(selection_range, current_rope)
                        .ok()?,
                    tags: None,
                    deprecated: None,
                    children: None,
                }))
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

                symbols.push(callback(DocumentSymbol {
                    name: current_rope.byte_slice(byte_range).to_string(),
                    kind: SymbolKind::VARIABLE,
                    range: char_range_from_byte_range(range, current_rope).ok()?,
                    detail: Some(format!(
                        "{}{}",
                        current_rope.byte_slice(name_range),
                        current_rope.byte_slice(index_range)
                    )),
                    selection_range: char_range_from_byte_range(selection_range, current_rope)
                        .ok()?,
                    tags: None,
                    deprecated: None,
                    children: None,
                }))
            }
            NodeKind::Obj => {
                let _ = tree_cursor.goto_first_child();
                let selection_range = tree_cursor.node().range();
                let byte_range = tree_cursor.node().byte_range();
                let _ = tree_cursor.goto_next_sibling();
                let name_range = tree_cursor.node().byte_range();
                tree_cursor.goto_parent();

                symbols.push(callback(DocumentSymbol {
                    name: current_rope.byte_slice(byte_range).to_string(),
                    kind: SymbolKind::OBJECT,
                    range: char_range_from_byte_range(range, current_rope).ok()?,
                    detail: Some(current_rope.byte_slice(name_range).to_string()),
                    selection_range: char_range_from_byte_range(selection_range, current_rope)
                        .ok()?,
                    tags: None,
                    deprecated: None,
                    children: None,
                }))
            }
            NodeKind::Register => { /* todo */ }
            NodeKind::SlideFunctions => {
                slide_num += 1;

                let _ = tree_cursor.goto_first_child();
                let selection_range = tree_cursor.node().range();
                tree_cursor.goto_parent();

                symbols.push(callback(DocumentSymbol {
                    name: "Actions".to_string(),
                    kind: SymbolKind::ARRAY,
                    range: char_range_from_byte_range(range, current_rope).ok()?,
                    detail: None,
                    selection_range: char_range_from_byte_range(selection_range, current_rope)
                        .ok()?,
                    tags: None,
                    deprecated: None,
                    children: None,
                }))
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

    Some(())
}

pub fn hover(
    app: &MyEguiApp,
    top_level_search_query: &Query,
    slide_index_query: &Query,
    vb_in_slide_query: &Query,
    obj_in_slide_query: &Query,
    lsp_egui_ctx: &eframe::egui::Context,
    hover: HoverParams,
    current_rope: &Rope,
    query_cursor: &mut QueryCursor,
    tree: &Tree,
    slideshow: &SlideShow,
) -> Option<Hover> {
    let point = Point {
        row: hover.text_document_position_params.position.line as usize,
        column: hover.text_document_position_params.position.character as usize,
    };

    let usage_node = tree
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
        tree.root_node(),
        RopeProvider(current_rope.slice(..)),
    );

    for query_match in iter {
        if current_rope.byte_slice(query_match.captures[1].node.byte_range())
            == current_rope.byte_slice(usage_node.byte_range())
        {
            let captured_node = query_match.captures[0].node;
            let start_point = captured_node.range().start_point;
            let byte_range = captured_node.byte_range();
            super::hover(
                app,
                tree,
                query_cursor,
                current_rope,
                slide_index_query,
                vb_in_slide_query,
                obj_in_slide_query,
                lsp_egui_ctx,
                usage_node.range().start_point,
                usage_node,
                slideshow,
            );
            super::hover(
                app,
                tree,
                query_cursor,
                current_rope,
                slide_index_query,
                vb_in_slide_query,
                obj_in_slide_query,
                lsp_egui_ctx,
                start_point,
                captured_node,
                slideshow,
            );
            lsp_egui_ctx.request_repaint();
            return Some(Hover {
                contents: lsp_types::HoverContents::Markup(lsp_types::MarkupContent {
                    kind: lsp_types::MarkupKind::Markdown,
                    value: format!("```grz\n{}\n```", current_rope.byte_slice(byte_range)),
                }),
                range: char_range_from_byte_range(usage_node.range(), current_rope).ok(),
            });
        }
    }

    None
}
