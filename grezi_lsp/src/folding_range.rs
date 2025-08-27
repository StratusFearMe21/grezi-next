use helix_core::{
    Rope,
    syntax::RopeProvider,
    tree_sitter::{Point, Query, QueryCursor, Tree},
};
use helix_lsp_types as lsp_types;
use lsp_types::FoldingRange;
use tree_sitter::StreamingIterator;

use super::formatter::char_range_from_byte_range;

pub fn folding_ranges(
    rope: &Rope,
    tree: &Option<Tree>,
    query_cursor: &mut QueryCursor,
    folding_ranges_query: &Query,
) -> Option<Vec<FoldingRange>> {
    let tree = tree.as_ref()?;
    query_cursor.set_point_range(
        Point { row: 0, column: 0 }..Point {
            row: usize::MAX,
            column: usize::MAX,
        },
    );

    let mut iter = query_cursor.matches(
        folding_ranges_query,
        tree.root_node(),
        RopeProvider(rope.slice(..)),
    );

    let mut locations = Vec::new();

    while let Some(query_match) = iter.next() {
        let range = char_range_from_byte_range(query_match.captures[0].node.range(), rope).ok()?;
        locations.push(FoldingRange {
            start_line: range.start.line,
            start_character: Some(range.start.character),
            end_line: range.end.line - 1,
            end_character: None,
            kind: Some(lsp_types::FoldingRangeKind::Region),
            collapsed_text: None,
        });
    }

    Some(locations)
}
