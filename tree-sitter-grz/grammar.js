module.exports = grammar({
  name: 'grz',

  extras: $ => [
    $.whitespace,
    $.comment
  ],

  externals: $ => [
    $.string_content,
    $.raw_string_content,
    $.obj_other,
  ],


  word: $ => $.identifier,

  rules: {
    source_file: $ => repeat($._definition),

    whitespace: _ => /(\s|\\\r?\n)+/,

    _definition: $ =>
      choice(
        $.slide,
        $.viewbox,
        $.obj,
        $.register,
        $.slide_functions,
        // Only used in LSP
        $.completion
      ),

    completion: $ => seq($.identifier, '.', $.identifier),

    // C Identifier
    identifier: _ =>
      /(\p{XID_Start}|_|\\u[0-9A-Fa-f]{4}|\\U[0-9A-Fa-f]{8})(\p{XID_Continue}|\\u[0-9A-Fa-f]{4}|\\U[0-9A-Fa-f]{8})*/,

    // C style escapes
    escape_sequence: _ => token(prec(1, seq(
      '\\',
      choice(
        /[^xuU]/,
        /\d{2,3}/,
        /x[0-9a-fA-F]{2,}/,
        /u[0-9a-fA-F]{4}/,
        /U[0-9a-fA-F]{8}/,
      ),
    ))),

    // Rust style string
    _simple_string_literal: $ => seq(
      alias(/b?"/, '"'),
      repeat(choice(
        $.escape_sequence,
        $.string_content,
      )),
      token.immediate('"'),
    ),

    _raw_string_literal: $ => seq(
      'r#"',
      optional($.raw_string_content),
      token.immediate('"#')
    ),

    string_literal: $ => choice(
      $._simple_string_literal,
      $._raw_string_literal,
    ),

    // C style numbers
    number_literal: _ => {
      const separator = '\'';
      const hex = /[0-9a-fA-F]/;
      const decimal = /[0-9]/;
      const hexDigits = seq(repeat1(hex), repeat(seq(separator, repeat1(hex))));
      const decimalDigits = seq(repeat1(decimal), repeat(seq(separator, repeat1(decimal))));
      return token(seq(
        optional(/[-\+]/),
        optional(choice(/0[xX]/, /0[bB]/)),
        choice(
          seq(
            choice(
              decimalDigits,
              seq(/0[bB]/, decimalDigits),
              seq(/0[xX]/, hexDigits),
            ),
            optional(seq('.', optional(hexDigits))),
          ),
          seq('.', decimalDigits),
        ),
        optional(seq(
          /[eEpP]/,
          optional(seq(
            optional(/[-\+]/),
            hexDigits,
          )),
        )),
        /[uUlLwWfFbBdD]*/,
      ));
    },

    integer_literal: _ => /[0-9][0-9_]*/,

    _text_ident: $ => choice($.string_literal, $.number_literal),
    index_parser: $ => seq('[', $.number_literal, ']'),

    edge_parser: _ => /[><^_.|]+/,

    _vb_identifier: $ => choice(
      seq(
        field("viewbox", choice(alias('Size', $.size), $.identifier)),
        field('viewbox_index', $.index_parser)
      ),
      seq(
        alias('()', $.inherit),
        optional(field('viewbox_index', $.index_parser))
      )
    ),

    slide_from: $ => seq(
      '{', $._vb_identifier, '}'
    ),

    slide_vb: $ => choice(
      seq(
        ':',
        $._vb_identifier,
      ),
      seq('|', 
        $._vb_identifier,
        field('body', $.viewbox_inner),
        field('viewbox_index', $.index_parser),
      ),
      '~'
    ),

    range: $ => choice(
      seq($.integer_literal, choice('..', '...', '..='), $.integer_literal),
      seq($.integer_literal, '..'),
      seq('..', $.integer_literal),
      '..',
    ),


    slide_obj: $ => seq(
      field('object', $.identifier),
      optional(seq('[', field('range', $.range), ']')),
      optional($.slide_vb),
      optional($.slide_from),
      optional($.edge_parser),
    ),

    slide_objects: $ => seq(
      '{',
      field('objects',
        sep($.slide_obj, ',')
      ),
      optional(','),
      '}'
    ),

    slide_function: $ => seq(field('function', $.identifier), '(',
      sep(choice($.identifier, $.string_literal, $.number_literal), ','),
      optional(','),
      ')'
    ),

    slide_functions: $ => seq('[',
      sep(
        $.slide_function,
        ','
      ),
      optional(','),
      ']'
    ),

    slide: $ => seq(
      $.slide_objects,
      $.slide_functions
    ),

    operation: _ => /[%\+~-]/,

    viewbox_obj: $ =>
      seq(
        field('value', $.number_literal),
        choice(
          field('operation', $.operation),
          seq(field('operation', ':'), field('denominator', $.number_literal))
        )
      ),

    direction: _ => /[\^_<>]/,

    viewbox_inner: $ => seq(
      field('direction', $.direction),
      sep($.viewbox_obj, ','),
      optional(','),
      ']'
    ),

    viewbox: $ =>
      seq(
        field('name', $.identifier),
        ':',
        $._vb_identifier,
        field('body', $.viewbox_inner)
      ),

    obj_param: $ => seq(field('key', $.identifier), ':', field('value', choice($._text_ident, $.obj_other))),

    obj_inner: $ => seq(
      '(',
      sep($.obj_param, ','),
      optional(','),
      ')'
    ),

    obj: $ => seq(
      field('name', $.identifier),
      ':',
      field('ty', $.identifier),
      $.obj_inner
    ),

    register: $ => seq('<', $.obj_param, '>'),

    comment: _ => token(choice(
      seq('//', /(\\+(.|\r?\n)|[^\\\n])*/),
      seq(
        '/*',
        /[^*]*\*+([^/*][^*]*\*+)*/,
        '/',
      ),
    )),
  }
});

function sep(rule, char) {
  return optional(sep1(rule, char));
}

/**
 * Creates a rule to match one or more of the rules separated by a comma
 *
 * @param {Rule} rule
 * @param {char} char
 * @return {SeqRule}
 *
 */
function sep1(rule, char) {
  return seq(rule, repeat(seq(char, rule)));
}
