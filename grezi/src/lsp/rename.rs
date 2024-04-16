use helix_core::{
    syntax::RopeProvider,
    tree_sitter::{Point, Query, QueryCursor, Tree},
    Rope,
};
use lsp_types::{
    AnnotatedTextEdit, OneOf, PrepareRenameResponse, RenameParams, TextDocumentPositionParams,
    TextEdit,
};

use crate::parser::NodeKind;

use super::formatter::char_range_from_byte_range;

pub fn prepare_rename(
    pos: TextDocumentPositionParams,
    current_rope: &Rope,
    tree: &Tree,
) -> Option<PrepareRenameResponse> {
    let point = Point {
        row: pos.position.line as usize,
        column: pos.position.character as usize,
    };

    tree.root_node()
        .descendant_for_point_range(point, point)
        .and_then(|f| {
            if matches!(NodeKind::from(f.kind_id()), NodeKind::Identifier) {
                let node_range = f.range();
                Some(PrepareRenameResponse::Range(
                    char_range_from_byte_range(node_range, current_rope).ok()?,
                ))
            } else {
                None
            }
        })
}

pub fn rename(
    rename: RenameParams,
    current_rope: &Rope,
    rename_query: &Query,
    query_cursor: &mut QueryCursor,
    tree: &Tree,
) -> Vec<OneOf<TextEdit, AnnotatedTextEdit>> {
    let mut workspace_edit: Vec<OneOf<TextEdit, AnnotatedTextEdit>> = Vec::new();
    let point = Point {
        row: rename.text_document_position.position.line as usize,
        column: rename.text_document_position.position.character as usize,
    };

    let rename_node = tree
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
        tree.root_node(),
        RopeProvider(current_rope.slice(..)),
    );

    for query_match in iter {
        let node = query_match.captures[0].node;
        if current_rope.byte_slice(node.byte_range()).eq(&rename_name) {
            let range = node.range();

            workspace_edit.push(OneOf::Left(TextEdit {
                range: char_range_from_byte_range(range, current_rope).unwrap(),
                new_text: rename.new_name.clone(),
            }));
        }
    }

    workspace_edit
}
