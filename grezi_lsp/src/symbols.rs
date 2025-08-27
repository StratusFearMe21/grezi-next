use std::sync::Arc;

use grezi_parser::parse::{cursor::GrzCursor, error::ErrsWithSource};
use helix_core::{
    Rope,
    syntax::RopeProvider,
    tree_sitter::{Point, Query, QueryCursor, Tree},
};
use helix_lsp_types as lsp_types;
use lsp_types::{
    DocumentSymbol, GotoDefinitionParams, GotoDefinitionResponse, Location, ReferenceParams,
    SymbolKind, Url,
};
use tree_sitter::StreamingIterator;
use tree_sitter_grz::NodeKind;

use super::formatter::char_range_from_byte_range;

pub fn references(
    rename_query: &Query,
    current_rope: &Rope,
    references: ReferenceParams,
    query_cursor: &mut QueryCursor,
    tree: &Option<Tree>,
) -> Option<Vec<Location>> {
    let tree = tree.as_ref()?;
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
    let mut iter = query_cursor.matches(
        rename_query,
        tree.root_node(),
        RopeProvider(current_rope.slice(..)),
    );

    let mut locations = Vec::new();

    while let Some(query_match) = iter.next() {
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
    tree: &Option<Tree>,
) -> Option<GotoDefinitionResponse> {
    let tree = tree.as_ref()?;
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
    let mut iter = query_cursor.matches(
        top_level_search_query,
        tree.root_node(),
        RopeProvider(current_rope.slice(..)),
    );

    while let Some(query_match) = iter.next() {
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
    tree: &Option<Tree>,
    symbols: &mut Vec<S>,
    callback: F,
) -> Option<()>
where
    F: Fn(DocumentSymbol) -> S,
{
    let tree = tree.as_ref()?;
    let mut tree_cursor = GrzCursor::new(tree, current_rope, Arc::new(ErrsWithSource::default()));

    let mut tree_cursor = tree_cursor
        .goto_first_child(NodeKind::SymSourceFile)
        .ok()??;
    let mut slide_num = 0;
    'parserloop: loop {
        let node = tree_cursor.node();
        let range = node.range();

        match NodeKind::from(node.kind_id()) {
            NodeKind::SymSlide => {
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
            NodeKind::SymViewbox => {
                let mut tree_cursor = tree_cursor.goto_first_child(NodeKind::SymViewbox).ok()??;
                let selection_range = tree_cursor.node().range();
                let byte_range = tree_cursor.node().byte_range();
                let _ = tree_cursor.goto_next_sibling();
                let vb_ref_range = tree_cursor.node().byte_range();

                symbols.push(callback(DocumentSymbol {
                    name: current_rope.byte_slice(byte_range).to_string(),
                    kind: SymbolKind::VARIABLE,
                    range: char_range_from_byte_range(range, current_rope).ok()?,
                    detail: Some(current_rope.byte_slice(vb_ref_range).to_string()),
                    selection_range: char_range_from_byte_range(selection_range, current_rope)
                        .ok()?,
                    tags: None,
                    deprecated: None,
                    children: None,
                }))
            }
            NodeKind::SymObj => {
                let mut tree_cursor = tree_cursor.goto_first_child(NodeKind::SymObj).ok()??;
                let selection_range = tree_cursor.node().range();
                let byte_range = tree_cursor.node().byte_range();
                let _ = tree_cursor.goto_next_sibling();
                let tree_cursor = tree_cursor.goto_first_child(NodeKind::SymObjInner).ok()??;
                let name_range = tree_cursor.node().byte_range();

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
            NodeKind::SymRegister => { /* todo */ }
            NodeKind::SymActions => {
                slide_num += 1;

                let tree_cursor = tree_cursor.goto_first_child(NodeKind::SymActions).ok()??;
                let selection_range = tree_cursor.node().range();

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
