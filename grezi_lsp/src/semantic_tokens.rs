use helix_core::{
    Rope,
    tree_sitter::{InactiveQueryCursor, Point, Query, RopeInput, Tree},
};
use helix_lsp_types as lsp_types;
use lsp_types::{SemanticToken, SemanticTokens, SemanticTokensResult};

pub fn semantic_tokens(
    semantic_token_query: &Query,
    current_rope: &Rope,
    tree: &Option<Tree>,
) -> Option<SemanticTokensResult> {
    let tree = tree.as_ref()?;
    let start_node = tree.root_node();

    let mut iter = InactiveQueryCursor::default().execute_query(
        semantic_token_query,
        &start_node,
        RopeInput::new(current_rope.slice(..)),
    );

    let mut tokens = Vec::new();
    let mut last_range = helix_core::tree_sitter::Range {
        start_byte: 0,
        end_byte: 0,
        start_point: Point { row: 0, col: 0 },
        end_point: Point { row: 0, col: 0 },
    };
    while let Some(query_match) = iter.next_match() {
        let capture = query_match.matched_nodes().last().unwrap();
        let range = capture.node.range();

        if last_range != range {
            let mut delta_line = range.start_point.row - last_range.end_point.row;
            let mut multiline = false;
            for line in range.start_point.row..=range.end_point.row {
                tokens.push(SemanticToken {
                    delta_line,
                    delta_start: if multiline {
                        0
                    } else if delta_line == 0 {
                        range.start_point.col - last_range.start_point.col
                    } else {
                        range.start_point.col
                    },
                    length: if line == range.start_point.row {
                        if (range.end_point.row - range.start_point.row) > 0 {
                            let slice = current_rope
                                .line(line as usize)
                                .get_byte_slice(range.start_point.col as usize..);

                            slice?.len_chars() as u32
                        } else {
                            range.end_point.col - range.start_point.col
                        }
                    } else if line == range.end_point.row {
                        let slice = current_rope
                            .line(line as usize)
                            .get_byte_slice(..range.end_point.col as usize);

                        slice?.len_chars() as u32
                    } else {
                        current_rope.line(line as usize).len_chars() as u32
                    },
                    token_type: capture.capture.idx() as u32,
                    token_modifiers_bitset: if semantic_token_query
                        .capture_name(capture.capture)
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

    Some(SemanticTokensResult::Tokens(SemanticTokens {
        data: tokens,
        ..Default::default()
    }))
}
