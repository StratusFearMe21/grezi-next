/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

// const CSS = require('tree-sitter-css/grammar');

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


  // word: $ => $.identifier,

  rules: {
    source_file: $ => repeat($._definition),

    whitespace: _ => /(\s|\\\r?\n)+/,

    _definition: $ =>
      choice(
        $.slide,
        $.viewbox,
        $.obj,
        $.register,
        $.actions,
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
        optional(choice(/0[xX]/, /0[bB]/, /0[oO]/)),
        choice(
          seq(
            choice(
              decimalDigits,
              seq(/0[bB]/, decimalDigits),
              seq(/0[oO]/, decimalDigits),
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

    edge_parser: $ => seq($.edge, optional($.edge)),
    edge: $ => choice('|', seq($.direction, $.direction)),

    _vb_identifier: $ => choice(alias(choice('Size', 'Screen'), $.size), alias('()', $.inherit), $.vb_rect, $.identifier),

    vb_rect: $ => seq('[', $.vb_rect_part, '-', $.vb_rect_part,  ']'),
    vb_rect_part: $ => seq('[', $.number_literal, $.number_literal, ']'),

    vb_ref: $ =>  seq(
      field("viewbox", $._vb_identifier),
      field('viewbox_index', $.index_parser)
    ),

    _slide_from: $ => seq(
      '{', $.vb_ref, '}'
    ),

    slide_vb: $ => choice(
      seq(
        ':',
        $.vb_ref,
      ),
      seq('|', 
        $.vb_ref,
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


    slide_obj: $ =>  seq(
      field('object', choice(alias('..', $.from_last_slide),  $.identifier)),
      optional(seq('[', field('range', $.range), ']')),
      optional($.slide_vb),
      optional($._slide_from),
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

    actions: $ => $.slide_functions,

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

    operation: _ => choice('%', '+', '~', '-', '#'),

    viewbox_obj: $ =>
      seq(
        field('value', $.number_literal),
        choice(
          field('operation', $.operation),
          seq(field('operation', ':'), field('denominator', $.number_literal))
        )
      ),

    direction: _ => choice('^', '_', '<', '>', '.'),

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
        $.vb_ref,
        field('body', $.viewbox_inner)
      ),

    obj_param: $ => seq(optional(seq(field('key', $.identifier), ':')), field('value', choice($._text_ident, $.obj_other))),

    obj_inner: $ => seq(
      field('ty', $.identifier),
      '(',
      sep($.obj_param, ','),
      optional(','),
      ')'
    ),

    obj: $ => seq(
      field('name', $.identifier),
      ':',
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

    // _css_value: _ => "",
    // plain_value: _ => token(seq(
    //   repeat(choice(
    //     /[-_]/,
    //     /\/[^\*\s,;!{}()\[\]<>]/, // Slash not followed by a '*' (which would be a comment)
    //   )),
    //   /[a-zA-Z]/,
    //   repeat(choice(
    //     /[^/\s,;!{}()\[\]<>]/, // Not a slash, not a delimiter character
    //     /\/[^\*\s,;!{}()\[\]<>]/, // Slash not followed by a '*' (which would be a comment)
    //   )),
    // )),
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

const DONT_IMPORT = { plain_value: 0, identifier: 0, escape_sequence: 0, string_content: 0 };
// const DONT_IMPORT = {};

function rule_name(prefix, name) {
  if (!name.startsWith(prefix + "_") && !name.startsWith("_" + prefix + "_")) {
    let conflict = module.exports.grammar.rules[name] != undefined;
    for (external_rule in module.exports.grammar.externals) {
      if (module.exports.grammar.externals[external_rule].name == name) {
        conflict = true;
      }
    }
    if (conflict) {
      console.error("Conflicting name `" + name + "`")
    }
    if (name.startsWith("_")) {
      name = "_" + prefix + "_" + name.substring(1) 
    } else {
      name = prefix + "_" + name
    }
  }

  return name
}

/**
 * Imports a rule from an external grammar
 * @param {String} prefix
 * @param {String} origin_name
 * @param {GrammarSchema<_>} origin_grammar
 */
function import_rule(prefix, origin_name, origin_grammar) {
  var rules = [origin_grammar.grammar.rules[origin_name], { type: 'SYMBOL', name: origin_name }];
  var full_import = {};
  var imported = {};
  while (rules.length > 0) {
    let rule = rules.pop()
    if (rule == undefined) {
      break;
    }
    switch (rule.type) {
      case 'ALIAS':
        if (DONT_IMPORT[rule.value] == undefined) {
          rule.value = rule_name(prefix, rule.value)
        }
      case 'PREC':
      case 'PREC_LEFT':
      case 'TOKEN':
      case 'REPEAT':
      case 'REPEAT1':
      case 'IMMEDIATE_TOKEN':
        rules.push(rule.content)
        break;
      case 'CHOICE':
      case 'SEQ':
        for (member in rule.members) {
          rules.push(rule.members[member])
        }
        break;
      case 'SYMBOL':
        if (DONT_IMPORT[rule.name] == undefined) {
          let css_rule = CSS.grammar.rules[rule.name];
          let name = rule_name(prefix, rule.name)
          if (imported[name] == undefined && css_rule != undefined) {
            imported[name] = true;
            rules.push(css_rule);
            full_import[name] = css_rule;
            module.exports.grammar.rules[name] = css_rule;
          }
          if (css_rule != undefined) {
            rule.name = name;
          }
        }
        break;
      case 'STRING':
      case 'BLANK':
      case 'PATTERN':
        break;
      default:
        console.error(rule)
    }
  }
}

// import_rule("css", "_value", CSS);
