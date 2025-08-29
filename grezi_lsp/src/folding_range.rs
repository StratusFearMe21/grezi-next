use helix_core::{
    Rope,
    tree_sitter::{InactiveQueryCursor, Node, Query, RopeInput},
};
use helix_lsp_types as lsp_types;
use lsp_types::FoldingRange;

use super::formatter::char_range_from_byte_range;

pub fn folding_ranges(
    rope: &Rope,
    node: Option<Node>,
    folding_ranges_query: &Query,
) -> Option<Vec<FoldingRange>> {
    let node = node.as_ref()?;

    let mut iter = InactiveQueryCursor::default().execute_query(
        folding_ranges_query,
        node,
        RopeInput::new(rope.slice(..)),
    );

    let mut locations = Vec::new();

    while let Some(query_match) = iter.next_match() {
        let range =
            char_range_from_byte_range(query_match.matched_node(0).node.range(), rope).ok()?;
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
