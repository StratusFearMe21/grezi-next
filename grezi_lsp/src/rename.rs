use grezi_parser::parse::byte_pos_from_char_pos;
use helix_core::tree_sitter::{InactiveQueryCursor, Query, RopeInput, Tree};
use helix_lsp_types as lsp_types;
use lsp_types::{
    AnnotatedTextEdit, OneOf, PrepareRenameResponse, RenameParams, TextDocumentPositionParams,
    TextEdit,
};
use ropey::Rope;
use tree_sitter_grz::NodeKind;

use super::formatter::char_range_from_byte_range;

pub fn prepare_rename(
    pos: TextDocumentPositionParams,
    current_rope: &Rope,
    tree: &Option<Tree>,
) -> Option<PrepareRenameResponse> {
    let point = byte_pos_from_char_pos(
        (pos.position.line as usize, pos.position.character as usize),
        current_rope,
    )
    .ok()?;

    tree.as_ref()?
        .root_node()
        .descendant_for_point_range(point, point)
        .and_then(|f| {
            if matches!(NodeKind::from(f.kind_id()), NodeKind::SymIdentifier) {
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
    tree: &Option<Tree>,
) -> Vec<OneOf<TextEdit, AnnotatedTextEdit>> {
    let mut workspace_edit: Vec<OneOf<TextEdit, AnnotatedTextEdit>> = Vec::new();
    let Some(tree) = tree else {
        return Vec::new();
    };
    let Ok(point) = byte_pos_from_char_pos(
        (
            rename.text_document_position.position.line as usize,
            rename.text_document_position.position.character as usize,
        ),
        current_rope,
    ) else {
        return Vec::new();
    };

    let rename_node = tree
        .root_node()
        .descendant_for_point_range(point, point)
        .unwrap();

    let range = rename_node.byte_range();
    let rename_name = current_rope.byte_slice(range.start as usize..range.end as usize);

    let mut iter = InactiveQueryCursor::default().execute_query(
        rename_query,
        &tree.root_node(),
        RopeInput::new(current_rope.slice(..)),
    );

    while let Some(query_match) = iter.next_match() {
        let node = query_match.matched_node(0);
        let range = node.node.byte_range();
        if current_rope
            .byte_slice(range.start as usize..range.end as usize)
            .eq(&rename_name)
        {
            let range = node.node.range();

            workspace_edit.push(OneOf::Left(TextEdit {
                range: char_range_from_byte_range(range, current_rope).unwrap(),
                new_text: rename.new_name.clone(),
            }));
        }
    }

    workspace_edit
}
