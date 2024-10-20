use std::{borrow::Cow, collections::HashMap, hash::BuildHasherDefault};

use helix_core::{
    syntax::RopeProvider,
    tree_sitter::{Point, Query, QueryCursor, Tree},
    Rope, RopeSlice,
};
use lsp_types::{InlayHint, InlayHintKind, InlayHintLabel};

use crate::parser::NodeKind;

use super::{formatter::char_pos_from_byte_pos, you_can};

pub fn inlay_hints(
    inlay_edge_query: &Query,
    inlay_edge_map: &mut HashMap<
        RopeSlice<'static>,
        RopeSlice<'static>,
        BuildHasherDefault<ahash::AHasher>,
    >,
    inlay_vb_map: &mut HashMap<
        RopeSlice<'static>,
        Cow<'_, str>,
        BuildHasherDefault<ahash::AHasher>,
    >,
    current_rope: &Rope,
    tree: &Tree,
    last_inlay_len: usize,
    query_cursor: &mut QueryCursor,
) -> Vec<InlayHint> {
    let mut hints = Vec::with_capacity(last_inlay_len);

    query_cursor.set_point_range(
        Point { row: 0, column: 0 }..Point {
            row: usize::MAX,
            column: usize::MAX,
        },
    );

    let mut slide_num = 0;

    let edge_iter = query_cursor.matches(
        inlay_edge_query,
        tree.root_node(),
        RopeProvider(current_rope.slice(..)),
    );

    let mut in_slide = false;
    for query_match in edge_iter {
        match query_match.pattern_index {
            0 => {
                let query_node = query_match.captures[0].node;
                let query_slice = unsafe {
                    you_can::borrow_unchecked(
                        current_rope.byte_slice(query_match.captures[0].node.byte_range()),
                    )
                };

                let mut vb = None;
                let mut edge = query_match.captures.get(2).map(|capture| capture.node);

                if let Some(v) = query_match.captures.get(1) {
                    match NodeKind::from(v.node.kind_id()) {
                        NodeKind::SlideVb => vb = Some(v.node),
                        NodeKind::EdgeParser => edge = Some(v.node),
                        _ => unreachable!(),
                    }
                }

                let mut walker = query_node.parent().unwrap().walk();
                while walker.goto_next_sibling() {}
                let range = walker.node().range();
                let mut position = char_pos_from_byte_pos(range.end_point, current_rope).unwrap();
                let mut hint = String::new();

                if let Some(vb) = vb {
                    let mut vb: Cow<'_, str> = unsafe {
                        you_can::borrow_unchecked(current_rope.byte_slice(vb.byte_range()))
                    }
                    .into();
                    if vb.starts_with('|') {
                        vb = Cow::Borrowed(": InlineVb[_]");
                        inlay_vb_map.insert(query_slice, vb);
                    } else if vb.starts_with('~') {
                        if let Some(vb) = inlay_vb_map.get(&query_slice) {
                            std::fmt::Write::write_fmt(&mut hint, format_args!("{}", &vb[1..]))
                                .unwrap();
                        }
                    } else {
                        inlay_vb_map.insert(query_slice, vb);
                    }
                } else {
                    let entry = inlay_vb_map
                        .entry(query_slice)
                        .or_insert(Cow::Borrowed(": Unknown[_]"));

                    std::fmt::Write::write_fmt(&mut hint, format_args!("{}", entry)).unwrap();
                }

                if let Some(edge) = edge {
                    let slice = unsafe {
                        you_can::borrow_unchecked(current_rope.byte_slice(edge.byte_range()))
                    };
                    let entry = inlay_edge_map.entry(query_slice).or_insert_with(|| {
                        if edge.byte_range().len() == 4 {
                            slice.byte_slice(2..)
                        } else {
                            slice
                        }
                    });

                    let range = edge.range();
                    position = char_pos_from_byte_pos(range.start_point, current_rope).unwrap();

                    if slice.len_chars() < 3 {
                        std::fmt::Write::write_fmt(&mut hint, format_args!("{}", entry)).unwrap();
                        *entry = slice;
                    } else {
                        *entry = slice.byte_slice(2..);
                    }
                } else {
                    let entry = inlay_edge_map.entry(query_slice);
                    let entry = entry.or_insert_with(|| RopeSlice::from(""));

                    std::fmt::Write::write_fmt(&mut hint, format_args!("{}{}", entry, entry))
                        .unwrap();
                }

                if !hint.is_empty() {
                    hints.push(InlayHint {
                        position,
                        // This parameter must NEVER be the actual borrowed slice
                        label: InlayHintLabel::String(hint),
                        kind: Some(InlayHintKind::PARAMETER),
                        text_edits: None,
                        tooltip: None,
                        padding_right: Some(false),
                        padding_left: Some(false),
                        data: None,
                    });
                }
            }
            1 => {
                in_slide = true;
                slide_num += 1;
                let range = query_match.captures[0].node.range();
                hints.push(InlayHint {
                    position: char_pos_from_byte_pos(range.start_point, current_rope).unwrap(),
                    label: InlayHintLabel::String(format!("Slide {}:", slide_num)),
                    kind: Some(InlayHintKind::PARAMETER),
                    text_edits: None,
                    tooltip: None,
                    padding_right: Some(true),
                    padding_left: Some(false),
                    data: None,
                });
            }
            2 => {
                let range = query_match.captures[1].node.range();
                let text = current_rope.byte_slice(query_match.captures[1].node.byte_range());

                let words = text
                    .chars()
                    .fold((false, 0u32), |state, c| {
                        if c.is_whitespace() {
                            if !state.0 {
                                return (true, state.1 + 1);
                            } else {
                                return state;
                            }
                        } else {
                            return (false, state.1);
                        }
                    })
                    .1
                    + 1;

                hints.push(InlayHint {
                    position: char_pos_from_byte_pos(range.end_point, current_rope).unwrap(),
                    label: InlayHintLabel::String(format!("{} words", words)),
                    kind: Some(InlayHintKind::PARAMETER),
                    text_edits: None,
                    tooltip: None,
                    padding_right: Some(false),
                    padding_left: Some(true),
                    data: None,
                });
            }
            3 => {
                if in_slide {
                    in_slide = false;
                } else {
                    slide_num += 1;
                }
            }
            n => unreachable!("{}", n),
        }
    }

    {
        inlay_edge_map.clear();
        inlay_vb_map.clear();
    }

    hints
}
