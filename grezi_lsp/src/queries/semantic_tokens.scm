(obj
  name: (identifier) @variable.declaration)
(viewbox
  name: (identifier) @variable.declaration)
(obj
  (obj_inner
    ty: (identifier) @type))
(obj
  (obj_inner
    (obj_param
      key: (identifier) @parameter)))
(viewbox_obj
  operation: (_) @operator)
(viewbox_obj
  operation: (":") @operator)
(slide_function
  function: (identifier) @function)

(identifier) @variable
(direction) @modifier
(comment) @comment
(number_literal) @number
(integer_literal) @number
(string_literal) @string
(from_last_slide) @operator
(size) @keyword

".." @operator
"..=" @operator
