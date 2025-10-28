// Properly formatted actions
//
// <actions srow="514" scol="0" erow="517" ecol="1">
// <slide_functions srow="514" scol="0" erow="517" ecol="1">
//   [
//   <whitespace srow="514" scol="1" erow="515" ecol="4">
// </whitespace>
//   <slide_function srow="515" scol="4" erow="515" ecol="41">
//     <identifier field="function" srow="515" scol="4" erow="515" ecol="13">highlight</identifier>
//     (
//     <identifier srow="515" scol="14" erow="515" ecol="25">StringClass</identifier>
//     ,
//     <whitespace srow="515" scol="26" erow="515" ecol="27"> </whitespace>
//     <string_literal srow="515" scol="27" erow="515" ecol="32">
//       "
//       <string_content srow="515" scol="28" erow="515" ecol="31">8:4</string_content>
//       "
//     </string_literal>
//     ,
//     <whitespace srow="515" scol="33" erow="515" ecol="34"> </whitespace>
//     <string_literal srow="515" scol="34" erow="515" ecol="40">
//       "
//       <string_content srow="515" scol="35" erow="515" ecol="39">8:27</string_content>
//       "
//     </string_literal>
//     )
//   </slide_function>
//   ,
//   <whitespace srow="515" scol="42" erow="516" ecol="4">
// </whitespace>
//   <slide_function srow="516" scol="4" erow="516" ecol="24">
//     <identifier field="function" srow="516" scol="4" erow="516" ecol="13">highlight</identifier>
//     (
//     <identifier srow="516" scol="14" erow="516" ecol="23">FoobarCap</identifier>
//     )
//   </slide_function>
//   ,
//   <whitespace srow="516" scol="25" erow="517" ecol="0">
// </whitespace>
//   ]
// </slide_functions>
// </actions>

use ropey::Rope;
use tracing::instrument;
use tree_sitter_grz::NodeKind;

#[instrument(skip_all)]
pub fn format_actions(current_rope: &Rope, cursor: &mut super::FormattingCursor) -> Result<(), ()> {
    // <actions srow="514" scol="0" erow="517" ecol="1">
    cursor.goto_first_child(
        super::WhitespaceEdit::Delete,
        NodeKind::SymActions,
        current_rope,
    )?;

    format_slide_functions(current_rope, cursor)?;

    cursor.goto_parent();
    Ok(())
}

#[instrument(skip_all)]
pub fn format_slide_functions(
    current_rope: &Rope,
    cursor: &mut super::FormattingCursor,
) -> Result<(), ()> {
    // <slide_functions srow="514" scol="0" erow="517" ecol="1">
    cursor.goto_first_child(
        super::WhitespaceEdit::Delete,
        NodeKind::SymSlideFunctions,
        current_rope,
    )?;

    //   [
    cursor.goto_next_sibling(super::WhitespaceEdit::Delete, current_rope)?;
    if cursor.node().kind_id() == NodeKind::SymSlideFunction as u16 {
        cursor.revisit(super::WhitespaceEdit::Assert("\n    "), current_rope)?;
        while cursor.node().kind_id() == NodeKind::SymSlideFunction as u16 {
            format_slide_function(current_rope, cursor)?;
            //   ,
            cursor.goto_next_sibling(super::WhitespaceEdit::Trailing(","), current_rope)?;
            cursor.goto_next_sibling(super::WhitespaceEdit::Assert("\n    "), current_rope)?;
        }
        //   ]
        cursor.revisit(super::WhitespaceEdit::Assert("\n"), current_rope)?;
    }

    cursor.goto_parent();
    Ok(())
}

#[instrument(skip_all)]
pub fn format_slide_function(
    current_rope: &Rope,
    cursor: &mut super::FormattingCursor,
) -> Result<(), ()> {
    //   <slide_function srow="515" scol="4" erow="515" ecol="41">
    cursor.goto_first_child(
        super::WhitespaceEdit::Delete,
        NodeKind::SymSlideFunction,
        current_rope,
    )?;
    //     <identifier field="function" srow="515" scol="4" erow="515" ecol="13">highlight</identifier>
    cursor.goto_next_sibling(super::WhitespaceEdit::Delete, current_rope)?;
    //     (
    cursor.goto_next_sibling(super::WhitespaceEdit::Delete, current_rope)?;
    while cursor.node().kind_id() != NodeKind::AnonSymRPAREN as u16 {
        //     <identifier srow="515" scol="14" erow="515" ecol="25">StringClass</identifier>
        cursor.goto_next_sibling(super::WhitespaceEdit::Delete, current_rope)?;
        //     ,
        cursor.goto_next_sibling(super::WhitespaceEdit::Assert(" "), current_rope)?;
    }
    cursor.revisit(super::WhitespaceEdit::Delete, current_rope)?;
    //     )
    // cursor.goto_next_sibling(super::WhitespaceEdit::Delete, current_rope)?;

    cursor.goto_parent();
    Ok(())
}
