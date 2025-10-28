// Properly formatted register:
//
// <register srow="0" scol="0" erow="0" ecol="12">
// &lt;
// <obj_param srow="0" scol="1" erow="0" ecol="11">
//   <identifier field="key" srow="0" scol="1" erow="0" ecol="7">MARGIN</identifier>
//   :
//   <whitespace srow="0" scol="8" erow="0" ecol="9"> </whitespace>
//   <number_literal field="value" srow="0" scol="9" erow="0" ecol="11">50</number_l
// iteral>
// </obj_param>
// &gt;
// </register>

use ropey::Rope;
use tracing::instrument;
use tree_sitter_grz::NodeKind;

use super::object::format_obj_param;

#[instrument(skip_all)]
pub fn format_registers(
    current_rope: &Rope,
    cursor: &mut super::FormattingCursor,
) -> Result<(), ()> {
    // <register srow="0" scol="0" erow="0" ecol="12">
    cursor.goto_first_child(
        super::WhitespaceEdit::Delete,
        NodeKind::SymRegister,
        current_rope,
    )?;

    // &lt;
    cursor.goto_next_sibling(super::WhitespaceEdit::Delete, current_rope)?;
    // <obj_param srow="0" scol="1" erow="0" ecol="11">
    format_obj_param(current_rope, cursor)?;
    // &gt;
    cursor.goto_next_sibling(super::WhitespaceEdit::Delete, current_rope)?;

    cursor.goto_parent();
    Ok(())
}
