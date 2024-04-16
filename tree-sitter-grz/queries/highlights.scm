(obj
  ty: (identifier) @type.builtin)
(viewbox_obj
  operation: (_) @keyword.operator)
(viewbox_obj
  operation: (":") @keyword.operator)
(slide_function
  function: (identifier) @function)

(direction) @keyword.control
(edge_parser) @keyword.control
(escape_sequence) @constant.character.escape
(identifier) @variable
(comment) @comment
(number_literal) @constant.numeric.integer
(integer_literal) @constant.numeric.integer
(string_literal) @string
(size) @variable.builtin

"(" @punctuation.bracket
")" @punctuation.bracket
"[" @punctuation.bracket
"]" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket
"," @punctuation.delimiter
":" @punctuation.delimiter
".." @operator
"..=" @operator

