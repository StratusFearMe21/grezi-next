// Properly formatted viewbox:
//
// <viewbox srow="2" scol="0" erow="5" ecol="1">
// <identifier field="name" srow="2" scol="0" erow="2" ecol="7">ViewBox</identifier>
// :
// <whitespace srow="2" scol="8" erow="2" ecol="9"> </whitespace>
// <vb_ref srow="2" scol="9" erow="2" ecol="16">
// <vb_rect field="viewbox" srow="9" scol="10" erow="9" ecol="42">
//   [
//   <vb_rect_part srow="9" scol="11" erow="9" ecol="24">
//     [
//     <number_literal srow="9" scol="12" erow="9" ecol="17">313.6</number_literal>
//     <whitespace srow="9" scol="17" erow="9" ecol="18"> </whitespace>
//     <number_literal srow="9" scol="18" erow="9" ecol="23">263.3</number_literal>
//     ]
//   </vb_rect_part>
//   <whitespace srow="9" scol="24" erow="9" ecol="25"> </whitespace>
//   -
//   <whitespace srow="9" scol="26" erow="9" ecol="27"> </whitespace>
//   <vb_rect_part srow="9" scol="27" erow="9" ecol="41">
//     [
//     <number_literal srow="9" scol="28" erow="9" ecol="34">1691.1</number_literal>
//     <whitespace srow="9" scol="34" erow="9" ecol="35"> </whitespace>
//     <number_literal srow="9" scol="35" erow="9" ecol="40">913.0</number_literal>
//     ]
//   </vb_rect_part>
//   ]
// </vb_rect>
//   <index_parser field="viewbox_index" srow="2" scol="13" erow="2" ecol="16">
//     [
//     <number_literal srow="2" scol="14" erow="2" ecol="15">0</number_literal>
//     ]
//   </index_parser>
// </vb_ref>
// <whitespace srow="2" scol="16" erow="2" ecol="17"> </whitespace>
// <viewbox_inner field="body" srow="2" scol="17" erow="5" ecol="1">
//   <direction field="direction" srow="2" scol="17" erow="2" ecol="18">
//     ^
//   </direction>
//   <whitespace srow="2" scol="18" erow="3" ecol="4">
// </whitespace>
//   <viewbox_obj srow="3" scol="4" erow="3" ecol="7">
//     <number_literal field="value" srow="3" scol="4" erow="3" ecol="5">1</number_literal>
//     :
//     <number_literal field="denominator" srow="3" scol="6" erow="3" ecol="7">2</number_literal>
//   </viewbox_obj>
//   ,
//   <whitespace srow="3" scol="8" erow="4" ecol="4">
// </whitespace>
//   <viewbox_obj srow="4" scol="4" erow="4" ecol="7">
//     <number_literal field="value" srow="4" scol="4" erow="4" ecol="5">1</number_literal>
//     :
//     <number_literal field="denominator" srow="4" scol="6" erow="4" ecol="7">2</number_literal>
//   </viewbox_obj>
//   ,
//   <whitespace srow="4" scol="8" erow="5" ecol="0">
// </whitespace>
//   ]
// </viewbox_inner>
// </viewbox>

use ropey::Rope;
use tracing::instrument;
use tree_sitter_grz::NodeKind;

#[instrument(skip_all)]
pub fn format_viewbox(current_rope: &Rope, cursor: &mut super::FormattingCursor) -> Result<(), ()> {
    // <viewbox srow="2" scol="0" erow="5" ecol="1">
    cursor.goto_first_child(
        super::WhitespaceEdit::Delete,
        NodeKind::SymViewbox,
        current_rope,
    )?;

    // <identifier field="name" srow="2" scol="0" erow="2" ecol="7">ViewBox</identifier>
    cursor.goto_next_sibling(super::WhitespaceEdit::Delete, current_rope)?;
    // :
    cursor.goto_next_sibling(super::WhitespaceEdit::Assert(" "), current_rope)?;
    format_vb_ref(current_rope, cursor)?;
    cursor.goto_next_sibling(super::WhitespaceEdit::Assert(" "), current_rope)?;
    format_viewbox_inner(current_rope, cursor, "\n    ", "\n")?;

    cursor.goto_parent();
    Ok(())
}

#[instrument(skip_all)]
pub fn format_vb_ref(current_rope: &Rope, cursor: &mut super::FormattingCursor) -> Result<(), ()> {
    // <vb_ref srow="2" scol="9" erow="2" ecol="16">
    cursor.goto_first_child(
        super::WhitespaceEdit::Delete,
        NodeKind::SymVbRef,
        current_rope,
    )?;
    if cursor.node().kind_id() == NodeKind::SymVbRect as u16 {
        format_vb_rect(current_rope, cursor)?;
        cursor.goto_next_sibling(super::WhitespaceEdit::Delete, current_rope)?;
    } else {
        //   <size field="viewbox" srow="2" scol="9" erow="2" ecol="13">Size</size>
        cursor.goto_next_sibling(super::WhitespaceEdit::Delete, current_rope)?;
    }
    //   <index_parser field="viewbox_index" srow="2" scol="13" erow="2" ecol="16">
    cursor.goto_first_child(
        super::WhitespaceEdit::Delete,
        NodeKind::SymIndexParser,
        current_rope,
    )?;
    //     [
    cursor.goto_next_sibling(super::WhitespaceEdit::Delete, current_rope)?;
    //     <number_literal srow="2" scol="14" erow="2" ecol="15">0</number_literal>
    cursor.goto_next_sibling(super::WhitespaceEdit::Delete, current_rope)?;
    //     ]
    cursor.goto_next_sibling(super::WhitespaceEdit::Delete, current_rope)?;

    cursor.goto_parent();
    cursor.goto_parent();
    Ok(())
}

#[instrument(skip_all)]
pub fn format_viewbox_inner(
    current_rope: &Rope,
    cursor: &mut super::FormattingCursor,
    one_tab: &'static str,
    zero_tab: &'static str,
) -> Result<(), ()> {
    // <viewbox_inner field="body" srow="2" scol="17" erow="5" ecol="1">
    cursor.goto_first_child(
        super::WhitespaceEdit::Delete,
        NodeKind::SymViewboxInner,
        current_rope,
    )?;
    //   <direction field="direction" srow="2" scol="17" erow="2" ecol="18">
    //     ^
    //   </direction>
    cursor.goto_next_sibling(super::WhitespaceEdit::Assert(one_tab), current_rope)?;
    while cursor.node().kind_id() == NodeKind::SymViewboxObj as u16 {
        //   <viewbox_obj srow="3" scol="4" erow="3" ecol="7">
        cursor.goto_first_child(
            super::WhitespaceEdit::Delete,
            NodeKind::SymViewboxObj,
            current_rope,
        )?;
        while cursor.goto_next_sibling(super::WhitespaceEdit::Delete, current_rope)? {}
        //   </viewbox_obj>
        cursor.goto_parent();
        //   ,
        cursor.goto_next_sibling(super::WhitespaceEdit::Trailing(","), current_rope)?;
        cursor.goto_next_sibling(super::WhitespaceEdit::Assert(one_tab), current_rope)?;
    }
    cursor.revisit(super::WhitespaceEdit::Assert(zero_tab), current_rope)?;

    cursor.goto_parent();
    Ok(())
}

#[instrument(skip_all)]
pub fn format_vb_rect(current_rope: &Rope, cursor: &mut super::FormattingCursor) -> Result<(), ()> {
    //   <vb_rect field="viewbox" srow="9" scol="10" erow="9" ecol="42">
    cursor.goto_first_child(
        super::WhitespaceEdit::Delete,
        NodeKind::SymVbRect,
        current_rope,
    )?;
    //   [
    cursor.goto_next_sibling(super::WhitespaceEdit::Delete, current_rope)?;
    format_vb_rect_part(current_rope, cursor)?;
    cursor.goto_next_sibling(super::WhitespaceEdit::Assert(" "), current_rope)?;
    //   -
    cursor.goto_next_sibling(super::WhitespaceEdit::Assert(" "), current_rope)?;
    format_vb_rect_part(current_rope, cursor)?;
    //   ]
    cursor.goto_next_sibling(super::WhitespaceEdit::Delete, current_rope)?;

    cursor.goto_parent();
    Ok(())
}

#[instrument(skip_all)]
pub fn format_vb_rect_part(
    current_rope: &Rope,
    cursor: &mut super::FormattingCursor,
) -> Result<(), ()> {
    //   <vb_rect_part srow="9" scol="11" erow="9" ecol="24">
    cursor.goto_first_child(
        super::WhitespaceEdit::Delete,
        NodeKind::SymVbRectPart,
        current_rope,
    )?;
    //     [
    cursor.goto_next_sibling(super::WhitespaceEdit::Delete, current_rope)?;
    //     <number_literal srow="9" scol="12" erow="9" ecol="17">313.6</number_literal>
    cursor.goto_next_sibling(super::WhitespaceEdit::Assert(" "), current_rope)?;
    //     <number_literal srow="9" scol="18" erow="9" ecol="23">263.3</number_literal>
    cursor.goto_next_sibling(super::WhitespaceEdit::Delete, current_rope)?;
    //     ]
    cursor.goto_next_sibling(super::WhitespaceEdit::Delete, current_rope)?;

    cursor.goto_parent();
    Ok(())
}
