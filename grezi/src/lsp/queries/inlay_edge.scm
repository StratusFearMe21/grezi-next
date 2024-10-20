[
    (slide_obj
        object: (identifier) @name)
    (slide_obj
        object: (identifier) @name
        (slide_vb) @vb)
    (slide_obj
        object: (identifier) @name
        (edge_parser) @edge)
    (slide_obj
        object: (identifier) @name
        (slide_vb) @vb
        (edge_parser) @edge)
]
(slide) @slide
(obj_param
    key: (identifier) @key
    value: (string_literal) @value
    (#eq? @key "value"))
(slide_functions) @slide_functions
