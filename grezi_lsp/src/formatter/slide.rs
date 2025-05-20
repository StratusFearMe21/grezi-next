// Properly formatted slide
//
// <slide srow="68" scol="0" erow="71" ecol="3">
// <slide_objects srow="68" scol="0" erow="71" ecol="1">
//   {
//   <whitespace srow="68" scol="1" erow="69" ecol="4">
// </whitespace>
//   <slide_obj field="objects" srow="69" scol="4" erow="69" ecol="32">
//     <identifier field="object" srow="69" scol="4" erow="69" ecol="9">Title</identifier>
//     <slide_vb srow="69" scol="9" erow="69" ecol="28">
//       :
//       <whitespace srow="69" scol="10" erow="69" ecol="11"> </whitespace>
//       <vb_ref srow="69" scol="11" erow="69" ecol="28">
//         <identifier field="viewbox" srow="69" scol="11" erow="69" ecol="25">VerticalHalves</identifi
// er>
//         <index_parser field="viewbox_index" srow="69" scol="25" erow="69" ecol="28">
//           [
//           <number_literal srow="69" scol="26" erow="69" ecol="27">0</number_literal>
//           ]
//         </index_parser>
//       </vb_ref>
//     </slide_vb>
//     <edge_parser srow="69" scol="28" erow="69" ecol="32">
//       <edge srow="69" scol="28" erow="69" ecol="30">
//         <direction srow="69" scol="28" erow="69" ecol="29">
//           _
//         </direction>
//         <direction srow="69" scol="29" erow="69" ecol="30">
//           _
//         </direction>
//       </edge>
//       <edge srow="69" scol="30" erow="69" ecol="32">
//         <direction srow="69" scol="30" erow="69" ecol="31">
//           .
//         </direction>
//         <direction srow="69" scol="31" erow="69" ecol="32">
//           .
//         </direction>
//       </edge>
//     </edge_parser>
//   </slide_obj>
//   ,
//   <whitespace srow="69" scol="33" erow="70" ecol="4">
// </whitespace>
//   <slide_obj field="objects" srow="70" scol="4" erow="70" ecol="35">
//     <identifier field="object" srow="70" scol="4" erow="70" ecol="12">Subtitle</identifier>
//     <slide_vb srow="70" scol="12" erow="70" ecol="31">
//       :
//       <whitespace srow="70" scol="13" erow="70" ecol="14"> </whitespace>
//       <vb_ref srow="70" scol="14" erow="70" ecol="31">
//         <identifier field="viewbox" srow="70" scol="14" erow="70" ecol="28">VerticalHalves</identifi
// er>
// <index_parser field="viewbox_index" srow="70" scol="28" erow="70" ecol="31">
//           [
//           <number_literal srow="70" scol="29" erow="70" ecol="30">1</number_literal>
//           ]
//         </index_parser>
//       </vb_ref>
//     </slide_vb>
//     <edge_parser srow="70" scol="31" erow="70" ecol="35">
//       <edge srow="70" scol="31" erow="70" ecol="33">
//         <direction srow="70" scol="31" erow="70" ecol="32">
//           ^
//         </direction>
//         <direction srow="70" scol="32" erow="70" ecol="33">
//           ^
//         </direction>
//       </edge>
//       <edge srow="70" scol="33" erow="70" ecol="35">
//         <direction srow="70" scol="33" erow="70" ecol="34">
//           .
//         </direction>
//         <direction srow="70" scol="34" erow="70" ecol="35">
//           .
//         </direction>
//       </edge>
//     </edge_parser>
//   </slide_obj>
//   ,
//   <whitespace srow="70" scol="36" erow="71" ecol="0">
// </whitespace>
//   }
// </slide_objects>
// <slide_functions srow="71" scol="1" erow="71" ecol="3">
//   [
//   ]
// </slide_functions>
// </slide>

use ropey::Rope;
use tracing::instrument;
use tree_sitter_grz::NodeKind;

use super::{
    actions::format_slide_functions,
    viewbox::{format_vb_ref, format_viewbox_inner},
};

#[instrument(skip_all)]
pub fn format_slide(current_rope: &Rope, cursor: &mut super::FormattingCursor) -> Result<(), ()> {
    // <slide srow="68" scol="0" erow="71" ecol="3">
    cursor.goto_first_child(
        super::WhitespaceEdit::Delete,
        NodeKind::SymSlide,
        current_rope,
    )?;

    format_slide_objects(current_rope, cursor)?;
    cursor.goto_next_sibling(super::WhitespaceEdit::Delete, current_rope)?;
    format_slide_functions(current_rope, cursor)?;

    cursor.goto_parent();
    Ok(())
}

#[instrument(skip_all)]
pub fn format_slide_objects(
    current_rope: &Rope,
    cursor: &mut super::FormattingCursor,
) -> Result<(), ()> {
    // <slide_objects srow="68" scol="0" erow="71" ecol="1">
    cursor.goto_first_child(
        super::WhitespaceEdit::Delete,
        NodeKind::SymSlideObjects,
        current_rope,
    )?;

    //   {
    cursor.goto_next_sibling(super::WhitespaceEdit::Delete, current_rope)?;
    if cursor.node().kind_id() == NodeKind::SymSlideObj as u16 {
        cursor.revisit(super::WhitespaceEdit::Assert("\n    "), current_rope)?;
        while cursor.node().kind_id() == NodeKind::SymSlideObj as u16 {
            format_slide_obj(current_rope, cursor)?;
            //   ,
            cursor.goto_next_sibling(super::WhitespaceEdit::Trailing(","), current_rope)?;
            cursor.goto_next_sibling(super::WhitespaceEdit::Assert("\n    "), current_rope)?;
        }
        //   }
        cursor.revisit(super::WhitespaceEdit::Assert("\n"), current_rope)?;
    }

    cursor.goto_parent();
    Ok(())
}

#[instrument(skip_all)]
pub fn format_slide_obj(
    current_rope: &Rope,
    cursor: &mut super::FormattingCursor,
) -> Result<(), ()> {
    //   <slide_obj field="objects" srow="69" scol="4" erow="69" ecol="32">
    cursor.goto_first_child(
        super::WhitespaceEdit::Delete,
        NodeKind::SymSlideObj,
        current_rope,
    )?;

    //     <identifier field="object" srow="69" scol="4" erow="69" ecol="9">Title</identifier>
    cursor.goto_next_sibling(super::WhitespaceEdit::Delete, current_rope)?;
    if cursor.node().kind_id() == NodeKind::SymSlideVb as u16 {
        format_slide_vb(current_rope, cursor)?;
        cursor.goto_next_sibling(super::WhitespaceEdit::Delete, current_rope)?;
    }
    if cursor.node().kind_id() == NodeKind::SymEdgeParser as u16 {
        //     <edge_parser srow="69" scol="28" erow="69" ecol="32">
        cursor.goto_first_child(
            super::WhitespaceEdit::Delete,
            NodeKind::SymEdgeParser,
            current_rope,
        )?;
        format_edge(current_rope, cursor)?;
        if cursor.goto_next_sibling(super::WhitespaceEdit::Delete, current_rope)? {
            format_edge(current_rope, cursor)?;
        }
        cursor.goto_parent();
    }

    cursor.goto_parent();
    Ok(())
}

#[instrument(skip_all)]
pub fn format_slide_vb(
    current_rope: &Rope,
    cursor: &mut super::FormattingCursor,
) -> Result<(), ()> {
    //     <slide_vb srow="69" scol="9" erow="69" ecol="28">
    cursor.goto_first_child(
        super::WhitespaceEdit::Delete,
        NodeKind::SymSlideVb,
        current_rope,
    )?;

    match NodeKind::from(cursor.node().kind_id()) {
        NodeKind::AnonSymTILDE => {
            cursor.goto_next_sibling(super::WhitespaceEdit::Delete, current_rope)?;
        }
        NodeKind::AnonSymCOLON => {
            cursor.goto_next_sibling(super::WhitespaceEdit::Assert(" "), current_rope)?;
            format_vb_ref(current_rope, cursor)?;
        }
        NodeKind::AnonSymPIPE => {
            cursor.goto_next_sibling(super::WhitespaceEdit::Assert(" "), current_rope)?;
            format_vb_ref(current_rope, cursor)?;
            cursor.goto_next_sibling(super::WhitespaceEdit::Assert(" "), current_rope)?;
            format_viewbox_inner(current_rope, cursor, "\n        ", "\n    ")?;
            cursor.goto_next_sibling(super::WhitespaceEdit::Delete, current_rope)?;
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
        }
        _ => {}
    }

    cursor.goto_parent();
    Ok(())
}

#[instrument(skip_all)]
pub fn format_edge(current_rope: &Rope, cursor: &mut super::FormattingCursor) -> Result<(), ()> {
    //       <edge srow="69" scol="28" erow="69" ecol="30">
    cursor.goto_first_child(
        super::WhitespaceEdit::Delete,
        NodeKind::SymEdge,
        current_rope,
    )?;
    //         <direction srow="69" scol="28" erow="69" ecol="29">
    //           _
    //         </direction>
    cursor.goto_next_sibling(super::WhitespaceEdit::Delete, current_rope)?;
    //         <direction srow="69" scol="29" erow="69" ecol="30">
    //           _
    //         </direction>
    cursor.goto_next_sibling(super::WhitespaceEdit::Delete, current_rope)?;
    cursor.goto_parent();
    Ok(())
}
