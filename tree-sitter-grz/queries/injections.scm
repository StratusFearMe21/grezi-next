((obj_param
  key: (identifier) @value_key
  value: (obj_other)) @injection.content
  (#set! injection.include-children)
  (#set! injection.language "css"))

(
  (obj_param
    key: (identifier) @value_key
    value: (string_literal (_) @injection.content))
  (obj_param
    key: (identifier) @lang_key
    value: (string_literal (_) @injection.language))
  (#eq? @value_key "code")
  (#eq? @lang_key "language")
)

(
  (obj_param
    key: (identifier) @lang_key
    value: (string_literal (_) @injection.language))
  (obj_param
    key: (identifier) @value_key
    value: (string_literal (_) @injection.content))
  (#eq? @value_key "code")
  (#eq? @lang_key "language")
)

((obj_param
  value: (string_literal (_) @injection.content))
  (#set! injection.language "djot"))
