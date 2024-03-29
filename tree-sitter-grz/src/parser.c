#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 136
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 61
#define ALIAS_COUNT 0
#define TOKEN_COUNT 31
#define EXTERNAL_TOKEN_COUNT 3
#define FIELD_COUNT 14
#define MAX_ALIAS_SEQUENCE_LENGTH 6
#define PRODUCTION_ID_COUNT 17

enum {
  sym_identifier = 1,
  sym_whitespace = 2,
  anon_sym_DOT = 3,
  sym_escape_sequence = 4,
  aux_sym__simple_string_literal_token1 = 5,
  anon_sym_DQUOTE = 6,
  anon_sym_r_POUND_DQUOTE = 7,
  anon_sym_DQUOTE_POUND = 8,
  sym_number_literal = 9,
  anon_sym_LBRACK = 10,
  anon_sym_RBRACK = 11,
  sym_edge_parser = 12,
  anon_sym_Size = 13,
  anon_sym_LPAREN_RPAREN = 14,
  anon_sym_LBRACE = 15,
  anon_sym_RBRACE = 16,
  anon_sym_COLON = 17,
  anon_sym_PIPE = 18,
  anon_sym_TILDE = 19,
  anon_sym_COMMA = 20,
  anon_sym_LPAREN = 21,
  anon_sym_RPAREN = 22,
  sym_operation = 23,
  sym_direction = 24,
  anon_sym_LT = 25,
  anon_sym_GT = 26,
  sym_comment = 27,
  sym_string_content = 28,
  sym_raw_string_content = 29,
  sym_obj_other = 30,
  sym_source_file = 31,
  sym__definition = 32,
  sym_completion = 33,
  sym__simple_string_literal = 34,
  sym__raw_string_literal = 35,
  sym_string_literal = 36,
  sym__text_ident = 37,
  sym_index_parser = 38,
  sym__vb_identifier = 39,
  sym_slide_from = 40,
  sym_slide_vb = 41,
  sym_slide_obj = 42,
  sym_slide_objects = 43,
  sym_slide_function = 44,
  sym_slide_functions = 45,
  sym_slide = 46,
  sym_viewbox_obj = 47,
  sym_viewbox_inner = 48,
  sym_viewbox = 49,
  sym_obj_param = 50,
  sym_obj_inner = 51,
  sym_obj = 52,
  sym_register = 53,
  aux_sym_source_file_repeat1 = 54,
  aux_sym__simple_string_literal_repeat1 = 55,
  aux_sym_slide_objects_repeat1 = 56,
  aux_sym_slide_function_repeat1 = 57,
  aux_sym_slide_functions_repeat1 = 58,
  aux_sym_viewbox_inner_repeat1 = 59,
  aux_sym_obj_inner_repeat1 = 60,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [sym_identifier] = "identifier",
  [sym_whitespace] = "whitespace",
  [anon_sym_DOT] = ".",
  [sym_escape_sequence] = "escape_sequence",
  [aux_sym__simple_string_literal_token1] = "\"",
  [anon_sym_DQUOTE] = "\"",
  [anon_sym_r_POUND_DQUOTE] = "r#\"",
  [anon_sym_DQUOTE_POUND] = "\"#",
  [sym_number_literal] = "number_literal",
  [anon_sym_LBRACK] = "[",
  [anon_sym_RBRACK] = "]",
  [sym_edge_parser] = "edge_parser",
  [anon_sym_Size] = "size",
  [anon_sym_LPAREN_RPAREN] = "inherit",
  [anon_sym_LBRACE] = "{",
  [anon_sym_RBRACE] = "}",
  [anon_sym_COLON] = ":",
  [anon_sym_PIPE] = "|",
  [anon_sym_TILDE] = "~",
  [anon_sym_COMMA] = ",",
  [anon_sym_LPAREN] = "(",
  [anon_sym_RPAREN] = ")",
  [sym_operation] = "operation",
  [sym_direction] = "direction",
  [anon_sym_LT] = "<",
  [anon_sym_GT] = ">",
  [sym_comment] = "comment",
  [sym_string_content] = "string_content",
  [sym_raw_string_content] = "raw_string_content",
  [sym_obj_other] = "obj_other",
  [sym_source_file] = "source_file",
  [sym__definition] = "_definition",
  [sym_completion] = "completion",
  [sym__simple_string_literal] = "_simple_string_literal",
  [sym__raw_string_literal] = "_raw_string_literal",
  [sym_string_literal] = "string_literal",
  [sym__text_ident] = "_text_ident",
  [sym_index_parser] = "index_parser",
  [sym__vb_identifier] = "_vb_identifier",
  [sym_slide_from] = "slide_from",
  [sym_slide_vb] = "slide_vb",
  [sym_slide_obj] = "slide_obj",
  [sym_slide_objects] = "slide_objects",
  [sym_slide_function] = "slide_function",
  [sym_slide_functions] = "slide_functions",
  [sym_slide] = "slide",
  [sym_viewbox_obj] = "viewbox_obj",
  [sym_viewbox_inner] = "viewbox_inner",
  [sym_viewbox] = "viewbox",
  [sym_obj_param] = "obj_param",
  [sym_obj_inner] = "obj_inner",
  [sym_obj] = "obj",
  [sym_register] = "register",
  [aux_sym_source_file_repeat1] = "source_file_repeat1",
  [aux_sym__simple_string_literal_repeat1] = "_simple_string_literal_repeat1",
  [aux_sym_slide_objects_repeat1] = "slide_objects_repeat1",
  [aux_sym_slide_function_repeat1] = "slide_function_repeat1",
  [aux_sym_slide_functions_repeat1] = "slide_functions_repeat1",
  [aux_sym_viewbox_inner_repeat1] = "viewbox_inner_repeat1",
  [aux_sym_obj_inner_repeat1] = "obj_inner_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [sym_identifier] = sym_identifier,
  [sym_whitespace] = sym_whitespace,
  [anon_sym_DOT] = anon_sym_DOT,
  [sym_escape_sequence] = sym_escape_sequence,
  [aux_sym__simple_string_literal_token1] = anon_sym_DQUOTE,
  [anon_sym_DQUOTE] = anon_sym_DQUOTE,
  [anon_sym_r_POUND_DQUOTE] = anon_sym_r_POUND_DQUOTE,
  [anon_sym_DQUOTE_POUND] = anon_sym_DQUOTE_POUND,
  [sym_number_literal] = sym_number_literal,
  [anon_sym_LBRACK] = anon_sym_LBRACK,
  [anon_sym_RBRACK] = anon_sym_RBRACK,
  [sym_edge_parser] = sym_edge_parser,
  [anon_sym_Size] = anon_sym_Size,
  [anon_sym_LPAREN_RPAREN] = anon_sym_LPAREN_RPAREN,
  [anon_sym_LBRACE] = anon_sym_LBRACE,
  [anon_sym_RBRACE] = anon_sym_RBRACE,
  [anon_sym_COLON] = anon_sym_COLON,
  [anon_sym_PIPE] = anon_sym_PIPE,
  [anon_sym_TILDE] = anon_sym_TILDE,
  [anon_sym_COMMA] = anon_sym_COMMA,
  [anon_sym_LPAREN] = anon_sym_LPAREN,
  [anon_sym_RPAREN] = anon_sym_RPAREN,
  [sym_operation] = sym_operation,
  [sym_direction] = sym_direction,
  [anon_sym_LT] = anon_sym_LT,
  [anon_sym_GT] = anon_sym_GT,
  [sym_comment] = sym_comment,
  [sym_string_content] = sym_string_content,
  [sym_raw_string_content] = sym_raw_string_content,
  [sym_obj_other] = sym_obj_other,
  [sym_source_file] = sym_source_file,
  [sym__definition] = sym__definition,
  [sym_completion] = sym_completion,
  [sym__simple_string_literal] = sym__simple_string_literal,
  [sym__raw_string_literal] = sym__raw_string_literal,
  [sym_string_literal] = sym_string_literal,
  [sym__text_ident] = sym__text_ident,
  [sym_index_parser] = sym_index_parser,
  [sym__vb_identifier] = sym__vb_identifier,
  [sym_slide_from] = sym_slide_from,
  [sym_slide_vb] = sym_slide_vb,
  [sym_slide_obj] = sym_slide_obj,
  [sym_slide_objects] = sym_slide_objects,
  [sym_slide_function] = sym_slide_function,
  [sym_slide_functions] = sym_slide_functions,
  [sym_slide] = sym_slide,
  [sym_viewbox_obj] = sym_viewbox_obj,
  [sym_viewbox_inner] = sym_viewbox_inner,
  [sym_viewbox] = sym_viewbox,
  [sym_obj_param] = sym_obj_param,
  [sym_obj_inner] = sym_obj_inner,
  [sym_obj] = sym_obj,
  [sym_register] = sym_register,
  [aux_sym_source_file_repeat1] = aux_sym_source_file_repeat1,
  [aux_sym__simple_string_literal_repeat1] = aux_sym__simple_string_literal_repeat1,
  [aux_sym_slide_objects_repeat1] = aux_sym_slide_objects_repeat1,
  [aux_sym_slide_function_repeat1] = aux_sym_slide_function_repeat1,
  [aux_sym_slide_functions_repeat1] = aux_sym_slide_functions_repeat1,
  [aux_sym_viewbox_inner_repeat1] = aux_sym_viewbox_inner_repeat1,
  [aux_sym_obj_inner_repeat1] = aux_sym_obj_inner_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [sym_identifier] = {
    .visible = true,
    .named = true,
  },
  [sym_whitespace] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_DOT] = {
    .visible = true,
    .named = false,
  },
  [sym_escape_sequence] = {
    .visible = true,
    .named = true,
  },
  [aux_sym__simple_string_literal_token1] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DQUOTE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_r_POUND_DQUOTE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DQUOTE_POUND] = {
    .visible = true,
    .named = false,
  },
  [sym_number_literal] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_LBRACK] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACK] = {
    .visible = true,
    .named = false,
  },
  [sym_edge_parser] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_Size] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_LPAREN_RPAREN] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_LBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COLON] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PIPE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_TILDE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COMMA] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RPAREN] = {
    .visible = true,
    .named = false,
  },
  [sym_operation] = {
    .visible = true,
    .named = true,
  },
  [sym_direction] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_LT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_GT] = {
    .visible = true,
    .named = false,
  },
  [sym_comment] = {
    .visible = true,
    .named = true,
  },
  [sym_string_content] = {
    .visible = true,
    .named = true,
  },
  [sym_raw_string_content] = {
    .visible = true,
    .named = true,
  },
  [sym_obj_other] = {
    .visible = true,
    .named = true,
  },
  [sym_source_file] = {
    .visible = true,
    .named = true,
  },
  [sym__definition] = {
    .visible = false,
    .named = true,
  },
  [sym_completion] = {
    .visible = true,
    .named = true,
  },
  [sym__simple_string_literal] = {
    .visible = false,
    .named = true,
  },
  [sym__raw_string_literal] = {
    .visible = false,
    .named = true,
  },
  [sym_string_literal] = {
    .visible = true,
    .named = true,
  },
  [sym__text_ident] = {
    .visible = false,
    .named = true,
  },
  [sym_index_parser] = {
    .visible = true,
    .named = true,
  },
  [sym__vb_identifier] = {
    .visible = false,
    .named = true,
  },
  [sym_slide_from] = {
    .visible = true,
    .named = true,
  },
  [sym_slide_vb] = {
    .visible = true,
    .named = true,
  },
  [sym_slide_obj] = {
    .visible = true,
    .named = true,
  },
  [sym_slide_objects] = {
    .visible = true,
    .named = true,
  },
  [sym_slide_function] = {
    .visible = true,
    .named = true,
  },
  [sym_slide_functions] = {
    .visible = true,
    .named = true,
  },
  [sym_slide] = {
    .visible = true,
    .named = true,
  },
  [sym_viewbox_obj] = {
    .visible = true,
    .named = true,
  },
  [sym_viewbox_inner] = {
    .visible = true,
    .named = true,
  },
  [sym_viewbox] = {
    .visible = true,
    .named = true,
  },
  [sym_obj_param] = {
    .visible = true,
    .named = true,
  },
  [sym_obj_inner] = {
    .visible = true,
    .named = true,
  },
  [sym_obj] = {
    .visible = true,
    .named = true,
  },
  [sym_register] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_source_file_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym__simple_string_literal_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_slide_objects_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_slide_function_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_slide_functions_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_viewbox_inner_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_obj_inner_repeat1] = {
    .visible = false,
    .named = false,
  },
};

enum {
  field_body = 1,
  field_denominator = 2,
  field_direction = 3,
  field_function = 4,
  field_key = 5,
  field_name = 6,
  field_object = 7,
  field_objects = 8,
  field_operation = 9,
  field_parameters = 10,
  field_ty = 11,
  field_value = 12,
  field_viewbox = 13,
  field_viewbox_index = 14,
};

static const char * const ts_field_names[] = {
  [0] = NULL,
  [field_body] = "body",
  [field_denominator] = "denominator",
  [field_direction] = "direction",
  [field_function] = "function",
  [field_key] = "key",
  [field_name] = "name",
  [field_object] = "object",
  [field_objects] = "objects",
  [field_operation] = "operation",
  [field_parameters] = "parameters",
  [field_ty] = "ty",
  [field_value] = "value",
  [field_viewbox] = "viewbox",
  [field_viewbox_index] = "viewbox_index",
};

static const TSFieldMapSlice ts_field_map_slices[PRODUCTION_ID_COUNT] = {
  [1] = {.index = 0, .length = 1},
  [2] = {.index = 1, .length = 1},
  [3] = {.index = 2, .length = 2},
  [4] = {.index = 4, .length = 2},
  [5] = {.index = 6, .length = 1},
  [6] = {.index = 7, .length = 4},
  [7] = {.index = 11, .length = 1},
  [8] = {.index = 12, .length = 2},
  [9] = {.index = 14, .length = 2},
  [10] = {.index = 16, .length = 2},
  [11] = {.index = 18, .length = 1},
  [12] = {.index = 19, .length = 2},
  [13] = {.index = 21, .length = 2},
  [14] = {.index = 23, .length = 4},
  [15] = {.index = 27, .length = 3},
  [16] = {.index = 30, .length = 3},
};

static const TSFieldMapEntry ts_field_map_entries[] = {
  [0] =
    {field_object, 0},
  [1] =
    {field_objects, 1},
  [2] =
    {field_viewbox, 0},
    {field_viewbox_index, 1},
  [4] =
    {field_name, 0},
    {field_ty, 2},
  [6] =
    {field_viewbox_index, 1},
  [7] =
    {field_body, 3},
    {field_name, 0},
    {field_viewbox, 2, .inherited = true},
    {field_viewbox_index, 2, .inherited = true},
  [11] =
    {field_function, 0},
  [12] =
    {field_viewbox, 1, .inherited = true},
    {field_viewbox_index, 1, .inherited = true},
  [14] =
    {field_objects, 1},
    {field_objects, 2},
  [16] =
    {field_key, 0},
    {field_value, 2},
  [18] =
    {field_direction, 0},
  [19] =
    {field_operation, 1},
    {field_value, 0},
  [21] =
    {field_direction, 0},
    {field_parameters, 1},
  [23] =
    {field_body, 2},
    {field_viewbox, 1, .inherited = true},
    {field_viewbox_index, 1, .inherited = true},
    {field_viewbox_index, 3},
  [27] =
    {field_denominator, 2},
    {field_operation, 1},
    {field_value, 0},
  [30] =
    {field_direction, 0},
    {field_parameters, 1},
    {field_parameters, 2},
};

static const TSSymbol ts_alias_sequences[PRODUCTION_ID_COUNT][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
};

static const uint16_t ts_non_terminal_alias_map[] = {
  0,
};

static const TSStateId ts_primary_state_ids[STATE_COUNT] = {
  [0] = 0,
  [1] = 1,
  [2] = 2,
  [3] = 3,
  [4] = 4,
  [5] = 5,
  [6] = 6,
  [7] = 7,
  [8] = 8,
  [9] = 9,
  [10] = 10,
  [11] = 11,
  [12] = 12,
  [13] = 13,
  [14] = 14,
  [15] = 15,
  [16] = 16,
  [17] = 17,
  [18] = 18,
  [19] = 19,
  [20] = 20,
  [21] = 21,
  [22] = 22,
  [23] = 23,
  [24] = 24,
  [25] = 25,
  [26] = 26,
  [27] = 27,
  [28] = 28,
  [29] = 29,
  [30] = 30,
  [31] = 31,
  [32] = 32,
  [33] = 33,
  [34] = 34,
  [35] = 10,
  [36] = 36,
  [37] = 37,
  [38] = 38,
  [39] = 39,
  [40] = 40,
  [41] = 41,
  [42] = 42,
  [43] = 43,
  [44] = 44,
  [45] = 45,
  [46] = 46,
  [47] = 47,
  [48] = 48,
  [49] = 49,
  [50] = 50,
  [51] = 51,
  [52] = 52,
  [53] = 53,
  [54] = 54,
  [55] = 55,
  [56] = 56,
  [57] = 57,
  [58] = 58,
  [59] = 59,
  [60] = 60,
  [61] = 61,
  [62] = 62,
  [63] = 63,
  [64] = 64,
  [65] = 65,
  [66] = 66,
  [67] = 67,
  [68] = 68,
  [69] = 69,
  [70] = 70,
  [71] = 71,
  [72] = 72,
  [73] = 73,
  [74] = 74,
  [75] = 75,
  [76] = 76,
  [77] = 77,
  [78] = 78,
  [79] = 79,
  [80] = 80,
  [81] = 81,
  [82] = 82,
  [83] = 83,
  [84] = 84,
  [85] = 85,
  [86] = 86,
  [87] = 84,
  [88] = 37,
  [89] = 36,
  [90] = 90,
  [91] = 91,
  [92] = 92,
  [93] = 93,
  [94] = 94,
  [95] = 34,
  [96] = 96,
  [97] = 97,
  [98] = 98,
  [99] = 99,
  [100] = 100,
  [101] = 101,
  [102] = 102,
  [103] = 103,
  [104] = 104,
  [105] = 105,
  [106] = 106,
  [107] = 107,
  [108] = 108,
  [109] = 109,
  [110] = 110,
  [111] = 111,
  [112] = 112,
  [113] = 113,
  [114] = 114,
  [115] = 115,
  [116] = 116,
  [117] = 117,
  [118] = 118,
  [119] = 119,
  [120] = 120,
  [121] = 121,
  [122] = 122,
  [123] = 123,
  [124] = 124,
  [125] = 125,
  [126] = 126,
  [127] = 127,
  [128] = 128,
  [129] = 129,
  [130] = 130,
  [131] = 131,
  [132] = 132,
  [133] = 133,
  [134] = 113,
  [135] = 128,
};

static inline bool sym_identifier_character_set_1(int32_t c) {
  return (c < 43514
    ? (c < 4193
      ? (c < 2707
        ? (c < 1994
          ? (c < 931
            ? (c < 748
              ? (c < 192
                ? (c < 170
                  ? (c < 'a'
                    ? (c >= 'A' && c <= 'Z')
                    : c <= 'z')
                  : (c <= 170 || (c < 186
                    ? c == 181
                    : c <= 186)))
                : (c <= 214 || (c < 710
                  ? (c < 248
                    ? (c >= 216 && c <= 246)
                    : c <= 705)
                  : (c <= 721 || (c >= 736 && c <= 740)))))
              : (c <= 748 || (c < 895
                ? (c < 886
                  ? (c < 880
                    ? c == 750
                    : c <= 884)
                  : (c <= 887 || (c >= 891 && c <= 893)))
                : (c <= 895 || (c < 908
                  ? (c < 904
                    ? c == 902
                    : c <= 906)
                  : (c <= 908 || (c >= 910 && c <= 929)))))))
            : (c <= 1013 || (c < 1649
              ? (c < 1376
                ? (c < 1329
                  ? (c < 1162
                    ? (c >= 1015 && c <= 1153)
                    : c <= 1327)
                  : (c <= 1366 || c == 1369))
                : (c <= 1416 || (c < 1568
                  ? (c < 1519
                    ? (c >= 1488 && c <= 1514)
                    : c <= 1522)
                  : (c <= 1610 || (c >= 1646 && c <= 1647)))))
              : (c <= 1747 || (c < 1791
                ? (c < 1774
                  ? (c < 1765
                    ? c == 1749
                    : c <= 1766)
                  : (c <= 1775 || (c >= 1786 && c <= 1788)))
                : (c <= 1791 || (c < 1869
                  ? (c < 1810
                    ? c == 1808
                    : c <= 1839)
                  : (c <= 1957 || c == 1969))))))))
          : (c <= 2026 || (c < 2482
            ? (c < 2208
              ? (c < 2088
                ? (c < 2048
                  ? (c < 2042
                    ? (c >= 2036 && c <= 2037)
                    : c <= 2042)
                  : (c <= 2069 || (c < 2084
                    ? c == 2074
                    : c <= 2084)))
                : (c <= 2088 || (c < 2160
                  ? (c < 2144
                    ? (c >= 2112 && c <= 2136)
                    : c <= 2154)
                  : (c <= 2183 || (c >= 2185 && c <= 2190)))))
              : (c <= 2249 || (c < 2417
                ? (c < 2384
                  ? (c < 2365
                    ? (c >= 2308 && c <= 2361)
                    : c <= 2365)
                  : (c <= 2384 || (c >= 2392 && c <= 2401)))
                : (c <= 2432 || (c < 2451
                  ? (c < 2447
                    ? (c >= 2437 && c <= 2444)
                    : c <= 2448)
                  : (c <= 2472 || (c >= 2474 && c <= 2480)))))))
            : (c <= 2482 || (c < 2579
              ? (c < 2527
                ? (c < 2510
                  ? (c < 2493
                    ? (c >= 2486 && c <= 2489)
                    : c <= 2493)
                  : (c <= 2510 || (c >= 2524 && c <= 2525)))
                : (c <= 2529 || (c < 2565
                  ? (c < 2556
                    ? (c >= 2544 && c <= 2545)
                    : c <= 2556)
                  : (c <= 2570 || (c >= 2575 && c <= 2576)))))
              : (c <= 2600 || (c < 2649
                ? (c < 2613
                  ? (c < 2610
                    ? (c >= 2602 && c <= 2608)
                    : c <= 2611)
                  : (c <= 2614 || (c >= 2616 && c <= 2617)))
                : (c <= 2652 || (c < 2693
                  ? (c < 2674
                    ? c == 2654
                    : c <= 2676)
                  : (c <= 2701 || (c >= 2703 && c <= 2705)))))))))))
        : (c <= 2728 || (c < 3242
          ? (c < 2962
            ? (c < 2858
              ? (c < 2784
                ? (c < 2741
                  ? (c < 2738
                    ? (c >= 2730 && c <= 2736)
                    : c <= 2739)
                  : (c <= 2745 || (c < 2768
                    ? c == 2749
                    : c <= 2768)))
                : (c <= 2785 || (c < 2831
                  ? (c < 2821
                    ? c == 2809
                    : c <= 2828)
                  : (c <= 2832 || (c >= 2835 && c <= 2856)))))
              : (c <= 2864 || (c < 2911
                ? (c < 2877
                  ? (c < 2869
                    ? (c >= 2866 && c <= 2867)
                    : c <= 2873)
                  : (c <= 2877 || (c >= 2908 && c <= 2909)))
                : (c <= 2913 || (c < 2949
                  ? (c < 2947
                    ? c == 2929
                    : c <= 2947)
                  : (c <= 2954 || (c >= 2958 && c <= 2960)))))))
            : (c <= 2965 || (c < 3090
              ? (c < 2984
                ? (c < 2974
                  ? (c < 2972
                    ? (c >= 2969 && c <= 2970)
                    : c <= 2972)
                  : (c <= 2975 || (c >= 2979 && c <= 2980)))
                : (c <= 2986 || (c < 3077
                  ? (c < 3024
                    ? (c >= 2990 && c <= 3001)
                    : c <= 3024)
                  : (c <= 3084 || (c >= 3086 && c <= 3088)))))
              : (c <= 3112 || (c < 3168
                ? (c < 3160
                  ? (c < 3133
                    ? (c >= 3114 && c <= 3129)
                    : c <= 3133)
                  : (c <= 3162 || c == 3165))
                : (c <= 3169 || (c < 3214
                  ? (c < 3205
                    ? c == 3200
                    : c <= 3212)
                  : (c <= 3216 || (c >= 3218 && c <= 3240)))))))))
          : (c <= 3251 || (c < 3648
            ? (c < 3412
              ? (c < 3332
                ? (c < 3293
                  ? (c < 3261
                    ? (c >= 3253 && c <= 3257)
                    : c <= 3261)
                  : (c <= 3294 || (c < 3313
                    ? (c >= 3296 && c <= 3297)
                    : c <= 3314)))
                : (c <= 3340 || (c < 3389
                  ? (c < 3346
                    ? (c >= 3342 && c <= 3344)
                    : c <= 3386)
                  : (c <= 3389 || c == 3406))))
              : (c <= 3414 || (c < 3507
                ? (c < 3461
                  ? (c < 3450
                    ? (c >= 3423 && c <= 3425)
                    : c <= 3455)
                  : (c <= 3478 || (c >= 3482 && c <= 3505)))
                : (c <= 3515 || (c < 3585
                  ? (c < 3520
                    ? c == 3517
                    : c <= 3526)
                  : (c <= 3632 || c == 3634))))))
            : (c <= 3654 || (c < 3782
              ? (c < 3749
                ? (c < 3718
                  ? (c < 3716
                    ? (c >= 3713 && c <= 3714)
                    : c <= 3716)
                  : (c <= 3722 || (c >= 3724 && c <= 3747)))
                : (c <= 3749 || (c < 3773
                  ? (c < 3762
                    ? (c >= 3751 && c <= 3760)
                    : c <= 3762)
                  : (c <= 3773 || (c >= 3776 && c <= 3780)))))
              : (c <= 3782 || (c < 3976
                ? (c < 3904
                  ? (c < 3840
                    ? (c >= 3804 && c <= 3807)
                    : c <= 3840)
                  : (c <= 3911 || (c >= 3913 && c <= 3948)))
                : (c <= 3980 || (c < 4176
                  ? (c < 4159
                    ? (c >= 4096 && c <= 4138)
                    : c <= 4159)
                  : (c <= 4181 || (c >= 4186 && c <= 4189)))))))))))))
      : (c <= 4193 || (c < 8134
        ? (c < 6176
          ? (c < 4808
            ? (c < 4688
              ? (c < 4295
                ? (c < 4213
                  ? (c < 4206
                    ? (c >= 4197 && c <= 4198)
                    : c <= 4208)
                  : (c <= 4225 || (c < 4256
                    ? c == 4238
                    : c <= 4293)))
                : (c <= 4295 || (c < 4348
                  ? (c < 4304
                    ? c == 4301
                    : c <= 4346)
                  : (c <= 4680 || (c >= 4682 && c <= 4685)))))
              : (c <= 4694 || (c < 4752
                ? (c < 4704
                  ? (c < 4698
                    ? c == 4696
                    : c <= 4701)
                  : (c <= 4744 || (c >= 4746 && c <= 4749)))
                : (c <= 4784 || (c < 4800
                  ? (c < 4792
                    ? (c >= 4786 && c <= 4789)
                    : c <= 4798)
                  : (c <= 4800 || (c >= 4802 && c <= 4805)))))))
            : (c <= 4822 || (c < 5792
              ? (c < 5024
                ? (c < 4888
                  ? (c < 4882
                    ? (c >= 4824 && c <= 4880)
                    : c <= 4885)
                  : (c <= 4954 || (c >= 4992 && c <= 5007)))
                : (c <= 5109 || (c < 5743
                  ? (c < 5121
                    ? (c >= 5112 && c <= 5117)
                    : c <= 5740)
                  : (c <= 5759 || (c >= 5761 && c <= 5786)))))
              : (c <= 5866 || (c < 5984
                ? (c < 5919
                  ? (c < 5888
                    ? (c >= 5870 && c <= 5880)
                    : c <= 5905)
                  : (c <= 5937 || (c >= 5952 && c <= 5969)))
                : (c <= 5996 || (c < 6103
                  ? (c < 6016
                    ? (c >= 5998 && c <= 6000)
                    : c <= 6067)
                  : (c <= 6103 || c == 6108))))))))
          : (c <= 6264 || (c < 7312
            ? (c < 6823
              ? (c < 6512
                ? (c < 6320
                  ? (c < 6314
                    ? (c >= 6272 && c <= 6312)
                    : c <= 6314)
                  : (c <= 6389 || (c < 6480
                    ? (c >= 6400 && c <= 6430)
                    : c <= 6509)))
                : (c <= 6516 || (c < 6656
                  ? (c < 6576
                    ? (c >= 6528 && c <= 6571)
                    : c <= 6601)
                  : (c <= 6678 || (c >= 6688 && c <= 6740)))))
              : (c <= 6823 || (c < 7098
                ? (c < 7043
                  ? (c < 6981
                    ? (c >= 6917 && c <= 6963)
                    : c <= 6988)
                  : (c <= 7072 || (c >= 7086 && c <= 7087)))
                : (c <= 7141 || (c < 7258
                  ? (c < 7245
                    ? (c >= 7168 && c <= 7203)
                    : c <= 7247)
                  : (c <= 7293 || (c >= 7296 && c <= 7304)))))))
            : (c <= 7354 || (c < 8008
              ? (c < 7418
                ? (c < 7406
                  ? (c < 7401
                    ? (c >= 7357 && c <= 7359)
                    : c <= 7404)
                  : (c <= 7411 || (c >= 7413 && c <= 7414)))
                : (c <= 7418 || (c < 7960
                  ? (c < 7680
                    ? (c >= 7424 && c <= 7615)
                    : c <= 7957)
                  : (c <= 7965 || (c >= 7968 && c <= 8005)))))
              : (c <= 8013 || (c < 8031
                ? (c < 8027
                  ? (c < 8025
                    ? (c >= 8016 && c <= 8023)
                    : c <= 8025)
                  : (c <= 8027 || c == 8029))
                : (c <= 8061 || (c < 8126
                  ? (c < 8118
                    ? (c >= 8064 && c <= 8116)
                    : c <= 8124)
                  : (c <= 8126 || (c >= 8130 && c <= 8132)))))))))))
        : (c <= 8140 || (c < 12337
          ? (c < 8544
            ? (c < 8458
              ? (c < 8305
                ? (c < 8160
                  ? (c < 8150
                    ? (c >= 8144 && c <= 8147)
                    : c <= 8155)
                  : (c <= 8172 || (c < 8182
                    ? (c >= 8178 && c <= 8180)
                    : c <= 8188)))
                : (c <= 8305 || (c < 8450
                  ? (c < 8336
                    ? c == 8319
                    : c <= 8348)
                  : (c <= 8450 || c == 8455))))
              : (c <= 8467 || (c < 8488
                ? (c < 8484
                  ? (c < 8472
                    ? c == 8469
                    : c <= 8477)
                  : (c <= 8484 || c == 8486))
                : (c <= 8488 || (c < 8517
                  ? (c < 8508
                    ? (c >= 8490 && c <= 8505)
                    : c <= 8511)
                  : (c <= 8521 || c == 8526))))))
            : (c <= 8584 || (c < 11680
              ? (c < 11559
                ? (c < 11506
                  ? (c < 11499
                    ? (c >= 11264 && c <= 11492)
                    : c <= 11502)
                  : (c <= 11507 || (c >= 11520 && c <= 11557)))
                : (c <= 11559 || (c < 11631
                  ? (c < 11568
                    ? c == 11565
                    : c <= 11623)
                  : (c <= 11631 || (c >= 11648 && c <= 11670)))))
              : (c <= 11686 || (c < 11720
                ? (c < 11704
                  ? (c < 11696
                    ? (c >= 11688 && c <= 11694)
                    : c <= 11702)
                  : (c <= 11710 || (c >= 11712 && c <= 11718)))
                : (c <= 11726 || (c < 12293
                  ? (c < 11736
                    ? (c >= 11728 && c <= 11734)
                    : c <= 11742)
                  : (c <= 12295 || (c >= 12321 && c <= 12329)))))))))
          : (c <= 12341 || (c < 42891
            ? (c < 19968
              ? (c < 12549
                ? (c < 12445
                  ? (c < 12353
                    ? (c >= 12344 && c <= 12348)
                    : c <= 12438)
                  : (c <= 12447 || (c < 12540
                    ? (c >= 12449 && c <= 12538)
                    : c <= 12543)))
                : (c <= 12591 || (c < 12784
                  ? (c < 12704
                    ? (c >= 12593 && c <= 12686)
                    : c <= 12735)
                  : (c <= 12799 || (c >= 13312 && c <= 19903)))))
              : (c <= 42124 || (c < 42560
                ? (c < 42512
                  ? (c < 42240
                    ? (c >= 42192 && c <= 42237)
                    : c <= 42508)
                  : (c <= 42527 || (c >= 42538 && c <= 42539)))
                : (c <= 42606 || (c < 42775
                  ? (c < 42656
                    ? (c >= 42623 && c <= 42653)
                    : c <= 42735)
                  : (c <= 42783 || (c >= 42786 && c <= 42888)))))))
            : (c <= 42954 || (c < 43250
              ? (c < 43011
                ? (c < 42965
                  ? (c < 42963
                    ? (c >= 42960 && c <= 42961)
                    : c <= 42963)
                  : (c <= 42969 || (c >= 42994 && c <= 43009)))
                : (c <= 43013 || (c < 43072
                  ? (c < 43020
                    ? (c >= 43015 && c <= 43018)
                    : c <= 43042)
                  : (c <= 43123 || (c >= 43138 && c <= 43187)))))
              : (c <= 43255 || (c < 43360
                ? (c < 43274
                  ? (c < 43261
                    ? c == 43259
                    : c <= 43262)
                  : (c <= 43301 || (c >= 43312 && c <= 43334)))
                : (c <= 43388 || (c < 43488
                  ? (c < 43471
                    ? (c >= 43396 && c <= 43442)
                    : c <= 43471)
                  : (c <= 43492 || (c >= 43494 && c <= 43503)))))))))))))))
    : (c <= 43518 || (c < 70727
      ? (c < 66956
        ? (c < 64914
          ? (c < 43868
            ? (c < 43714
              ? (c < 43646
                ? (c < 43588
                  ? (c < 43584
                    ? (c >= 43520 && c <= 43560)
                    : c <= 43586)
                  : (c <= 43595 || (c < 43642
                    ? (c >= 43616 && c <= 43638)
                    : c <= 43642)))
                : (c <= 43695 || (c < 43705
                  ? (c < 43701
                    ? c == 43697
                    : c <= 43702)
                  : (c <= 43709 || c == 43712))))
              : (c <= 43714 || (c < 43785
                ? (c < 43762
                  ? (c < 43744
                    ? (c >= 43739 && c <= 43741)
                    : c <= 43754)
                  : (c <= 43764 || (c >= 43777 && c <= 43782)))
                : (c <= 43790 || (c < 43816
                  ? (c < 43808
                    ? (c >= 43793 && c <= 43798)
                    : c <= 43814)
                  : (c <= 43822 || (c >= 43824 && c <= 43866)))))))
            : (c <= 43881 || (c < 64287
              ? (c < 63744
                ? (c < 55216
                  ? (c < 44032
                    ? (c >= 43888 && c <= 44002)
                    : c <= 55203)
                  : (c <= 55238 || (c >= 55243 && c <= 55291)))
                : (c <= 64109 || (c < 64275
                  ? (c < 64256
                    ? (c >= 64112 && c <= 64217)
                    : c <= 64262)
                  : (c <= 64279 || c == 64285))))
              : (c <= 64296 || (c < 64323
                ? (c < 64318
                  ? (c < 64312
                    ? (c >= 64298 && c <= 64310)
                    : c <= 64316)
                  : (c <= 64318 || (c >= 64320 && c <= 64321)))
                : (c <= 64324 || (c < 64612
                  ? (c < 64467
                    ? (c >= 64326 && c <= 64433)
                    : c <= 64605)
                  : (c <= 64829 || (c >= 64848 && c <= 64911)))))))))
          : (c <= 64967 || (c < 65599
            ? (c < 65382
              ? (c < 65147
                ? (c < 65139
                  ? (c < 65137
                    ? (c >= 65008 && c <= 65017)
                    : c <= 65137)
                  : (c <= 65139 || (c < 65145
                    ? c == 65143
                    : c <= 65145)))
                : (c <= 65147 || (c < 65313
                  ? (c < 65151
                    ? c == 65149
                    : c <= 65276)
                  : (c <= 65338 || (c >= 65345 && c <= 65370)))))
              : (c <= 65437 || (c < 65498
                ? (c < 65482
                  ? (c < 65474
                    ? (c >= 65440 && c <= 65470)
                    : c <= 65479)
                  : (c <= 65487 || (c >= 65490 && c <= 65495)))
                : (c <= 65500 || (c < 65576
                  ? (c < 65549
                    ? (c >= 65536 && c <= 65547)
                    : c <= 65574)
                  : (c <= 65594 || (c >= 65596 && c <= 65597)))))))
            : (c <= 65613 || (c < 66464
              ? (c < 66208
                ? (c < 65856
                  ? (c < 65664
                    ? (c >= 65616 && c <= 65629)
                    : c <= 65786)
                  : (c <= 65908 || (c >= 66176 && c <= 66204)))
                : (c <= 66256 || (c < 66384
                  ? (c < 66349
                    ? (c >= 66304 && c <= 66335)
                    : c <= 66378)
                  : (c <= 66421 || (c >= 66432 && c <= 66461)))))
              : (c <= 66499 || (c < 66776
                ? (c < 66560
                  ? (c < 66513
                    ? (c >= 66504 && c <= 66511)
                    : c <= 66517)
                  : (c <= 66717 || (c >= 66736 && c <= 66771)))
                : (c <= 66811 || (c < 66928
                  ? (c < 66864
                    ? (c >= 66816 && c <= 66855)
                    : c <= 66915)
                  : (c <= 66938 || (c >= 66940 && c <= 66954)))))))))))
        : (c <= 66962 || (c < 68864
          ? (c < 67828
            ? (c < 67506
              ? (c < 67072
                ? (c < 66979
                  ? (c < 66967
                    ? (c >= 66964 && c <= 66965)
                    : c <= 66977)
                  : (c <= 66993 || (c < 67003
                    ? (c >= 66995 && c <= 67001)
                    : c <= 67004)))
                : (c <= 67382 || (c < 67456
                  ? (c < 67424
                    ? (c >= 67392 && c <= 67413)
                    : c <= 67431)
                  : (c <= 67461 || (c >= 67463 && c <= 67504)))))
              : (c <= 67514 || (c < 67644
                ? (c < 67594
                  ? (c < 67592
                    ? (c >= 67584 && c <= 67589)
                    : c <= 67592)
                  : (c <= 67637 || (c >= 67639 && c <= 67640)))
                : (c <= 67644 || (c < 67712
                  ? (c < 67680
                    ? (c >= 67647 && c <= 67669)
                    : c <= 67702)
                  : (c <= 67742 || (c >= 67808 && c <= 67826)))))))
            : (c <= 67829 || (c < 68224
              ? (c < 68096
                ? (c < 67968
                  ? (c < 67872
                    ? (c >= 67840 && c <= 67861)
                    : c <= 67897)
                  : (c <= 68023 || (c >= 68030 && c <= 68031)))
                : (c <= 68096 || (c < 68121
                  ? (c < 68117
                    ? (c >= 68112 && c <= 68115)
                    : c <= 68119)
                  : (c <= 68149 || (c >= 68192 && c <= 68220)))))
              : (c <= 68252 || (c < 68448
                ? (c < 68352
                  ? (c < 68297
                    ? (c >= 68288 && c <= 68295)
                    : c <= 68324)
                  : (c <= 68405 || (c >= 68416 && c <= 68437)))
                : (c <= 68466 || (c < 68736
                  ? (c < 68608
                    ? (c >= 68480 && c <= 68497)
                    : c <= 68680)
                  : (c <= 68786 || (c >= 68800 && c <= 68850)))))))))
          : (c <= 68899 || (c < 70106
            ? (c < 69749
              ? (c < 69488
                ? (c < 69376
                  ? (c < 69296
                    ? (c >= 69248 && c <= 69289)
                    : c <= 69297)
                  : (c <= 69404 || (c < 69424
                    ? c == 69415
                    : c <= 69445)))
                : (c <= 69505 || (c < 69635
                  ? (c < 69600
                    ? (c >= 69552 && c <= 69572)
                    : c <= 69622)
                  : (c <= 69687 || (c >= 69745 && c <= 69746)))))
              : (c <= 69749 || (c < 69959
                ? (c < 69891
                  ? (c < 69840
                    ? (c >= 69763 && c <= 69807)
                    : c <= 69864)
                  : (c <= 69926 || c == 69956))
                : (c <= 69959 || (c < 70019
                  ? (c < 70006
                    ? (c >= 69968 && c <= 70002)
                    : c <= 70006)
                  : (c <= 70066 || (c >= 70081 && c <= 70084)))))))
            : (c <= 70106 || (c < 70405
              ? (c < 70280
                ? (c < 70163
                  ? (c < 70144
                    ? c == 70108
                    : c <= 70161)
                  : (c <= 70187 || (c >= 70272 && c <= 70278)))
                : (c <= 70280 || (c < 70303
                  ? (c < 70287
                    ? (c >= 70282 && c <= 70285)
                    : c <= 70301)
                  : (c <= 70312 || (c >= 70320 && c <= 70366)))))
              : (c <= 70412 || (c < 70453
                ? (c < 70442
                  ? (c < 70419
                    ? (c >= 70415 && c <= 70416)
                    : c <= 70440)
                  : (c <= 70448 || (c >= 70450 && c <= 70451)))
                : (c <= 70457 || (c < 70493
                  ? (c < 70480
                    ? c == 70461
                    : c <= 70480)
                  : (c <= 70497 || (c >= 70656 && c <= 70708)))))))))))))
      : (c <= 70730 || (c < 119894
        ? (c < 73056
          ? (c < 72001
            ? (c < 71424
              ? (c < 71128
                ? (c < 70852
                  ? (c < 70784
                    ? (c >= 70751 && c <= 70753)
                    : c <= 70831)
                  : (c <= 70853 || (c < 71040
                    ? c == 70855
                    : c <= 71086)))
                : (c <= 71131 || (c < 71296
                  ? (c < 71236
                    ? (c >= 71168 && c <= 71215)
                    : c <= 71236)
                  : (c <= 71338 || c == 71352))))
              : (c <= 71450 || (c < 71945
                ? (c < 71840
                  ? (c < 71680
                    ? (c >= 71488 && c <= 71494)
                    : c <= 71723)
                  : (c <= 71903 || (c >= 71935 && c <= 71942)))
                : (c <= 71945 || (c < 71960
                  ? (c < 71957
                    ? (c >= 71948 && c <= 71955)
                    : c <= 71958)
                  : (c <= 71983 || c == 71999))))))
            : (c <= 72001 || (c < 72349
              ? (c < 72192
                ? (c < 72161
                  ? (c < 72106
                    ? (c >= 72096 && c <= 72103)
                    : c <= 72144)
                  : (c <= 72161 || c == 72163))
                : (c <= 72192 || (c < 72272
                  ? (c < 72250
                    ? (c >= 72203 && c <= 72242)
                    : c <= 72250)
                  : (c <= 72272 || (c >= 72284 && c <= 72329)))))
              : (c <= 72349 || (c < 72818
                ? (c < 72714
                  ? (c < 72704
                    ? (c >= 72368 && c <= 72440)
                    : c <= 72712)
                  : (c <= 72750 || c == 72768))
                : (c <= 72847 || (c < 72971
                  ? (c < 72968
                    ? (c >= 72960 && c <= 72966)
                    : c <= 72969)
                  : (c <= 73008 || c == 73030))))))))
          : (c <= 73061 || (c < 93952
            ? (c < 82944
              ? (c < 73728
                ? (c < 73112
                  ? (c < 73066
                    ? (c >= 73063 && c <= 73064)
                    : c <= 73097)
                  : (c <= 73112 || (c < 73648
                    ? (c >= 73440 && c <= 73458)
                    : c <= 73648)))
                : (c <= 74649 || (c < 77712
                  ? (c < 74880
                    ? (c >= 74752 && c <= 74862)
                    : c <= 75075)
                  : (c <= 77808 || (c >= 77824 && c <= 78894)))))
              : (c <= 83526 || (c < 92928
                ? (c < 92784
                  ? (c < 92736
                    ? (c >= 92160 && c <= 92728)
                    : c <= 92766)
                  : (c <= 92862 || (c >= 92880 && c <= 92909)))
                : (c <= 92975 || (c < 93053
                  ? (c < 93027
                    ? (c >= 92992 && c <= 92995)
                    : c <= 93047)
                  : (c <= 93071 || (c >= 93760 && c <= 93823)))))))
            : (c <= 94026 || (c < 110589
              ? (c < 94208
                ? (c < 94176
                  ? (c < 94099
                    ? c == 94032
                    : c <= 94111)
                  : (c <= 94177 || c == 94179))
                : (c <= 100343 || (c < 110576
                  ? (c < 101632
                    ? (c >= 100352 && c <= 101589)
                    : c <= 101640)
                  : (c <= 110579 || (c >= 110581 && c <= 110587)))))
              : (c <= 110590 || (c < 113664
                ? (c < 110948
                  ? (c < 110928
                    ? (c >= 110592 && c <= 110882)
                    : c <= 110930)
                  : (c <= 110951 || (c >= 110960 && c <= 111355)))
                : (c <= 113770 || (c < 113808
                  ? (c < 113792
                    ? (c >= 113776 && c <= 113788)
                    : c <= 113800)
                  : (c <= 113817 || (c >= 119808 && c <= 119892)))))))))))
        : (c <= 119964 || (c < 125259
          ? (c < 120572
            ? (c < 120086
              ? (c < 119995
                ? (c < 119973
                  ? (c < 119970
                    ? (c >= 119966 && c <= 119967)
                    : c <= 119970)
                  : (c <= 119974 || (c < 119982
                    ? (c >= 119977 && c <= 119980)
                    : c <= 119993)))
                : (c <= 119995 || (c < 120071
                  ? (c < 120005
                    ? (c >= 119997 && c <= 120003)
                    : c <= 120069)
                  : (c <= 120074 || (c >= 120077 && c <= 120084)))))
              : (c <= 120092 || (c < 120138
                ? (c < 120128
                  ? (c < 120123
                    ? (c >= 120094 && c <= 120121)
                    : c <= 120126)
                  : (c <= 120132 || c == 120134))
                : (c <= 120144 || (c < 120514
                  ? (c < 120488
                    ? (c >= 120146 && c <= 120485)
                    : c <= 120512)
                  : (c <= 120538 || (c >= 120540 && c <= 120570)))))))
            : (c <= 120596 || (c < 123191
              ? (c < 120714
                ? (c < 120656
                  ? (c < 120630
                    ? (c >= 120598 && c <= 120628)
                    : c <= 120654)
                  : (c <= 120686 || (c >= 120688 && c <= 120712)))
                : (c <= 120744 || (c < 122624
                  ? (c < 120772
                    ? (c >= 120746 && c <= 120770)
                    : c <= 120779)
                  : (c <= 122654 || (c >= 123136 && c <= 123180)))))
              : (c <= 123197 || (c < 124904
                ? (c < 123584
                  ? (c < 123536
                    ? c == 123214
                    : c <= 123565)
                  : (c <= 123627 || (c >= 124896 && c <= 124902)))
                : (c <= 124907 || (c < 124928
                  ? (c < 124912
                    ? (c >= 124909 && c <= 124910)
                    : c <= 124926)
                  : (c <= 125124 || (c >= 125184 && c <= 125251)))))))))
          : (c <= 125259 || (c < 126559
            ? (c < 126535
              ? (c < 126505
                ? (c < 126497
                  ? (c < 126469
                    ? (c >= 126464 && c <= 126467)
                    : c <= 126495)
                  : (c <= 126498 || (c < 126503
                    ? c == 126500
                    : c <= 126503)))
                : (c <= 126514 || (c < 126523
                  ? (c < 126521
                    ? (c >= 126516 && c <= 126519)
                    : c <= 126521)
                  : (c <= 126523 || c == 126530))))
              : (c <= 126535 || (c < 126548
                ? (c < 126541
                  ? (c < 126539
                    ? c == 126537
                    : c <= 126539)
                  : (c <= 126543 || (c >= 126545 && c <= 126546)))
                : (c <= 126548 || (c < 126555
                  ? (c < 126553
                    ? c == 126551
                    : c <= 126553)
                  : (c <= 126555 || c == 126557))))))
            : (c <= 126559 || (c < 126625
              ? (c < 126580
                ? (c < 126567
                  ? (c < 126564
                    ? (c >= 126561 && c <= 126562)
                    : c <= 126564)
                  : (c <= 126570 || (c >= 126572 && c <= 126578)))
                : (c <= 126583 || (c < 126592
                  ? (c < 126590
                    ? (c >= 126585 && c <= 126588)
                    : c <= 126590)
                  : (c <= 126601 || (c >= 126603 && c <= 126619)))))
              : (c <= 126627 || (c < 177984
                ? (c < 131072
                  ? (c < 126635
                    ? (c >= 126629 && c <= 126633)
                    : c <= 126651)
                  : (c <= 173791 || (c >= 173824 && c <= 177976)))
                : (c <= 178205 || (c < 194560
                  ? (c < 183984
                    ? (c >= 178208 && c <= 183969)
                    : c <= 191456)
                  : (c <= 195101 || (c >= 196608 && c <= 201546)))))))))))))))));
}

static inline bool sym_identifier_character_set_2(int32_t c) {
  return (c < 43494
    ? (c < 4176
      ? (c < 2693
        ? (c < 1869
          ? (c < 904
            ? (c < 248
              ? (c < 170
                ? (c < 'a'
                  ? (c < '_'
                    ? (c >= 'A' && c <= 'Z')
                    : c <= '_')
                  : (c <= 'a' || (c < 's'
                    ? (c >= 'c' && c <= 'q')
                    : c <= 'z')))
                : (c <= 170 || (c < 192
                  ? (c < 186
                    ? c == 181
                    : c <= 186)
                  : (c <= 214 || (c >= 216 && c <= 246)))))
              : (c <= 705 || (c < 880
                ? (c < 748
                  ? (c < 736
                    ? (c >= 710 && c <= 721)
                    : c <= 740)
                  : (c <= 748 || c == 750))
                : (c <= 884 || (c < 895
                  ? (c < 891
                    ? (c >= 886 && c <= 887)
                    : c <= 893)
                  : (c <= 895 || c == 902))))))
            : (c <= 906 || (c < 1568
              ? (c < 1329
                ? (c < 931
                  ? (c < 910
                    ? c == 908
                    : c <= 929)
                  : (c <= 1013 || (c < 1162
                    ? (c >= 1015 && c <= 1153)
                    : c <= 1327)))
                : (c <= 1366 || (c < 1488
                  ? (c < 1376
                    ? c == 1369
                    : c <= 1416)
                  : (c <= 1514 || (c >= 1519 && c <= 1522)))))
              : (c <= 1610 || (c < 1774
                ? (c < 1749
                  ? (c < 1649
                    ? (c >= 1646 && c <= 1647)
                    : c <= 1747)
                  : (c <= 1749 || (c >= 1765 && c <= 1766)))
                : (c <= 1775 || (c < 1808
                  ? (c < 1791
                    ? (c >= 1786 && c <= 1788)
                    : c <= 1791)
                  : (c <= 1808 || (c >= 1810 && c <= 1839)))))))))
          : (c <= 1957 || (c < 2451
            ? (c < 2160
              ? (c < 2074
                ? (c < 2036
                  ? (c < 1994
                    ? c == 1969
                    : c <= 2026)
                  : (c <= 2037 || (c < 2048
                    ? c == 2042
                    : c <= 2069)))
                : (c <= 2074 || (c < 2112
                  ? (c < 2088
                    ? c == 2084
                    : c <= 2088)
                  : (c <= 2136 || (c >= 2144 && c <= 2154)))))
              : (c <= 2183 || (c < 2384
                ? (c < 2308
                  ? (c < 2208
                    ? (c >= 2185 && c <= 2190)
                    : c <= 2249)
                  : (c <= 2361 || c == 2365))
                : (c <= 2384 || (c < 2437
                  ? (c < 2417
                    ? (c >= 2392 && c <= 2401)
                    : c <= 2432)
                  : (c <= 2444 || (c >= 2447 && c <= 2448)))))))
            : (c <= 2472 || (c < 2565
              ? (c < 2510
                ? (c < 2486
                  ? (c < 2482
                    ? (c >= 2474 && c <= 2480)
                    : c <= 2482)
                  : (c <= 2489 || c == 2493))
                : (c <= 2510 || (c < 2544
                  ? (c < 2527
                    ? (c >= 2524 && c <= 2525)
                    : c <= 2529)
                  : (c <= 2545 || c == 2556))))
              : (c <= 2570 || (c < 2613
                ? (c < 2602
                  ? (c < 2579
                    ? (c >= 2575 && c <= 2576)
                    : c <= 2600)
                  : (c <= 2608 || (c >= 2610 && c <= 2611)))
                : (c <= 2614 || (c < 2654
                  ? (c < 2649
                    ? (c >= 2616 && c <= 2617)
                    : c <= 2652)
                  : (c <= 2654 || (c >= 2674 && c <= 2676)))))))))))
        : (c <= 2701 || (c < 3214
          ? (c < 2949
            ? (c < 2831
              ? (c < 2749
                ? (c < 2730
                  ? (c < 2707
                    ? (c >= 2703 && c <= 2705)
                    : c <= 2728)
                  : (c <= 2736 || (c < 2741
                    ? (c >= 2738 && c <= 2739)
                    : c <= 2745)))
                : (c <= 2749 || (c < 2809
                  ? (c < 2784
                    ? c == 2768
                    : c <= 2785)
                  : (c <= 2809 || (c >= 2821 && c <= 2828)))))
              : (c <= 2832 || (c < 2877
                ? (c < 2866
                  ? (c < 2858
                    ? (c >= 2835 && c <= 2856)
                    : c <= 2864)
                  : (c <= 2867 || (c >= 2869 && c <= 2873)))
                : (c <= 2877 || (c < 2929
                  ? (c < 2911
                    ? (c >= 2908 && c <= 2909)
                    : c <= 2913)
                  : (c <= 2929 || c == 2947))))))
            : (c <= 2954 || (c < 3077
              ? (c < 2974
                ? (c < 2969
                  ? (c < 2962
                    ? (c >= 2958 && c <= 2960)
                    : c <= 2965)
                  : (c <= 2970 || c == 2972))
                : (c <= 2975 || (c < 2990
                  ? (c < 2984
                    ? (c >= 2979 && c <= 2980)
                    : c <= 2986)
                  : (c <= 3001 || c == 3024))))
              : (c <= 3084 || (c < 3160
                ? (c < 3114
                  ? (c < 3090
                    ? (c >= 3086 && c <= 3088)
                    : c <= 3112)
                  : (c <= 3129 || c == 3133))
                : (c <= 3162 || (c < 3200
                  ? (c < 3168
                    ? c == 3165
                    : c <= 3169)
                  : (c <= 3200 || (c >= 3205 && c <= 3212)))))))))
          : (c <= 3216 || (c < 3585
            ? (c < 3389
              ? (c < 3296
                ? (c < 3253
                  ? (c < 3242
                    ? (c >= 3218 && c <= 3240)
                    : c <= 3251)
                  : (c <= 3257 || (c < 3293
                    ? c == 3261
                    : c <= 3294)))
                : (c <= 3297 || (c < 3342
                  ? (c < 3332
                    ? (c >= 3313 && c <= 3314)
                    : c <= 3340)
                  : (c <= 3344 || (c >= 3346 && c <= 3386)))))
              : (c <= 3389 || (c < 3461
                ? (c < 3423
                  ? (c < 3412
                    ? c == 3406
                    : c <= 3414)
                  : (c <= 3425 || (c >= 3450 && c <= 3455)))
                : (c <= 3478 || (c < 3517
                  ? (c < 3507
                    ? (c >= 3482 && c <= 3505)
                    : c <= 3515)
                  : (c <= 3517 || (c >= 3520 && c <= 3526)))))))
            : (c <= 3632 || (c < 3773
              ? (c < 3718
                ? (c < 3713
                  ? (c < 3648
                    ? c == 3634
                    : c <= 3654)
                  : (c <= 3714 || c == 3716))
                : (c <= 3722 || (c < 3751
                  ? (c < 3749
                    ? (c >= 3724 && c <= 3747)
                    : c <= 3749)
                  : (c <= 3760 || c == 3762))))
              : (c <= 3773 || (c < 3904
                ? (c < 3804
                  ? (c < 3782
                    ? (c >= 3776 && c <= 3780)
                    : c <= 3782)
                  : (c <= 3807 || c == 3840))
                : (c <= 3911 || (c < 4096
                  ? (c < 3976
                    ? (c >= 3913 && c <= 3948)
                    : c <= 3980)
                  : (c <= 4138 || c == 4159))))))))))))
      : (c <= 4181 || (c < 8130
        ? (c < 6108
          ? (c < 4800
            ? (c < 4348
              ? (c < 4238
                ? (c < 4197
                  ? (c < 4193
                    ? (c >= 4186 && c <= 4189)
                    : c <= 4193)
                  : (c <= 4198 || (c < 4213
                    ? (c >= 4206 && c <= 4208)
                    : c <= 4225)))
                : (c <= 4238 || (c < 4301
                  ? (c < 4295
                    ? (c >= 4256 && c <= 4293)
                    : c <= 4295)
                  : (c <= 4301 || (c >= 4304 && c <= 4346)))))
              : (c <= 4680 || (c < 4704
                ? (c < 4696
                  ? (c < 4688
                    ? (c >= 4682 && c <= 4685)
                    : c <= 4694)
                  : (c <= 4696 || (c >= 4698 && c <= 4701)))
                : (c <= 4744 || (c < 4786
                  ? (c < 4752
                    ? (c >= 4746 && c <= 4749)
                    : c <= 4784)
                  : (c <= 4789 || (c >= 4792 && c <= 4798)))))))
            : (c <= 4800 || (c < 5761
              ? (c < 4992
                ? (c < 4824
                  ? (c < 4808
                    ? (c >= 4802 && c <= 4805)
                    : c <= 4822)
                  : (c <= 4880 || (c < 4888
                    ? (c >= 4882 && c <= 4885)
                    : c <= 4954)))
                : (c <= 5007 || (c < 5121
                  ? (c < 5112
                    ? (c >= 5024 && c <= 5109)
                    : c <= 5117)
                  : (c <= 5740 || (c >= 5743 && c <= 5759)))))
              : (c <= 5786 || (c < 5952
                ? (c < 5888
                  ? (c < 5870
                    ? (c >= 5792 && c <= 5866)
                    : c <= 5880)
                  : (c <= 5905 || (c >= 5919 && c <= 5937)))
                : (c <= 5969 || (c < 6016
                  ? (c < 5998
                    ? (c >= 5984 && c <= 5996)
                    : c <= 6000)
                  : (c <= 6067 || c == 6103))))))))
          : (c <= 6108 || (c < 7296
            ? (c < 6688
              ? (c < 6480
                ? (c < 6314
                  ? (c < 6272
                    ? (c >= 6176 && c <= 6264)
                    : c <= 6312)
                  : (c <= 6314 || (c < 6400
                    ? (c >= 6320 && c <= 6389)
                    : c <= 6430)))
                : (c <= 6509 || (c < 6576
                  ? (c < 6528
                    ? (c >= 6512 && c <= 6516)
                    : c <= 6571)
                  : (c <= 6601 || (c >= 6656 && c <= 6678)))))
              : (c <= 6740 || (c < 7086
                ? (c < 6981
                  ? (c < 6917
                    ? c == 6823
                    : c <= 6963)
                  : (c <= 6988 || (c >= 7043 && c <= 7072)))
                : (c <= 7087 || (c < 7245
                  ? (c < 7168
                    ? (c >= 7098 && c <= 7141)
                    : c <= 7203)
                  : (c <= 7247 || (c >= 7258 && c <= 7293)))))))
            : (c <= 7304 || (c < 7968
              ? (c < 7413
                ? (c < 7401
                  ? (c < 7357
                    ? (c >= 7312 && c <= 7354)
                    : c <= 7359)
                  : (c <= 7404 || (c >= 7406 && c <= 7411)))
                : (c <= 7414 || (c < 7680
                  ? (c < 7424
                    ? c == 7418
                    : c <= 7615)
                  : (c <= 7957 || (c >= 7960 && c <= 7965)))))
              : (c <= 8005 || (c < 8029
                ? (c < 8025
                  ? (c < 8016
                    ? (c >= 8008 && c <= 8013)
                    : c <= 8023)
                  : (c <= 8025 || c == 8027))
                : (c <= 8029 || (c < 8118
                  ? (c < 8064
                    ? (c >= 8031 && c <= 8061)
                    : c <= 8116)
                  : (c <= 8124 || c == 8126))))))))))
        : (c <= 8132 || (c < 12321
          ? (c < 8526
            ? (c < 8455
              ? (c < 8182
                ? (c < 8150
                  ? (c < 8144
                    ? (c >= 8134 && c <= 8140)
                    : c <= 8147)
                  : (c <= 8155 || (c < 8178
                    ? (c >= 8160 && c <= 8172)
                    : c <= 8180)))
                : (c <= 8188 || (c < 8336
                  ? (c < 8319
                    ? c == 8305
                    : c <= 8319)
                  : (c <= 8348 || c == 8450))))
              : (c <= 8455 || (c < 8486
                ? (c < 8472
                  ? (c < 8469
                    ? (c >= 8458 && c <= 8467)
                    : c <= 8469)
                  : (c <= 8477 || c == 8484))
                : (c <= 8486 || (c < 8508
                  ? (c < 8490
                    ? c == 8488
                    : c <= 8505)
                  : (c <= 8511 || (c >= 8517 && c <= 8521)))))))
            : (c <= 8526 || (c < 11648
              ? (c < 11520
                ? (c < 11499
                  ? (c < 11264
                    ? (c >= 8544 && c <= 8584)
                    : c <= 11492)
                  : (c <= 11502 || (c >= 11506 && c <= 11507)))
                : (c <= 11557 || (c < 11568
                  ? (c < 11565
                    ? c == 11559
                    : c <= 11565)
                  : (c <= 11623 || c == 11631))))
              : (c <= 11670 || (c < 11712
                ? (c < 11696
                  ? (c < 11688
                    ? (c >= 11680 && c <= 11686)
                    : c <= 11694)
                  : (c <= 11702 || (c >= 11704 && c <= 11710)))
                : (c <= 11718 || (c < 11736
                  ? (c < 11728
                    ? (c >= 11720 && c <= 11726)
                    : c <= 11734)
                  : (c <= 11742 || (c >= 12293 && c <= 12295)))))))))
          : (c <= 12329 || (c < 42786
            ? (c < 13312
              ? (c < 12540
                ? (c < 12353
                  ? (c < 12344
                    ? (c >= 12337 && c <= 12341)
                    : c <= 12348)
                  : (c <= 12438 || (c < 12449
                    ? (c >= 12445 && c <= 12447)
                    : c <= 12538)))
                : (c <= 12543 || (c < 12704
                  ? (c < 12593
                    ? (c >= 12549 && c <= 12591)
                    : c <= 12686)
                  : (c <= 12735 || (c >= 12784 && c <= 12799)))))
              : (c <= 19903 || (c < 42538
                ? (c < 42240
                  ? (c < 42192
                    ? (c >= 19968 && c <= 42124)
                    : c <= 42237)
                  : (c <= 42508 || (c >= 42512 && c <= 42527)))
                : (c <= 42539 || (c < 42656
                  ? (c < 42623
                    ? (c >= 42560 && c <= 42606)
                    : c <= 42653)
                  : (c <= 42735 || (c >= 42775 && c <= 42783)))))))
            : (c <= 42888 || (c < 43138
              ? (c < 42994
                ? (c < 42963
                  ? (c < 42960
                    ? (c >= 42891 && c <= 42954)
                    : c <= 42961)
                  : (c <= 42963 || (c >= 42965 && c <= 42969)))
                : (c <= 43009 || (c < 43020
                  ? (c < 43015
                    ? (c >= 43011 && c <= 43013)
                    : c <= 43018)
                  : (c <= 43042 || (c >= 43072 && c <= 43123)))))
              : (c <= 43187 || (c < 43312
                ? (c < 43261
                  ? (c < 43259
                    ? (c >= 43250 && c <= 43255)
                    : c <= 43259)
                  : (c <= 43262 || (c >= 43274 && c <= 43301)))
                : (c <= 43334 || (c < 43471
                  ? (c < 43396
                    ? (c >= 43360 && c <= 43388)
                    : c <= 43442)
                  : (c <= 43471 || (c >= 43488 && c <= 43492)))))))))))))))
    : (c <= 43503 || (c < 70727
      ? (c < 66956
        ? (c < 64914
          ? (c < 43824
            ? (c < 43712
              ? (c < 43642
                ? (c < 43584
                  ? (c < 43520
                    ? (c >= 43514 && c <= 43518)
                    : c <= 43560)
                  : (c <= 43586 || (c < 43616
                    ? (c >= 43588 && c <= 43595)
                    : c <= 43638)))
                : (c <= 43642 || (c < 43701
                  ? (c < 43697
                    ? (c >= 43646 && c <= 43695)
                    : c <= 43697)
                  : (c <= 43702 || (c >= 43705 && c <= 43709)))))
              : (c <= 43712 || (c < 43777
                ? (c < 43744
                  ? (c < 43739
                    ? c == 43714
                    : c <= 43741)
                  : (c <= 43754 || (c >= 43762 && c <= 43764)))
                : (c <= 43782 || (c < 43808
                  ? (c < 43793
                    ? (c >= 43785 && c <= 43790)
                    : c <= 43798)
                  : (c <= 43814 || (c >= 43816 && c <= 43822)))))))
            : (c <= 43866 || (c < 64287
              ? (c < 63744
                ? (c < 44032
                  ? (c < 43888
                    ? (c >= 43868 && c <= 43881)
                    : c <= 44002)
                  : (c <= 55203 || (c < 55243
                    ? (c >= 55216 && c <= 55238)
                    : c <= 55291)))
                : (c <= 64109 || (c < 64275
                  ? (c < 64256
                    ? (c >= 64112 && c <= 64217)
                    : c <= 64262)
                  : (c <= 64279 || c == 64285))))
              : (c <= 64296 || (c < 64323
                ? (c < 64318
                  ? (c < 64312
                    ? (c >= 64298 && c <= 64310)
                    : c <= 64316)
                  : (c <= 64318 || (c >= 64320 && c <= 64321)))
                : (c <= 64324 || (c < 64612
                  ? (c < 64467
                    ? (c >= 64326 && c <= 64433)
                    : c <= 64605)
                  : (c <= 64829 || (c >= 64848 && c <= 64911)))))))))
          : (c <= 64967 || (c < 65599
            ? (c < 65382
              ? (c < 65147
                ? (c < 65139
                  ? (c < 65137
                    ? (c >= 65008 && c <= 65017)
                    : c <= 65137)
                  : (c <= 65139 || (c < 65145
                    ? c == 65143
                    : c <= 65145)))
                : (c <= 65147 || (c < 65313
                  ? (c < 65151
                    ? c == 65149
                    : c <= 65276)
                  : (c <= 65338 || (c >= 65345 && c <= 65370)))))
              : (c <= 65437 || (c < 65498
                ? (c < 65482
                  ? (c < 65474
                    ? (c >= 65440 && c <= 65470)
                    : c <= 65479)
                  : (c <= 65487 || (c >= 65490 && c <= 65495)))
                : (c <= 65500 || (c < 65576
                  ? (c < 65549
                    ? (c >= 65536 && c <= 65547)
                    : c <= 65574)
                  : (c <= 65594 || (c >= 65596 && c <= 65597)))))))
            : (c <= 65613 || (c < 66464
              ? (c < 66208
                ? (c < 65856
                  ? (c < 65664
                    ? (c >= 65616 && c <= 65629)
                    : c <= 65786)
                  : (c <= 65908 || (c >= 66176 && c <= 66204)))
                : (c <= 66256 || (c < 66384
                  ? (c < 66349
                    ? (c >= 66304 && c <= 66335)
                    : c <= 66378)
                  : (c <= 66421 || (c >= 66432 && c <= 66461)))))
              : (c <= 66499 || (c < 66776
                ? (c < 66560
                  ? (c < 66513
                    ? (c >= 66504 && c <= 66511)
                    : c <= 66517)
                  : (c <= 66717 || (c >= 66736 && c <= 66771)))
                : (c <= 66811 || (c < 66928
                  ? (c < 66864
                    ? (c >= 66816 && c <= 66855)
                    : c <= 66915)
                  : (c <= 66938 || (c >= 66940 && c <= 66954)))))))))))
        : (c <= 66962 || (c < 68864
          ? (c < 67828
            ? (c < 67506
              ? (c < 67072
                ? (c < 66979
                  ? (c < 66967
                    ? (c >= 66964 && c <= 66965)
                    : c <= 66977)
                  : (c <= 66993 || (c < 67003
                    ? (c >= 66995 && c <= 67001)
                    : c <= 67004)))
                : (c <= 67382 || (c < 67456
                  ? (c < 67424
                    ? (c >= 67392 && c <= 67413)
                    : c <= 67431)
                  : (c <= 67461 || (c >= 67463 && c <= 67504)))))
              : (c <= 67514 || (c < 67644
                ? (c < 67594
                  ? (c < 67592
                    ? (c >= 67584 && c <= 67589)
                    : c <= 67592)
                  : (c <= 67637 || (c >= 67639 && c <= 67640)))
                : (c <= 67644 || (c < 67712
                  ? (c < 67680
                    ? (c >= 67647 && c <= 67669)
                    : c <= 67702)
                  : (c <= 67742 || (c >= 67808 && c <= 67826)))))))
            : (c <= 67829 || (c < 68224
              ? (c < 68096
                ? (c < 67968
                  ? (c < 67872
                    ? (c >= 67840 && c <= 67861)
                    : c <= 67897)
                  : (c <= 68023 || (c >= 68030 && c <= 68031)))
                : (c <= 68096 || (c < 68121
                  ? (c < 68117
                    ? (c >= 68112 && c <= 68115)
                    : c <= 68119)
                  : (c <= 68149 || (c >= 68192 && c <= 68220)))))
              : (c <= 68252 || (c < 68448
                ? (c < 68352
                  ? (c < 68297
                    ? (c >= 68288 && c <= 68295)
                    : c <= 68324)
                  : (c <= 68405 || (c >= 68416 && c <= 68437)))
                : (c <= 68466 || (c < 68736
                  ? (c < 68608
                    ? (c >= 68480 && c <= 68497)
                    : c <= 68680)
                  : (c <= 68786 || (c >= 68800 && c <= 68850)))))))))
          : (c <= 68899 || (c < 70106
            ? (c < 69749
              ? (c < 69488
                ? (c < 69376
                  ? (c < 69296
                    ? (c >= 69248 && c <= 69289)
                    : c <= 69297)
                  : (c <= 69404 || (c < 69424
                    ? c == 69415
                    : c <= 69445)))
                : (c <= 69505 || (c < 69635
                  ? (c < 69600
                    ? (c >= 69552 && c <= 69572)
                    : c <= 69622)
                  : (c <= 69687 || (c >= 69745 && c <= 69746)))))
              : (c <= 69749 || (c < 69959
                ? (c < 69891
                  ? (c < 69840
                    ? (c >= 69763 && c <= 69807)
                    : c <= 69864)
                  : (c <= 69926 || c == 69956))
                : (c <= 69959 || (c < 70019
                  ? (c < 70006
                    ? (c >= 69968 && c <= 70002)
                    : c <= 70006)
                  : (c <= 70066 || (c >= 70081 && c <= 70084)))))))
            : (c <= 70106 || (c < 70405
              ? (c < 70280
                ? (c < 70163
                  ? (c < 70144
                    ? c == 70108
                    : c <= 70161)
                  : (c <= 70187 || (c >= 70272 && c <= 70278)))
                : (c <= 70280 || (c < 70303
                  ? (c < 70287
                    ? (c >= 70282 && c <= 70285)
                    : c <= 70301)
                  : (c <= 70312 || (c >= 70320 && c <= 70366)))))
              : (c <= 70412 || (c < 70453
                ? (c < 70442
                  ? (c < 70419
                    ? (c >= 70415 && c <= 70416)
                    : c <= 70440)
                  : (c <= 70448 || (c >= 70450 && c <= 70451)))
                : (c <= 70457 || (c < 70493
                  ? (c < 70480
                    ? c == 70461
                    : c <= 70480)
                  : (c <= 70497 || (c >= 70656 && c <= 70708)))))))))))))
      : (c <= 70730 || (c < 119894
        ? (c < 73056
          ? (c < 72001
            ? (c < 71424
              ? (c < 71128
                ? (c < 70852
                  ? (c < 70784
                    ? (c >= 70751 && c <= 70753)
                    : c <= 70831)
                  : (c <= 70853 || (c < 71040
                    ? c == 70855
                    : c <= 71086)))
                : (c <= 71131 || (c < 71296
                  ? (c < 71236
                    ? (c >= 71168 && c <= 71215)
                    : c <= 71236)
                  : (c <= 71338 || c == 71352))))
              : (c <= 71450 || (c < 71945
                ? (c < 71840
                  ? (c < 71680
                    ? (c >= 71488 && c <= 71494)
                    : c <= 71723)
                  : (c <= 71903 || (c >= 71935 && c <= 71942)))
                : (c <= 71945 || (c < 71960
                  ? (c < 71957
                    ? (c >= 71948 && c <= 71955)
                    : c <= 71958)
                  : (c <= 71983 || c == 71999))))))
            : (c <= 72001 || (c < 72349
              ? (c < 72192
                ? (c < 72161
                  ? (c < 72106
                    ? (c >= 72096 && c <= 72103)
                    : c <= 72144)
                  : (c <= 72161 || c == 72163))
                : (c <= 72192 || (c < 72272
                  ? (c < 72250
                    ? (c >= 72203 && c <= 72242)
                    : c <= 72250)
                  : (c <= 72272 || (c >= 72284 && c <= 72329)))))
              : (c <= 72349 || (c < 72818
                ? (c < 72714
                  ? (c < 72704
                    ? (c >= 72368 && c <= 72440)
                    : c <= 72712)
                  : (c <= 72750 || c == 72768))
                : (c <= 72847 || (c < 72971
                  ? (c < 72968
                    ? (c >= 72960 && c <= 72966)
                    : c <= 72969)
                  : (c <= 73008 || c == 73030))))))))
          : (c <= 73061 || (c < 93952
            ? (c < 82944
              ? (c < 73728
                ? (c < 73112
                  ? (c < 73066
                    ? (c >= 73063 && c <= 73064)
                    : c <= 73097)
                  : (c <= 73112 || (c < 73648
                    ? (c >= 73440 && c <= 73458)
                    : c <= 73648)))
                : (c <= 74649 || (c < 77712
                  ? (c < 74880
                    ? (c >= 74752 && c <= 74862)
                    : c <= 75075)
                  : (c <= 77808 || (c >= 77824 && c <= 78894)))))
              : (c <= 83526 || (c < 92928
                ? (c < 92784
                  ? (c < 92736
                    ? (c >= 92160 && c <= 92728)
                    : c <= 92766)
                  : (c <= 92862 || (c >= 92880 && c <= 92909)))
                : (c <= 92975 || (c < 93053
                  ? (c < 93027
                    ? (c >= 92992 && c <= 92995)
                    : c <= 93047)
                  : (c <= 93071 || (c >= 93760 && c <= 93823)))))))
            : (c <= 94026 || (c < 110589
              ? (c < 94208
                ? (c < 94176
                  ? (c < 94099
                    ? c == 94032
                    : c <= 94111)
                  : (c <= 94177 || c == 94179))
                : (c <= 100343 || (c < 110576
                  ? (c < 101632
                    ? (c >= 100352 && c <= 101589)
                    : c <= 101640)
                  : (c <= 110579 || (c >= 110581 && c <= 110587)))))
              : (c <= 110590 || (c < 113664
                ? (c < 110948
                  ? (c < 110928
                    ? (c >= 110592 && c <= 110882)
                    : c <= 110930)
                  : (c <= 110951 || (c >= 110960 && c <= 111355)))
                : (c <= 113770 || (c < 113808
                  ? (c < 113792
                    ? (c >= 113776 && c <= 113788)
                    : c <= 113800)
                  : (c <= 113817 || (c >= 119808 && c <= 119892)))))))))))
        : (c <= 119964 || (c < 125259
          ? (c < 120572
            ? (c < 120086
              ? (c < 119995
                ? (c < 119973
                  ? (c < 119970
                    ? (c >= 119966 && c <= 119967)
                    : c <= 119970)
                  : (c <= 119974 || (c < 119982
                    ? (c >= 119977 && c <= 119980)
                    : c <= 119993)))
                : (c <= 119995 || (c < 120071
                  ? (c < 120005
                    ? (c >= 119997 && c <= 120003)
                    : c <= 120069)
                  : (c <= 120074 || (c >= 120077 && c <= 120084)))))
              : (c <= 120092 || (c < 120138
                ? (c < 120128
                  ? (c < 120123
                    ? (c >= 120094 && c <= 120121)
                    : c <= 120126)
                  : (c <= 120132 || c == 120134))
                : (c <= 120144 || (c < 120514
                  ? (c < 120488
                    ? (c >= 120146 && c <= 120485)
                    : c <= 120512)
                  : (c <= 120538 || (c >= 120540 && c <= 120570)))))))
            : (c <= 120596 || (c < 123191
              ? (c < 120714
                ? (c < 120656
                  ? (c < 120630
                    ? (c >= 120598 && c <= 120628)
                    : c <= 120654)
                  : (c <= 120686 || (c >= 120688 && c <= 120712)))
                : (c <= 120744 || (c < 122624
                  ? (c < 120772
                    ? (c >= 120746 && c <= 120770)
                    : c <= 120779)
                  : (c <= 122654 || (c >= 123136 && c <= 123180)))))
              : (c <= 123197 || (c < 124904
                ? (c < 123584
                  ? (c < 123536
                    ? c == 123214
                    : c <= 123565)
                  : (c <= 123627 || (c >= 124896 && c <= 124902)))
                : (c <= 124907 || (c < 124928
                  ? (c < 124912
                    ? (c >= 124909 && c <= 124910)
                    : c <= 124926)
                  : (c <= 125124 || (c >= 125184 && c <= 125251)))))))))
          : (c <= 125259 || (c < 126559
            ? (c < 126535
              ? (c < 126505
                ? (c < 126497
                  ? (c < 126469
                    ? (c >= 126464 && c <= 126467)
                    : c <= 126495)
                  : (c <= 126498 || (c < 126503
                    ? c == 126500
                    : c <= 126503)))
                : (c <= 126514 || (c < 126523
                  ? (c < 126521
                    ? (c >= 126516 && c <= 126519)
                    : c <= 126521)
                  : (c <= 126523 || c == 126530))))
              : (c <= 126535 || (c < 126548
                ? (c < 126541
                  ? (c < 126539
                    ? c == 126537
                    : c <= 126539)
                  : (c <= 126543 || (c >= 126545 && c <= 126546)))
                : (c <= 126548 || (c < 126555
                  ? (c < 126553
                    ? c == 126551
                    : c <= 126553)
                  : (c <= 126555 || c == 126557))))))
            : (c <= 126559 || (c < 126625
              ? (c < 126580
                ? (c < 126567
                  ? (c < 126564
                    ? (c >= 126561 && c <= 126562)
                    : c <= 126564)
                  : (c <= 126570 || (c >= 126572 && c <= 126578)))
                : (c <= 126583 || (c < 126592
                  ? (c < 126590
                    ? (c >= 126585 && c <= 126588)
                    : c <= 126590)
                  : (c <= 126601 || (c >= 126603 && c <= 126619)))))
              : (c <= 126627 || (c < 177984
                ? (c < 131072
                  ? (c < 126635
                    ? (c >= 126629 && c <= 126633)
                    : c <= 126651)
                  : (c <= 173791 || (c >= 173824 && c <= 177976)))
                : (c <= 178205 || (c < 194560
                  ? (c < 183984
                    ? (c >= 178208 && c <= 183969)
                    : c <= 191456)
                  : (c <= 195101 || (c >= 196608 && c <= 201546)))))))))))))))));
}

static inline bool sym_identifier_character_set_3(int32_t c) {
  return (c < 43616
    ? (c < 3782
      ? (c < 2748
        ? (c < 2045
          ? (c < 1015
            ? (c < 710
              ? (c < 181
                ? (c < '_'
                  ? (c < 'A'
                    ? (c >= '0' && c <= '9')
                    : c <= 'Z')
                  : (c <= '_' || (c < 170
                    ? (c >= 'a' && c <= 'z')
                    : c <= 170)))
                : (c <= 181 || (c < 192
                  ? (c < 186
                    ? c == 183
                    : c <= 186)
                  : (c <= 214 || (c < 248
                    ? (c >= 216 && c <= 246)
                    : c <= 705)))))
              : (c <= 721 || (c < 891
                ? (c < 750
                  ? (c < 748
                    ? (c >= 736 && c <= 740)
                    : c <= 748)
                  : (c <= 750 || (c < 886
                    ? (c >= 768 && c <= 884)
                    : c <= 887)))
                : (c <= 893 || (c < 908
                  ? (c < 902
                    ? c == 895
                    : c <= 906)
                  : (c <= 908 || (c < 931
                    ? (c >= 910 && c <= 929)
                    : c <= 1013)))))))
            : (c <= 1153 || (c < 1519
              ? (c < 1425
                ? (c < 1329
                  ? (c < 1162
                    ? (c >= 1155 && c <= 1159)
                    : c <= 1327)
                  : (c <= 1366 || (c < 1376
                    ? c == 1369
                    : c <= 1416)))
                : (c <= 1469 || (c < 1476
                  ? (c < 1473
                    ? c == 1471
                    : c <= 1474)
                  : (c <= 1477 || (c < 1488
                    ? c == 1479
                    : c <= 1514)))))
              : (c <= 1522 || (c < 1770
                ? (c < 1646
                  ? (c < 1568
                    ? (c >= 1552 && c <= 1562)
                    : c <= 1641)
                  : (c <= 1747 || (c < 1759
                    ? (c >= 1749 && c <= 1756)
                    : c <= 1768)))
                : (c <= 1788 || (c < 1869
                  ? (c < 1808
                    ? c == 1791
                    : c <= 1866)
                  : (c <= 1969 || (c < 2042
                    ? (c >= 1984 && c <= 2037)
                    : c <= 2042)))))))))
          : (c <= 2045 || (c < 2558
            ? (c < 2451
              ? (c < 2200
                ? (c < 2144
                  ? (c < 2112
                    ? (c >= 2048 && c <= 2093)
                    : c <= 2139)
                  : (c <= 2154 || (c < 2185
                    ? (c >= 2160 && c <= 2183)
                    : c <= 2190)))
                : (c <= 2273 || (c < 2417
                  ? (c < 2406
                    ? (c >= 2275 && c <= 2403)
                    : c <= 2415)
                  : (c <= 2435 || (c < 2447
                    ? (c >= 2437 && c <= 2444)
                    : c <= 2448)))))
              : (c <= 2472 || (c < 2507
                ? (c < 2486
                  ? (c < 2482
                    ? (c >= 2474 && c <= 2480)
                    : c <= 2482)
                  : (c <= 2489 || (c < 2503
                    ? (c >= 2492 && c <= 2500)
                    : c <= 2504)))
                : (c <= 2510 || (c < 2527
                  ? (c < 2524
                    ? c == 2519
                    : c <= 2525)
                  : (c <= 2531 || (c < 2556
                    ? (c >= 2534 && c <= 2545)
                    : c <= 2556)))))))
            : (c <= 2558 || (c < 2635
              ? (c < 2610
                ? (c < 2575
                  ? (c < 2565
                    ? (c >= 2561 && c <= 2563)
                    : c <= 2570)
                  : (c <= 2576 || (c < 2602
                    ? (c >= 2579 && c <= 2600)
                    : c <= 2608)))
                : (c <= 2611 || (c < 2620
                  ? (c < 2616
                    ? (c >= 2613 && c <= 2614)
                    : c <= 2617)
                  : (c <= 2620 || (c < 2631
                    ? (c >= 2622 && c <= 2626)
                    : c <= 2632)))))
              : (c <= 2637 || (c < 2693
                ? (c < 2654
                  ? (c < 2649
                    ? c == 2641
                    : c <= 2652)
                  : (c <= 2654 || (c < 2689
                    ? (c >= 2662 && c <= 2677)
                    : c <= 2691)))
                : (c <= 2701 || (c < 2730
                  ? (c < 2707
                    ? (c >= 2703 && c <= 2705)
                    : c <= 2728)
                  : (c <= 2736 || (c < 2741
                    ? (c >= 2738 && c <= 2739)
                    : c <= 2745)))))))))))
        : (c <= 2757 || (c < 3168
          ? (c < 2958
            ? (c < 2866
              ? (c < 2809
                ? (c < 2768
                  ? (c < 2763
                    ? (c >= 2759 && c <= 2761)
                    : c <= 2765)
                  : (c <= 2768 || (c < 2790
                    ? (c >= 2784 && c <= 2787)
                    : c <= 2799)))
                : (c <= 2815 || (c < 2831
                  ? (c < 2821
                    ? (c >= 2817 && c <= 2819)
                    : c <= 2828)
                  : (c <= 2832 || (c < 2858
                    ? (c >= 2835 && c <= 2856)
                    : c <= 2864)))))
              : (c <= 2867 || (c < 2908
                ? (c < 2887
                  ? (c < 2876
                    ? (c >= 2869 && c <= 2873)
                    : c <= 2884)
                  : (c <= 2888 || (c < 2901
                    ? (c >= 2891 && c <= 2893)
                    : c <= 2903)))
                : (c <= 2909 || (c < 2929
                  ? (c < 2918
                    ? (c >= 2911 && c <= 2915)
                    : c <= 2927)
                  : (c <= 2929 || (c < 2949
                    ? (c >= 2946 && c <= 2947)
                    : c <= 2954)))))))
            : (c <= 2960 || (c < 3031
              ? (c < 2984
                ? (c < 2972
                  ? (c < 2969
                    ? (c >= 2962 && c <= 2965)
                    : c <= 2970)
                  : (c <= 2972 || (c < 2979
                    ? (c >= 2974 && c <= 2975)
                    : c <= 2980)))
                : (c <= 2986 || (c < 3014
                  ? (c < 3006
                    ? (c >= 2990 && c <= 3001)
                    : c <= 3010)
                  : (c <= 3016 || (c < 3024
                    ? (c >= 3018 && c <= 3021)
                    : c <= 3024)))))
              : (c <= 3031 || (c < 3132
                ? (c < 3086
                  ? (c < 3072
                    ? (c >= 3046 && c <= 3055)
                    : c <= 3084)
                  : (c <= 3088 || (c < 3114
                    ? (c >= 3090 && c <= 3112)
                    : c <= 3129)))
                : (c <= 3140 || (c < 3157
                  ? (c < 3146
                    ? (c >= 3142 && c <= 3144)
                    : c <= 3149)
                  : (c <= 3158 || (c < 3165
                    ? (c >= 3160 && c <= 3162)
                    : c <= 3165)))))))))
          : (c <= 3171 || (c < 3450
            ? (c < 3293
              ? (c < 3242
                ? (c < 3205
                  ? (c < 3200
                    ? (c >= 3174 && c <= 3183)
                    : c <= 3203)
                  : (c <= 3212 || (c < 3218
                    ? (c >= 3214 && c <= 3216)
                    : c <= 3240)))
                : (c <= 3251 || (c < 3270
                  ? (c < 3260
                    ? (c >= 3253 && c <= 3257)
                    : c <= 3268)
                  : (c <= 3272 || (c < 3285
                    ? (c >= 3274 && c <= 3277)
                    : c <= 3286)))))
              : (c <= 3294 || (c < 3346
                ? (c < 3313
                  ? (c < 3302
                    ? (c >= 3296 && c <= 3299)
                    : c <= 3311)
                  : (c <= 3314 || (c < 3342
                    ? (c >= 3328 && c <= 3340)
                    : c <= 3344)))
                : (c <= 3396 || (c < 3412
                  ? (c < 3402
                    ? (c >= 3398 && c <= 3400)
                    : c <= 3406)
                  : (c <= 3415 || (c < 3430
                    ? (c >= 3423 && c <= 3427)
                    : c <= 3439)))))))
            : (c <= 3455 || (c < 3570
              ? (c < 3520
                ? (c < 3482
                  ? (c < 3461
                    ? (c >= 3457 && c <= 3459)
                    : c <= 3478)
                  : (c <= 3505 || (c < 3517
                    ? (c >= 3507 && c <= 3515)
                    : c <= 3517)))
                : (c <= 3526 || (c < 3542
                  ? (c < 3535
                    ? c == 3530
                    : c <= 3540)
                  : (c <= 3542 || (c < 3558
                    ? (c >= 3544 && c <= 3551)
                    : c <= 3567)))))
              : (c <= 3571 || (c < 3718
                ? (c < 3664
                  ? (c < 3648
                    ? (c >= 3585 && c <= 3642)
                    : c <= 3662)
                  : (c <= 3673 || (c < 3716
                    ? (c >= 3713 && c <= 3714)
                    : c <= 3716)))
                : (c <= 3722 || (c < 3751
                  ? (c < 3749
                    ? (c >= 3724 && c <= 3747)
                    : c <= 3749)
                  : (c <= 3773 || (c >= 3776 && c <= 3780)))))))))))))
      : (c <= 3782 || (c < 8025
        ? (c < 5888
          ? (c < 4688
            ? (c < 3953
              ? (c < 3872
                ? (c < 3804
                  ? (c < 3792
                    ? (c >= 3784 && c <= 3789)
                    : c <= 3801)
                  : (c <= 3807 || (c < 3864
                    ? c == 3840
                    : c <= 3865)))
                : (c <= 3881 || (c < 3897
                  ? (c < 3895
                    ? c == 3893
                    : c <= 3895)
                  : (c <= 3897 || (c < 3913
                    ? (c >= 3902 && c <= 3911)
                    : c <= 3948)))))
              : (c <= 3972 || (c < 4256
                ? (c < 4038
                  ? (c < 3993
                    ? (c >= 3974 && c <= 3991)
                    : c <= 4028)
                  : (c <= 4038 || (c < 4176
                    ? (c >= 4096 && c <= 4169)
                    : c <= 4253)))
                : (c <= 4293 || (c < 4304
                  ? (c < 4301
                    ? c == 4295
                    : c <= 4301)
                  : (c <= 4346 || (c < 4682
                    ? (c >= 4348 && c <= 4680)
                    : c <= 4685)))))))
            : (c <= 4694 || (c < 4882
              ? (c < 4786
                ? (c < 4704
                  ? (c < 4698
                    ? c == 4696
                    : c <= 4701)
                  : (c <= 4744 || (c < 4752
                    ? (c >= 4746 && c <= 4749)
                    : c <= 4784)))
                : (c <= 4789 || (c < 4802
                  ? (c < 4800
                    ? (c >= 4792 && c <= 4798)
                    : c <= 4800)
                  : (c <= 4805 || (c < 4824
                    ? (c >= 4808 && c <= 4822)
                    : c <= 4880)))))
              : (c <= 4885 || (c < 5112
                ? (c < 4969
                  ? (c < 4957
                    ? (c >= 4888 && c <= 4954)
                    : c <= 4959)
                  : (c <= 4977 || (c < 5024
                    ? (c >= 4992 && c <= 5007)
                    : c <= 5109)))
                : (c <= 5117 || (c < 5761
                  ? (c < 5743
                    ? (c >= 5121 && c <= 5740)
                    : c <= 5759)
                  : (c <= 5786 || (c < 5870
                    ? (c >= 5792 && c <= 5866)
                    : c <= 5880)))))))))
          : (c <= 5909 || (c < 6688
            ? (c < 6176
              ? (c < 6016
                ? (c < 5984
                  ? (c < 5952
                    ? (c >= 5919 && c <= 5940)
                    : c <= 5971)
                  : (c <= 5996 || (c < 6002
                    ? (c >= 5998 && c <= 6000)
                    : c <= 6003)))
                : (c <= 6099 || (c < 6112
                  ? (c < 6108
                    ? c == 6103
                    : c <= 6109)
                  : (c <= 6121 || (c < 6159
                    ? (c >= 6155 && c <= 6157)
                    : c <= 6169)))))
              : (c <= 6264 || (c < 6470
                ? (c < 6400
                  ? (c < 6320
                    ? (c >= 6272 && c <= 6314)
                    : c <= 6389)
                  : (c <= 6430 || (c < 6448
                    ? (c >= 6432 && c <= 6443)
                    : c <= 6459)))
                : (c <= 6509 || (c < 6576
                  ? (c < 6528
                    ? (c >= 6512 && c <= 6516)
                    : c <= 6571)
                  : (c <= 6601 || (c < 6656
                    ? (c >= 6608 && c <= 6618)
                    : c <= 6683)))))))
            : (c <= 6750 || (c < 7232
              ? (c < 6847
                ? (c < 6800
                  ? (c < 6783
                    ? (c >= 6752 && c <= 6780)
                    : c <= 6793)
                  : (c <= 6809 || (c < 6832
                    ? c == 6823
                    : c <= 6845)))
                : (c <= 6862 || (c < 7019
                  ? (c < 6992
                    ? (c >= 6912 && c <= 6988)
                    : c <= 7001)
                  : (c <= 7027 || (c < 7168
                    ? (c >= 7040 && c <= 7155)
                    : c <= 7223)))))
              : (c <= 7241 || (c < 7380
                ? (c < 7312
                  ? (c < 7296
                    ? (c >= 7245 && c <= 7293)
                    : c <= 7304)
                  : (c <= 7354 || (c < 7376
                    ? (c >= 7357 && c <= 7359)
                    : c <= 7378)))
                : (c <= 7418 || (c < 7968
                  ? (c < 7960
                    ? (c >= 7424 && c <= 7957)
                    : c <= 7965)
                  : (c <= 8005 || (c < 8016
                    ? (c >= 8008 && c <= 8013)
                    : c <= 8023)))))))))))
        : (c <= 8025 || (c < 11720
          ? (c < 8458
            ? (c < 8178
              ? (c < 8126
                ? (c < 8031
                  ? (c < 8029
                    ? c == 8027
                    : c <= 8029)
                  : (c <= 8061 || (c < 8118
                    ? (c >= 8064 && c <= 8116)
                    : c <= 8124)))
                : (c <= 8126 || (c < 8144
                  ? (c < 8134
                    ? (c >= 8130 && c <= 8132)
                    : c <= 8140)
                  : (c <= 8147 || (c < 8160
                    ? (c >= 8150 && c <= 8155)
                    : c <= 8172)))))
              : (c <= 8180 || (c < 8336
                ? (c < 8276
                  ? (c < 8255
                    ? (c >= 8182 && c <= 8188)
                    : c <= 8256)
                  : (c <= 8276 || (c < 8319
                    ? c == 8305
                    : c <= 8319)))
                : (c <= 8348 || (c < 8421
                  ? (c < 8417
                    ? (c >= 8400 && c <= 8412)
                    : c <= 8417)
                  : (c <= 8432 || (c < 8455
                    ? c == 8450
                    : c <= 8455)))))))
            : (c <= 8467 || (c < 11499
              ? (c < 8490
                ? (c < 8484
                  ? (c < 8472
                    ? c == 8469
                    : c <= 8477)
                  : (c <= 8484 || (c < 8488
                    ? c == 8486
                    : c <= 8488)))
                : (c <= 8505 || (c < 8526
                  ? (c < 8517
                    ? (c >= 8508 && c <= 8511)
                    : c <= 8521)
                  : (c <= 8526 || (c < 11264
                    ? (c >= 8544 && c <= 8584)
                    : c <= 11492)))))
              : (c <= 11507 || (c < 11647
                ? (c < 11565
                  ? (c < 11559
                    ? (c >= 11520 && c <= 11557)
                    : c <= 11559)
                  : (c <= 11565 || (c < 11631
                    ? (c >= 11568 && c <= 11623)
                    : c <= 11631)))
                : (c <= 11670 || (c < 11696
                  ? (c < 11688
                    ? (c >= 11680 && c <= 11686)
                    : c <= 11694)
                  : (c <= 11702 || (c < 11712
                    ? (c >= 11704 && c <= 11710)
                    : c <= 11718)))))))))
          : (c <= 11726 || (c < 42623
            ? (c < 12540
              ? (c < 12337
                ? (c < 11744
                  ? (c < 11736
                    ? (c >= 11728 && c <= 11734)
                    : c <= 11742)
                  : (c <= 11775 || (c < 12321
                    ? (c >= 12293 && c <= 12295)
                    : c <= 12335)))
                : (c <= 12341 || (c < 12441
                  ? (c < 12353
                    ? (c >= 12344 && c <= 12348)
                    : c <= 12438)
                  : (c <= 12442 || (c < 12449
                    ? (c >= 12445 && c <= 12447)
                    : c <= 12538)))))
              : (c <= 12543 || (c < 19968
                ? (c < 12704
                  ? (c < 12593
                    ? (c >= 12549 && c <= 12591)
                    : c <= 12686)
                  : (c <= 12735 || (c < 13312
                    ? (c >= 12784 && c <= 12799)
                    : c <= 19903)))
                : (c <= 42124 || (c < 42512
                  ? (c < 42240
                    ? (c >= 42192 && c <= 42237)
                    : c <= 42508)
                  : (c <= 42539 || (c < 42612
                    ? (c >= 42560 && c <= 42607)
                    : c <= 42621)))))))
            : (c <= 42737 || (c < 43232
              ? (c < 42965
                ? (c < 42891
                  ? (c < 42786
                    ? (c >= 42775 && c <= 42783)
                    : c <= 42888)
                  : (c <= 42954 || (c < 42963
                    ? (c >= 42960 && c <= 42961)
                    : c <= 42963)))
                : (c <= 42969 || (c < 43072
                  ? (c < 43052
                    ? (c >= 42994 && c <= 43047)
                    : c <= 43052)
                  : (c <= 43123 || (c < 43216
                    ? (c >= 43136 && c <= 43205)
                    : c <= 43225)))))
              : (c <= 43255 || (c < 43471
                ? (c < 43312
                  ? (c < 43261
                    ? c == 43259
                    : c <= 43309)
                  : (c <= 43347 || (c < 43392
                    ? (c >= 43360 && c <= 43388)
                    : c <= 43456)))
                : (c <= 43481 || (c < 43584
                  ? (c < 43520
                    ? (c >= 43488 && c <= 43518)
                    : c <= 43574)
                  : (c <= 43597 || (c >= 43600 && c <= 43609)))))))))))))))
    : (c <= 43638 || (c < 71453
      ? (c < 67639
        ? (c < 65345
          ? (c < 64312
            ? (c < 43888
              ? (c < 43785
                ? (c < 43744
                  ? (c < 43739
                    ? (c >= 43642 && c <= 43714)
                    : c <= 43741)
                  : (c <= 43759 || (c < 43777
                    ? (c >= 43762 && c <= 43766)
                    : c <= 43782)))
                : (c <= 43790 || (c < 43816
                  ? (c < 43808
                    ? (c >= 43793 && c <= 43798)
                    : c <= 43814)
                  : (c <= 43822 || (c < 43868
                    ? (c >= 43824 && c <= 43866)
                    : c <= 43881)))))
              : (c <= 44010 || (c < 63744
                ? (c < 44032
                  ? (c < 44016
                    ? (c >= 44012 && c <= 44013)
                    : c <= 44025)
                  : (c <= 55203 || (c < 55243
                    ? (c >= 55216 && c <= 55238)
                    : c <= 55291)))
                : (c <= 64109 || (c < 64275
                  ? (c < 64256
                    ? (c >= 64112 && c <= 64217)
                    : c <= 64262)
                  : (c <= 64279 || (c < 64298
                    ? (c >= 64285 && c <= 64296)
                    : c <= 64310)))))))
            : (c <= 64316 || (c < 65075
              ? (c < 64612
                ? (c < 64323
                  ? (c < 64320
                    ? c == 64318
                    : c <= 64321)
                  : (c <= 64324 || (c < 64467
                    ? (c >= 64326 && c <= 64433)
                    : c <= 64605)))
                : (c <= 64829 || (c < 65008
                  ? (c < 64914
                    ? (c >= 64848 && c <= 64911)
                    : c <= 64967)
                  : (c <= 65017 || (c < 65056
                    ? (c >= 65024 && c <= 65039)
                    : c <= 65071)))))
              : (c <= 65076 || (c < 65147
                ? (c < 65139
                  ? (c < 65137
                    ? (c >= 65101 && c <= 65103)
                    : c <= 65137)
                  : (c <= 65139 || (c < 65145
                    ? c == 65143
                    : c <= 65145)))
                : (c <= 65147 || (c < 65296
                  ? (c < 65151
                    ? c == 65149
                    : c <= 65276)
                  : (c <= 65305 || (c < 65343
                    ? (c >= 65313 && c <= 65338)
                    : c <= 65343)))))))))
          : (c <= 65370 || (c < 66513
            ? (c < 65664
              ? (c < 65536
                ? (c < 65482
                  ? (c < 65474
                    ? (c >= 65382 && c <= 65470)
                    : c <= 65479)
                  : (c <= 65487 || (c < 65498
                    ? (c >= 65490 && c <= 65495)
                    : c <= 65500)))
                : (c <= 65547 || (c < 65596
                  ? (c < 65576
                    ? (c >= 65549 && c <= 65574)
                    : c <= 65594)
                  : (c <= 65597 || (c < 65616
                    ? (c >= 65599 && c <= 65613)
                    : c <= 65629)))))
              : (c <= 65786 || (c < 66304
                ? (c < 66176
                  ? (c < 66045
                    ? (c >= 65856 && c <= 65908)
                    : c <= 66045)
                  : (c <= 66204 || (c < 66272
                    ? (c >= 66208 && c <= 66256)
                    : c <= 66272)))
                : (c <= 66335 || (c < 66432
                  ? (c < 66384
                    ? (c >= 66349 && c <= 66378)
                    : c <= 66426)
                  : (c <= 66461 || (c < 66504
                    ? (c >= 66464 && c <= 66499)
                    : c <= 66511)))))))
            : (c <= 66517 || (c < 66979
              ? (c < 66864
                ? (c < 66736
                  ? (c < 66720
                    ? (c >= 66560 && c <= 66717)
                    : c <= 66729)
                  : (c <= 66771 || (c < 66816
                    ? (c >= 66776 && c <= 66811)
                    : c <= 66855)))
                : (c <= 66915 || (c < 66956
                  ? (c < 66940
                    ? (c >= 66928 && c <= 66938)
                    : c <= 66954)
                  : (c <= 66962 || (c < 66967
                    ? (c >= 66964 && c <= 66965)
                    : c <= 66977)))))
              : (c <= 66993 || (c < 67456
                ? (c < 67072
                  ? (c < 67003
                    ? (c >= 66995 && c <= 67001)
                    : c <= 67004)
                  : (c <= 67382 || (c < 67424
                    ? (c >= 67392 && c <= 67413)
                    : c <= 67431)))
                : (c <= 67461 || (c < 67584
                  ? (c < 67506
                    ? (c >= 67463 && c <= 67504)
                    : c <= 67514)
                  : (c <= 67589 || (c < 67594
                    ? c == 67592
                    : c <= 67637)))))))))))
        : (c <= 67640 || (c < 69956
          ? (c < 68448
            ? (c < 68101
              ? (c < 67828
                ? (c < 67680
                  ? (c < 67647
                    ? c == 67644
                    : c <= 67669)
                  : (c <= 67702 || (c < 67808
                    ? (c >= 67712 && c <= 67742)
                    : c <= 67826)))
                : (c <= 67829 || (c < 67968
                  ? (c < 67872
                    ? (c >= 67840 && c <= 67861)
                    : c <= 67897)
                  : (c <= 68023 || (c < 68096
                    ? (c >= 68030 && c <= 68031)
                    : c <= 68099)))))
              : (c <= 68102 || (c < 68192
                ? (c < 68121
                  ? (c < 68117
                    ? (c >= 68108 && c <= 68115)
                    : c <= 68119)
                  : (c <= 68149 || (c < 68159
                    ? (c >= 68152 && c <= 68154)
                    : c <= 68159)))
                : (c <= 68220 || (c < 68297
                  ? (c < 68288
                    ? (c >= 68224 && c <= 68252)
                    : c <= 68295)
                  : (c <= 68326 || (c < 68416
                    ? (c >= 68352 && c <= 68405)
                    : c <= 68437)))))))
            : (c <= 68466 || (c < 69424
              ? (c < 68912
                ? (c < 68736
                  ? (c < 68608
                    ? (c >= 68480 && c <= 68497)
                    : c <= 68680)
                  : (c <= 68786 || (c < 68864
                    ? (c >= 68800 && c <= 68850)
                    : c <= 68903)))
                : (c <= 68921 || (c < 69296
                  ? (c < 69291
                    ? (c >= 69248 && c <= 69289)
                    : c <= 69292)
                  : (c <= 69297 || (c < 69415
                    ? (c >= 69376 && c <= 69404)
                    : c <= 69415)))))
              : (c <= 69456 || (c < 69759
                ? (c < 69600
                  ? (c < 69552
                    ? (c >= 69488 && c <= 69509)
                    : c <= 69572)
                  : (c <= 69622 || (c < 69734
                    ? (c >= 69632 && c <= 69702)
                    : c <= 69749)))
                : (c <= 69818 || (c < 69872
                  ? (c < 69840
                    ? c == 69826
                    : c <= 69864)
                  : (c <= 69881 || (c < 69942
                    ? (c >= 69888 && c <= 69940)
                    : c <= 69951)))))))))
          : (c <= 69959 || (c < 70459
            ? (c < 70282
              ? (c < 70108
                ? (c < 70016
                  ? (c < 70006
                    ? (c >= 69968 && c <= 70003)
                    : c <= 70006)
                  : (c <= 70084 || (c < 70094
                    ? (c >= 70089 && c <= 70092)
                    : c <= 70106)))
                : (c <= 70108 || (c < 70206
                  ? (c < 70163
                    ? (c >= 70144 && c <= 70161)
                    : c <= 70199)
                  : (c <= 70206 || (c < 70280
                    ? (c >= 70272 && c <= 70278)
                    : c <= 70280)))))
              : (c <= 70285 || (c < 70405
                ? (c < 70320
                  ? (c < 70303
                    ? (c >= 70287 && c <= 70301)
                    : c <= 70312)
                  : (c <= 70378 || (c < 70400
                    ? (c >= 70384 && c <= 70393)
                    : c <= 70403)))
                : (c <= 70412 || (c < 70442
                  ? (c < 70419
                    ? (c >= 70415 && c <= 70416)
                    : c <= 70440)
                  : (c <= 70448 || (c < 70453
                    ? (c >= 70450 && c <= 70451)
                    : c <= 70457)))))))
            : (c <= 70468 || (c < 70855
              ? (c < 70502
                ? (c < 70480
                  ? (c < 70475
                    ? (c >= 70471 && c <= 70472)
                    : c <= 70477)
                  : (c <= 70480 || (c < 70493
                    ? c == 70487
                    : c <= 70499)))
                : (c <= 70508 || (c < 70736
                  ? (c < 70656
                    ? (c >= 70512 && c <= 70516)
                    : c <= 70730)
                  : (c <= 70745 || (c < 70784
                    ? (c >= 70750 && c <= 70753)
                    : c <= 70853)))))
              : (c <= 70855 || (c < 71236
                ? (c < 71096
                  ? (c < 71040
                    ? (c >= 70864 && c <= 70873)
                    : c <= 71093)
                  : (c <= 71104 || (c < 71168
                    ? (c >= 71128 && c <= 71133)
                    : c <= 71232)))
                : (c <= 71236 || (c < 71360
                  ? (c < 71296
                    ? (c >= 71248 && c <= 71257)
                    : c <= 71352)
                  : (c <= 71369 || (c >= 71424 && c <= 71450)))))))))))))
      : (c <= 71467 || (c < 119973
        ? (c < 77824
          ? (c < 72760
            ? (c < 72016
              ? (c < 71945
                ? (c < 71680
                  ? (c < 71488
                    ? (c >= 71472 && c <= 71481)
                    : c <= 71494)
                  : (c <= 71738 || (c < 71935
                    ? (c >= 71840 && c <= 71913)
                    : c <= 71942)))
                : (c <= 71945 || (c < 71960
                  ? (c < 71957
                    ? (c >= 71948 && c <= 71955)
                    : c <= 71958)
                  : (c <= 71989 || (c < 71995
                    ? (c >= 71991 && c <= 71992)
                    : c <= 72003)))))
              : (c <= 72025 || (c < 72263
                ? (c < 72154
                  ? (c < 72106
                    ? (c >= 72096 && c <= 72103)
                    : c <= 72151)
                  : (c <= 72161 || (c < 72192
                    ? (c >= 72163 && c <= 72164)
                    : c <= 72254)))
                : (c <= 72263 || (c < 72368
                  ? (c < 72349
                    ? (c >= 72272 && c <= 72345)
                    : c <= 72349)
                  : (c <= 72440 || (c < 72714
                    ? (c >= 72704 && c <= 72712)
                    : c <= 72758)))))))
            : (c <= 72768 || (c < 73056
              ? (c < 72968
                ? (c < 72850
                  ? (c < 72818
                    ? (c >= 72784 && c <= 72793)
                    : c <= 72847)
                  : (c <= 72871 || (c < 72960
                    ? (c >= 72873 && c <= 72886)
                    : c <= 72966)))
                : (c <= 72969 || (c < 73020
                  ? (c < 73018
                    ? (c >= 72971 && c <= 73014)
                    : c <= 73018)
                  : (c <= 73021 || (c < 73040
                    ? (c >= 73023 && c <= 73031)
                    : c <= 73049)))))
              : (c <= 73061 || (c < 73440
                ? (c < 73104
                  ? (c < 73066
                    ? (c >= 73063 && c <= 73064)
                    : c <= 73102)
                  : (c <= 73105 || (c < 73120
                    ? (c >= 73107 && c <= 73112)
                    : c <= 73129)))
                : (c <= 73462 || (c < 74752
                  ? (c < 73728
                    ? c == 73648
                    : c <= 74649)
                  : (c <= 74862 || (c < 77712
                    ? (c >= 74880 && c <= 75075)
                    : c <= 77808)))))))))
          : (c <= 78894 || (c < 110576
            ? (c < 93027
              ? (c < 92864
                ? (c < 92736
                  ? (c < 92160
                    ? (c >= 82944 && c <= 83526)
                    : c <= 92728)
                  : (c <= 92766 || (c < 92784
                    ? (c >= 92768 && c <= 92777)
                    : c <= 92862)))
                : (c <= 92873 || (c < 92928
                  ? (c < 92912
                    ? (c >= 92880 && c <= 92909)
                    : c <= 92916)
                  : (c <= 92982 || (c < 93008
                    ? (c >= 92992 && c <= 92995)
                    : c <= 93017)))))
              : (c <= 93047 || (c < 94176
                ? (c < 93952
                  ? (c < 93760
                    ? (c >= 93053 && c <= 93071)
                    : c <= 93823)
                  : (c <= 94026 || (c < 94095
                    ? (c >= 94031 && c <= 94087)
                    : c <= 94111)))
                : (c <= 94177 || (c < 94208
                  ? (c < 94192
                    ? (c >= 94179 && c <= 94180)
                    : c <= 94193)
                  : (c <= 100343 || (c < 101632
                    ? (c >= 100352 && c <= 101589)
                    : c <= 101640)))))))
            : (c <= 110579 || (c < 118528
              ? (c < 110960
                ? (c < 110592
                  ? (c < 110589
                    ? (c >= 110581 && c <= 110587)
                    : c <= 110590)
                  : (c <= 110882 || (c < 110948
                    ? (c >= 110928 && c <= 110930)
                    : c <= 110951)))
                : (c <= 111355 || (c < 113792
                  ? (c < 113776
                    ? (c >= 113664 && c <= 113770)
                    : c <= 113788)
                  : (c <= 113800 || (c < 113821
                    ? (c >= 113808 && c <= 113817)
                    : c <= 113822)))))
              : (c <= 118573 || (c < 119210
                ? (c < 119149
                  ? (c < 119141
                    ? (c >= 118576 && c <= 118598)
                    : c <= 119145)
                  : (c <= 119154 || (c < 119173
                    ? (c >= 119163 && c <= 119170)
                    : c <= 119179)))
                : (c <= 119213 || (c < 119894
                  ? (c < 119808
                    ? (c >= 119362 && c <= 119364)
                    : c <= 119892)
                  : (c <= 119964 || (c < 119970
                    ? (c >= 119966 && c <= 119967)
                    : c <= 119970)))))))))))
        : (c <= 119974 || (c < 124912
          ? (c < 120746
            ? (c < 120134
              ? (c < 120071
                ? (c < 119995
                  ? (c < 119982
                    ? (c >= 119977 && c <= 119980)
                    : c <= 119993)
                  : (c <= 119995 || (c < 120005
                    ? (c >= 119997 && c <= 120003)
                    : c <= 120069)))
                : (c <= 120074 || (c < 120094
                  ? (c < 120086
                    ? (c >= 120077 && c <= 120084)
                    : c <= 120092)
                  : (c <= 120121 || (c < 120128
                    ? (c >= 120123 && c <= 120126)
                    : c <= 120132)))))
              : (c <= 120134 || (c < 120572
                ? (c < 120488
                  ? (c < 120146
                    ? (c >= 120138 && c <= 120144)
                    : c <= 120485)
                  : (c <= 120512 || (c < 120540
                    ? (c >= 120514 && c <= 120538)
                    : c <= 120570)))
                : (c <= 120596 || (c < 120656
                  ? (c < 120630
                    ? (c >= 120598 && c <= 120628)
                    : c <= 120654)
                  : (c <= 120686 || (c < 120714
                    ? (c >= 120688 && c <= 120712)
                    : c <= 120744)))))))
            : (c <= 120770 || (c < 122907
              ? (c < 121476
                ? (c < 121344
                  ? (c < 120782
                    ? (c >= 120772 && c <= 120779)
                    : c <= 120831)
                  : (c <= 121398 || (c < 121461
                    ? (c >= 121403 && c <= 121452)
                    : c <= 121461)))
                : (c <= 121476 || (c < 122624
                  ? (c < 121505
                    ? (c >= 121499 && c <= 121503)
                    : c <= 121519)
                  : (c <= 122654 || (c < 122888
                    ? (c >= 122880 && c <= 122886)
                    : c <= 122904)))))
              : (c <= 122913 || (c < 123214
                ? (c < 123136
                  ? (c < 122918
                    ? (c >= 122915 && c <= 122916)
                    : c <= 122922)
                  : (c <= 123180 || (c < 123200
                    ? (c >= 123184 && c <= 123197)
                    : c <= 123209)))
                : (c <= 123214 || (c < 124896
                  ? (c < 123584
                    ? (c >= 123536 && c <= 123566)
                    : c <= 123641)
                  : (c <= 124902 || (c < 124909
                    ? (c >= 124904 && c <= 124907)
                    : c <= 124910)))))))))
          : (c <= 124926 || (c < 126557
            ? (c < 126521
              ? (c < 126469
                ? (c < 125184
                  ? (c < 125136
                    ? (c >= 124928 && c <= 125124)
                    : c <= 125142)
                  : (c <= 125259 || (c < 126464
                    ? (c >= 125264 && c <= 125273)
                    : c <= 126467)))
                : (c <= 126495 || (c < 126503
                  ? (c < 126500
                    ? (c >= 126497 && c <= 126498)
                    : c <= 126500)
                  : (c <= 126503 || (c < 126516
                    ? (c >= 126505 && c <= 126514)
                    : c <= 126519)))))
              : (c <= 126521 || (c < 126541
                ? (c < 126535
                  ? (c < 126530
                    ? c == 126523
                    : c <= 126530)
                  : (c <= 126535 || (c < 126539
                    ? c == 126537
                    : c <= 126539)))
                : (c <= 126543 || (c < 126551
                  ? (c < 126548
                    ? (c >= 126545 && c <= 126546)
                    : c <= 126548)
                  : (c <= 126551 || (c < 126555
                    ? c == 126553
                    : c <= 126555)))))))
            : (c <= 126557 || (c < 126629
              ? (c < 126580
                ? (c < 126564
                  ? (c < 126561
                    ? c == 126559
                    : c <= 126562)
                  : (c <= 126564 || (c < 126572
                    ? (c >= 126567 && c <= 126570)
                    : c <= 126578)))
                : (c <= 126583 || (c < 126592
                  ? (c < 126590
                    ? (c >= 126585 && c <= 126588)
                    : c <= 126590)
                  : (c <= 126601 || (c < 126625
                    ? (c >= 126603 && c <= 126619)
                    : c <= 126627)))))
              : (c <= 126633 || (c < 178208
                ? (c < 131072
                  ? (c < 130032
                    ? (c >= 126635 && c <= 126651)
                    : c <= 130041)
                  : (c <= 173791 || (c < 177984
                    ? (c >= 173824 && c <= 177976)
                    : c <= 178205)))
                : (c <= 183969 || (c < 196608
                  ? (c < 194560
                    ? (c >= 183984 && c <= 191456)
                    : c <= 195101)
                  : (c <= 201546 || (c >= 917760 && c <= 917999)))))))))))))))));
}

static inline bool sym_number_literal_character_set_1(int32_t c) {
  return (c < 'b'
    ? (c < 'L'
      ? (c < 'D'
        ? c == 'B'
        : c <= 'F')
      : (c <= 'L' || (c < 'W'
        ? c == 'U'
        : c <= 'W')))
    : (c <= 'b' || (c < 'u'
      ? (c < 'l'
        ? (c >= 'd' && c <= 'f')
        : c <= 'l')
      : (c <= 'u' || c == 'w'))));
}

static inline bool sym_number_literal_character_set_2(int32_t c) {
  return (c < 'b'
    ? (c < 'L'
      ? (c < 'D'
        ? c == 'B'
        : (c <= 'D' || c == 'F'))
      : (c <= 'L' || (c < 'W'
        ? c == 'U'
        : c <= 'W')))
    : (c <= 'b' || (c < 'l'
      ? (c < 'f'
        ? c == 'd'
        : c <= 'f')
      : (c <= 'l' || (c < 'w'
        ? c == 'u'
        : c <= 'w')))));
}

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(42);
      if (lookahead == '"') ADVANCE(54);
      if (lookahead == '%') ADVANCE(87);
      if (lookahead == '(') ADVANCE(85);
      if (lookahead == ')') ADVANCE(86);
      if (lookahead == ',') ADVANCE(84);
      if (lookahead == '.') ADVANCE(45);
      if (lookahead == '/') ADVANCE(10);
      if (lookahead == '0') ADVANCE(57);
      if (lookahead == ':') ADVANCE(80);
      if (lookahead == '<') ADVANCE(90);
      if (lookahead == '>') ADVANCE(91);
      if (lookahead == '[') ADVANCE(74);
      if (lookahead == '\\') ADVANCE(4);
      if (lookahead == ']') ADVANCE(75);
      if (lookahead == '^') ADVANCE(89);
      if (lookahead == '_') ADVANCE(48);
      if (lookahead == 'b') ADVANCE(46);
      if (lookahead == 'r') ADVANCE(47);
      if (lookahead == '{') ADVANCE(78);
      if (lookahead == '|') ADVANCE(81);
      if (lookahead == '}') ADVANCE(79);
      if (lookahead == '~') ADVANCE(83);
      if (('+' <= lookahead && lookahead <= '-')) ADVANCE(88);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(43);
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(59);
      if (sym_identifier_character_set_1(lookahead)) ADVANCE(48);
      END_STATE();
    case 1:
      if (lookahead == '\n') ADVANCE(43);
      END_STATE();
    case 2:
      if (lookahead == '\n') ADVANCE(43);
      if (lookahead == '\r') ADVANCE(1);
      END_STATE();
    case 3:
      if (lookahead == '\n') ADVANCE(43);
      if (lookahead == '\r') ADVANCE(1);
      if (lookahead == 'U') ADVANCE(39);
      if (lookahead == 'u') ADVANCE(31);
      END_STATE();
    case 4:
      if (lookahead == '\n') ADVANCE(49);
      if (lookahead == '\r') ADVANCE(49);
      if (lookahead == 'U') ADVANCE(38);
      if (lookahead == 'u') ADVANCE(30);
      if (lookahead == 'x') ADVANCE(26);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(51);
      if (lookahead != 0) ADVANCE(49);
      END_STATE();
    case 5:
      if (lookahead == '"') ADVANCE(55);
      END_STATE();
    case 6:
      if (lookahead == '"') ADVANCE(8);
      if (lookahead == '(') ADVANCE(85);
      if (lookahead == ',') ADVANCE(84);
      if (lookahead == '/') ADVANCE(10);
      if (lookahead == ':') ADVANCE(80);
      if (lookahead == '[') ADVANCE(74);
      if (lookahead == '\\') ADVANCE(2);
      if (lookahead == '{') ADVANCE(78);
      if (lookahead == '|') ADVANCE(82);
      if (lookahead == '}') ADVANCE(79);
      if (lookahead == '~') ADVANCE(83);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(43);
      if (lookahead == '.' ||
          lookahead == '<' ||
          lookahead == '>' ||
          lookahead == '^' ||
          lookahead == '_') ADVANCE(76);
      END_STATE();
    case 7:
      if (lookahead == '"') ADVANCE(53);
      if (lookahead == '/') ADVANCE(10);
      if (lookahead == '\\') ADVANCE(4);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(43);
      END_STATE();
    case 8:
      if (lookahead == '#') ADVANCE(56);
      END_STATE();
    case 9:
      if (lookahead == ')') ADVANCE(77);
      END_STATE();
    case 10:
      if (lookahead == '*') ADVANCE(12);
      if (lookahead == '/') ADVANCE(93);
      END_STATE();
    case 11:
      if (lookahead == '*') ADVANCE(11);
      if (lookahead == '/') ADVANCE(92);
      if (lookahead != 0) ADVANCE(12);
      END_STATE();
    case 12:
      if (lookahead == '*') ADVANCE(11);
      if (lookahead != 0) ADVANCE(12);
      END_STATE();
    case 13:
      if (lookahead == ',') ADVANCE(84);
      if (lookahead == '/') ADVANCE(10);
      if (lookahead == ':') ADVANCE(80);
      if (lookahead == '[') ADVANCE(74);
      if (lookahead == '\\') ADVANCE(2);
      if (lookahead == '{') ADVANCE(78);
      if (lookahead == '}') ADVANCE(79);
      if (lookahead == '%' ||
          ('+' <= lookahead && lookahead <= '-') ||
          lookahead == '~') ADVANCE(87);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(43);
      if (lookahead == '.' ||
          lookahead == '<' ||
          lookahead == '>' ||
          lookahead == '^' ||
          lookahead == '_' ||
          lookahead == '|') ADVANCE(76);
      END_STATE();
    case 14:
      if (lookahead == '.') ADVANCE(44);
      if (lookahead == '/') ADVANCE(10);
      if (lookahead == ':') ADVANCE(80);
      if (lookahead == '[') ADVANCE(74);
      if (lookahead == '\\') ADVANCE(2);
      if (lookahead == '}') ADVANCE(79);
      if (lookahead == '<' ||
          lookahead == '>' ||
          lookahead == '^' ||
          lookahead == '_') ADVANCE(89);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(43);
      END_STATE();
    case 15:
      if (lookahead == '.') ADVANCE(19);
      if (lookahead == '0') ADVANCE(57);
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(59);
      END_STATE();
    case 16:
      if (lookahead == '.') ADVANCE(19);
      if (lookahead == '0') ADVANCE(60);
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(61);
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(68);
      END_STATE();
    case 17:
      if (lookahead == 'U') ADVANCE(39);
      if (lookahead == 'u') ADVANCE(31);
      END_STATE();
    case 18:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(59);
      END_STATE();
    case 19:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(62);
      END_STATE();
    case 20:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(61);
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(68);
      END_STATE();
    case 21:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(48);
      END_STATE();
    case 22:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(49);
      END_STATE();
    case 23:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(65);
      END_STATE();
    case 24:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(68);
      END_STATE();
    case 25:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(63);
      END_STATE();
    case 26:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(22);
      END_STATE();
    case 27:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(21);
      END_STATE();
    case 28:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(26);
      END_STATE();
    case 29:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(27);
      END_STATE();
    case 30:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(28);
      END_STATE();
    case 31:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(29);
      END_STATE();
    case 32:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(30);
      END_STATE();
    case 33:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(31);
      END_STATE();
    case 34:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(32);
      END_STATE();
    case 35:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(33);
      END_STATE();
    case 36:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(34);
      END_STATE();
    case 37:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(35);
      END_STATE();
    case 38:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(36);
      END_STATE();
    case 39:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(37);
      END_STATE();
    case 40:
      if (lookahead != 0 &&
          lookahead != '\r' &&
          lookahead != '\\') ADVANCE(93);
      if (lookahead == '\r') ADVANCE(95);
      if (lookahead == '\\') ADVANCE(94);
      END_STATE();
    case 41:
      if (eof) ADVANCE(42);
      if (lookahead == '"') ADVANCE(52);
      if (lookahead == '(') ADVANCE(9);
      if (lookahead == ')') ADVANCE(86);
      if (lookahead == ',') ADVANCE(84);
      if (lookahead == '.') ADVANCE(19);
      if (lookahead == '/') ADVANCE(10);
      if (lookahead == '0') ADVANCE(57);
      if (lookahead == ':') ADVANCE(80);
      if (lookahead == '<') ADVANCE(90);
      if (lookahead == '>') ADVANCE(91);
      if (lookahead == '[') ADVANCE(74);
      if (lookahead == '\\') ADVANCE(3);
      if (lookahead == ']') ADVANCE(75);
      if (sym_identifier_character_set_2(lookahead)) ADVANCE(48);
      if (lookahead == 'b') ADVANCE(46);
      if (lookahead == 'r') ADVANCE(47);
      if (lookahead == '{') ADVANCE(78);
      if (lookahead == '}') ADVANCE(79);
      if (('+' <= lookahead && lookahead <= '-')) ADVANCE(15);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(43);
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(59);
      END_STATE();
    case 42:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 43:
      ACCEPT_TOKEN(sym_whitespace);
      if (lookahead == '\\') ADVANCE(2);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(43);
      END_STATE();
    case 44:
      ACCEPT_TOKEN(anon_sym_DOT);
      END_STATE();
    case 45:
      ACCEPT_TOKEN(anon_sym_DOT);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(62);
      END_STATE();
    case 46:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == '"') ADVANCE(52);
      if (lookahead == '\\') ADVANCE(17);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(48);
      END_STATE();
    case 47:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == '#') ADVANCE(5);
      if (lookahead == '\\') ADVANCE(17);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(48);
      END_STATE();
    case 48:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == '\\') ADVANCE(17);
      if (sym_identifier_character_set_3(lookahead)) ADVANCE(48);
      END_STATE();
    case 49:
      ACCEPT_TOKEN(sym_escape_sequence);
      END_STATE();
    case 50:
      ACCEPT_TOKEN(sym_escape_sequence);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(49);
      END_STATE();
    case 51:
      ACCEPT_TOKEN(sym_escape_sequence);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(50);
      END_STATE();
    case 52:
      ACCEPT_TOKEN(aux_sym__simple_string_literal_token1);
      END_STATE();
    case 53:
      ACCEPT_TOKEN(anon_sym_DQUOTE);
      END_STATE();
    case 54:
      ACCEPT_TOKEN(anon_sym_DQUOTE);
      if (lookahead == '#') ADVANCE(56);
      END_STATE();
    case 55:
      ACCEPT_TOKEN(anon_sym_r_POUND_DQUOTE);
      END_STATE();
    case 56:
      ACCEPT_TOKEN(anon_sym_DQUOTE_POUND);
      END_STATE();
    case 57:
      ACCEPT_TOKEN(sym_number_literal);
      if (lookahead == '\'') ADVANCE(18);
      if (lookahead == '.') ADVANCE(71);
      if (lookahead == 'B' ||
          lookahead == 'b') ADVANCE(69);
      if (lookahead == 'X' ||
          lookahead == 'x') ADVANCE(16);
      if (lookahead == 'E' ||
          lookahead == 'P' ||
          lookahead == 'e' ||
          lookahead == 'p') ADVANCE(70);
      if (('D' <= lookahead && lookahead <= 'F') ||
          lookahead == 'L' ||
          lookahead == 'U' ||
          lookahead == 'W' ||
          ('d' <= lookahead && lookahead <= 'f') ||
          lookahead == 'l' ||
          lookahead == 'u' ||
          lookahead == 'w') ADVANCE(73);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(59);
      END_STATE();
    case 58:
      ACCEPT_TOKEN(sym_number_literal);
      if (lookahead == '\'') ADVANCE(18);
      if (lookahead == '.') ADVANCE(71);
      if (lookahead == 'B' ||
          lookahead == 'b') ADVANCE(72);
      if (lookahead == 'X' ||
          lookahead == 'x') ADVANCE(24);
      if (lookahead == 'E' ||
          lookahead == 'P' ||
          lookahead == 'e' ||
          lookahead == 'p') ADVANCE(70);
      if (('D' <= lookahead && lookahead <= 'F') ||
          lookahead == 'L' ||
          lookahead == 'U' ||
          lookahead == 'W' ||
          ('d' <= lookahead && lookahead <= 'f') ||
          lookahead == 'l' ||
          lookahead == 'u' ||
          lookahead == 'w') ADVANCE(73);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(59);
      END_STATE();
    case 59:
      ACCEPT_TOKEN(sym_number_literal);
      if (lookahead == '\'') ADVANCE(18);
      if (lookahead == '.') ADVANCE(71);
      if (lookahead == 'E' ||
          lookahead == 'P' ||
          lookahead == 'e' ||
          lookahead == 'p') ADVANCE(70);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(59);
      if (sym_number_literal_character_set_1(lookahead)) ADVANCE(73);
      END_STATE();
    case 60:
      ACCEPT_TOKEN(sym_number_literal);
      if (lookahead == '\'') ADVANCE(20);
      if (lookahead == '.') ADVANCE(71);
      if (lookahead == 'B' ||
          lookahead == 'b') ADVANCE(67);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(66);
      if (lookahead == 'P' ||
          lookahead == 'p') ADVANCE(70);
      if (lookahead == 'X' ||
          lookahead == 'x') ADVANCE(24);
      if (('A' <= lookahead && lookahead <= 'C') ||
          ('a' <= lookahead && lookahead <= 'c')) ADVANCE(68);
      if (('D' <= lookahead && lookahead <= 'F') ||
          ('d' <= lookahead && lookahead <= 'f')) ADVANCE(68);
      if (lookahead == 'L' ||
          lookahead == 'U' ||
          lookahead == 'W' ||
          lookahead == 'l' ||
          lookahead == 'u' ||
          lookahead == 'w') ADVANCE(73);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(61);
      END_STATE();
    case 61:
      ACCEPT_TOKEN(sym_number_literal);
      if (lookahead == '\'') ADVANCE(20);
      if (lookahead == '.') ADVANCE(71);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(66);
      if (lookahead == 'P' ||
          lookahead == 'p') ADVANCE(70);
      if (lookahead == 'A' ||
          lookahead == 'C' ||
          lookahead == 'a' ||
          lookahead == 'c') ADVANCE(68);
      if (('B' <= lookahead && lookahead <= 'F') ||
          ('b' <= lookahead && lookahead <= 'f')) ADVANCE(68);
      if (lookahead == 'L' ||
          lookahead == 'U' ||
          lookahead == 'W' ||
          lookahead == 'l' ||
          lookahead == 'u' ||
          lookahead == 'w') ADVANCE(73);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(61);
      END_STATE();
    case 62:
      ACCEPT_TOKEN(sym_number_literal);
      if (lookahead == '\'') ADVANCE(19);
      if (lookahead == 'E' ||
          lookahead == 'P' ||
          lookahead == 'e' ||
          lookahead == 'p') ADVANCE(70);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(62);
      if (sym_number_literal_character_set_1(lookahead)) ADVANCE(73);
      END_STATE();
    case 63:
      ACCEPT_TOKEN(sym_number_literal);
      if (lookahead == '\'') ADVANCE(25);
      if (lookahead == 'B' ||
          lookahead == 'D' ||
          lookahead == 'F' ||
          lookahead == 'b' ||
          lookahead == 'd' ||
          lookahead == 'f') ADVANCE(63);
      if (lookahead == 'L' ||
          lookahead == 'U' ||
          lookahead == 'W' ||
          lookahead == 'l' ||
          lookahead == 'u' ||
          lookahead == 'w') ADVANCE(73);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'E') ||
          ('a' <= lookahead && lookahead <= 'e')) ADVANCE(63);
      END_STATE();
    case 64:
      ACCEPT_TOKEN(sym_number_literal);
      if (lookahead == '\'') ADVANCE(23);
      if (lookahead == '+' ||
          lookahead == '-') ADVANCE(25);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(64);
      if (lookahead == 'P' ||
          lookahead == 'p') ADVANCE(70);
      if (lookahead == 'B' ||
          ('D' <= lookahead && lookahead <= 'F') ||
          lookahead == 'b' ||
          ('d' <= lookahead && lookahead <= 'f')) ADVANCE(65);
      if (lookahead == 'L' ||
          lookahead == 'U' ||
          lookahead == 'W' ||
          lookahead == 'l' ||
          lookahead == 'u' ||
          lookahead == 'w') ADVANCE(73);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'C') ||
          ('a' <= lookahead && lookahead <= 'c')) ADVANCE(65);
      END_STATE();
    case 65:
      ACCEPT_TOKEN(sym_number_literal);
      if (lookahead == '\'') ADVANCE(23);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(64);
      if (lookahead == 'P' ||
          lookahead == 'p') ADVANCE(70);
      if (lookahead == 'B' ||
          ('D' <= lookahead && lookahead <= 'F') ||
          lookahead == 'b' ||
          ('d' <= lookahead && lookahead <= 'f')) ADVANCE(65);
      if (lookahead == 'L' ||
          lookahead == 'U' ||
          lookahead == 'W' ||
          lookahead == 'l' ||
          lookahead == 'u' ||
          lookahead == 'w') ADVANCE(73);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'C') ||
          ('a' <= lookahead && lookahead <= 'c')) ADVANCE(65);
      END_STATE();
    case 66:
      ACCEPT_TOKEN(sym_number_literal);
      if (lookahead == '\'') ADVANCE(24);
      if (lookahead == '.') ADVANCE(71);
      if (lookahead == '+' ||
          lookahead == '-') ADVANCE(25);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(66);
      if (lookahead == 'P' ||
          lookahead == 'p') ADVANCE(70);
      if (lookahead == 'B' ||
          ('D' <= lookahead && lookahead <= 'F') ||
          lookahead == 'b' ||
          ('d' <= lookahead && lookahead <= 'f')) ADVANCE(68);
      if (lookahead == 'L' ||
          lookahead == 'U' ||
          lookahead == 'W' ||
          lookahead == 'l' ||
          lookahead == 'u' ||
          lookahead == 'w') ADVANCE(73);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'C') ||
          ('a' <= lookahead && lookahead <= 'c')) ADVANCE(68);
      END_STATE();
    case 67:
      ACCEPT_TOKEN(sym_number_literal);
      if (lookahead == '\'') ADVANCE(24);
      if (lookahead == '.') ADVANCE(71);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(66);
      if (lookahead == 'P' ||
          lookahead == 'p') ADVANCE(70);
      if (lookahead == 'A' ||
          lookahead == 'C' ||
          lookahead == 'a' ||
          lookahead == 'c') ADVANCE(68);
      if (('B' <= lookahead && lookahead <= 'F') ||
          ('b' <= lookahead && lookahead <= 'f')) ADVANCE(68);
      if (lookahead == 'L' ||
          lookahead == 'U' ||
          lookahead == 'W' ||
          lookahead == 'l' ||
          lookahead == 'u' ||
          lookahead == 'w') ADVANCE(73);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(61);
      END_STATE();
    case 68:
      ACCEPT_TOKEN(sym_number_literal);
      if (lookahead == '\'') ADVANCE(24);
      if (lookahead == '.') ADVANCE(71);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(66);
      if (lookahead == 'P' ||
          lookahead == 'p') ADVANCE(70);
      if (lookahead == 'B' ||
          ('D' <= lookahead && lookahead <= 'F') ||
          lookahead == 'b' ||
          ('d' <= lookahead && lookahead <= 'f')) ADVANCE(68);
      if (lookahead == 'L' ||
          lookahead == 'U' ||
          lookahead == 'W' ||
          lookahead == 'l' ||
          lookahead == 'u' ||
          lookahead == 'w') ADVANCE(73);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'C') ||
          ('a' <= lookahead && lookahead <= 'c')) ADVANCE(68);
      END_STATE();
    case 69:
      ACCEPT_TOKEN(sym_number_literal);
      if (lookahead == '.') ADVANCE(19);
      if (lookahead == '0') ADVANCE(58);
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(59);
      if (sym_number_literal_character_set_2(lookahead)) ADVANCE(73);
      END_STATE();
    case 70:
      ACCEPT_TOKEN(sym_number_literal);
      if (lookahead == '+' ||
          lookahead == '-') ADVANCE(25);
      if (lookahead == 'B' ||
          lookahead == 'D' ||
          lookahead == 'F' ||
          lookahead == 'b' ||
          lookahead == 'd' ||
          lookahead == 'f') ADVANCE(63);
      if (lookahead == 'L' ||
          lookahead == 'U' ||
          lookahead == 'W' ||
          lookahead == 'l' ||
          lookahead == 'u' ||
          lookahead == 'w') ADVANCE(73);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'E') ||
          ('a' <= lookahead && lookahead <= 'e')) ADVANCE(63);
      END_STATE();
    case 71:
      ACCEPT_TOKEN(sym_number_literal);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(64);
      if (lookahead == 'P' ||
          lookahead == 'p') ADVANCE(70);
      if (lookahead == 'B' ||
          ('D' <= lookahead && lookahead <= 'F') ||
          lookahead == 'b' ||
          ('d' <= lookahead && lookahead <= 'f')) ADVANCE(65);
      if (lookahead == 'L' ||
          lookahead == 'U' ||
          lookahead == 'W' ||
          lookahead == 'l' ||
          lookahead == 'u' ||
          lookahead == 'w') ADVANCE(73);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'C') ||
          ('a' <= lookahead && lookahead <= 'c')) ADVANCE(65);
      END_STATE();
    case 72:
      ACCEPT_TOKEN(sym_number_literal);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(59);
      if (sym_number_literal_character_set_2(lookahead)) ADVANCE(73);
      END_STATE();
    case 73:
      ACCEPT_TOKEN(sym_number_literal);
      if (sym_number_literal_character_set_2(lookahead)) ADVANCE(73);
      END_STATE();
    case 74:
      ACCEPT_TOKEN(anon_sym_LBRACK);
      END_STATE();
    case 75:
      ACCEPT_TOKEN(anon_sym_RBRACK);
      END_STATE();
    case 76:
      ACCEPT_TOKEN(sym_edge_parser);
      if (lookahead == '.' ||
          lookahead == '<' ||
          lookahead == '>' ||
          lookahead == '^' ||
          lookahead == '_' ||
          lookahead == '|') ADVANCE(76);
      END_STATE();
    case 77:
      ACCEPT_TOKEN(anon_sym_LPAREN_RPAREN);
      END_STATE();
    case 78:
      ACCEPT_TOKEN(anon_sym_LBRACE);
      END_STATE();
    case 79:
      ACCEPT_TOKEN(anon_sym_RBRACE);
      END_STATE();
    case 80:
      ACCEPT_TOKEN(anon_sym_COLON);
      END_STATE();
    case 81:
      ACCEPT_TOKEN(anon_sym_PIPE);
      END_STATE();
    case 82:
      ACCEPT_TOKEN(anon_sym_PIPE);
      if (lookahead == '.' ||
          lookahead == '<' ||
          lookahead == '>' ||
          lookahead == '^' ||
          lookahead == '_' ||
          lookahead == '|') ADVANCE(76);
      END_STATE();
    case 83:
      ACCEPT_TOKEN(anon_sym_TILDE);
      END_STATE();
    case 84:
      ACCEPT_TOKEN(anon_sym_COMMA);
      END_STATE();
    case 85:
      ACCEPT_TOKEN(anon_sym_LPAREN);
      END_STATE();
    case 86:
      ACCEPT_TOKEN(anon_sym_RPAREN);
      END_STATE();
    case 87:
      ACCEPT_TOKEN(sym_operation);
      END_STATE();
    case 88:
      ACCEPT_TOKEN(sym_operation);
      if (lookahead == '.') ADVANCE(19);
      if (lookahead == '0') ADVANCE(57);
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(59);
      END_STATE();
    case 89:
      ACCEPT_TOKEN(sym_direction);
      END_STATE();
    case 90:
      ACCEPT_TOKEN(anon_sym_LT);
      END_STATE();
    case 91:
      ACCEPT_TOKEN(anon_sym_GT);
      END_STATE();
    case 92:
      ACCEPT_TOKEN(sym_comment);
      END_STATE();
    case 93:
      ACCEPT_TOKEN(sym_comment);
      if (lookahead == '\\') ADVANCE(40);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(93);
      END_STATE();
    case 94:
      ACCEPT_TOKEN(sym_comment);
      if (lookahead != 0 &&
          lookahead != '\r' &&
          lookahead != '\\') ADVANCE(93);
      if (lookahead == '\r') ADVANCE(95);
      if (lookahead == '\\') ADVANCE(94);
      END_STATE();
    case 95:
      ACCEPT_TOKEN(sym_comment);
      if (lookahead != 0 &&
          lookahead != '\\') ADVANCE(93);
      if (lookahead == '\\') ADVANCE(40);
      END_STATE();
    default:
      return false;
  }
}

static bool ts_lex_keywords(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (lookahead == 'S') ADVANCE(1);
      END_STATE();
    case 1:
      if (lookahead == 'i') ADVANCE(2);
      END_STATE();
    case 2:
      if (lookahead == 'z') ADVANCE(3);
      END_STATE();
    case 3:
      if (lookahead == 'e') ADVANCE(4);
      END_STATE();
    case 4:
      ACCEPT_TOKEN(anon_sym_Size);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0, .external_lex_state = 1},
  [1] = {.lex_state = 41},
  [2] = {.lex_state = 41},
  [3] = {.lex_state = 41},
  [4] = {.lex_state = 6},
  [5] = {.lex_state = 41},
  [6] = {.lex_state = 41, .external_lex_state = 2},
  [7] = {.lex_state = 41},
  [8] = {.lex_state = 41},
  [9] = {.lex_state = 41},
  [10] = {.lex_state = 13},
  [11] = {.lex_state = 41},
  [12] = {.lex_state = 41},
  [13] = {.lex_state = 41},
  [14] = {.lex_state = 41},
  [15] = {.lex_state = 13},
  [16] = {.lex_state = 41},
  [17] = {.lex_state = 41},
  [18] = {.lex_state = 41},
  [19] = {.lex_state = 41},
  [20] = {.lex_state = 41},
  [21] = {.lex_state = 41},
  [22] = {.lex_state = 41},
  [23] = {.lex_state = 41},
  [24] = {.lex_state = 41},
  [25] = {.lex_state = 41},
  [26] = {.lex_state = 41},
  [27] = {.lex_state = 41},
  [28] = {.lex_state = 41},
  [29] = {.lex_state = 41},
  [30] = {.lex_state = 41},
  [31] = {.lex_state = 41},
  [32] = {.lex_state = 13},
  [33] = {.lex_state = 6},
  [34] = {.lex_state = 13},
  [35] = {.lex_state = 14},
  [36] = {.lex_state = 13},
  [37] = {.lex_state = 13},
  [38] = {.lex_state = 41},
  [39] = {.lex_state = 41},
  [40] = {.lex_state = 41},
  [41] = {.lex_state = 41},
  [42] = {.lex_state = 13},
  [43] = {.lex_state = 41},
  [44] = {.lex_state = 13},
  [45] = {.lex_state = 7, .external_lex_state = 3},
  [46] = {.lex_state = 7, .external_lex_state = 3},
  [47] = {.lex_state = 7, .external_lex_state = 3},
  [48] = {.lex_state = 41},
  [49] = {.lex_state = 41},
  [50] = {.lex_state = 41},
  [51] = {.lex_state = 13},
  [52] = {.lex_state = 41},
  [53] = {.lex_state = 41},
  [54] = {.lex_state = 13},
  [55] = {.lex_state = 41},
  [56] = {.lex_state = 41},
  [57] = {.lex_state = 41},
  [58] = {.lex_state = 41},
  [59] = {.lex_state = 41},
  [60] = {.lex_state = 41},
  [61] = {.lex_state = 41},
  [62] = {.lex_state = 41},
  [63] = {.lex_state = 41},
  [64] = {.lex_state = 41},
  [65] = {.lex_state = 41},
  [66] = {.lex_state = 13},
  [67] = {.lex_state = 41},
  [68] = {.lex_state = 41},
  [69] = {.lex_state = 41},
  [70] = {.lex_state = 41},
  [71] = {.lex_state = 41},
  [72] = {.lex_state = 41},
  [73] = {.lex_state = 41},
  [74] = {.lex_state = 41},
  [75] = {.lex_state = 41},
  [76] = {.lex_state = 41},
  [77] = {.lex_state = 41},
  [78] = {.lex_state = 41},
  [79] = {.lex_state = 41},
  [80] = {.lex_state = 41},
  [81] = {.lex_state = 41},
  [82] = {.lex_state = 41},
  [83] = {.lex_state = 41},
  [84] = {.lex_state = 41},
  [85] = {.lex_state = 14},
  [86] = {.lex_state = 41},
  [87] = {.lex_state = 41},
  [88] = {.lex_state = 14},
  [89] = {.lex_state = 14},
  [90] = {.lex_state = 41},
  [91] = {.lex_state = 41},
  [92] = {.lex_state = 41},
  [93] = {.lex_state = 41},
  [94] = {.lex_state = 41},
  [95] = {.lex_state = 14},
  [96] = {.lex_state = 6, .external_lex_state = 4},
  [97] = {.lex_state = 14},
  [98] = {.lex_state = 41},
  [99] = {.lex_state = 41},
  [100] = {.lex_state = 41},
  [101] = {.lex_state = 13},
  [102] = {.lex_state = 41},
  [103] = {.lex_state = 41},
  [104] = {.lex_state = 41},
  [105] = {.lex_state = 41},
  [106] = {.lex_state = 41},
  [107] = {.lex_state = 41},
  [108] = {.lex_state = 41},
  [109] = {.lex_state = 41},
  [110] = {.lex_state = 41},
  [111] = {.lex_state = 14},
  [112] = {.lex_state = 41},
  [113] = {.lex_state = 41},
  [114] = {.lex_state = 41},
  [115] = {.lex_state = 41},
  [116] = {.lex_state = 41},
  [117] = {.lex_state = 41},
  [118] = {.lex_state = 41},
  [119] = {.lex_state = 6},
  [120] = {.lex_state = 41},
  [121] = {.lex_state = 41},
  [122] = {.lex_state = 41},
  [123] = {.lex_state = 41},
  [124] = {.lex_state = 41},
  [125] = {.lex_state = 41},
  [126] = {.lex_state = 41},
  [127] = {.lex_state = 41},
  [128] = {.lex_state = 41},
  [129] = {.lex_state = 41},
  [130] = {.lex_state = 41},
  [131] = {.lex_state = 6},
  [132] = {.lex_state = 41},
  [133] = {.lex_state = 41},
  [134] = {.lex_state = 41},
  [135] = {.lex_state = 41},
};

enum {
  ts_external_token_string_content = 0,
  ts_external_token_raw_string_content = 1,
  ts_external_token_obj_other = 2,
};

static const TSSymbol ts_external_scanner_symbol_map[EXTERNAL_TOKEN_COUNT] = {
  [ts_external_token_string_content] = sym_string_content,
  [ts_external_token_raw_string_content] = sym_raw_string_content,
  [ts_external_token_obj_other] = sym_obj_other,
};

static const bool ts_external_scanner_states[5][EXTERNAL_TOKEN_COUNT] = {
  [1] = {
    [ts_external_token_string_content] = true,
    [ts_external_token_raw_string_content] = true,
    [ts_external_token_obj_other] = true,
  },
  [2] = {
    [ts_external_token_obj_other] = true,
  },
  [3] = {
    [ts_external_token_string_content] = true,
  },
  [4] = {
    [ts_external_token_raw_string_content] = true,
  },
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [sym_identifier] = ACTIONS(1),
    [sym_whitespace] = ACTIONS(3),
    [anon_sym_DOT] = ACTIONS(1),
    [sym_escape_sequence] = ACTIONS(1),
    [aux_sym__simple_string_literal_token1] = ACTIONS(1),
    [anon_sym_DQUOTE] = ACTIONS(1),
    [anon_sym_r_POUND_DQUOTE] = ACTIONS(1),
    [anon_sym_DQUOTE_POUND] = ACTIONS(1),
    [sym_number_literal] = ACTIONS(1),
    [anon_sym_LBRACK] = ACTIONS(1),
    [anon_sym_RBRACK] = ACTIONS(1),
    [anon_sym_Size] = ACTIONS(1),
    [anon_sym_LBRACE] = ACTIONS(1),
    [anon_sym_RBRACE] = ACTIONS(1),
    [anon_sym_COLON] = ACTIONS(1),
    [anon_sym_PIPE] = ACTIONS(1),
    [anon_sym_TILDE] = ACTIONS(1),
    [anon_sym_COMMA] = ACTIONS(1),
    [anon_sym_LPAREN] = ACTIONS(1),
    [anon_sym_RPAREN] = ACTIONS(1),
    [sym_operation] = ACTIONS(1),
    [sym_direction] = ACTIONS(1),
    [anon_sym_LT] = ACTIONS(1),
    [anon_sym_GT] = ACTIONS(1),
    [sym_comment] = ACTIONS(5),
    [sym_string_content] = ACTIONS(1),
    [sym_raw_string_content] = ACTIONS(1),
    [sym_obj_other] = ACTIONS(1),
  },
  [1] = {
    [sym_source_file] = STATE(121),
    [sym__definition] = STATE(3),
    [sym_completion] = STATE(3),
    [sym_slide_objects] = STATE(103),
    [sym_slide_functions] = STATE(3),
    [sym_slide] = STATE(3),
    [sym_viewbox] = STATE(3),
    [sym_obj] = STATE(3),
    [sym_register] = STATE(3),
    [aux_sym_source_file_repeat1] = STATE(3),
    [ts_builtin_sym_end] = ACTIONS(7),
    [sym_identifier] = ACTIONS(9),
    [sym_whitespace] = ACTIONS(5),
    [anon_sym_LBRACK] = ACTIONS(11),
    [anon_sym_LBRACE] = ACTIONS(13),
    [anon_sym_LT] = ACTIONS(15),
    [sym_comment] = ACTIONS(5),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 8,
    ACTIONS(17), 1,
      ts_builtin_sym_end,
    ACTIONS(19), 1,
      sym_identifier,
    ACTIONS(22), 1,
      anon_sym_LBRACK,
    ACTIONS(25), 1,
      anon_sym_LBRACE,
    ACTIONS(28), 1,
      anon_sym_LT,
    STATE(103), 1,
      sym_slide_objects,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    STATE(2), 8,
      sym__definition,
      sym_completion,
      sym_slide_functions,
      sym_slide,
      sym_viewbox,
      sym_obj,
      sym_register,
      aux_sym_source_file_repeat1,
  [33] = 8,
    ACTIONS(9), 1,
      sym_identifier,
    ACTIONS(11), 1,
      anon_sym_LBRACK,
    ACTIONS(13), 1,
      anon_sym_LBRACE,
    ACTIONS(15), 1,
      anon_sym_LT,
    ACTIONS(31), 1,
      ts_builtin_sym_end,
    STATE(103), 1,
      sym_slide_objects,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    STATE(2), 8,
      sym__definition,
      sym_completion,
      sym_slide_functions,
      sym_slide,
      sym_viewbox,
      sym_obj,
      sym_register,
      aux_sym_source_file_repeat1,
  [66] = 9,
    ACTIONS(33), 1,
      sym_edge_parser,
    ACTIONS(35), 1,
      anon_sym_LBRACE,
    ACTIONS(39), 1,
      anon_sym_COLON,
    ACTIONS(41), 1,
      anon_sym_PIPE,
    ACTIONS(43), 1,
      anon_sym_TILDE,
    STATE(15), 1,
      sym_slide_vb,
    STATE(54), 1,
      sym_slide_from,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(37), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [96] = 9,
    ACTIONS(45), 1,
      sym_identifier,
    ACTIONS(47), 1,
      aux_sym__simple_string_literal_token1,
    ACTIONS(49), 1,
      anon_sym_r_POUND_DQUOTE,
    ACTIONS(51), 1,
      sym_number_literal,
    ACTIONS(53), 1,
      anon_sym_COMMA,
    ACTIONS(55), 1,
      anon_sym_RPAREN,
    STATE(55), 1,
      sym_string_literal,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    STATE(59), 2,
      sym__simple_string_literal,
      sym__raw_string_literal,
  [126] = 6,
    ACTIONS(47), 1,
      aux_sym__simple_string_literal_token1,
    ACTIONS(49), 1,
      anon_sym_r_POUND_DQUOTE,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(57), 2,
      sym_obj_other,
      sym_number_literal,
    STATE(59), 2,
      sym__simple_string_literal,
      sym__raw_string_literal,
    STATE(72), 2,
      sym_string_literal,
      sym__text_ident,
  [149] = 8,
    ACTIONS(47), 1,
      aux_sym__simple_string_literal_token1,
    ACTIONS(49), 1,
      anon_sym_r_POUND_DQUOTE,
    ACTIONS(59), 1,
      sym_identifier,
    ACTIONS(61), 1,
      sym_number_literal,
    ACTIONS(63), 1,
      anon_sym_RPAREN,
    STATE(105), 1,
      sym_string_literal,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    STATE(59), 2,
      sym__simple_string_literal,
      sym__raw_string_literal,
  [176] = 8,
    ACTIONS(47), 1,
      aux_sym__simple_string_literal_token1,
    ACTIONS(49), 1,
      anon_sym_r_POUND_DQUOTE,
    ACTIONS(59), 1,
      sym_identifier,
    ACTIONS(61), 1,
      sym_number_literal,
    ACTIONS(65), 1,
      anon_sym_RPAREN,
    STATE(105), 1,
      sym_string_literal,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    STATE(59), 2,
      sym__simple_string_literal,
      sym__raw_string_literal,
  [203] = 7,
    ACTIONS(47), 1,
      aux_sym__simple_string_literal_token1,
    ACTIONS(49), 1,
      anon_sym_r_POUND_DQUOTE,
    ACTIONS(59), 1,
      sym_identifier,
    ACTIONS(61), 1,
      sym_number_literal,
    STATE(105), 1,
      sym_string_literal,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    STATE(59), 2,
      sym__simple_string_literal,
      sym__raw_string_literal,
  [227] = 4,
    ACTIONS(67), 1,
      anon_sym_LBRACK,
    STATE(36), 1,
      sym_index_parser,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(69), 4,
      sym_edge_parser,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [244] = 2,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(71), 5,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_LBRACK,
      anon_sym_LBRACE,
      anon_sym_LT,
  [256] = 2,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(73), 5,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_LBRACK,
      anon_sym_LBRACE,
      anon_sym_LT,
  [268] = 2,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(75), 5,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_LBRACK,
      anon_sym_LBRACE,
      anon_sym_LT,
  [280] = 2,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(77), 5,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_LBRACK,
      anon_sym_LBRACE,
      anon_sym_LT,
  [292] = 5,
    ACTIONS(35), 1,
      anon_sym_LBRACE,
    ACTIONS(79), 1,
      sym_edge_parser,
    STATE(66), 1,
      sym_slide_from,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(81), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [310] = 2,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(83), 5,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_LBRACK,
      anon_sym_LBRACE,
      anon_sym_LT,
  [322] = 2,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(85), 5,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_LBRACK,
      anon_sym_LBRACE,
      anon_sym_LT,
  [334] = 2,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(87), 5,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_LBRACK,
      anon_sym_LBRACE,
      anon_sym_LT,
  [346] = 2,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(89), 5,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_LBRACK,
      anon_sym_LBRACE,
      anon_sym_LT,
  [358] = 2,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(91), 5,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_LBRACK,
      anon_sym_LBRACE,
      anon_sym_LT,
  [370] = 2,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(93), 5,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_LBRACK,
      anon_sym_LBRACE,
      anon_sym_LT,
  [382] = 2,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(95), 5,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_LBRACK,
      anon_sym_LBRACE,
      anon_sym_LT,
  [394] = 2,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(97), 5,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_LBRACK,
      anon_sym_LBRACE,
      anon_sym_LT,
  [406] = 2,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(99), 5,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_LBRACK,
      anon_sym_LBRACE,
      anon_sym_LT,
  [418] = 2,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(101), 5,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_LBRACK,
      anon_sym_LBRACE,
      anon_sym_LT,
  [430] = 2,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(103), 5,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_LBRACK,
      anon_sym_LBRACE,
      anon_sym_LT,
  [442] = 2,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(105), 5,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_LBRACK,
      anon_sym_LBRACE,
      anon_sym_LT,
  [454] = 2,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(107), 5,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_LBRACK,
      anon_sym_LBRACE,
      anon_sym_LT,
  [466] = 2,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(109), 5,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_LBRACK,
      anon_sym_LBRACE,
      anon_sym_LT,
  [478] = 2,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(111), 5,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_LBRACK,
      anon_sym_LBRACE,
      anon_sym_LT,
  [490] = 4,
    ACTIONS(115), 1,
      anon_sym_LPAREN_RPAREN,
    STATE(32), 1,
      sym__vb_identifier,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(113), 2,
      sym_identifier,
      anon_sym_Size,
  [505] = 2,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(117), 4,
      sym_edge_parser,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [516] = 5,
    ACTIONS(119), 1,
      anon_sym_LBRACK,
    ACTIONS(121), 1,
      anon_sym_LPAREN,
    STATE(24), 1,
      sym_obj_inner,
    STATE(88), 1,
      sym_index_parser,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [533] = 2,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(123), 4,
      sym_edge_parser,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [544] = 4,
    ACTIONS(119), 1,
      anon_sym_LBRACK,
    STATE(89), 1,
      sym_index_parser,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(69), 2,
      anon_sym_RBRACE,
      sym_direction,
  [559] = 2,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(125), 4,
      sym_edge_parser,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [570] = 2,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(127), 4,
      sym_edge_parser,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [581] = 5,
    ACTIONS(129), 1,
      sym_identifier,
    ACTIONS(131), 1,
      anon_sym_RBRACK,
    ACTIONS(133), 1,
      anon_sym_COMMA,
    STATE(69), 1,
      sym_slide_function,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [598] = 5,
    ACTIONS(135), 1,
      sym_identifier,
    ACTIONS(137), 1,
      anon_sym_RBRACE,
    ACTIONS(139), 1,
      anon_sym_COMMA,
    STATE(81), 1,
      sym_slide_obj,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [615] = 4,
    ACTIONS(143), 1,
      anon_sym_LPAREN_RPAREN,
    STATE(115), 1,
      sym__vb_identifier,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(141), 2,
      sym_identifier,
      anon_sym_Size,
  [630] = 4,
    ACTIONS(143), 1,
      anon_sym_LPAREN_RPAREN,
    STATE(111), 1,
      sym__vb_identifier,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(141), 2,
      sym_identifier,
      anon_sym_Size,
  [645] = 2,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(145), 4,
      sym_edge_parser,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [656] = 5,
    ACTIONS(141), 1,
      anon_sym_Size,
    ACTIONS(143), 1,
      anon_sym_LPAREN_RPAREN,
    ACTIONS(147), 1,
      sym_identifier,
    STATE(85), 1,
      sym__vb_identifier,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [673] = 2,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(149), 4,
      sym_edge_parser,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [684] = 5,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(154), 1,
      anon_sym_DQUOTE,
    STATE(45), 1,
      aux_sym__simple_string_literal_repeat1,
    ACTIONS(151), 2,
      sym_string_content,
      sym_escape_sequence,
  [701] = 5,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(158), 1,
      anon_sym_DQUOTE,
    STATE(45), 1,
      aux_sym__simple_string_literal_repeat1,
    ACTIONS(156), 2,
      sym_string_content,
      sym_escape_sequence,
  [718] = 5,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(5), 1,
      sym_comment,
    ACTIONS(162), 1,
      anon_sym_DQUOTE,
    STATE(46), 1,
      aux_sym__simple_string_literal_repeat1,
    ACTIONS(160), 2,
      sym_string_content,
      sym_escape_sequence,
  [735] = 5,
    ACTIONS(164), 1,
      sym_number_literal,
    ACTIONS(166), 1,
      anon_sym_RBRACK,
    ACTIONS(168), 1,
      anon_sym_COMMA,
    STATE(80), 1,
      sym_viewbox_obj,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [752] = 5,
    ACTIONS(170), 1,
      sym_identifier,
    ACTIONS(172), 1,
      anon_sym_COMMA,
    ACTIONS(174), 1,
      anon_sym_RPAREN,
    STATE(76), 1,
      sym_obj_param,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [769] = 2,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(176), 3,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_GT,
  [779] = 2,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(178), 3,
      sym_edge_parser,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [789] = 4,
    ACTIONS(129), 1,
      sym_identifier,
    ACTIONS(180), 1,
      anon_sym_RBRACK,
    STATE(109), 1,
      sym_slide_function,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [803] = 4,
    ACTIONS(180), 1,
      anon_sym_RBRACK,
    ACTIONS(182), 1,
      anon_sym_COMMA,
    STATE(62), 1,
      aux_sym_slide_functions_repeat1,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [817] = 3,
    ACTIONS(79), 1,
      sym_edge_parser,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(81), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [829] = 4,
    ACTIONS(184), 1,
      anon_sym_COMMA,
    ACTIONS(186), 1,
      anon_sym_RPAREN,
    STATE(77), 1,
      aux_sym_slide_function_repeat1,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [843] = 4,
    ACTIONS(188), 1,
      anon_sym_RBRACK,
    ACTIONS(190), 1,
      anon_sym_COMMA,
    STATE(56), 1,
      aux_sym_viewbox_inner_repeat1,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [857] = 4,
    ACTIONS(164), 1,
      sym_number_literal,
    ACTIONS(193), 1,
      anon_sym_RBRACK,
    STATE(107), 1,
      sym_viewbox_obj,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [871] = 4,
    ACTIONS(195), 1,
      anon_sym_COMMA,
    ACTIONS(198), 1,
      anon_sym_RPAREN,
    STATE(58), 1,
      aux_sym_obj_inner_repeat1,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [885] = 2,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(200), 3,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_GT,
  [895] = 4,
    ACTIONS(170), 1,
      sym_identifier,
    ACTIONS(202), 1,
      anon_sym_RPAREN,
    STATE(112), 1,
      sym_obj_param,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [909] = 4,
    ACTIONS(129), 1,
      sym_identifier,
    ACTIONS(204), 1,
      anon_sym_RBRACK,
    STATE(109), 1,
      sym_slide_function,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [923] = 4,
    ACTIONS(206), 1,
      anon_sym_RBRACK,
    ACTIONS(208), 1,
      anon_sym_COMMA,
    STATE(62), 1,
      aux_sym_slide_functions_repeat1,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [937] = 2,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(211), 3,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_GT,
  [947] = 4,
    ACTIONS(135), 1,
      sym_identifier,
    ACTIONS(213), 1,
      anon_sym_RBRACE,
    STATE(104), 1,
      sym_slide_obj,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [961] = 4,
    ACTIONS(215), 1,
      anon_sym_COMMA,
    ACTIONS(218), 1,
      anon_sym_RPAREN,
    STATE(65), 1,
      aux_sym_slide_function_repeat1,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [975] = 3,
    ACTIONS(220), 1,
      sym_edge_parser,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(222), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [987] = 4,
    ACTIONS(224), 1,
      anon_sym_RBRACK,
    ACTIONS(226), 1,
      anon_sym_COMMA,
    STATE(56), 1,
      aux_sym_viewbox_inner_repeat1,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [1001] = 4,
    ACTIONS(164), 1,
      sym_number_literal,
    ACTIONS(228), 1,
      anon_sym_RBRACK,
    STATE(107), 1,
      sym_viewbox_obj,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [1015] = 4,
    ACTIONS(230), 1,
      anon_sym_RBRACK,
    ACTIONS(232), 1,
      anon_sym_COMMA,
    STATE(53), 1,
      aux_sym_slide_functions_repeat1,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [1029] = 4,
    ACTIONS(135), 1,
      sym_identifier,
    ACTIONS(234), 1,
      anon_sym_RBRACE,
    STATE(104), 1,
      sym_slide_obj,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [1043] = 4,
    ACTIONS(236), 1,
      anon_sym_RBRACE,
    ACTIONS(238), 1,
      anon_sym_COMMA,
    STATE(71), 1,
      aux_sym_slide_objects_repeat1,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [1057] = 2,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(241), 3,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_GT,
  [1067] = 4,
    ACTIONS(243), 1,
      anon_sym_COMMA,
    ACTIONS(245), 1,
      anon_sym_RPAREN,
    STATE(58), 1,
      aux_sym_obj_inner_repeat1,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [1081] = 4,
    ACTIONS(170), 1,
      sym_identifier,
    ACTIONS(245), 1,
      anon_sym_RPAREN,
    STATE(112), 1,
      sym_obj_param,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [1095] = 4,
    ACTIONS(247), 1,
      anon_sym_RBRACE,
    ACTIONS(249), 1,
      anon_sym_COMMA,
    STATE(71), 1,
      aux_sym_slide_objects_repeat1,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [1109] = 4,
    ACTIONS(251), 1,
      anon_sym_COMMA,
    ACTIONS(253), 1,
      anon_sym_RPAREN,
    STATE(73), 1,
      aux_sym_obj_inner_repeat1,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [1123] = 4,
    ACTIONS(63), 1,
      anon_sym_RPAREN,
    ACTIONS(255), 1,
      anon_sym_COMMA,
    STATE(65), 1,
      aux_sym_slide_function_repeat1,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [1137] = 2,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(257), 3,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_GT,
  [1147] = 2,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(259), 3,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_GT,
  [1157] = 4,
    ACTIONS(261), 1,
      anon_sym_RBRACK,
    ACTIONS(263), 1,
      anon_sym_COMMA,
    STATE(67), 1,
      aux_sym_viewbox_inner_repeat1,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [1171] = 4,
    ACTIONS(265), 1,
      anon_sym_RBRACE,
    ACTIONS(267), 1,
      anon_sym_COMMA,
    STATE(75), 1,
      aux_sym_slide_objects_repeat1,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [1185] = 3,
    ACTIONS(135), 1,
      sym_identifier,
    STATE(104), 1,
      sym_slide_obj,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [1196] = 2,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(269), 2,
      anon_sym_RBRACK,
      anon_sym_COMMA,
  [1205] = 3,
    ACTIONS(67), 1,
      anon_sym_LBRACK,
    STATE(37), 1,
      sym_index_parser,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [1216] = 3,
    ACTIONS(271), 1,
      sym_direction,
    STATE(26), 1,
      sym_viewbox_inner,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [1227] = 2,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(273), 2,
      anon_sym_RBRACK,
      anon_sym_COMMA,
  [1236] = 3,
    ACTIONS(119), 1,
      anon_sym_LBRACK,
    STATE(88), 1,
      sym_index_parser,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [1247] = 2,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(127), 2,
      anon_sym_RBRACE,
      sym_direction,
  [1256] = 2,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(125), 2,
      anon_sym_RBRACE,
      sym_direction,
  [1265] = 3,
    ACTIONS(129), 1,
      sym_identifier,
    STATE(109), 1,
      sym_slide_function,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [1276] = 3,
    ACTIONS(67), 1,
      anon_sym_LBRACK,
    STATE(44), 1,
      sym_index_parser,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [1287] = 2,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(275), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [1296] = 2,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(81), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [1305] = 3,
    ACTIONS(164), 1,
      sym_number_literal,
    STATE(107), 1,
      sym_viewbox_obj,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [1316] = 2,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(123), 2,
      anon_sym_RBRACE,
      sym_direction,
  [1325] = 3,
    ACTIONS(277), 1,
      anon_sym_DQUOTE_POUND,
    ACTIONS(279), 1,
      sym_raw_string_content,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [1336] = 3,
    ACTIONS(281), 1,
      anon_sym_DOT,
    ACTIONS(283), 1,
      anon_sym_COLON,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [1347] = 3,
    ACTIONS(170), 1,
      sym_identifier,
    STATE(112), 1,
      sym_obj_param,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [1358] = 3,
    ACTIONS(170), 1,
      sym_identifier,
    STATE(116), 1,
      sym_obj_param,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [1369] = 2,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(285), 2,
      anon_sym_RBRACK,
      anon_sym_COMMA,
  [1378] = 3,
    ACTIONS(287), 1,
      anon_sym_COLON,
    ACTIONS(289), 1,
      sym_operation,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [1389] = 2,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(291), 2,
      anon_sym_RBRACK,
      anon_sym_COMMA,
  [1398] = 3,
    ACTIONS(11), 1,
      anon_sym_LBRACK,
    STATE(21), 1,
      sym_slide_functions,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [1409] = 2,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(236), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [1418] = 2,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(218), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
  [1427] = 2,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(293), 2,
      anon_sym_RBRACK,
      anon_sym_COMMA,
  [1436] = 2,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(188), 2,
      anon_sym_RBRACK,
      anon_sym_COMMA,
  [1445] = 2,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(295), 2,
      anon_sym_RBRACK,
      anon_sym_COMMA,
  [1454] = 2,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(206), 2,
      anon_sym_RBRACK,
      anon_sym_COMMA,
  [1463] = 2,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(222), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [1472] = 3,
    ACTIONS(271), 1,
      sym_direction,
    STATE(91), 1,
      sym_viewbox_inner,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [1483] = 2,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(198), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
  [1492] = 2,
    ACTIONS(297), 1,
      anon_sym_RBRACK,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [1500] = 2,
    ACTIONS(230), 1,
      anon_sym_RBRACK,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [1508] = 2,
    ACTIONS(299), 1,
      anon_sym_RBRACE,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [1516] = 2,
    ACTIONS(301), 1,
      anon_sym_GT,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [1524] = 2,
    ACTIONS(303), 1,
      anon_sym_LBRACK,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [1532] = 2,
    ACTIONS(305), 1,
      sym_identifier,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [1540] = 2,
    ACTIONS(307), 1,
      anon_sym_LPAREN,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [1548] = 2,
    ACTIONS(309), 1,
      sym_number_literal,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [1556] = 2,
    ACTIONS(311), 1,
      ts_builtin_sym_end,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [1564] = 2,
    ACTIONS(186), 1,
      anon_sym_RPAREN,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [1572] = 2,
    ACTIONS(313), 1,
      anon_sym_LBRACK,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [1580] = 2,
    ACTIONS(315), 1,
      anon_sym_LBRACK,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [1588] = 2,
    ACTIONS(317), 1,
      anon_sym_LBRACK,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [1596] = 2,
    ACTIONS(253), 1,
      anon_sym_RPAREN,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [1604] = 2,
    ACTIONS(319), 1,
      anon_sym_LBRACK,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [1612] = 2,
    ACTIONS(321), 1,
      sym_number_literal,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [1620] = 2,
    ACTIONS(323), 1,
      anon_sym_LBRACK,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [1628] = 2,
    ACTIONS(325), 1,
      anon_sym_RBRACE,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [1636] = 2,
    ACTIONS(327), 1,
      anon_sym_DQUOTE_POUND,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [1644] = 2,
    ACTIONS(329), 1,
      anon_sym_RBRACK,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [1652] = 2,
    ACTIONS(331), 1,
      anon_sym_COLON,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [1660] = 2,
    ACTIONS(333), 1,
      anon_sym_RBRACK,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
  [1668] = 2,
    ACTIONS(335), 1,
      sym_number_literal,
    ACTIONS(5), 2,
      sym_whitespace,
      sym_comment,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 33,
  [SMALL_STATE(4)] = 66,
  [SMALL_STATE(5)] = 96,
  [SMALL_STATE(6)] = 126,
  [SMALL_STATE(7)] = 149,
  [SMALL_STATE(8)] = 176,
  [SMALL_STATE(9)] = 203,
  [SMALL_STATE(10)] = 227,
  [SMALL_STATE(11)] = 244,
  [SMALL_STATE(12)] = 256,
  [SMALL_STATE(13)] = 268,
  [SMALL_STATE(14)] = 280,
  [SMALL_STATE(15)] = 292,
  [SMALL_STATE(16)] = 310,
  [SMALL_STATE(17)] = 322,
  [SMALL_STATE(18)] = 334,
  [SMALL_STATE(19)] = 346,
  [SMALL_STATE(20)] = 358,
  [SMALL_STATE(21)] = 370,
  [SMALL_STATE(22)] = 382,
  [SMALL_STATE(23)] = 394,
  [SMALL_STATE(24)] = 406,
  [SMALL_STATE(25)] = 418,
  [SMALL_STATE(26)] = 430,
  [SMALL_STATE(27)] = 442,
  [SMALL_STATE(28)] = 454,
  [SMALL_STATE(29)] = 466,
  [SMALL_STATE(30)] = 478,
  [SMALL_STATE(31)] = 490,
  [SMALL_STATE(32)] = 505,
  [SMALL_STATE(33)] = 516,
  [SMALL_STATE(34)] = 533,
  [SMALL_STATE(35)] = 544,
  [SMALL_STATE(36)] = 559,
  [SMALL_STATE(37)] = 570,
  [SMALL_STATE(38)] = 581,
  [SMALL_STATE(39)] = 598,
  [SMALL_STATE(40)] = 615,
  [SMALL_STATE(41)] = 630,
  [SMALL_STATE(42)] = 645,
  [SMALL_STATE(43)] = 656,
  [SMALL_STATE(44)] = 673,
  [SMALL_STATE(45)] = 684,
  [SMALL_STATE(46)] = 701,
  [SMALL_STATE(47)] = 718,
  [SMALL_STATE(48)] = 735,
  [SMALL_STATE(49)] = 752,
  [SMALL_STATE(50)] = 769,
  [SMALL_STATE(51)] = 779,
  [SMALL_STATE(52)] = 789,
  [SMALL_STATE(53)] = 803,
  [SMALL_STATE(54)] = 817,
  [SMALL_STATE(55)] = 829,
  [SMALL_STATE(56)] = 843,
  [SMALL_STATE(57)] = 857,
  [SMALL_STATE(58)] = 871,
  [SMALL_STATE(59)] = 885,
  [SMALL_STATE(60)] = 895,
  [SMALL_STATE(61)] = 909,
  [SMALL_STATE(62)] = 923,
  [SMALL_STATE(63)] = 937,
  [SMALL_STATE(64)] = 947,
  [SMALL_STATE(65)] = 961,
  [SMALL_STATE(66)] = 975,
  [SMALL_STATE(67)] = 987,
  [SMALL_STATE(68)] = 1001,
  [SMALL_STATE(69)] = 1015,
  [SMALL_STATE(70)] = 1029,
  [SMALL_STATE(71)] = 1043,
  [SMALL_STATE(72)] = 1057,
  [SMALL_STATE(73)] = 1067,
  [SMALL_STATE(74)] = 1081,
  [SMALL_STATE(75)] = 1095,
  [SMALL_STATE(76)] = 1109,
  [SMALL_STATE(77)] = 1123,
  [SMALL_STATE(78)] = 1137,
  [SMALL_STATE(79)] = 1147,
  [SMALL_STATE(80)] = 1157,
  [SMALL_STATE(81)] = 1171,
  [SMALL_STATE(82)] = 1185,
  [SMALL_STATE(83)] = 1196,
  [SMALL_STATE(84)] = 1205,
  [SMALL_STATE(85)] = 1216,
  [SMALL_STATE(86)] = 1227,
  [SMALL_STATE(87)] = 1236,
  [SMALL_STATE(88)] = 1247,
  [SMALL_STATE(89)] = 1256,
  [SMALL_STATE(90)] = 1265,
  [SMALL_STATE(91)] = 1276,
  [SMALL_STATE(92)] = 1287,
  [SMALL_STATE(93)] = 1296,
  [SMALL_STATE(94)] = 1305,
  [SMALL_STATE(95)] = 1316,
  [SMALL_STATE(96)] = 1325,
  [SMALL_STATE(97)] = 1336,
  [SMALL_STATE(98)] = 1347,
  [SMALL_STATE(99)] = 1358,
  [SMALL_STATE(100)] = 1369,
  [SMALL_STATE(101)] = 1378,
  [SMALL_STATE(102)] = 1389,
  [SMALL_STATE(103)] = 1398,
  [SMALL_STATE(104)] = 1409,
  [SMALL_STATE(105)] = 1418,
  [SMALL_STATE(106)] = 1427,
  [SMALL_STATE(107)] = 1436,
  [SMALL_STATE(108)] = 1445,
  [SMALL_STATE(109)] = 1454,
  [SMALL_STATE(110)] = 1463,
  [SMALL_STATE(111)] = 1472,
  [SMALL_STATE(112)] = 1483,
  [SMALL_STATE(113)] = 1492,
  [SMALL_STATE(114)] = 1500,
  [SMALL_STATE(115)] = 1508,
  [SMALL_STATE(116)] = 1516,
  [SMALL_STATE(117)] = 1524,
  [SMALL_STATE(118)] = 1532,
  [SMALL_STATE(119)] = 1540,
  [SMALL_STATE(120)] = 1548,
  [SMALL_STATE(121)] = 1556,
  [SMALL_STATE(122)] = 1564,
  [SMALL_STATE(123)] = 1572,
  [SMALL_STATE(124)] = 1580,
  [SMALL_STATE(125)] = 1588,
  [SMALL_STATE(126)] = 1596,
  [SMALL_STATE(127)] = 1604,
  [SMALL_STATE(128)] = 1612,
  [SMALL_STATE(129)] = 1620,
  [SMALL_STATE(130)] = 1628,
  [SMALL_STATE(131)] = 1636,
  [SMALL_STATE(132)] = 1644,
  [SMALL_STATE(133)] = 1652,
  [SMALL_STATE(134)] = 1660,
  [SMALL_STATE(135)] = 1668,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = false}}, SHIFT_EXTRA(),
  [5] = {.entry = {.count = 1, .reusable = true}}, SHIFT_EXTRA(),
  [7] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 0),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(97),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(38),
  [13] = {.entry = {.count = 1, .reusable = true}}, SHIFT(39),
  [15] = {.entry = {.count = 1, .reusable = true}}, SHIFT(99),
  [17] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2),
  [19] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(97),
  [22] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(38),
  [25] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(39),
  [28] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(99),
  [31] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1),
  [33] = {.entry = {.count = 1, .reusable = false}}, SHIFT(93),
  [35] = {.entry = {.count = 1, .reusable = true}}, SHIFT(40),
  [37] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_slide_obj, 1, .production_id = 1),
  [39] = {.entry = {.count = 1, .reusable = true}}, SHIFT(31),
  [41] = {.entry = {.count = 1, .reusable = false}}, SHIFT(41),
  [43] = {.entry = {.count = 1, .reusable = true}}, SHIFT(42),
  [45] = {.entry = {.count = 1, .reusable = false}}, SHIFT(55),
  [47] = {.entry = {.count = 1, .reusable = true}}, SHIFT(47),
  [49] = {.entry = {.count = 1, .reusable = true}}, SHIFT(96),
  [51] = {.entry = {.count = 1, .reusable = true}}, SHIFT(55),
  [53] = {.entry = {.count = 1, .reusable = true}}, SHIFT(122),
  [55] = {.entry = {.count = 1, .reusable = true}}, SHIFT(102),
  [57] = {.entry = {.count = 1, .reusable = true}}, SHIFT(72),
  [59] = {.entry = {.count = 1, .reusable = false}}, SHIFT(105),
  [61] = {.entry = {.count = 1, .reusable = true}}, SHIFT(105),
  [63] = {.entry = {.count = 1, .reusable = true}}, SHIFT(106),
  [65] = {.entry = {.count = 1, .reusable = true}}, SHIFT(83),
  [67] = {.entry = {.count = 1, .reusable = true}}, SHIFT(135),
  [69] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__vb_identifier, 1),
  [71] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_obj_inner, 3),
  [73] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_viewbox_inner, 3, .production_id = 13),
  [75] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_slide_functions, 2),
  [77] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_viewbox_inner, 3, .production_id = 11),
  [79] = {.entry = {.count = 1, .reusable = true}}, SHIFT(110),
  [81] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_slide_obj, 2, .production_id = 1),
  [83] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_slide_functions, 5),
  [85] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_viewbox_inner, 5, .production_id = 16),
  [87] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_viewbox_inner, 2, .production_id = 11),
  [89] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_obj_inner, 2),
  [91] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_register, 3),
  [93] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_slide, 2),
  [95] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_obj_inner, 4),
  [97] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_completion, 3),
  [99] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_obj, 4, .production_id = 4),
  [101] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_viewbox_inner, 4, .production_id = 13),
  [103] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_viewbox, 4, .production_id = 6),
  [105] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_slide_functions, 4),
  [107] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_viewbox_inner, 4, .production_id = 16),
  [109] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_slide_functions, 3),
  [111] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_obj_inner, 5),
  [113] = {.entry = {.count = 1, .reusable = false}}, SHIFT(84),
  [115] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
  [117] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_slide_vb, 2, .production_id = 8),
  [119] = {.entry = {.count = 1, .reusable = true}}, SHIFT(128),
  [121] = {.entry = {.count = 1, .reusable = true}}, SHIFT(49),
  [123] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_index_parser, 3),
  [125] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__vb_identifier, 2, .production_id = 5),
  [127] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__vb_identifier, 2, .production_id = 3),
  [129] = {.entry = {.count = 1, .reusable = true}}, SHIFT(119),
  [131] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [133] = {.entry = {.count = 1, .reusable = true}}, SHIFT(114),
  [135] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [137] = {.entry = {.count = 1, .reusable = true}}, SHIFT(127),
  [139] = {.entry = {.count = 1, .reusable = true}}, SHIFT(130),
  [141] = {.entry = {.count = 1, .reusable = false}}, SHIFT(87),
  [143] = {.entry = {.count = 1, .reusable = true}}, SHIFT(35),
  [145] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_slide_vb, 1),
  [147] = {.entry = {.count = 1, .reusable = false}}, SHIFT(33),
  [149] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_slide_vb, 4, .production_id = 14),
  [151] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym__simple_string_literal_repeat1, 2), SHIFT_REPEAT(45),
  [154] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym__simple_string_literal_repeat1, 2),
  [156] = {.entry = {.count = 1, .reusable = true}}, SHIFT(45),
  [158] = {.entry = {.count = 1, .reusable = true}}, SHIFT(50),
  [160] = {.entry = {.count = 1, .reusable = true}}, SHIFT(46),
  [162] = {.entry = {.count = 1, .reusable = true}}, SHIFT(78),
  [164] = {.entry = {.count = 1, .reusable = true}}, SHIFT(101),
  [166] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [168] = {.entry = {.count = 1, .reusable = true}}, SHIFT(132),
  [170] = {.entry = {.count = 1, .reusable = true}}, SHIFT(133),
  [172] = {.entry = {.count = 1, .reusable = true}}, SHIFT(126),
  [174] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [176] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__simple_string_literal, 3),
  [178] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_slide_from, 3, .production_id = 8),
  [180] = {.entry = {.count = 1, .reusable = true}}, SHIFT(27),
  [182] = {.entry = {.count = 1, .reusable = true}}, SHIFT(61),
  [184] = {.entry = {.count = 1, .reusable = true}}, SHIFT(7),
  [186] = {.entry = {.count = 1, .reusable = true}}, SHIFT(86),
  [188] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_viewbox_inner_repeat1, 2),
  [190] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_viewbox_inner_repeat1, 2), SHIFT_REPEAT(94),
  [193] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [195] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_obj_inner_repeat1, 2), SHIFT_REPEAT(98),
  [198] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_obj_inner_repeat1, 2),
  [200] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string_literal, 1),
  [202] = {.entry = {.count = 1, .reusable = true}}, SHIFT(30),
  [204] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [206] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_slide_functions_repeat1, 2),
  [208] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_slide_functions_repeat1, 2), SHIFT_REPEAT(90),
  [211] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__raw_string_literal, 3),
  [213] = {.entry = {.count = 1, .reusable = true}}, SHIFT(117),
  [215] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_slide_function_repeat1, 2), SHIFT_REPEAT(9),
  [218] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_slide_function_repeat1, 2),
  [220] = {.entry = {.count = 1, .reusable = true}}, SHIFT(92),
  [222] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_slide_obj, 3, .production_id = 1),
  [224] = {.entry = {.count = 1, .reusable = true}}, SHIFT(28),
  [226] = {.entry = {.count = 1, .reusable = true}}, SHIFT(57),
  [228] = {.entry = {.count = 1, .reusable = true}}, SHIFT(25),
  [230] = {.entry = {.count = 1, .reusable = true}}, SHIFT(29),
  [232] = {.entry = {.count = 1, .reusable = true}}, SHIFT(52),
  [234] = {.entry = {.count = 1, .reusable = true}}, SHIFT(129),
  [236] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_slide_objects_repeat1, 2),
  [238] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_slide_objects_repeat1, 2), SHIFT_REPEAT(82),
  [241] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_obj_param, 3, .production_id = 10),
  [243] = {.entry = {.count = 1, .reusable = true}}, SHIFT(60),
  [245] = {.entry = {.count = 1, .reusable = true}}, SHIFT(22),
  [247] = {.entry = {.count = 1, .reusable = true}}, SHIFT(125),
  [249] = {.entry = {.count = 1, .reusable = true}}, SHIFT(70),
  [251] = {.entry = {.count = 1, .reusable = true}}, SHIFT(74),
  [253] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [255] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [257] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__simple_string_literal, 2),
  [259] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__raw_string_literal, 2),
  [261] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [263] = {.entry = {.count = 1, .reusable = true}}, SHIFT(68),
  [265] = {.entry = {.count = 1, .reusable = true}}, SHIFT(123),
  [267] = {.entry = {.count = 1, .reusable = true}}, SHIFT(64),
  [269] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_slide_function, 6, .production_id = 7),
  [271] = {.entry = {.count = 1, .reusable = true}}, SHIFT(48),
  [273] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_slide_function, 4, .production_id = 7),
  [275] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_slide_obj, 4, .production_id = 1),
  [277] = {.entry = {.count = 1, .reusable = true}}, SHIFT(79),
  [279] = {.entry = {.count = 1, .reusable = true}}, SHIFT(131),
  [281] = {.entry = {.count = 1, .reusable = true}}, SHIFT(118),
  [283] = {.entry = {.count = 1, .reusable = true}}, SHIFT(43),
  [285] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_viewbox_obj, 2, .production_id = 12),
  [287] = {.entry = {.count = 1, .reusable = true}}, SHIFT(120),
  [289] = {.entry = {.count = 1, .reusable = true}}, SHIFT(100),
  [291] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_slide_function, 3, .production_id = 7),
  [293] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_slide_function, 5, .production_id = 7),
  [295] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_viewbox_obj, 3, .production_id = 15),
  [297] = {.entry = {.count = 1, .reusable = true}}, SHIFT(95),
  [299] = {.entry = {.count = 1, .reusable = true}}, SHIFT(51),
  [301] = {.entry = {.count = 1, .reusable = true}}, SHIFT(20),
  [303] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_slide_objects, 4, .production_id = 2),
  [305] = {.entry = {.count = 1, .reusable = true}}, SHIFT(23),
  [307] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [309] = {.entry = {.count = 1, .reusable = true}}, SHIFT(108),
  [311] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [313] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_slide_objects, 3, .production_id = 2),
  [315] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_slide_objects, 3),
  [317] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_slide_objects, 4, .production_id = 9),
  [319] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_slide_objects, 2),
  [321] = {.entry = {.count = 1, .reusable = true}}, SHIFT(113),
  [323] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_slide_objects, 5, .production_id = 9),
  [325] = {.entry = {.count = 1, .reusable = true}}, SHIFT(124),
  [327] = {.entry = {.count = 1, .reusable = true}}, SHIFT(63),
  [329] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [331] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
  [333] = {.entry = {.count = 1, .reusable = true}}, SHIFT(34),
  [335] = {.entry = {.count = 1, .reusable = true}}, SHIFT(134),
};

#ifdef __cplusplus
extern "C" {
#endif
void *tree_sitter_grz_external_scanner_create(void);
void tree_sitter_grz_external_scanner_destroy(void *);
bool tree_sitter_grz_external_scanner_scan(void *, TSLexer *, const bool *);
unsigned tree_sitter_grz_external_scanner_serialize(void *, char *);
void tree_sitter_grz_external_scanner_deserialize(void *, const char *, unsigned);

#ifdef _WIN32
#define extern __declspec(dllexport)
#endif

extern const TSLanguage *tree_sitter_grz(void) {
  static const TSLanguage language = {
    .version = LANGUAGE_VERSION,
    .symbol_count = SYMBOL_COUNT,
    .alias_count = ALIAS_COUNT,
    .token_count = TOKEN_COUNT,
    .external_token_count = EXTERNAL_TOKEN_COUNT,
    .state_count = STATE_COUNT,
    .large_state_count = LARGE_STATE_COUNT,
    .production_id_count = PRODUCTION_ID_COUNT,
    .field_count = FIELD_COUNT,
    .max_alias_sequence_length = MAX_ALIAS_SEQUENCE_LENGTH,
    .parse_table = &ts_parse_table[0][0],
    .small_parse_table = ts_small_parse_table,
    .small_parse_table_map = ts_small_parse_table_map,
    .parse_actions = ts_parse_actions,
    .symbol_names = ts_symbol_names,
    .field_names = ts_field_names,
    .field_map_slices = ts_field_map_slices,
    .field_map_entries = ts_field_map_entries,
    .symbol_metadata = ts_symbol_metadata,
    .public_symbol_map = ts_symbol_map,
    .alias_map = ts_non_terminal_alias_map,
    .alias_sequences = &ts_alias_sequences[0][0],
    .lex_modes = ts_lex_modes,
    .lex_fn = ts_lex,
    .keyword_lex_fn = ts_lex_keywords,
    .keyword_capture_token = sym_identifier,
    .external_scanner = {
      &ts_external_scanner_states[0][0],
      ts_external_scanner_symbol_map,
      tree_sitter_grz_external_scanner_create,
      tree_sitter_grz_external_scanner_destroy,
      tree_sitter_grz_external_scanner_scan,
      tree_sitter_grz_external_scanner_serialize,
      tree_sitter_grz_external_scanner_deserialize,
    },
    .primary_state_ids = ts_primary_state_ids,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
