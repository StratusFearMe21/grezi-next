module.exports = grammar({
  name: 'grz',

  extras: $ => [
    /\s|\\\r?\n/,
    $.comment
  ],

  word: $ => $.identifier,

  rules: {
    source_file: $ => repeat($._definition),

    _definition: $ =>
        choice(
          $.slide,
          $.viewbox,
          $.obj,
          $.register,
          $.action
        ),

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

    // C style string
    string_literal: $ => seq(
      choice('L"', 'u"', 'U"', 'u8"', '"'),
      repeat(choice(
        alias(token.immediate(prec(1, /[^\\"]+/)), $.string_content),
        $.escape_sequence,
      )),
      '"',
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

    _text_ident: $ => choice($.string_literal, $.number_literal),
    index_parser: $ => seq('[', $.number_literal, ']'),

    edge_parser: _ => /[><^_.|]{2,4}/,

    _vb_identifier: $ => choice(alias('Size', $.size), $.identifier),

    slide_from: $ => seq(
      '{', $._vb_identifier, $.index_parser, '}'
    ),

    slide_obj: $ => seq(
      field('obj_w_viewbox', 
        seq($.identifier, ':', $._vb_identifier)
      ),
      field('viewbox_index', $.index_parser),
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

    viewbox_obj: $ => seq(
      field('value', $.number_literal),
      choice(
        field('operation', $.operation),
        seq(field('operation', ':'), field('denominator', $.number_literal))
      )
    ),

    direction: _ => /[\^_<>]/,

    viewbox_inner: $ => seq(
      field('direction', $.direction),
      field('parameters', sep($.viewbox_obj, ',')),
      optional(','),
      ']'
    ),

    viewbox: $ => 
      seq(
        $.identifier,
        ':',
        $._vb_identifier,
        field('attached_box', $.index_parser),
        field('body', $.viewbox_inner)
      ),

    obj_inner: $ => seq(      
      '(',
      field('parameters', sep(seq($.identifier, ':', $._text_ident), ',')),
      optional(','),
      ')'
    ),

    obj: $ => seq(
      $.identifier,
      ':',
      field('ty', $.identifier),
      $.obj_inner
    ),

    action_obj: $ => seq(
      seq($.identifier, ':', field('function', $.identifier)),
      '(',
      sep($._text_ident, ','),
      optional(','),
      ')',
    ),

    action: $ => seq(
      '[',
      sep($.action_obj, ','),
      optional(','),
      ']'
    ),

    register: $ => seq($.identifier, ':', $._text_ident),

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