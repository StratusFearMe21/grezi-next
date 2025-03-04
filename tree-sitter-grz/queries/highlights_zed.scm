(obj
  ty: (identifier) @type)
(viewbox_obj
  operation: (_) @operator)
(viewbox_obj
  operation: (":") @operator)
(slide_function
  function: (identifier) @function)

(direction) @punctuation.special
(edge_parser) @punctuation.special
(escape_sequence) @string.escape
(identifier) @variable
(comment) @comment
(number_literal) @number
(integer_literal) @number
(string_literal) @string
(size) @keyword

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

