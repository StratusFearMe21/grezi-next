(obj
  name: (identifier) @variable.declaration
  ty: (identifier) @type)
(viewbox
  name: (identifier) @variable.declaration)
(viewbox_obj
  operation: (operation) @operator)
(viewbox_obj
  operation: (":") @operator)
(viewbox_inner
  direction: (direction) @modifier)
(slide_function
  function: (identifier) @function)

(edge_parser) @modifier
(identifier) @variable
(comment) @comment
(escape_sequence) @macro
(number_literal) @number
(string_literal) @string
(size) @keyword
