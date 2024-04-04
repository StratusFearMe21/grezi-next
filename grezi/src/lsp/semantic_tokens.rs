use helix_core::{
    syntax::RopeProvider,
    tree_sitter::{Point, Query, QueryCursor},
    Rope,
};
use lsp_types::{SemanticToken, SemanticTokens, SemanticTokensResult};

use crate::MyEguiApp;

pub fn semantic_tokens(
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
