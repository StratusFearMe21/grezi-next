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

(identifier) @variable
(direction) @keyword.control
(escape_sequence) @constant.character.escape
(comment) @comment
(number_literal) @constant.numeric.integer
(integer_literal) @constant.numeric.integer
(string_literal) @string
(from_last_slide) @operator
(size) @variable.builtin

(obj
  (obj_inner
    ty: (identifier) @type.builtin))
(obj
  (obj_inner
    (obj_param
      key: (identifier) @variable.other.member)))
(viewbox_obj
  operation: (_) @keyword.operator)
(viewbox_obj
  operation: (":") @keyword.operator)
(slide_function
  function: (identifier) @function)
