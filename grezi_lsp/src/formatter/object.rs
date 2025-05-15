use ropey::Rope;
use tracing::instrument;
use tree_sitter_grz::NodeKind;

// Properly formatted object:
//
// <obj srow="39" scol="0" erow="42" ecol="1">
// <identifier field="name" srow="39" scol="0" erow="39" ecol="13">TemplateImage</identifier>
// :
// <whitespace srow="39" scol="14" erow="39" ecol="15"> </whitespace>
// <obj_inner srow="39" scol="15" erow="42" ecol="1">
//   <identifier field="ty" srow="39" scol="15" erow="39" ecol="20">Image</identifier>
//   (
//   <whitespace srow="39" scol="21" erow="40" ecol="4">
// </whitespace>
//   <obj_param srow="40" scol="4" erow="40" ecol="34">
//     <identifier field="key" srow="40" scol="4" erow="40" ecol="9">value</identifier>
//     :
//     <whitespace srow="40" scol="10" erow="40" ecol="11"> </whitespace>
//     <string_literal field="value" srow="40" scol="11" erow="40" ecol="34">
//       "
//       <string_content srow="40" scol="12" erow="40" ecol="33">file:templatemain.png</string_content>
//       "
//     </string_literal>
//   </obj_param>
//   ,
//   <whitespace srow="40" scol="35" erow="41" ecol="4">
// </whitespace>
//   <obj_param srow="41" scol="4" erow="41" ecol="15">
//     <identifier field="key" srow="41" scol="4" erow="41" ecol="9">scale</identifier>
//     :
//     <whitespace srow="41" scol="10" erow="41" ecol="11"> </whitespace>
//     <number_literal field="value" srow="41" scol="11" erow="41" ecol="15">2160</number_literal>
//   </obj_param>
//   ,
//   <whitespace srow="41" scol="16" erow="42" ecol="0">
// </whitespace>
//   )
// </obj_inner>
// </obj>

#[instrument(skip_all)]
pub fn format_object(current_rope: &Rope, cursor: &mut super::FormattingCursor) -> Result<(), ()> {
    // <obj srow="39" scol="0" erow="42" ecol="1">
    cursor.goto_first_child(
        super::WhitespaceEdit::Delete,
        NodeKind::SymObj,
        current_rope,
    )?;

    // <identifier field="name" srow="39" scol="0" erow="39" ecol="13">TemplateImage</identifier>
    cursor.goto_next_sibling(super::WhitespaceEdit::Delete, current_rope)?;
    // :
    cursor.goto_next_sibling(super::WhitespaceEdit::Assert(" "), current_rope)?;

    format_obj_inner(current_rope, cursor)?;

    cursor.goto_parent();
    Ok(())
}

#[instrument(skip_all)]
pub fn format_obj_inner(
    current_rope: &Rope,
    cursor: &mut super::FormattingCursor,
) -> Result<(), ()> {
    // <obj_inner srow="39" scol="15" erow="42" ecol="1">
    cursor.goto_first_child(
        super::WhitespaceEdit::Delete,
        NodeKind::SymObjInner,
        current_rope,
    )?;

    //   <identifier field="ty" srow="39" scol="15" erow="39" ecol="20">Image</identifier>
    cursor.goto_next_sibling(super::WhitespaceEdit::Delete, current_rope)?;
    //   (
    cursor.goto_next_sibling(super::WhitespaceEdit::Assert("\n    "), current_rope)?;
    while cursor.node().kind_id() == NodeKind::SymObjParam as u16 {
        format_obj_param(current_rope, cursor)?;
        //   ,
        cursor.goto_next_sibling(super::WhitespaceEdit::Trailing(","), current_rope)?;
        cursor.goto_next_sibling(super::WhitespaceEdit::Assert("\n    "), current_rope)?;
    }
    //   )
    cursor.revisit(super::WhitespaceEdit::Assert("\n"), current_rope)?;

    cursor.goto_parent();
    Ok(())
}

#[instrument(skip_all)]
pub fn format_obj_param(
    current_rope: &Rope,
    cursor: &mut super::FormattingCursor,
) -> Result<(), ()> {
    //   <obj_param srow="40" scol="4" erow="40" ecol="34">
    cursor.goto_first_child(
        super::WhitespaceEdit::Delete,
        NodeKind::SymObjParam,
        current_rope,
    )?;

    //     <identifier field="key" srow="40" scol="4" erow="40" ecol="9">value</identifier>
    cursor.goto_next_sibling(super::WhitespaceEdit::Delete, current_rope)?;
    if cursor.node().kind_id() == NodeKind::AnonSymCOLON as u16 {
        //     :
        cursor.goto_next_sibling(super::WhitespaceEdit::Assert(" "), current_rope)?;
    }
    //     <string_literal field="value" srow="40" scol="11" erow="40" ecol="34">
    cursor.goto_next_sibling(super::WhitespaceEdit::Delete, current_rope)?;

    cursor.goto_parent();
    Ok(())
}
