module.exports = grammar({
  name: 'ntbib',

  extras: _ => [],

  rules: {
    // TODO: add the actual grammar rules
    source_file: $ => repeat($.element),

    element: $ => seq(choice($.tag_start, $.tag_end), $.content),

    content: _ => /[^<]*/,
    tag_start: $ => seq('<', $.tag, '>'),
    tag_end: $ => seq('</', $.tag, '>'),
    tag: _ => /[^>\/]*/
  }
});

