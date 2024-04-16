(obj
  name: (identifier) @variable.declaration
  ty: (identifier) @type)
(viewbox
  name: (identifier) @variable.declaration)
(viewbox_obj
  operation: (_) @operator)
(viewbox_inner
  direction: (direction) @modifier)
(slide_function
  function: (identifier) @function)

(edge_parser) @modifier
(identifier) @variable
(comment) @comment
(number_literal) @number
(integer_literal) @number
(string_literal) @string
(size) @keyword
".." @operator
"..=" @operator
