#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 152
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 55
#define ALIAS_COUNT 0
#define TOKEN_COUNT 25
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 13
#define MAX_ALIAS_SEQUENCE_LENGTH 8
#define PRODUCTION_ID_COUNT 16

enum {
  sym_identifier = 1,
  sym_whitespace = 2,
  anon_sym_DOT = 3,
  sym_escape_sequence = 4,
  anon_sym_L_DQUOTE = 5,
  anon_sym_u_DQUOTE = 6,
  anon_sym_U_DQUOTE = 7,
  anon_sym_u8_DQUOTE = 8,
  anon_sym_DQUOTE = 9,
  aux_sym_string_literal_token1 = 10,
  sym_number_literal = 11,
  anon_sym_LBRACK = 12,
  anon_sym_RBRACK = 13,
  sym_edge_parser = 14,
  anon_sym_Size = 15,
  anon_sym_LBRACE = 16,
  anon_sym_RBRACE = 17,
  anon_sym_COLON = 18,
  anon_sym_COMMA = 19,
  anon_sym_LPAREN = 20,
  anon_sym_RPAREN = 21,
  sym_operation = 22,
  sym_direction = 23,
  sym_comment = 24,
  sym_source_file = 25,
  sym__definition = 26,
  sym_completion = 27,
  sym_string_literal = 28,
  sym__text_ident = 29,
  sym_index_parser = 30,
  sym__vb_identifier = 31,
  sym_slide_from = 32,
  sym_slide_obj = 33,
  sym_slide_objects = 34,
  sym_slide_function = 35,
  sym_slide_functions = 36,
  sym_slide = 37,
  sym_viewbox_obj = 38,
  sym_viewbox_inner = 39,
  sym_viewbox = 40,
  sym_obj_inner = 41,
  sym_obj = 42,
  sym_action_obj = 43,
  sym_action = 44,
  sym_register = 45,
  aux_sym_source_file_repeat1 = 46,
  aux_sym_string_literal_repeat1 = 47,
  aux_sym_slide_objects_repeat1 = 48,
  aux_sym_slide_function_repeat1 = 49,
  aux_sym_slide_functions_repeat1 = 50,
  aux_sym_viewbox_inner_repeat1 = 51,
  aux_sym_obj_inner_repeat1 = 52,
  aux_sym_action_obj_repeat1 = 53,
  aux_sym_action_repeat1 = 54,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [sym_identifier] = "identifier",
  [sym_whitespace] = "whitespace",
  [anon_sym_DOT] = ".",
  [sym_escape_sequence] = "escape_sequence",
  [anon_sym_L_DQUOTE] = "L\"",
  [anon_sym_u_DQUOTE] = "u\"",
  [anon_sym_U_DQUOTE] = "U\"",
  [anon_sym_u8_DQUOTE] = "u8\"",
  [anon_sym_DQUOTE] = "\"",
  [aux_sym_string_literal_token1] = "string_content",
  [sym_number_literal] = "number_literal",
  [anon_sym_LBRACK] = "[",
  [anon_sym_RBRACK] = "]",
  [sym_edge_parser] = "edge_parser",
  [anon_sym_Size] = "size",
  [anon_sym_LBRACE] = "{",
  [anon_sym_RBRACE] = "}",
  [anon_sym_COLON] = ":",
  [anon_sym_COMMA] = ",",
  [anon_sym_LPAREN] = "(",
  [anon_sym_RPAREN] = ")",
  [sym_operation] = "operation",
  [sym_direction] = "direction",
  [sym_comment] = "comment",
  [sym_source_file] = "source_file",
  [sym__definition] = "_definition",
  [sym_completion] = "completion",
  [sym_string_literal] = "string_literal",
  [sym__text_ident] = "_text_ident",
  [sym_index_parser] = "index_parser",
  [sym__vb_identifier] = "_vb_identifier",
  [sym_slide_from] = "slide_from",
  [sym_slide_obj] = "slide_obj",
  [sym_slide_objects] = "slide_objects",
  [sym_slide_function] = "slide_function",
  [sym_slide_functions] = "slide_functions",
  [sym_slide] = "slide",
  [sym_viewbox_obj] = "viewbox_obj",
  [sym_viewbox_inner] = "viewbox_inner",
  [sym_viewbox] = "viewbox",
  [sym_obj_inner] = "obj_inner",
  [sym_obj] = "obj",
  [sym_action_obj] = "action_obj",
  [sym_action] = "action",
  [sym_register] = "register",
  [aux_sym_source_file_repeat1] = "source_file_repeat1",
  [aux_sym_string_literal_repeat1] = "string_literal_repeat1",
  [aux_sym_slide_objects_repeat1] = "slide_objects_repeat1",
  [aux_sym_slide_function_repeat1] = "slide_function_repeat1",
  [aux_sym_slide_functions_repeat1] = "slide_functions_repeat1",
  [aux_sym_viewbox_inner_repeat1] = "viewbox_inner_repeat1",
  [aux_sym_obj_inner_repeat1] = "obj_inner_repeat1",
  [aux_sym_action_obj_repeat1] = "action_obj_repeat1",
  [aux_sym_action_repeat1] = "action_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [sym_identifier] = sym_identifier,
  [sym_whitespace] = sym_whitespace,
  [anon_sym_DOT] = anon_sym_DOT,
  [sym_escape_sequence] = sym_escape_sequence,
  [anon_sym_L_DQUOTE] = anon_sym_L_DQUOTE,
  [anon_sym_u_DQUOTE] = anon_sym_u_DQUOTE,
  [anon_sym_U_DQUOTE] = anon_sym_U_DQUOTE,
  [anon_sym_u8_DQUOTE] = anon_sym_u8_DQUOTE,
  [anon_sym_DQUOTE] = anon_sym_DQUOTE,
  [aux_sym_string_literal_token1] = aux_sym_string_literal_token1,
  [sym_number_literal] = sym_number_literal,
  [anon_sym_LBRACK] = anon_sym_LBRACK,
  [anon_sym_RBRACK] = anon_sym_RBRACK,
  [sym_edge_parser] = sym_edge_parser,
  [anon_sym_Size] = anon_sym_Size,
  [anon_sym_LBRACE] = anon_sym_LBRACE,
  [anon_sym_RBRACE] = anon_sym_RBRACE,
  [anon_sym_COLON] = anon_sym_COLON,
  [anon_sym_COMMA] = anon_sym_COMMA,
  [anon_sym_LPAREN] = anon_sym_LPAREN,
  [anon_sym_RPAREN] = anon_sym_RPAREN,
  [sym_operation] = sym_operation,
  [sym_direction] = sym_direction,
  [sym_comment] = sym_comment,
  [sym_source_file] = sym_source_file,
  [sym__definition] = sym__definition,
  [sym_completion] = sym_completion,
  [sym_string_literal] = sym_string_literal,
  [sym__text_ident] = sym__text_ident,
  [sym_index_parser] = sym_index_parser,
  [sym__vb_identifier] = sym__vb_identifier,
  [sym_slide_from] = sym_slide_from,
  [sym_slide_obj] = sym_slide_obj,
  [sym_slide_objects] = sym_slide_objects,
  [sym_slide_function] = sym_slide_function,
  [sym_slide_functions] = sym_slide_functions,
  [sym_slide] = sym_slide,
  [sym_viewbox_obj] = sym_viewbox_obj,
  [sym_viewbox_inner] = sym_viewbox_inner,
  [sym_viewbox] = sym_viewbox,
  [sym_obj_inner] = sym_obj_inner,
  [sym_obj] = sym_obj,
  [sym_action_obj] = sym_action_obj,
  [sym_action] = sym_action,
  [sym_register] = sym_register,
  [aux_sym_source_file_repeat1] = aux_sym_source_file_repeat1,
  [aux_sym_string_literal_repeat1] = aux_sym_string_literal_repeat1,
  [aux_sym_slide_objects_repeat1] = aux_sym_slide_objects_repeat1,
  [aux_sym_slide_function_repeat1] = aux_sym_slide_function_repeat1,
  [aux_sym_slide_functions_repeat1] = aux_sym_slide_functions_repeat1,
  [aux_sym_viewbox_inner_repeat1] = aux_sym_viewbox_inner_repeat1,
  [aux_sym_obj_inner_repeat1] = aux_sym_obj_inner_repeat1,
  [aux_sym_action_obj_repeat1] = aux_sym_action_obj_repeat1,
  [aux_sym_action_repeat1] = aux_sym_action_repeat1,
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
  [anon_sym_L_DQUOTE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_u_DQUOTE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_U_DQUOTE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_u8_DQUOTE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DQUOTE] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_string_literal_token1] = {
    .visible = true,
    .named = true,
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
  [sym_comment] = {
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
  [sym_obj_inner] = {
    .visible = true,
    .named = true,
  },
  [sym_obj] = {
    .visible = true,
    .named = true,
  },
  [sym_action_obj] = {
    .visible = true,
    .named = true,
  },
  [sym_action] = {
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
  [aux_sym_string_literal_repeat1] = {
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
  [aux_sym_action_obj_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_action_repeat1] = {
    .visible = false,
    .named = false,
  },
};

enum {
  field_attached_box = 1,
  field_body = 2,
  field_denominator = 3,
  field_direction = 4,
  field_function = 5,
  field_name = 6,
  field_object = 7,
  field_objects = 8,
  field_operation = 9,
  field_parameters = 10,
  field_ty = 11,
  field_value = 12,
  field_viewbox_index = 13,
};

static const char * const ts_field_names[] = {
  [0] = NULL,
  [field_attached_box] = "attached_box",
  [field_body] = "body",
  [field_denominator] = "denominator",
  [field_direction] = "direction",
  [field_function] = "function",
  [field_name] = "name",
  [field_object] = "object",
  [field_objects] = "objects",
  [field_operation] = "operation",
  [field_parameters] = "parameters",
  [field_ty] = "ty",
  [field_value] = "value",
  [field_viewbox_index] = "viewbox_index",
};

static const TSFieldMapSlice ts_field_map_slices[PRODUCTION_ID_COUNT] = {
  [1] = {.index = 0, .length = 1},
  [2] = {.index = 1, .length = 1},
  [3] = {.index = 2, .length = 2},
  [4] = {.index = 4, .length = 2},
  [5] = {.index = 6, .length = 3},
  [6] = {.index = 9, .length = 2},
  [7] = {.index = 11, .length = 1},
  [8] = {.index = 12, .length = 1},
  [9] = {.index = 13, .length = 1},
  [10] = {.index = 14, .length = 2},
  [11] = {.index = 16, .length = 2},
  [12] = {.index = 18, .length = 3},
  [13] = {.index = 21, .length = 3},
  [14] = {.index = 24, .length = 3},
  [15] = {.index = 27, .length = 4},
};

static const TSFieldMapEntry ts_field_map_entries[] = {
  [0] =
    {field_name, 0},
  [1] =
    {field_objects, 1},
  [2] =
    {field_name, 0},
    {field_ty, 2},
  [4] =
    {field_objects, 1},
    {field_objects, 2},
  [6] =
    {field_attached_box, 3},
    {field_body, 4},
    {field_name, 0},
  [9] =
    {field_object, 0},
    {field_viewbox_index, 3},
  [11] =
    {field_function, 0},
  [12] =
    {field_direction, 0},
  [13] =
    {field_function, 2},
  [14] =
    {field_operation, 1},
    {field_value, 0},
  [16] =
    {field_direction, 0},
    {field_parameters, 1},
  [18] =
    {field_parameters, 1},
    {field_parameters, 2},
    {field_parameters, 3},
  [21] =
    {field_denominator, 2},
    {field_operation, 1},
    {field_value, 0},
  [24] =
    {field_direction, 0},
    {field_parameters, 1},
    {field_parameters, 2},
  [27] =
    {field_parameters, 1},
    {field_parameters, 2},
    {field_parameters, 3},
    {field_parameters, 4},
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
  [35] = 35,
  [36] = 36,
  [37] = 37,
  [38] = 24,
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
  [50] = 42,
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
  [87] = 87,
  [88] = 88,
  [89] = 89,
  [90] = 90,
  [91] = 91,
  [92] = 28,
  [93] = 41,
  [94] = 94,
  [95] = 95,
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
  [107] = 34,
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
  [134] = 134,
  [135] = 135,
  [136] = 136,
  [137] = 137,
  [138] = 138,
  [139] = 139,
  [140] = 140,
  [141] = 141,
  [142] = 142,
  [143] = 143,
  [144] = 144,
  [145] = 145,
  [146] = 146,
  [147] = 147,
  [148] = 148,
  [149] = 149,
  [150] = 127,
  [151] = 123,
};

static inline bool sym_identifier_character_set_1(int32_t c) {
  return (c < 43494
    ? (c < 4186
      ? (c < 2703
        ? (c < 1969
          ? (c < 908
            ? (c < 710
              ? (c < 181
                ? (c < 'a'
                  ? (c < '_'
                    ? (c >= 'A' && c <= 'Z')
                    : c <= '_')
                  : (c <= 't' || (c < 170
                    ? (c >= 'v' && c <= 'z')
                    : c <= 170)))
                : (c <= 181 || (c < 216
                  ? (c < 192
                    ? c == 186
                    : c <= 214)
                  : (c <= 246 || (c >= 248 && c <= 705)))))
              : (c <= 721 || (c < 886
                ? (c < 750
                  ? (c < 748
                    ? (c >= 736 && c <= 740)
                    : c <= 748)
                  : (c <= 750 || (c >= 880 && c <= 884)))
                : (c <= 887 || (c < 902
                  ? (c < 895
                    ? (c >= 891 && c <= 893)
                    : c <= 895)
                  : (c <= 902 || (c >= 904 && c <= 906)))))))
            : (c <= 908 || (c < 1646
              ? (c < 1369
                ? (c < 1015
                  ? (c < 931
                    ? (c >= 910 && c <= 929)
                    : c <= 1013)
                  : (c <= 1153 || (c < 1329
                    ? (c >= 1162 && c <= 1327)
                    : c <= 1366)))
                : (c <= 1369 || (c < 1519
                  ? (c < 1488
                    ? (c >= 1376 && c <= 1416)
                    : c <= 1514)
                  : (c <= 1522 || (c >= 1568 && c <= 1610)))))
              : (c <= 1647 || (c < 1786
                ? (c < 1765
                  ? (c < 1749
                    ? (c >= 1649 && c <= 1747)
                    : c <= 1749)
                  : (c <= 1766 || (c >= 1774 && c <= 1775)))
                : (c <= 1788 || (c < 1810
                  ? (c < 1808
                    ? c == 1791
                    : c <= 1808)
                  : (c <= 1839 || (c >= 1869 && c <= 1957)))))))))
          : (c <= 1969 || (c < 2474
            ? (c < 2185
              ? (c < 2084
                ? (c < 2042
                  ? (c < 2036
                    ? (c >= 1994 && c <= 2026)
                    : c <= 2037)
                  : (c <= 2042 || (c < 2074
                    ? (c >= 2048 && c <= 2069)
                    : c <= 2074)))
                : (c <= 2084 || (c < 2144
                  ? (c < 2112
                    ? c == 2088
                    : c <= 2136)
                  : (c <= 2154 || (c >= 2160 && c <= 2183)))))
              : (c <= 2190 || (c < 2392
                ? (c < 2365
                  ? (c < 2308
                    ? (c >= 2208 && c <= 2249)
                    : c <= 2361)
                  : (c <= 2365 || c == 2384))
                : (c <= 2401 || (c < 2447
                  ? (c < 2437
                    ? (c >= 2417 && c <= 2432)
                    : c <= 2444)
                  : (c <= 2448 || (c >= 2451 && c <= 2472)))))))
            : (c <= 2480 || (c < 2575
              ? (c < 2524
                ? (c < 2493
                  ? (c < 2486
                    ? c == 2482
                    : c <= 2489)
                  : (c <= 2493 || c == 2510))
                : (c <= 2525 || (c < 2556
                  ? (c < 2544
                    ? (c >= 2527 && c <= 2529)
                    : c <= 2545)
                  : (c <= 2556 || (c >= 2565 && c <= 2570)))))
              : (c <= 2576 || (c < 2616
                ? (c < 2610
                  ? (c < 2602
                    ? (c >= 2579 && c <= 2600)
                    : c <= 2608)
                  : (c <= 2611 || (c >= 2613 && c <= 2614)))
                : (c <= 2617 || (c < 2674
                  ? (c < 2654
                    ? (c >= 2649 && c <= 2652)
                    : c <= 2654)
                  : (c <= 2676 || (c >= 2693 && c <= 2701)))))))))))
        : (c <= 2705 || (c < 3218
          ? (c < 2958
            ? (c < 2835
              ? (c < 2768
                ? (c < 2738
                  ? (c < 2730
                    ? (c >= 2707 && c <= 2728)
                    : c <= 2736)
                  : (c <= 2739 || (c < 2749
                    ? (c >= 2741 && c <= 2745)
                    : c <= 2749)))
                : (c <= 2768 || (c < 2821
                  ? (c < 2809
                    ? (c >= 2784 && c <= 2785)
                    : c <= 2809)
                  : (c <= 2828 || (c >= 2831 && c <= 2832)))))
              : (c <= 2856 || (c < 2908
                ? (c < 2869
                  ? (c < 2866
                    ? (c >= 2858 && c <= 2864)
                    : c <= 2867)
                  : (c <= 2873 || c == 2877))
                : (c <= 2909 || (c < 2947
                  ? (c < 2929
                    ? (c >= 2911 && c <= 2913)
                    : c <= 2929)
                  : (c <= 2947 || (c >= 2949 && c <= 2954)))))))
            : (c <= 2960 || (c < 3086
              ? (c < 2979
                ? (c < 2972
                  ? (c < 2969
                    ? (c >= 2962 && c <= 2965)
                    : c <= 2970)
                  : (c <= 2972 || (c >= 2974 && c <= 2975)))
                : (c <= 2980 || (c < 3024
                  ? (c < 2990
                    ? (c >= 2984 && c <= 2986)
                    : c <= 3001)
                  : (c <= 3024 || (c >= 3077 && c <= 3084)))))
              : (c <= 3088 || (c < 3165
                ? (c < 3133
                  ? (c < 3114
                    ? (c >= 3090 && c <= 3112)
                    : c <= 3129)
                  : (c <= 3133 || (c >= 3160 && c <= 3162)))
                : (c <= 3165 || (c < 3205
                  ? (c < 3200
                    ? (c >= 3168 && c <= 3169)
                    : c <= 3200)
                  : (c <= 3212 || (c >= 3214 && c <= 3216)))))))))
          : (c <= 3240 || (c < 3634
            ? (c < 3406
              ? (c < 3313
                ? (c < 3261
                  ? (c < 3253
                    ? (c >= 3242 && c <= 3251)
                    : c <= 3257)
                  : (c <= 3261 || (c < 3296
                    ? (c >= 3293 && c <= 3294)
                    : c <= 3297)))
                : (c <= 3314 || (c < 3346
                  ? (c < 3342
                    ? (c >= 3332 && c <= 3340)
                    : c <= 3344)
                  : (c <= 3386 || c == 3389))))
              : (c <= 3406 || (c < 3482
                ? (c < 3450
                  ? (c < 3423
                    ? (c >= 3412 && c <= 3414)
                    : c <= 3425)
                  : (c <= 3455 || (c >= 3461 && c <= 3478)))
                : (c <= 3505 || (c < 3520
                  ? (c < 3517
                    ? (c >= 3507 && c <= 3515)
                    : c <= 3517)
                  : (c <= 3526 || (c >= 3585 && c <= 3632)))))))
            : (c <= 3634 || (c < 3776
              ? (c < 3724
                ? (c < 3716
                  ? (c < 3713
                    ? (c >= 3648 && c <= 3654)
                    : c <= 3714)
                  : (c <= 3716 || (c >= 3718 && c <= 3722)))
                : (c <= 3747 || (c < 3762
                  ? (c < 3751
                    ? c == 3749
                    : c <= 3760)
                  : (c <= 3762 || c == 3773))))
              : (c <= 3780 || (c < 3913
                ? (c < 3840
                  ? (c < 3804
                    ? c == 3782
                    : c <= 3807)
                  : (c <= 3840 || (c >= 3904 && c <= 3911)))
                : (c <= 3948 || (c < 4159
                  ? (c < 4096
                    ? (c >= 3976 && c <= 3980)
                    : c <= 4138)
                  : (c <= 4159 || (c >= 4176 && c <= 4181)))))))))))))
      : (c <= 4189 || (c < 8130
        ? (c < 6108
          ? (c < 4802
            ? (c < 4682
              ? (c < 4256
                ? (c < 4206
                  ? (c < 4197
                    ? c == 4193
                    : c <= 4198)
                  : (c <= 4208 || (c < 4238
                    ? (c >= 4213 && c <= 4225)
                    : c <= 4238)))
                : (c <= 4293 || (c < 4304
                  ? (c < 4301
                    ? c == 4295
                    : c <= 4301)
                  : (c <= 4346 || (c >= 4348 && c <= 4680)))))
              : (c <= 4685 || (c < 4746
                ? (c < 4698
                  ? (c < 4696
                    ? (c >= 4688 && c <= 4694)
                    : c <= 4696)
                  : (c <= 4701 || (c >= 4704 && c <= 4744)))
                : (c <= 4749 || (c < 4792
                  ? (c < 4786
                    ? (c >= 4752 && c <= 4784)
                    : c <= 4789)
                  : (c <= 4798 || c == 4800))))))
            : (c <= 4805 || (c < 5761
              ? (c < 4992
                ? (c < 4882
                  ? (c < 4824
                    ? (c >= 4808 && c <= 4822)
                    : c <= 4880)
                  : (c <= 4885 || (c >= 4888 && c <= 4954)))
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

static inline bool sym_identifier_character_set_2(int32_t c) {
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

static inline bool aux_sym_string_literal_token1_character_set_1(int32_t c) {
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

static inline bool aux_sym_string_literal_token1_character_set_2(int32_t c) {
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

static inline bool aux_sym_string_literal_token1_character_set_3(int32_t c) {
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
      if (eof) ADVANCE(40);
      if (lookahead == '"') ADVANCE(60);
      if (lookahead == '(') ADVANCE(93);
      if (lookahead == ')') ADVANCE(93);
      if (lookahead == ',') ADVANCE(93);
      if (lookahead == '.') ADVANCE(86);
      if (lookahead == '/') ADVANCE(74);
      if (lookahead == '0') ADVANCE(66);
      if (lookahead == ':') ADVANCE(93);
      if (lookahead == 'L') ADVANCE(92);
      if (lookahead == 'U') ADVANCE(92);
      if (lookahead == '[') ADVANCE(93);
      if (lookahead == '\\') ADVANCE(41);
      if (lookahead == ']') ADVANCE(93);
      if (lookahead == '_') ADVANCE(92);
      if (lookahead == 'u') ADVANCE(80);
      if (lookahead == '{') ADVANCE(93);
      if (lookahead == '}') ADVANCE(93);
      if (lookahead == '%' ||
          lookahead == '~') ADVANCE(93);
      if (('+' <= lookahead && lookahead <= '-')) ADVANCE(77);
      if (lookahead == '<' ||
          lookahead == '>' ||
          lookahead == '^') ADVANCE(93);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ' ||
          lookahead == '?' ||
          lookahead == '|') ADVANCE(83);
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(68);
      if (aux_sym_string_literal_token1_character_set_1(lookahead)) ADVANCE(92);
      if (lookahead != 0) ADVANCE(93);
      END_STATE();
    case 1:
      if (lookahead == '"') ADVANCE(60);
      if (lookahead == '/') ADVANCE(74);
      if (lookahead == '\\') ADVANCE(41);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ' ||
          lookahead == '?' ||
          lookahead == '|') ADVANCE(83);
      if (lookahead != 0) ADVANCE(93);
      END_STATE();
    case 2:
      if (lookahead == '"') ADVANCE(56);
      END_STATE();
    case 3:
      if (lookahead == '"') ADVANCE(58);
      END_STATE();
    case 4:
      if (lookahead == '"') ADVANCE(57);
      if (lookahead == '8') ADVANCE(5);
      END_STATE();
    case 5:
      if (lookahead == '"') ADVANCE(59);
      END_STATE();
    case 6:
      if (lookahead == '*') ADVANCE(8);
      if (lookahead == '/') ADVANCE(125);
      END_STATE();
    case 7:
      if (lookahead == '*') ADVANCE(7);
      if (lookahead == '/') ADVANCE(124);
      if (lookahead != 0) ADVANCE(8);
      END_STATE();
    case 8:
      if (lookahead == '*') ADVANCE(7);
      if (lookahead != 0) ADVANCE(8);
      END_STATE();
    case 9:
      if (lookahead == ',') ADVANCE(119);
      if (lookahead == '/') ADVANCE(6);
      if (lookahead == '{') ADVANCE(116);
      if (lookahead == '|') ADVANCE(45);
      if (lookahead == '}') ADVANCE(117);
      if (lookahead == '.' ||
          lookahead == '<' ||
          lookahead == '>' ||
          lookahead == '^' ||
          lookahead == '_') ADVANCE(14);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ' ||
          lookahead == '?' ||
          lookahead == '\\') ADVANCE(46);
      END_STATE();
    case 10:
      if (lookahead == '.') ADVANCE(16);
      if (lookahead == '0') ADVANCE(97);
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(99);
      END_STATE();
    case 11:
      if (lookahead == '.') ADVANCE(16);
      if (lookahead == '0') ADVANCE(95);
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(96);
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(105);
      END_STATE();
    case 12:
      if (lookahead == '.') ADVANCE(47);
      if (lookahead == '/') ADVANCE(6);
      if (lookahead == ':') ADVANCE(118);
      if (lookahead == '%' ||
          lookahead == '+' ||
          lookahead == '-' ||
          lookahead == '~') ADVANCE(122);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ' ||
          lookahead == '?' ||
          lookahead == '\\' ||
          lookahead == '|') ADVANCE(46);
      END_STATE();
    case 13:
      if (lookahead == 'U') ADVANCE(36);
      if (lookahead == 'u') ADVANCE(28);
      END_STATE();
    case 14:
      if (lookahead == '.' ||
          lookahead == '<' ||
          lookahead == '>' ||
          lookahead == '^' ||
          lookahead == '_' ||
          lookahead == '|') ADVANCE(115);
      END_STATE();
    case 15:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(99);
      END_STATE();
    case 16:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(94);
      END_STATE();
    case 17:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(96);
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(105);
      END_STATE();
    case 18:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(53);
      END_STATE();
    case 19:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(52);
      END_STATE();
    case 20:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(102);
      END_STATE();
    case 21:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(105);
      END_STATE();
    case 22:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(100);
      END_STATE();
    case 23:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(18);
      END_STATE();
    case 24:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(19);
      END_STATE();
    case 25:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(23);
      END_STATE();
    case 26:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(24);
      END_STATE();
    case 27:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(25);
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
      if (lookahead != 0 &&
          lookahead != '\r' &&
          lookahead != '\\') ADVANCE(125);
      if (lookahead == '\r') ADVANCE(127);
      if (lookahead == '\\') ADVANCE(126);
      END_STATE();
    case 38:
      if (eof) ADVANCE(40);
      if (lookahead == '"') ADVANCE(60);
      if (lookahead == '(') ADVANCE(120);
      if (lookahead == ')') ADVANCE(121);
      if (lookahead == ',') ADVANCE(119);
      if (lookahead == '.') ADVANCE(16);
      if (lookahead == '/') ADVANCE(6);
      if (lookahead == '0') ADVANCE(97);
      if (lookahead == ':') ADVANCE(118);
      if (lookahead == 'L') ADVANCE(2);
      if (lookahead == 'U') ADVANCE(3);
      if (lookahead == '[') ADVANCE(111);
      if (lookahead == ']') ADVANCE(112);
      if (lookahead == 'u') ADVANCE(4);
      if (lookahead == '}') ADVANCE(117);
      if (('+' <= lookahead && lookahead <= '-')) ADVANCE(10);
      if (lookahead == '<' ||
          lookahead == '>' ||
          lookahead == '^' ||
          lookahead == '_') ADVANCE(123);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ' ||
          lookahead == '?' ||
          lookahead == '\\' ||
          lookahead == '|') ADVANCE(46);
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(99);
      END_STATE();
    case 39:
      if (eof) ADVANCE(40);
      if (lookahead == '"') ADVANCE(60);
      if (lookahead == ')') ADVANCE(121);
      if (lookahead == ',') ADVANCE(119);
      if (lookahead == '.') ADVANCE(16);
      if (lookahead == '/') ADVANCE(6);
      if (lookahead == '0') ADVANCE(97);
      if (lookahead == 'L') ADVANCE(48);
      if (lookahead == 'U') ADVANCE(49);
      if (lookahead == '[') ADVANCE(111);
      if (lookahead == '\\') ADVANCE(42);
      if (lookahead == ']') ADVANCE(112);
      if (sym_identifier_character_set_1(lookahead)) ADVANCE(52);
      if (lookahead == 'u') ADVANCE(50);
      if (lookahead == '{') ADVANCE(116);
      if (lookahead == '}') ADVANCE(117);
      if (('+' <= lookahead && lookahead <= '-')) ADVANCE(10);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ' ||
          lookahead == '?' ||
          lookahead == '|') ADVANCE(46);
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(99);
      END_STATE();
    case 40:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 41:
      ACCEPT_TOKEN(sym_whitespace);
      if (lookahead == 'U') ADVANCE(35);
      if (lookahead == 'u') ADVANCE(27);
      if (lookahead == 'x') ADVANCE(23);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ' ||
          lookahead == '?' ||
          lookahead == '\\' ||
          lookahead == '|') ADVANCE(53);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(55);
      if (lookahead != 0) ADVANCE(53);
      END_STATE();
    case 42:
      ACCEPT_TOKEN(sym_whitespace);
      if (lookahead == 'U') ADVANCE(36);
      if (lookahead == 'u') ADVANCE(28);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ' ||
          lookahead == '?' ||
          lookahead == '\\' ||
          lookahead == '|') ADVANCE(46);
      END_STATE();
    case 43:
      ACCEPT_TOKEN(sym_whitespace);
      if (lookahead == '|') ADVANCE(46);
      if (lookahead == '.' ||
          lookahead == '<' ||
          lookahead == '>' ||
          lookahead == '^' ||
          lookahead == '_') ADVANCE(113);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ' ||
          lookahead == '?' ||
          lookahead == '\\') ADVANCE(46);
      END_STATE();
    case 44:
      ACCEPT_TOKEN(sym_whitespace);
      if (lookahead == '|') ADVANCE(43);
      if (lookahead == '.' ||
          lookahead == '<' ||
          lookahead == '>' ||
          lookahead == '^' ||
          lookahead == '_') ADVANCE(114);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ' ||
          lookahead == '?' ||
          lookahead == '\\') ADVANCE(46);
      END_STATE();
    case 45:
      ACCEPT_TOKEN(sym_whitespace);
      if (lookahead == '|') ADVANCE(44);
      if (lookahead == '.' ||
          lookahead == '<' ||
          lookahead == '>' ||
          lookahead == '^' ||
          lookahead == '_') ADVANCE(115);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ' ||
          lookahead == '?' ||
          lookahead == '\\') ADVANCE(46);
      END_STATE();
    case 46:
      ACCEPT_TOKEN(sym_whitespace);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ' ||
          lookahead == '?' ||
          lookahead == '\\' ||
          lookahead == '|') ADVANCE(46);
      END_STATE();
    case 47:
      ACCEPT_TOKEN(anon_sym_DOT);
      END_STATE();
    case 48:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == '"') ADVANCE(56);
      if (lookahead == '\\') ADVANCE(13);
      if (sym_identifier_character_set_2(lookahead)) ADVANCE(52);
      END_STATE();
    case 49:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == '"') ADVANCE(58);
      if (lookahead == '\\') ADVANCE(13);
      if (sym_identifier_character_set_2(lookahead)) ADVANCE(52);
      END_STATE();
    case 50:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == '"') ADVANCE(57);
      if (lookahead == '8') ADVANCE(51);
      if (lookahead == '\\') ADVANCE(13);
      if (sym_identifier_character_set_2(lookahead)) ADVANCE(52);
      END_STATE();
    case 51:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == '"') ADVANCE(59);
      if (lookahead == '\\') ADVANCE(13);
      if (sym_identifier_character_set_2(lookahead)) ADVANCE(52);
      END_STATE();
    case 52:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == '\\') ADVANCE(13);
      if (sym_identifier_character_set_2(lookahead)) ADVANCE(52);
      END_STATE();
    case 53:
      ACCEPT_TOKEN(sym_escape_sequence);
      END_STATE();
    case 54:
      ACCEPT_TOKEN(sym_escape_sequence);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(53);
      END_STATE();
    case 55:
      ACCEPT_TOKEN(sym_escape_sequence);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(54);
      END_STATE();
    case 56:
      ACCEPT_TOKEN(anon_sym_L_DQUOTE);
      END_STATE();
    case 57:
      ACCEPT_TOKEN(anon_sym_u_DQUOTE);
      END_STATE();
    case 58:
      ACCEPT_TOKEN(anon_sym_U_DQUOTE);
      END_STATE();
    case 59:
      ACCEPT_TOKEN(anon_sym_u8_DQUOTE);
      END_STATE();
    case 60:
      ACCEPT_TOKEN(anon_sym_DQUOTE);
      END_STATE();
    case 61:
      ACCEPT_TOKEN(aux_sym_string_literal_token1);
      if (lookahead == '\n') ADVANCE(93);
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '\\') ADVANCE(61);
      END_STATE();
    case 62:
      ACCEPT_TOKEN(aux_sym_string_literal_token1);
      if (lookahead == '\'') ADVANCE(86);
      if (lookahead == 'E' ||
          lookahead == 'P' ||
          lookahead == 'e' ||
          lookahead == 'p') ADVANCE(81);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(62);
      if (aux_sym_string_literal_token1_character_set_2(lookahead)) ADVANCE(88);
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '\\') ADVANCE(93);
      END_STATE();
    case 63:
      ACCEPT_TOKEN(aux_sym_string_literal_token1);
      if (lookahead == '\'') ADVANCE(91);
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
          lookahead == 'w') ADVANCE(88);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'E') ||
          ('a' <= lookahead && lookahead <= 'e')) ADVANCE(63);
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '\\') ADVANCE(93);
      END_STATE();
    case 64:
      ACCEPT_TOKEN(aux_sym_string_literal_token1);
      if (lookahead == '\'') ADVANCE(87);
      if (lookahead == '.') ADVANCE(82);
      if (lookahead == 'B' ||
          lookahead == 'b') ADVANCE(72);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(71);
      if (lookahead == 'P' ||
          lookahead == 'p') ADVANCE(81);
      if (lookahead == 'X' ||
          lookahead == 'x') ADVANCE(90);
      if (('A' <= lookahead && lookahead <= 'C') ||
          ('a' <= lookahead && lookahead <= 'c')) ADVANCE(73);
      if (('D' <= lookahead && lookahead <= 'F') ||
          ('d' <= lookahead && lookahead <= 'f')) ADVANCE(73);
      if (lookahead == 'L' ||
          lookahead == 'U' ||
          lookahead == 'W' ||
          lookahead == 'l' ||
          lookahead == 'u' ||
          lookahead == 'w') ADVANCE(88);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(65);
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '\\') ADVANCE(93);
      END_STATE();
    case 65:
      ACCEPT_TOKEN(aux_sym_string_literal_token1);
      if (lookahead == '\'') ADVANCE(87);
      if (lookahead == '.') ADVANCE(82);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(71);
      if (lookahead == 'P' ||
          lookahead == 'p') ADVANCE(81);
      if (lookahead == 'A' ||
          lookahead == 'C' ||
          lookahead == 'a' ||
          lookahead == 'c') ADVANCE(73);
      if (('B' <= lookahead && lookahead <= 'F') ||
          ('b' <= lookahead && lookahead <= 'f')) ADVANCE(73);
      if (lookahead == 'L' ||
          lookahead == 'U' ||
          lookahead == 'W' ||
          lookahead == 'l' ||
          lookahead == 'u' ||
          lookahead == 'w') ADVANCE(88);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(65);
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '\\') ADVANCE(93);
      END_STATE();
    case 66:
      ACCEPT_TOKEN(aux_sym_string_literal_token1);
      if (lookahead == '\'') ADVANCE(85);
      if (lookahead == '.') ADVANCE(82);
      if (lookahead == 'B' ||
          lookahead == 'b') ADVANCE(79);
      if (lookahead == 'X' ||
          lookahead == 'x') ADVANCE(78);
      if (lookahead == 'E' ||
          lookahead == 'P' ||
          lookahead == 'e' ||
          lookahead == 'p') ADVANCE(81);
      if (('D' <= lookahead && lookahead <= 'F') ||
          lookahead == 'L' ||
          lookahead == 'U' ||
          lookahead == 'W' ||
          ('d' <= lookahead && lookahead <= 'f') ||
          lookahead == 'l' ||
          lookahead == 'u' ||
          lookahead == 'w') ADVANCE(88);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(68);
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '\\') ADVANCE(93);
      END_STATE();
    case 67:
      ACCEPT_TOKEN(aux_sym_string_literal_token1);
      if (lookahead == '\'') ADVANCE(85);
      if (lookahead == '.') ADVANCE(82);
      if (lookahead == 'B' ||
          lookahead == 'b') ADVANCE(84);
      if (lookahead == 'X' ||
          lookahead == 'x') ADVANCE(90);
      if (lookahead == 'E' ||
          lookahead == 'P' ||
          lookahead == 'e' ||
          lookahead == 'p') ADVANCE(81);
      if (('D' <= lookahead && lookahead <= 'F') ||
          lookahead == 'L' ||
          lookahead == 'U' ||
          lookahead == 'W' ||
          ('d' <= lookahead && lookahead <= 'f') ||
          lookahead == 'l' ||
          lookahead == 'u' ||
          lookahead == 'w') ADVANCE(88);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(68);
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '\\') ADVANCE(93);
      END_STATE();
    case 68:
      ACCEPT_TOKEN(aux_sym_string_literal_token1);
      if (lookahead == '\'') ADVANCE(85);
      if (lookahead == '.') ADVANCE(82);
      if (lookahead == 'E' ||
          lookahead == 'P' ||
          lookahead == 'e' ||
          lookahead == 'p') ADVANCE(81);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(68);
      if (aux_sym_string_literal_token1_character_set_2(lookahead)) ADVANCE(88);
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '\\') ADVANCE(93);
      END_STATE();
    case 69:
      ACCEPT_TOKEN(aux_sym_string_literal_token1);
      if (lookahead == '\'') ADVANCE(89);
      if (lookahead == '+' ||
          lookahead == '-') ADVANCE(91);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(69);
      if (lookahead == 'P' ||
          lookahead == 'p') ADVANCE(81);
      if (lookahead == 'B' ||
          ('D' <= lookahead && lookahead <= 'F') ||
          lookahead == 'b' ||
          ('d' <= lookahead && lookahead <= 'f')) ADVANCE(70);
      if (lookahead == 'L' ||
          lookahead == 'U' ||
          lookahead == 'W' ||
          lookahead == 'l' ||
          lookahead == 'u' ||
          lookahead == 'w') ADVANCE(88);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'C') ||
          ('a' <= lookahead && lookahead <= 'c')) ADVANCE(70);
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '\\') ADVANCE(93);
      END_STATE();
    case 70:
      ACCEPT_TOKEN(aux_sym_string_literal_token1);
      if (lookahead == '\'') ADVANCE(89);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(69);
      if (lookahead == 'P' ||
          lookahead == 'p') ADVANCE(81);
      if (lookahead == 'B' ||
          ('D' <= lookahead && lookahead <= 'F') ||
          lookahead == 'b' ||
          ('d' <= lookahead && lookahead <= 'f')) ADVANCE(70);
      if (lookahead == 'L' ||
          lookahead == 'U' ||
          lookahead == 'W' ||
          lookahead == 'l' ||
          lookahead == 'u' ||
          lookahead == 'w') ADVANCE(88);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'C') ||
          ('a' <= lookahead && lookahead <= 'c')) ADVANCE(70);
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '\\') ADVANCE(93);
      END_STATE();
    case 71:
      ACCEPT_TOKEN(aux_sym_string_literal_token1);
      if (lookahead == '\'') ADVANCE(90);
      if (lookahead == '.') ADVANCE(82);
      if (lookahead == '+' ||
          lookahead == '-') ADVANCE(91);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(71);
      if (lookahead == 'P' ||
          lookahead == 'p') ADVANCE(81);
      if (lookahead == 'B' ||
          ('D' <= lookahead && lookahead <= 'F') ||
          lookahead == 'b' ||
          ('d' <= lookahead && lookahead <= 'f')) ADVANCE(73);
      if (lookahead == 'L' ||
          lookahead == 'U' ||
          lookahead == 'W' ||
          lookahead == 'l' ||
          lookahead == 'u' ||
          lookahead == 'w') ADVANCE(88);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'C') ||
          ('a' <= lookahead && lookahead <= 'c')) ADVANCE(73);
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '\\') ADVANCE(93);
      END_STATE();
    case 72:
      ACCEPT_TOKEN(aux_sym_string_literal_token1);
      if (lookahead == '\'') ADVANCE(90);
      if (lookahead == '.') ADVANCE(82);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(71);
      if (lookahead == 'P' ||
          lookahead == 'p') ADVANCE(81);
      if (lookahead == 'A' ||
          lookahead == 'C' ||
          lookahead == 'a' ||
          lookahead == 'c') ADVANCE(73);
      if (('B' <= lookahead && lookahead <= 'F') ||
          ('b' <= lookahead && lookahead <= 'f')) ADVANCE(73);
      if (lookahead == 'L' ||
          lookahead == 'U' ||
          lookahead == 'W' ||
          lookahead == 'l' ||
          lookahead == 'u' ||
          lookahead == 'w') ADVANCE(88);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(65);
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '\\') ADVANCE(93);
      END_STATE();
    case 73:
      ACCEPT_TOKEN(aux_sym_string_literal_token1);
      if (lookahead == '\'') ADVANCE(90);
      if (lookahead == '.') ADVANCE(82);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(71);
      if (lookahead == 'P' ||
          lookahead == 'p') ADVANCE(81);
      if (lookahead == 'B' ||
          ('D' <= lookahead && lookahead <= 'F') ||
          lookahead == 'b' ||
          ('d' <= lookahead && lookahead <= 'f')) ADVANCE(73);
      if (lookahead == 'L' ||
          lookahead == 'U' ||
          lookahead == 'W' ||
          lookahead == 'l' ||
          lookahead == 'u' ||
          lookahead == 'w') ADVANCE(88);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'C') ||
          ('a' <= lookahead && lookahead <= 'c')) ADVANCE(73);
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '\\') ADVANCE(93);
      END_STATE();
    case 74:
      ACCEPT_TOKEN(aux_sym_string_literal_token1);
      if (lookahead == '*') ADVANCE(76);
      if (lookahead == '/') ADVANCE(61);
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '\\') ADVANCE(93);
      END_STATE();
    case 75:
      ACCEPT_TOKEN(aux_sym_string_literal_token1);
      if (lookahead == '*') ADVANCE(75);
      if (lookahead == '/') ADVANCE(93);
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '\\') ADVANCE(76);
      END_STATE();
    case 76:
      ACCEPT_TOKEN(aux_sym_string_literal_token1);
      if (lookahead == '*') ADVANCE(75);
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '\\') ADVANCE(76);
      END_STATE();
    case 77:
      ACCEPT_TOKEN(aux_sym_string_literal_token1);
      if (lookahead == '.') ADVANCE(86);
      if (lookahead == '0') ADVANCE(66);
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(68);
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '\\') ADVANCE(93);
      END_STATE();
    case 78:
      ACCEPT_TOKEN(aux_sym_string_literal_token1);
      if (lookahead == '.') ADVANCE(86);
      if (lookahead == '0') ADVANCE(64);
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(65);
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(73);
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '\\') ADVANCE(93);
      END_STATE();
    case 79:
      ACCEPT_TOKEN(aux_sym_string_literal_token1);
      if (lookahead == '.') ADVANCE(86);
      if (lookahead == '0') ADVANCE(67);
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(68);
      if (aux_sym_string_literal_token1_character_set_3(lookahead)) ADVANCE(88);
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '\\') ADVANCE(93);
      END_STATE();
    case 80:
      ACCEPT_TOKEN(aux_sym_string_literal_token1);
      if (lookahead == '8') ADVANCE(92);
      if (sym_identifier_character_set_2(lookahead)) ADVANCE(92);
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '\\') ADVANCE(93);
      END_STATE();
    case 81:
      ACCEPT_TOKEN(aux_sym_string_literal_token1);
      if (lookahead == '+' ||
          lookahead == '-') ADVANCE(91);
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
          lookahead == 'w') ADVANCE(88);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'E') ||
          ('a' <= lookahead && lookahead <= 'e')) ADVANCE(63);
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '\\') ADVANCE(93);
      END_STATE();
    case 82:
      ACCEPT_TOKEN(aux_sym_string_literal_token1);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(69);
      if (lookahead == 'P' ||
          lookahead == 'p') ADVANCE(81);
      if (lookahead == 'B' ||
          ('D' <= lookahead && lookahead <= 'F') ||
          lookahead == 'b' ||
          ('d' <= lookahead && lookahead <= 'f')) ADVANCE(70);
      if (lookahead == 'L' ||
          lookahead == 'U' ||
          lookahead == 'W' ||
          lookahead == 'l' ||
          lookahead == 'u' ||
          lookahead == 'w') ADVANCE(88);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'C') ||
          ('a' <= lookahead && lookahead <= 'c')) ADVANCE(70);
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '\\') ADVANCE(93);
      END_STATE();
    case 83:
      ACCEPT_TOKEN(aux_sym_string_literal_token1);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ' ||
          lookahead == '?' ||
          lookahead == '|') ADVANCE(83);
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '\\') ADVANCE(93);
      END_STATE();
    case 84:
      ACCEPT_TOKEN(aux_sym_string_literal_token1);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(68);
      if (aux_sym_string_literal_token1_character_set_3(lookahead)) ADVANCE(88);
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '\\') ADVANCE(93);
      END_STATE();
    case 85:
      ACCEPT_TOKEN(aux_sym_string_literal_token1);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(68);
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '\\') ADVANCE(93);
      END_STATE();
    case 86:
      ACCEPT_TOKEN(aux_sym_string_literal_token1);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(62);
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '\\') ADVANCE(93);
      END_STATE();
    case 87:
      ACCEPT_TOKEN(aux_sym_string_literal_token1);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(65);
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(73);
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '\\') ADVANCE(93);
      END_STATE();
    case 88:
      ACCEPT_TOKEN(aux_sym_string_literal_token1);
      if (aux_sym_string_literal_token1_character_set_3(lookahead)) ADVANCE(88);
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '\\') ADVANCE(93);
      END_STATE();
    case 89:
      ACCEPT_TOKEN(aux_sym_string_literal_token1);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(70);
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '\\') ADVANCE(93);
      END_STATE();
    case 90:
      ACCEPT_TOKEN(aux_sym_string_literal_token1);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(73);
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '\\') ADVANCE(93);
      END_STATE();
    case 91:
      ACCEPT_TOKEN(aux_sym_string_literal_token1);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(63);
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '\\') ADVANCE(93);
      END_STATE();
    case 92:
      ACCEPT_TOKEN(aux_sym_string_literal_token1);
      if (sym_identifier_character_set_2(lookahead)) ADVANCE(92);
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '\\') ADVANCE(93);
      END_STATE();
    case 93:
      ACCEPT_TOKEN(aux_sym_string_literal_token1);
      if (lookahead != 0 &&
          lookahead != '"' &&
          lookahead != '\\') ADVANCE(93);
      END_STATE();
    case 94:
      ACCEPT_TOKEN(sym_number_literal);
      if (lookahead == '\'') ADVANCE(16);
      if (lookahead == 'E' ||
          lookahead == 'P' ||
          lookahead == 'e' ||
          lookahead == 'p') ADVANCE(107);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(94);
      if (aux_sym_string_literal_token1_character_set_2(lookahead)) ADVANCE(110);
      END_STATE();
    case 95:
      ACCEPT_TOKEN(sym_number_literal);
      if (lookahead == '\'') ADVANCE(17);
      if (lookahead == '.') ADVANCE(108);
      if (lookahead == 'B' ||
          lookahead == 'b') ADVANCE(104);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(103);
      if (lookahead == 'P' ||
          lookahead == 'p') ADVANCE(107);
      if (lookahead == 'X' ||
          lookahead == 'x') ADVANCE(21);
      if (('A' <= lookahead && lookahead <= 'C') ||
          ('a' <= lookahead && lookahead <= 'c')) ADVANCE(105);
      if (('D' <= lookahead && lookahead <= 'F') ||
          ('d' <= lookahead && lookahead <= 'f')) ADVANCE(105);
      if (lookahead == 'L' ||
          lookahead == 'U' ||
          lookahead == 'W' ||
          lookahead == 'l' ||
          lookahead == 'u' ||
          lookahead == 'w') ADVANCE(110);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(96);
      END_STATE();
    case 96:
      ACCEPT_TOKEN(sym_number_literal);
      if (lookahead == '\'') ADVANCE(17);
      if (lookahead == '.') ADVANCE(108);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(103);
      if (lookahead == 'P' ||
          lookahead == 'p') ADVANCE(107);
      if (lookahead == 'A' ||
          lookahead == 'C' ||
          lookahead == 'a' ||
          lookahead == 'c') ADVANCE(105);
      if (('B' <= lookahead && lookahead <= 'F') ||
          ('b' <= lookahead && lookahead <= 'f')) ADVANCE(105);
      if (lookahead == 'L' ||
          lookahead == 'U' ||
          lookahead == 'W' ||
          lookahead == 'l' ||
          lookahead == 'u' ||
          lookahead == 'w') ADVANCE(110);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(96);
      END_STATE();
    case 97:
      ACCEPT_TOKEN(sym_number_literal);
      if (lookahead == '\'') ADVANCE(15);
      if (lookahead == '.') ADVANCE(108);
      if (lookahead == 'B' ||
          lookahead == 'b') ADVANCE(106);
      if (lookahead == 'X' ||
          lookahead == 'x') ADVANCE(11);
      if (lookahead == 'E' ||
          lookahead == 'P' ||
          lookahead == 'e' ||
          lookahead == 'p') ADVANCE(107);
      if (('D' <= lookahead && lookahead <= 'F') ||
          lookahead == 'L' ||
          lookahead == 'U' ||
          lookahead == 'W' ||
          ('d' <= lookahead && lookahead <= 'f') ||
          lookahead == 'l' ||
          lookahead == 'u' ||
          lookahead == 'w') ADVANCE(110);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(99);
      END_STATE();
    case 98:
      ACCEPT_TOKEN(sym_number_literal);
      if (lookahead == '\'') ADVANCE(15);
      if (lookahead == '.') ADVANCE(108);
      if (lookahead == 'B' ||
          lookahead == 'b') ADVANCE(109);
      if (lookahead == 'X' ||
          lookahead == 'x') ADVANCE(21);
      if (lookahead == 'E' ||
          lookahead == 'P' ||
          lookahead == 'e' ||
          lookahead == 'p') ADVANCE(107);
      if (('D' <= lookahead && lookahead <= 'F') ||
          lookahead == 'L' ||
          lookahead == 'U' ||
          lookahead == 'W' ||
          ('d' <= lookahead && lookahead <= 'f') ||
          lookahead == 'l' ||
          lookahead == 'u' ||
          lookahead == 'w') ADVANCE(110);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(99);
      END_STATE();
    case 99:
      ACCEPT_TOKEN(sym_number_literal);
      if (lookahead == '\'') ADVANCE(15);
      if (lookahead == '.') ADVANCE(108);
      if (lookahead == 'E' ||
          lookahead == 'P' ||
          lookahead == 'e' ||
          lookahead == 'p') ADVANCE(107);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(99);
      if (aux_sym_string_literal_token1_character_set_2(lookahead)) ADVANCE(110);
      END_STATE();
    case 100:
      ACCEPT_TOKEN(sym_number_literal);
      if (lookahead == '\'') ADVANCE(22);
      if (lookahead == 'B' ||
          lookahead == 'D' ||
          lookahead == 'F' ||
          lookahead == 'b' ||
          lookahead == 'd' ||
          lookahead == 'f') ADVANCE(100);
      if (lookahead == 'L' ||
          lookahead == 'U' ||
          lookahead == 'W' ||
          lookahead == 'l' ||
          lookahead == 'u' ||
          lookahead == 'w') ADVANCE(110);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'E') ||
          ('a' <= lookahead && lookahead <= 'e')) ADVANCE(100);
      END_STATE();
    case 101:
      ACCEPT_TOKEN(sym_number_literal);
      if (lookahead == '\'') ADVANCE(20);
      if (lookahead == '+' ||
          lookahead == '-') ADVANCE(22);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(101);
      if (lookahead == 'P' ||
          lookahead == 'p') ADVANCE(107);
      if (lookahead == 'B' ||
          ('D' <= lookahead && lookahead <= 'F') ||
          lookahead == 'b' ||
          ('d' <= lookahead && lookahead <= 'f')) ADVANCE(102);
      if (lookahead == 'L' ||
          lookahead == 'U' ||
          lookahead == 'W' ||
          lookahead == 'l' ||
          lookahead == 'u' ||
          lookahead == 'w') ADVANCE(110);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'C') ||
          ('a' <= lookahead && lookahead <= 'c')) ADVANCE(102);
      END_STATE();
    case 102:
      ACCEPT_TOKEN(sym_number_literal);
      if (lookahead == '\'') ADVANCE(20);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(101);
      if (lookahead == 'P' ||
          lookahead == 'p') ADVANCE(107);
      if (lookahead == 'B' ||
          ('D' <= lookahead && lookahead <= 'F') ||
          lookahead == 'b' ||
          ('d' <= lookahead && lookahead <= 'f')) ADVANCE(102);
      if (lookahead == 'L' ||
          lookahead == 'U' ||
          lookahead == 'W' ||
          lookahead == 'l' ||
          lookahead == 'u' ||
          lookahead == 'w') ADVANCE(110);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'C') ||
          ('a' <= lookahead && lookahead <= 'c')) ADVANCE(102);
      END_STATE();
    case 103:
      ACCEPT_TOKEN(sym_number_literal);
      if (lookahead == '\'') ADVANCE(21);
      if (lookahead == '.') ADVANCE(108);
      if (lookahead == '+' ||
          lookahead == '-') ADVANCE(22);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(103);
      if (lookahead == 'P' ||
          lookahead == 'p') ADVANCE(107);
      if (lookahead == 'B' ||
          ('D' <= lookahead && lookahead <= 'F') ||
          lookahead == 'b' ||
          ('d' <= lookahead && lookahead <= 'f')) ADVANCE(105);
      if (lookahead == 'L' ||
          lookahead == 'U' ||
          lookahead == 'W' ||
          lookahead == 'l' ||
          lookahead == 'u' ||
          lookahead == 'w') ADVANCE(110);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'C') ||
          ('a' <= lookahead && lookahead <= 'c')) ADVANCE(105);
      END_STATE();
    case 104:
      ACCEPT_TOKEN(sym_number_literal);
      if (lookahead == '\'') ADVANCE(21);
      if (lookahead == '.') ADVANCE(108);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(103);
      if (lookahead == 'P' ||
          lookahead == 'p') ADVANCE(107);
      if (lookahead == 'A' ||
          lookahead == 'C' ||
          lookahead == 'a' ||
          lookahead == 'c') ADVANCE(105);
      if (('B' <= lookahead && lookahead <= 'F') ||
          ('b' <= lookahead && lookahead <= 'f')) ADVANCE(105);
      if (lookahead == 'L' ||
          lookahead == 'U' ||
          lookahead == 'W' ||
          lookahead == 'l' ||
          lookahead == 'u' ||
          lookahead == 'w') ADVANCE(110);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(96);
      END_STATE();
    case 105:
      ACCEPT_TOKEN(sym_number_literal);
      if (lookahead == '\'') ADVANCE(21);
      if (lookahead == '.') ADVANCE(108);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(103);
      if (lookahead == 'P' ||
          lookahead == 'p') ADVANCE(107);
      if (lookahead == 'B' ||
          ('D' <= lookahead && lookahead <= 'F') ||
          lookahead == 'b' ||
          ('d' <= lookahead && lookahead <= 'f')) ADVANCE(105);
      if (lookahead == 'L' ||
          lookahead == 'U' ||
          lookahead == 'W' ||
          lookahead == 'l' ||
          lookahead == 'u' ||
          lookahead == 'w') ADVANCE(110);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'C') ||
          ('a' <= lookahead && lookahead <= 'c')) ADVANCE(105);
      END_STATE();
    case 106:
      ACCEPT_TOKEN(sym_number_literal);
      if (lookahead == '.') ADVANCE(16);
      if (lookahead == '0') ADVANCE(98);
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(99);
      if (aux_sym_string_literal_token1_character_set_3(lookahead)) ADVANCE(110);
      END_STATE();
    case 107:
      ACCEPT_TOKEN(sym_number_literal);
      if (lookahead == '+' ||
          lookahead == '-') ADVANCE(22);
      if (lookahead == 'B' ||
          lookahead == 'D' ||
          lookahead == 'F' ||
          lookahead == 'b' ||
          lookahead == 'd' ||
          lookahead == 'f') ADVANCE(100);
      if (lookahead == 'L' ||
          lookahead == 'U' ||
          lookahead == 'W' ||
          lookahead == 'l' ||
          lookahead == 'u' ||
          lookahead == 'w') ADVANCE(110);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'E') ||
          ('a' <= lookahead && lookahead <= 'e')) ADVANCE(100);
      END_STATE();
    case 108:
      ACCEPT_TOKEN(sym_number_literal);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(101);
      if (lookahead == 'P' ||
          lookahead == 'p') ADVANCE(107);
      if (lookahead == 'B' ||
          ('D' <= lookahead && lookahead <= 'F') ||
          lookahead == 'b' ||
          ('d' <= lookahead && lookahead <= 'f')) ADVANCE(102);
      if (lookahead == 'L' ||
          lookahead == 'U' ||
          lookahead == 'W' ||
          lookahead == 'l' ||
          lookahead == 'u' ||
          lookahead == 'w') ADVANCE(110);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'C') ||
          ('a' <= lookahead && lookahead <= 'c')) ADVANCE(102);
      END_STATE();
    case 109:
      ACCEPT_TOKEN(sym_number_literal);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(99);
      if (aux_sym_string_literal_token1_character_set_3(lookahead)) ADVANCE(110);
      END_STATE();
    case 110:
      ACCEPT_TOKEN(sym_number_literal);
      if (aux_sym_string_literal_token1_character_set_3(lookahead)) ADVANCE(110);
      END_STATE();
    case 111:
      ACCEPT_TOKEN(anon_sym_LBRACK);
      END_STATE();
    case 112:
      ACCEPT_TOKEN(anon_sym_RBRACK);
      END_STATE();
    case 113:
      ACCEPT_TOKEN(sym_edge_parser);
      END_STATE();
    case 114:
      ACCEPT_TOKEN(sym_edge_parser);
      if (lookahead == '.' ||
          lookahead == '<' ||
          lookahead == '>' ||
          lookahead == '^' ||
          lookahead == '_' ||
          lookahead == '|') ADVANCE(113);
      END_STATE();
    case 115:
      ACCEPT_TOKEN(sym_edge_parser);
      if (lookahead == '.' ||
          lookahead == '<' ||
          lookahead == '>' ||
          lookahead == '^' ||
          lookahead == '_' ||
          lookahead == '|') ADVANCE(114);
      END_STATE();
    case 116:
      ACCEPT_TOKEN(anon_sym_LBRACE);
      END_STATE();
    case 117:
      ACCEPT_TOKEN(anon_sym_RBRACE);
      END_STATE();
    case 118:
      ACCEPT_TOKEN(anon_sym_COLON);
      END_STATE();
    case 119:
      ACCEPT_TOKEN(anon_sym_COMMA);
      END_STATE();
    case 120:
      ACCEPT_TOKEN(anon_sym_LPAREN);
      END_STATE();
    case 121:
      ACCEPT_TOKEN(anon_sym_RPAREN);
      END_STATE();
    case 122:
      ACCEPT_TOKEN(sym_operation);
      END_STATE();
    case 123:
      ACCEPT_TOKEN(sym_direction);
      END_STATE();
    case 124:
      ACCEPT_TOKEN(sym_comment);
      END_STATE();
    case 125:
      ACCEPT_TOKEN(sym_comment);
      if (lookahead == '\\') ADVANCE(37);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(125);
      END_STATE();
    case 126:
      ACCEPT_TOKEN(sym_comment);
      if (lookahead != 0 &&
          lookahead != '\r' &&
          lookahead != '\\') ADVANCE(125);
      if (lookahead == '\r') ADVANCE(127);
      if (lookahead == '\\') ADVANCE(126);
      END_STATE();
    case 127:
      ACCEPT_TOKEN(sym_comment);
      if (lookahead != 0 &&
          lookahead != '\\') ADVANCE(125);
      if (lookahead == '\\') ADVANCE(37);
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
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 39},
  [2] = {.lex_state = 39},
  [3] = {.lex_state = 39},
  [4] = {.lex_state = 39},
  [5] = {.lex_state = 38},
  [6] = {.lex_state = 39},
  [7] = {.lex_state = 39},
  [8] = {.lex_state = 38},
  [9] = {.lex_state = 39},
  [10] = {.lex_state = 38},
  [11] = {.lex_state = 39},
  [12] = {.lex_state = 38},
  [13] = {.lex_state = 38},
  [14] = {.lex_state = 38},
  [15] = {.lex_state = 9},
  [16] = {.lex_state = 39},
  [17] = {.lex_state = 39},
  [18] = {.lex_state = 39},
  [19] = {.lex_state = 39},
  [20] = {.lex_state = 39},
  [21] = {.lex_state = 39},
  [22] = {.lex_state = 39},
  [23] = {.lex_state = 39},
  [24] = {.lex_state = 1},
  [25] = {.lex_state = 39},
  [26] = {.lex_state = 39},
  [27] = {.lex_state = 39},
  [28] = {.lex_state = 39},
  [29] = {.lex_state = 39},
  [30] = {.lex_state = 39},
  [31] = {.lex_state = 39},
  [32] = {.lex_state = 39},
  [33] = {.lex_state = 39},
  [34] = {.lex_state = 9},
  [35] = {.lex_state = 39},
  [36] = {.lex_state = 39},
  [37] = {.lex_state = 39},
  [38] = {.lex_state = 1},
  [39] = {.lex_state = 39},
  [40] = {.lex_state = 39},
  [41] = {.lex_state = 39},
  [42] = {.lex_state = 1},
  [43] = {.lex_state = 1},
  [44] = {.lex_state = 39},
  [45] = {.lex_state = 39},
  [46] = {.lex_state = 39},
  [47] = {.lex_state = 39},
  [48] = {.lex_state = 38},
  [49] = {.lex_state = 39},
  [50] = {.lex_state = 1},
  [51] = {.lex_state = 39},
  [52] = {.lex_state = 39},
  [53] = {.lex_state = 38},
  [54] = {.lex_state = 9},
  [55] = {.lex_state = 9},
  [56] = {.lex_state = 38},
  [57] = {.lex_state = 38},
  [58] = {.lex_state = 39},
  [59] = {.lex_state = 38},
  [60] = {.lex_state = 38},
  [61] = {.lex_state = 38},
  [62] = {.lex_state = 38},
  [63] = {.lex_state = 38},
  [64] = {.lex_state = 39},
  [65] = {.lex_state = 38},
  [66] = {.lex_state = 39},
  [67] = {.lex_state = 38},
  [68] = {.lex_state = 38},
  [69] = {.lex_state = 38},
  [70] = {.lex_state = 39},
  [71] = {.lex_state = 38},
  [72] = {.lex_state = 39},
  [73] = {.lex_state = 38},
  [74] = {.lex_state = 38},
  [75] = {.lex_state = 38},
  [76] = {.lex_state = 38},
  [77] = {.lex_state = 39},
  [78] = {.lex_state = 38},
  [79] = {.lex_state = 39},
  [80] = {.lex_state = 38},
  [81] = {.lex_state = 39},
  [82] = {.lex_state = 38},
  [83] = {.lex_state = 38},
  [84] = {.lex_state = 38},
  [85] = {.lex_state = 38},
  [86] = {.lex_state = 39},
  [87] = {.lex_state = 38},
  [88] = {.lex_state = 12},
  [89] = {.lex_state = 38},
  [90] = {.lex_state = 12},
  [91] = {.lex_state = 38},
  [92] = {.lex_state = 38},
  [93] = {.lex_state = 38},
  [94] = {.lex_state = 38},
  [95] = {.lex_state = 38},
  [96] = {.lex_state = 38},
  [97] = {.lex_state = 38},
  [98] = {.lex_state = 39},
  [99] = {.lex_state = 38},
  [100] = {.lex_state = 38},
  [101] = {.lex_state = 38},
  [102] = {.lex_state = 38},
  [103] = {.lex_state = 38},
  [104] = {.lex_state = 38},
  [105] = {.lex_state = 38},
  [106] = {.lex_state = 38},
  [107] = {.lex_state = 38},
  [108] = {.lex_state = 39},
  [109] = {.lex_state = 38},
  [110] = {.lex_state = 38},
  [111] = {.lex_state = 38},
  [112] = {.lex_state = 38},
  [113] = {.lex_state = 39},
  [114] = {.lex_state = 38},
  [115] = {.lex_state = 39},
  [116] = {.lex_state = 39},
  [117] = {.lex_state = 38},
  [118] = {.lex_state = 38},
  [119] = {.lex_state = 38},
  [120] = {.lex_state = 38},
  [121] = {.lex_state = 38},
  [122] = {.lex_state = 38},
  [123] = {.lex_state = 38},
  [124] = {.lex_state = 38},
  [125] = {.lex_state = 39},
  [126] = {.lex_state = 38},
  [127] = {.lex_state = 38},
  [128] = {.lex_state = 38},
  [129] = {.lex_state = 38},
  [130] = {.lex_state = 38},
  [131] = {.lex_state = 38},
  [132] = {.lex_state = 38},
  [133] = {.lex_state = 38},
  [134] = {.lex_state = 38},
  [135] = {.lex_state = 38},
  [136] = {.lex_state = 38},
  [137] = {.lex_state = 38},
  [138] = {.lex_state = 38},
  [139] = {.lex_state = 38},
  [140] = {.lex_state = 38},
  [141] = {.lex_state = 38},
  [142] = {.lex_state = 38},
  [143] = {.lex_state = 39},
  [144] = {.lex_state = 38},
  [145] = {.lex_state = 38},
  [146] = {.lex_state = 38},
  [147] = {.lex_state = 39},
  [148] = {.lex_state = 38},
  [149] = {.lex_state = 38},
  [150] = {.lex_state = 38},
  [151] = {.lex_state = 38},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [sym_identifier] = ACTIONS(1),
    [sym_whitespace] = ACTIONS(3),
    [anon_sym_DOT] = ACTIONS(1),
    [sym_escape_sequence] = ACTIONS(1),
    [anon_sym_L_DQUOTE] = ACTIONS(1),
    [anon_sym_u_DQUOTE] = ACTIONS(1),
    [anon_sym_U_DQUOTE] = ACTIONS(1),
    [anon_sym_u8_DQUOTE] = ACTIONS(1),
    [anon_sym_DQUOTE] = ACTIONS(1),
    [aux_sym_string_literal_token1] = ACTIONS(1),
    [sym_number_literal] = ACTIONS(1),
    [anon_sym_LBRACK] = ACTIONS(1),
    [anon_sym_RBRACK] = ACTIONS(1),
    [anon_sym_Size] = ACTIONS(1),
    [anon_sym_LBRACE] = ACTIONS(1),
    [anon_sym_RBRACE] = ACTIONS(1),
    [anon_sym_COLON] = ACTIONS(1),
    [anon_sym_COMMA] = ACTIONS(1),
    [anon_sym_LPAREN] = ACTIONS(1),
    [anon_sym_RPAREN] = ACTIONS(1),
    [sym_operation] = ACTIONS(1),
    [sym_direction] = ACTIONS(1),
    [sym_comment] = ACTIONS(3),
  },
  [1] = {
    [sym_source_file] = STATE(149),
    [sym__definition] = STATE(2),
    [sym_completion] = STATE(2),
    [sym_slide_objects] = STATE(91),
    [sym_slide] = STATE(2),
    [sym_viewbox] = STATE(2),
    [sym_obj] = STATE(2),
    [sym_action] = STATE(2),
    [sym_register] = STATE(2),
    [aux_sym_source_file_repeat1] = STATE(2),
    [ts_builtin_sym_end] = ACTIONS(5),
    [sym_identifier] = ACTIONS(7),
    [sym_whitespace] = ACTIONS(3),
    [anon_sym_LBRACK] = ACTIONS(9),
    [anon_sym_LBRACE] = ACTIONS(11),
    [sym_comment] = ACTIONS(13),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 8,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(7), 1,
      sym_identifier,
    ACTIONS(9), 1,
      anon_sym_LBRACK,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(13), 1,
      sym_comment,
    ACTIONS(15), 1,
      ts_builtin_sym_end,
    STATE(91), 1,
      sym_slide_objects,
    STATE(3), 8,
      sym__definition,
      sym_completion,
      sym_slide,
      sym_viewbox,
      sym_obj,
      sym_action,
      sym_register,
      aux_sym_source_file_repeat1,
  [32] = 8,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(13), 1,
      sym_comment,
    ACTIONS(17), 1,
      ts_builtin_sym_end,
    ACTIONS(19), 1,
      sym_identifier,
    ACTIONS(22), 1,
      anon_sym_LBRACK,
    ACTIONS(25), 1,
      anon_sym_LBRACE,
    STATE(91), 1,
      sym_slide_objects,
    STATE(3), 8,
      sym__definition,
      sym_completion,
      sym_slide,
      sym_viewbox,
      sym_obj,
      sym_action,
      sym_register,
      aux_sym_source_file_repeat1,
  [64] = 8,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(13), 1,
      sym_comment,
    ACTIONS(28), 1,
      sym_identifier,
    ACTIONS(32), 1,
      sym_number_literal,
    ACTIONS(34), 1,
      anon_sym_Size,
    STATE(121), 1,
      sym__vb_identifier,
    STATE(25), 2,
      sym_string_literal,
      sym__text_ident,
    ACTIONS(30), 5,
      anon_sym_L_DQUOTE,
      anon_sym_u_DQUOTE,
      anon_sym_U_DQUOTE,
      anon_sym_u8_DQUOTE,
      anon_sym_DQUOTE,
  [94] = 6,
    ACTIONS(38), 1,
      sym_number_literal,
    ACTIONS(40), 1,
      anon_sym_COMMA,
    ACTIONS(42), 1,
      anon_sym_RPAREN,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
    STATE(84), 2,
      sym_string_literal,
      sym__text_ident,
    ACTIONS(36), 5,
      anon_sym_L_DQUOTE,
      anon_sym_u_DQUOTE,
      anon_sym_U_DQUOTE,
      anon_sym_u8_DQUOTE,
      anon_sym_DQUOTE,
  [119] = 8,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(13), 1,
      sym_comment,
    ACTIONS(44), 1,
      sym_identifier,
    ACTIONS(46), 1,
      sym_number_literal,
    ACTIONS(48), 1,
      anon_sym_COMMA,
    ACTIONS(50), 1,
      anon_sym_RPAREN,
    STATE(74), 1,
      sym_string_literal,
    ACTIONS(36), 5,
      anon_sym_L_DQUOTE,
      anon_sym_u_DQUOTE,
      anon_sym_U_DQUOTE,
      anon_sym_u8_DQUOTE,
      anon_sym_DQUOTE,
  [148] = 7,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(13), 1,
      sym_comment,
    ACTIONS(52), 1,
      sym_identifier,
    ACTIONS(54), 1,
      sym_number_literal,
    ACTIONS(56), 1,
      anon_sym_RPAREN,
    STATE(111), 1,
      sym_string_literal,
    ACTIONS(36), 5,
      anon_sym_L_DQUOTE,
      anon_sym_u_DQUOTE,
      anon_sym_U_DQUOTE,
      anon_sym_u8_DQUOTE,
      anon_sym_DQUOTE,
  [174] = 5,
    ACTIONS(58), 1,
      sym_number_literal,
    ACTIONS(60), 1,
      anon_sym_RPAREN,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
    STATE(119), 2,
      sym_string_literal,
      sym__text_ident,
    ACTIONS(36), 5,
      anon_sym_L_DQUOTE,
      anon_sym_u_DQUOTE,
      anon_sym_U_DQUOTE,
      anon_sym_u8_DQUOTE,
      anon_sym_DQUOTE,
  [196] = 7,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(13), 1,
      sym_comment,
    ACTIONS(52), 1,
      sym_identifier,
    ACTIONS(54), 1,
      sym_number_literal,
    ACTIONS(62), 1,
      anon_sym_RPAREN,
    STATE(111), 1,
      sym_string_literal,
    ACTIONS(36), 5,
      anon_sym_L_DQUOTE,
      anon_sym_u_DQUOTE,
      anon_sym_U_DQUOTE,
      anon_sym_u8_DQUOTE,
      anon_sym_DQUOTE,
  [222] = 5,
    ACTIONS(58), 1,
      sym_number_literal,
    ACTIONS(64), 1,
      anon_sym_RPAREN,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
    STATE(119), 2,
      sym_string_literal,
      sym__text_ident,
    ACTIONS(36), 5,
      anon_sym_L_DQUOTE,
      anon_sym_u_DQUOTE,
      anon_sym_U_DQUOTE,
      anon_sym_u8_DQUOTE,
      anon_sym_DQUOTE,
  [244] = 6,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(13), 1,
      sym_comment,
    ACTIONS(52), 1,
      sym_identifier,
    ACTIONS(54), 1,
      sym_number_literal,
    STATE(111), 1,
      sym_string_literal,
    ACTIONS(36), 5,
      anon_sym_L_DQUOTE,
      anon_sym_u_DQUOTE,
      anon_sym_U_DQUOTE,
      anon_sym_u8_DQUOTE,
      anon_sym_DQUOTE,
  [267] = 4,
    ACTIONS(66), 1,
      sym_number_literal,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
    STATE(96), 2,
      sym_string_literal,
      sym__text_ident,
    ACTIONS(36), 5,
      anon_sym_L_DQUOTE,
      anon_sym_u_DQUOTE,
      anon_sym_U_DQUOTE,
      anon_sym_u8_DQUOTE,
      anon_sym_DQUOTE,
  [286] = 4,
    ACTIONS(58), 1,
      sym_number_literal,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
    STATE(119), 2,
      sym_string_literal,
      sym__text_ident,
    ACTIONS(36), 5,
      anon_sym_L_DQUOTE,
      anon_sym_u_DQUOTE,
      anon_sym_U_DQUOTE,
      anon_sym_u8_DQUOTE,
      anon_sym_DQUOTE,
  [305] = 4,
    ACTIONS(68), 1,
      sym_number_literal,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
    STATE(78), 2,
      sym_string_literal,
      sym__text_ident,
    ACTIONS(36), 5,
      anon_sym_L_DQUOTE,
      anon_sym_u_DQUOTE,
      anon_sym_U_DQUOTE,
      anon_sym_u8_DQUOTE,
      anon_sym_DQUOTE,
  [324] = 6,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(13), 1,
      sym_comment,
    ACTIONS(70), 1,
      sym_edge_parser,
    ACTIONS(72), 1,
      anon_sym_LBRACE,
    STATE(54), 1,
      sym_slide_from,
    ACTIONS(74), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [344] = 3,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(13), 1,
      sym_comment,
    ACTIONS(76), 4,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_LBRACK,
      anon_sym_LBRACE,
  [357] = 3,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(13), 1,
      sym_comment,
    ACTIONS(78), 4,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_LBRACK,
      anon_sym_LBRACE,
  [370] = 3,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(13), 1,
      sym_comment,
    ACTIONS(80), 4,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_LBRACK,
      anon_sym_LBRACE,
  [383] = 3,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(13), 1,
      sym_comment,
    ACTIONS(82), 4,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_LBRACK,
      anon_sym_LBRACE,
  [396] = 6,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(13), 1,
      sym_comment,
    ACTIONS(84), 1,
      sym_identifier,
    ACTIONS(86), 1,
      anon_sym_RBRACK,
    ACTIONS(88), 1,
      anon_sym_COMMA,
    STATE(82), 1,
      sym_slide_function,
  [415] = 3,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(13), 1,
      sym_comment,
    ACTIONS(90), 4,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_LBRACK,
      anon_sym_LBRACE,
  [428] = 3,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(13), 1,
      sym_comment,
    ACTIONS(92), 4,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_LBRACK,
      anon_sym_LBRACE,
  [441] = 3,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(13), 1,
      sym_comment,
    ACTIONS(94), 4,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_LBRACK,
      anon_sym_LBRACE,
  [454] = 4,
    ACTIONS(98), 1,
      anon_sym_DQUOTE,
    STATE(42), 1,
      aux_sym_string_literal_repeat1,
    ACTIONS(3), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(96), 2,
      sym_escape_sequence,
      aux_sym_string_literal_token1,
  [469] = 3,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(13), 1,
      sym_comment,
    ACTIONS(100), 4,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_LBRACK,
      anon_sym_LBRACE,
  [482] = 3,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(13), 1,
      sym_comment,
    ACTIONS(102), 4,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_LBRACK,
      anon_sym_LBRACE,
  [495] = 3,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(13), 1,
      sym_comment,
    ACTIONS(104), 4,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_LBRACK,
      anon_sym_LBRACE,
  [508] = 3,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(13), 1,
      sym_comment,
    ACTIONS(106), 4,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_LBRACK,
      anon_sym_LBRACE,
  [521] = 3,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(13), 1,
      sym_comment,
    ACTIONS(108), 4,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_LBRACK,
      anon_sym_LBRACE,
  [534] = 3,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(13), 1,
      sym_comment,
    ACTIONS(110), 4,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_LBRACK,
      anon_sym_LBRACE,
  [547] = 3,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(13), 1,
      sym_comment,
    ACTIONS(112), 4,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_LBRACK,
      anon_sym_LBRACE,
  [560] = 3,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(13), 1,
      sym_comment,
    ACTIONS(114), 4,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_LBRACK,
      anon_sym_LBRACE,
  [573] = 3,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(13), 1,
      sym_comment,
    ACTIONS(116), 4,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_LBRACK,
      anon_sym_LBRACE,
  [586] = 4,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(13), 1,
      sym_comment,
    ACTIONS(118), 1,
      sym_edge_parser,
    ACTIONS(120), 3,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [601] = 3,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(13), 1,
      sym_comment,
    ACTIONS(122), 4,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_LBRACK,
      anon_sym_LBRACE,
  [614] = 3,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(13), 1,
      sym_comment,
    ACTIONS(124), 4,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_LBRACK,
      anon_sym_LBRACE,
  [627] = 3,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(13), 1,
      sym_comment,
    ACTIONS(126), 4,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_LBRACK,
      anon_sym_LBRACE,
  [640] = 4,
    ACTIONS(130), 1,
      anon_sym_DQUOTE,
    STATE(50), 1,
      aux_sym_string_literal_repeat1,
    ACTIONS(3), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(128), 2,
      sym_escape_sequence,
      aux_sym_string_literal_token1,
  [655] = 3,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(13), 1,
      sym_comment,
    ACTIONS(132), 4,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_LBRACK,
      anon_sym_LBRACE,
  [668] = 3,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(13), 1,
      sym_comment,
    ACTIONS(134), 4,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_LBRACK,
      anon_sym_LBRACE,
  [681] = 3,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(13), 1,
      sym_comment,
    ACTIONS(136), 4,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_LBRACK,
      anon_sym_LBRACE,
  [694] = 4,
    ACTIONS(140), 1,
      anon_sym_DQUOTE,
    STATE(43), 1,
      aux_sym_string_literal_repeat1,
    ACTIONS(3), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(138), 2,
      sym_escape_sequence,
      aux_sym_string_literal_token1,
  [709] = 4,
    ACTIONS(145), 1,
      anon_sym_DQUOTE,
    STATE(43), 1,
      aux_sym_string_literal_repeat1,
    ACTIONS(3), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(142), 2,
      sym_escape_sequence,
      aux_sym_string_literal_token1,
  [724] = 6,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(13), 1,
      sym_comment,
    ACTIONS(147), 1,
      sym_identifier,
    ACTIONS(149), 1,
      anon_sym_RBRACE,
    ACTIONS(151), 1,
      anon_sym_COMMA,
    STATE(60), 1,
      sym_slide_obj,
  [743] = 6,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(13), 1,
      sym_comment,
    ACTIONS(153), 1,
      sym_identifier,
    ACTIONS(155), 1,
      anon_sym_RBRACK,
    ACTIONS(157), 1,
      anon_sym_COMMA,
    STATE(56), 1,
      sym_action_obj,
  [762] = 3,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(13), 1,
      sym_comment,
    ACTIONS(159), 4,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_LBRACK,
      anon_sym_LBRACE,
  [775] = 3,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(13), 1,
      sym_comment,
    ACTIONS(161), 4,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_LBRACK,
      anon_sym_LBRACE,
  [788] = 5,
    ACTIONS(163), 1,
      sym_number_literal,
    ACTIONS(165), 1,
      anon_sym_RBRACK,
    ACTIONS(167), 1,
      anon_sym_COMMA,
    STATE(87), 1,
      sym_viewbox_obj,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
  [805] = 3,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(13), 1,
      sym_comment,
    ACTIONS(169), 4,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_LBRACK,
      anon_sym_LBRACE,
  [818] = 4,
    ACTIONS(171), 1,
      anon_sym_DQUOTE,
    STATE(43), 1,
      aux_sym_string_literal_repeat1,
    ACTIONS(3), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(138), 2,
      sym_escape_sequence,
      aux_sym_string_literal_token1,
  [833] = 3,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(13), 1,
      sym_comment,
    ACTIONS(173), 4,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_LBRACK,
      anon_sym_LBRACE,
  [846] = 3,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(13), 1,
      sym_comment,
    ACTIONS(175), 4,
      ts_builtin_sym_end,
      sym_identifier,
      anon_sym_LBRACK,
      anon_sym_LBRACE,
  [859] = 4,
    ACTIONS(177), 1,
      anon_sym_COMMA,
    ACTIONS(180), 1,
      anon_sym_RPAREN,
    STATE(53), 1,
      aux_sym_slide_function_repeat1,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
  [873] = 4,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(13), 1,
      sym_comment,
    ACTIONS(182), 1,
      sym_edge_parser,
    ACTIONS(184), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [887] = 4,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(13), 1,
      sym_comment,
    ACTIONS(186), 1,
      sym_edge_parser,
    ACTIONS(188), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [901] = 4,
    ACTIONS(190), 1,
      anon_sym_RBRACK,
    ACTIONS(192), 1,
      anon_sym_COMMA,
    STATE(71), 1,
      aux_sym_action_repeat1,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
  [915] = 4,
    ACTIONS(194), 1,
      anon_sym_COMMA,
    ACTIONS(197), 1,
      anon_sym_RPAREN,
    STATE(57), 1,
      aux_sym_obj_inner_repeat1,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
  [929] = 5,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(13), 1,
      sym_comment,
    ACTIONS(153), 1,
      sym_identifier,
    ACTIONS(199), 1,
      anon_sym_RBRACK,
    STATE(100), 1,
      sym_action_obj,
  [945] = 4,
    ACTIONS(201), 1,
      anon_sym_RBRACK,
    ACTIONS(203), 1,
      anon_sym_COMMA,
    STATE(59), 1,
      aux_sym_action_repeat1,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
  [959] = 4,
    ACTIONS(206), 1,
      anon_sym_RBRACE,
    ACTIONS(208), 1,
      anon_sym_COMMA,
    STATE(85), 1,
      aux_sym_slide_objects_repeat1,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
  [973] = 4,
    ACTIONS(210), 1,
      anon_sym_COMMA,
    ACTIONS(213), 1,
      anon_sym_RPAREN,
    STATE(61), 1,
      aux_sym_action_obj_repeat1,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
  [987] = 4,
    ACTIONS(215), 1,
      anon_sym_LBRACK,
    ACTIONS(217), 1,
      anon_sym_LPAREN,
    STATE(40), 1,
      sym_obj_inner,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
  [1001] = 4,
    ACTIONS(219), 1,
      anon_sym_RBRACK,
    ACTIONS(221), 1,
      anon_sym_COMMA,
    STATE(63), 1,
      aux_sym_viewbox_inner_repeat1,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
  [1015] = 5,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(13), 1,
      sym_comment,
    ACTIONS(147), 1,
      sym_identifier,
    ACTIONS(224), 1,
      anon_sym_RBRACE,
    STATE(117), 1,
      sym_slide_obj,
  [1031] = 4,
    ACTIONS(226), 1,
      anon_sym_RBRACE,
    ACTIONS(228), 1,
      anon_sym_COMMA,
    STATE(65), 1,
      aux_sym_slide_objects_repeat1,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
  [1045] = 5,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(13), 1,
      sym_comment,
    ACTIONS(84), 1,
      sym_identifier,
    ACTIONS(231), 1,
      anon_sym_RBRACK,
    STATE(102), 1,
      sym_slide_function,
  [1061] = 4,
    ACTIONS(231), 1,
      anon_sym_RBRACK,
    ACTIONS(233), 1,
      anon_sym_COMMA,
    STATE(80), 1,
      aux_sym_slide_functions_repeat1,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
  [1075] = 4,
    ACTIONS(163), 1,
      sym_number_literal,
    ACTIONS(235), 1,
      anon_sym_RBRACK,
    STATE(120), 1,
      sym_viewbox_obj,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
  [1089] = 4,
    ACTIONS(237), 1,
      anon_sym_COMMA,
    ACTIONS(239), 1,
      anon_sym_RPAREN,
    STATE(57), 1,
      aux_sym_obj_inner_repeat1,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
  [1103] = 5,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(13), 1,
      sym_comment,
    ACTIONS(153), 1,
      sym_identifier,
    ACTIONS(241), 1,
      anon_sym_RBRACK,
    STATE(100), 1,
      sym_action_obj,
  [1119] = 4,
    ACTIONS(241), 1,
      anon_sym_RBRACK,
    ACTIONS(243), 1,
      anon_sym_COMMA,
    STATE(59), 1,
      aux_sym_action_repeat1,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
  [1133] = 4,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(13), 1,
      sym_comment,
    STATE(89), 1,
      sym__vb_identifier,
    ACTIONS(245), 2,
      sym_identifier,
      anon_sym_Size,
  [1147] = 4,
    ACTIONS(60), 1,
      anon_sym_RPAREN,
    ACTIONS(247), 1,
      anon_sym_COMMA,
    STATE(61), 1,
      aux_sym_action_obj_repeat1,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
  [1161] = 4,
    ACTIONS(249), 1,
      anon_sym_COMMA,
    ACTIONS(251), 1,
      anon_sym_RPAREN,
    STATE(83), 1,
      aux_sym_slide_function_repeat1,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
  [1175] = 4,
    ACTIONS(253), 1,
      anon_sym_RBRACK,
    ACTIONS(255), 1,
      anon_sym_COMMA,
    STATE(63), 1,
      aux_sym_viewbox_inner_repeat1,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
  [1189] = 4,
    ACTIONS(163), 1,
      sym_number_literal,
    ACTIONS(257), 1,
      anon_sym_RBRACK,
    STATE(120), 1,
      sym_viewbox_obj,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
  [1203] = 5,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(13), 1,
      sym_comment,
    ACTIONS(147), 1,
      sym_identifier,
    ACTIONS(259), 1,
      anon_sym_RBRACE,
    STATE(117), 1,
      sym_slide_obj,
  [1219] = 4,
    ACTIONS(261), 1,
      anon_sym_COMMA,
    ACTIONS(263), 1,
      anon_sym_RPAREN,
    STATE(69), 1,
      aux_sym_obj_inner_repeat1,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
  [1233] = 5,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(13), 1,
      sym_comment,
    ACTIONS(84), 1,
      sym_identifier,
    ACTIONS(265), 1,
      anon_sym_RBRACK,
    STATE(102), 1,
      sym_slide_function,
  [1249] = 4,
    ACTIONS(267), 1,
      anon_sym_RBRACK,
    ACTIONS(269), 1,
      anon_sym_COMMA,
    STATE(80), 1,
      aux_sym_slide_functions_repeat1,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
  [1263] = 5,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(13), 1,
      sym_comment,
    ACTIONS(272), 1,
      sym_identifier,
    ACTIONS(274), 1,
      anon_sym_COMMA,
    ACTIONS(276), 1,
      anon_sym_RPAREN,
  [1279] = 4,
    ACTIONS(278), 1,
      anon_sym_RBRACK,
    ACTIONS(280), 1,
      anon_sym_COMMA,
    STATE(67), 1,
      aux_sym_slide_functions_repeat1,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
  [1293] = 4,
    ACTIONS(56), 1,
      anon_sym_RPAREN,
    ACTIONS(282), 1,
      anon_sym_COMMA,
    STATE(53), 1,
      aux_sym_slide_function_repeat1,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
  [1307] = 4,
    ACTIONS(284), 1,
      anon_sym_COMMA,
    ACTIONS(286), 1,
      anon_sym_RPAREN,
    STATE(73), 1,
      aux_sym_action_obj_repeat1,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
  [1321] = 4,
    ACTIONS(288), 1,
      anon_sym_RBRACE,
    ACTIONS(290), 1,
      anon_sym_COMMA,
    STATE(65), 1,
      aux_sym_slide_objects_repeat1,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
  [1335] = 4,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(13), 1,
      sym_comment,
    STATE(109), 1,
      sym__vb_identifier,
    ACTIONS(292), 2,
      sym_identifier,
      anon_sym_Size,
  [1349] = 4,
    ACTIONS(294), 1,
      anon_sym_RBRACK,
    ACTIONS(296), 1,
      anon_sym_COMMA,
    STATE(75), 1,
      aux_sym_viewbox_inner_repeat1,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
  [1363] = 3,
    ACTIONS(298), 1,
      anon_sym_DOT,
    ACTIONS(300), 1,
      anon_sym_COLON,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
  [1374] = 3,
    ACTIONS(302), 1,
      anon_sym_LBRACK,
    STATE(15), 1,
      sym_index_parser,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
  [1385] = 3,
    ACTIONS(304), 1,
      anon_sym_COLON,
    ACTIONS(306), 1,
      sym_operation,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
  [1396] = 3,
    ACTIONS(308), 1,
      anon_sym_LBRACK,
    STATE(21), 1,
      sym_slide_functions,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
  [1407] = 2,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(106), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
  [1416] = 2,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(136), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
  [1425] = 2,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(310), 2,
      anon_sym_RBRACK,
      anon_sym_COMMA,
  [1434] = 2,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(184), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [1443] = 2,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(312), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
  [1452] = 2,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(314), 2,
      anon_sym_RBRACK,
      anon_sym_COMMA,
  [1461] = 4,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(13), 1,
      sym_comment,
    ACTIONS(84), 1,
      sym_identifier,
    STATE(102), 1,
      sym_slide_function,
  [1474] = 3,
    ACTIONS(316), 1,
      sym_direction,
    STATE(49), 1,
      sym_viewbox_inner,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
  [1485] = 2,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(201), 2,
      anon_sym_RBRACK,
      anon_sym_COMMA,
  [1494] = 2,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(318), 2,
      anon_sym_RBRACK,
      anon_sym_COMMA,
  [1503] = 2,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(267), 2,
      anon_sym_RBRACK,
      anon_sym_COMMA,
  [1512] = 2,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(320), 2,
      anon_sym_RBRACK,
      anon_sym_COMMA,
  [1521] = 3,
    ACTIONS(163), 1,
      sym_number_literal,
    STATE(120), 1,
      sym_viewbox_obj,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
  [1532] = 2,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(322), 2,
      anon_sym_RBRACK,
      anon_sym_COMMA,
  [1541] = 2,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(324), 2,
      anon_sym_RBRACK,
      anon_sym_COMMA,
  [1550] = 2,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(120), 2,
      anon_sym_RBRACE,
      sym_direction,
  [1559] = 4,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(13), 1,
      sym_comment,
    ACTIONS(326), 1,
      sym_identifier,
    ACTIONS(328), 1,
      anon_sym_RPAREN,
  [1572] = 3,
    ACTIONS(330), 1,
      anon_sym_LBRACK,
    STATE(128), 1,
      sym_index_parser,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
  [1583] = 2,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(332), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [1592] = 2,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(180), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
  [1601] = 2,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(334), 2,
      anon_sym_RBRACK,
      anon_sym_COMMA,
  [1610] = 4,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(13), 1,
      sym_comment,
    ACTIONS(147), 1,
      sym_identifier,
    STATE(117), 1,
      sym_slide_obj,
  [1623] = 2,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(336), 2,
      anon_sym_RBRACK,
      anon_sym_COMMA,
  [1632] = 4,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(13), 1,
      sym_comment,
    ACTIONS(326), 1,
      sym_identifier,
    ACTIONS(338), 1,
      anon_sym_RPAREN,
  [1645] = 4,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(13), 1,
      sym_comment,
    ACTIONS(153), 1,
      sym_identifier,
    STATE(100), 1,
      sym_action_obj,
  [1658] = 2,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(226), 2,
      anon_sym_RBRACE,
      anon_sym_COMMA,
  [1667] = 2,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(340), 2,
      anon_sym_RBRACK,
      anon_sym_COMMA,
  [1676] = 2,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(213), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
  [1685] = 2,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(219), 2,
      anon_sym_RBRACK,
      anon_sym_COMMA,
  [1694] = 3,
    ACTIONS(330), 1,
      anon_sym_LBRACK,
    STATE(99), 1,
      sym_index_parser,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
  [1705] = 2,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
    ACTIONS(342), 2,
      anon_sym_RBRACK,
      anon_sym_COMMA,
  [1714] = 2,
    ACTIONS(344), 1,
      sym_number_literal,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
  [1722] = 2,
    ACTIONS(346), 1,
      anon_sym_COLON,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
  [1730] = 3,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(13), 1,
      sym_comment,
    ACTIONS(348), 1,
      sym_identifier,
  [1740] = 2,
    ACTIONS(350), 1,
      anon_sym_LBRACK,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
  [1748] = 2,
    ACTIONS(352), 1,
      anon_sym_RBRACK,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
  [1756] = 2,
    ACTIONS(354), 1,
      anon_sym_RBRACE,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
  [1764] = 2,
    ACTIONS(356), 1,
      anon_sym_LBRACK,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
  [1772] = 2,
    ACTIONS(358), 1,
      anon_sym_LBRACK,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
  [1780] = 2,
    ACTIONS(360), 1,
      anon_sym_COLON,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
  [1788] = 2,
    ACTIONS(362), 1,
      anon_sym_RBRACE,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
  [1796] = 2,
    ACTIONS(251), 1,
      anon_sym_RPAREN,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
  [1804] = 2,
    ACTIONS(364), 1,
      anon_sym_LBRACK,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
  [1812] = 2,
    ACTIONS(366), 1,
      anon_sym_LBRACK,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
  [1820] = 2,
    ACTIONS(368), 1,
      anon_sym_COLON,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
  [1828] = 2,
    ACTIONS(370), 1,
      anon_sym_RPAREN,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
  [1836] = 2,
    ACTIONS(372), 1,
      sym_number_literal,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
  [1844] = 2,
    ACTIONS(374), 1,
      anon_sym_COLON,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
  [1852] = 2,
    ACTIONS(376), 1,
      anon_sym_LPAREN,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
  [1860] = 2,
    ACTIONS(190), 1,
      anon_sym_RBRACK,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
  [1868] = 2,
    ACTIONS(378), 1,
      anon_sym_LBRACK,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
  [1876] = 3,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(13), 1,
      sym_comment,
    ACTIONS(326), 1,
      sym_identifier,
  [1886] = 2,
    ACTIONS(380), 1,
      anon_sym_LPAREN,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
  [1894] = 2,
    ACTIONS(286), 1,
      anon_sym_RPAREN,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
  [1902] = 2,
    ACTIONS(382), 1,
      anon_sym_RBRACK,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
  [1910] = 3,
    ACTIONS(3), 1,
      sym_whitespace,
    ACTIONS(13), 1,
      sym_comment,
    ACTIONS(384), 1,
      sym_identifier,
  [1920] = 2,
    ACTIONS(278), 1,
      anon_sym_RBRACK,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
  [1928] = 2,
    ACTIONS(386), 1,
      ts_builtin_sym_end,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
  [1936] = 2,
    ACTIONS(388), 1,
      anon_sym_RBRACK,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
  [1944] = 2,
    ACTIONS(390), 1,
      sym_number_literal,
    ACTIONS(13), 2,
      sym_whitespace,
      sym_comment,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 32,
  [SMALL_STATE(4)] = 64,
  [SMALL_STATE(5)] = 94,
  [SMALL_STATE(6)] = 119,
  [SMALL_STATE(7)] = 148,
  [SMALL_STATE(8)] = 174,
  [SMALL_STATE(9)] = 196,
  [SMALL_STATE(10)] = 222,
  [SMALL_STATE(11)] = 244,
  [SMALL_STATE(12)] = 267,
  [SMALL_STATE(13)] = 286,
  [SMALL_STATE(14)] = 305,
  [SMALL_STATE(15)] = 324,
  [SMALL_STATE(16)] = 344,
  [SMALL_STATE(17)] = 357,
  [SMALL_STATE(18)] = 370,
  [SMALL_STATE(19)] = 383,
  [SMALL_STATE(20)] = 396,
  [SMALL_STATE(21)] = 415,
  [SMALL_STATE(22)] = 428,
  [SMALL_STATE(23)] = 441,
  [SMALL_STATE(24)] = 454,
  [SMALL_STATE(25)] = 469,
  [SMALL_STATE(26)] = 482,
  [SMALL_STATE(27)] = 495,
  [SMALL_STATE(28)] = 508,
  [SMALL_STATE(29)] = 521,
  [SMALL_STATE(30)] = 534,
  [SMALL_STATE(31)] = 547,
  [SMALL_STATE(32)] = 560,
  [SMALL_STATE(33)] = 573,
  [SMALL_STATE(34)] = 586,
  [SMALL_STATE(35)] = 601,
  [SMALL_STATE(36)] = 614,
  [SMALL_STATE(37)] = 627,
  [SMALL_STATE(38)] = 640,
  [SMALL_STATE(39)] = 655,
  [SMALL_STATE(40)] = 668,
  [SMALL_STATE(41)] = 681,
  [SMALL_STATE(42)] = 694,
  [SMALL_STATE(43)] = 709,
  [SMALL_STATE(44)] = 724,
  [SMALL_STATE(45)] = 743,
  [SMALL_STATE(46)] = 762,
  [SMALL_STATE(47)] = 775,
  [SMALL_STATE(48)] = 788,
  [SMALL_STATE(49)] = 805,
  [SMALL_STATE(50)] = 818,
  [SMALL_STATE(51)] = 833,
  [SMALL_STATE(52)] = 846,
  [SMALL_STATE(53)] = 859,
  [SMALL_STATE(54)] = 873,
  [SMALL_STATE(55)] = 887,
  [SMALL_STATE(56)] = 901,
  [SMALL_STATE(57)] = 915,
  [SMALL_STATE(58)] = 929,
  [SMALL_STATE(59)] = 945,
  [SMALL_STATE(60)] = 959,
  [SMALL_STATE(61)] = 973,
  [SMALL_STATE(62)] = 987,
  [SMALL_STATE(63)] = 1001,
  [SMALL_STATE(64)] = 1015,
  [SMALL_STATE(65)] = 1031,
  [SMALL_STATE(66)] = 1045,
  [SMALL_STATE(67)] = 1061,
  [SMALL_STATE(68)] = 1075,
  [SMALL_STATE(69)] = 1089,
  [SMALL_STATE(70)] = 1103,
  [SMALL_STATE(71)] = 1119,
  [SMALL_STATE(72)] = 1133,
  [SMALL_STATE(73)] = 1147,
  [SMALL_STATE(74)] = 1161,
  [SMALL_STATE(75)] = 1175,
  [SMALL_STATE(76)] = 1189,
  [SMALL_STATE(77)] = 1203,
  [SMALL_STATE(78)] = 1219,
  [SMALL_STATE(79)] = 1233,
  [SMALL_STATE(80)] = 1249,
  [SMALL_STATE(81)] = 1263,
  [SMALL_STATE(82)] = 1279,
  [SMALL_STATE(83)] = 1293,
  [SMALL_STATE(84)] = 1307,
  [SMALL_STATE(85)] = 1321,
  [SMALL_STATE(86)] = 1335,
  [SMALL_STATE(87)] = 1349,
  [SMALL_STATE(88)] = 1363,
  [SMALL_STATE(89)] = 1374,
  [SMALL_STATE(90)] = 1385,
  [SMALL_STATE(91)] = 1396,
  [SMALL_STATE(92)] = 1407,
  [SMALL_STATE(93)] = 1416,
  [SMALL_STATE(94)] = 1425,
  [SMALL_STATE(95)] = 1434,
  [SMALL_STATE(96)] = 1443,
  [SMALL_STATE(97)] = 1452,
  [SMALL_STATE(98)] = 1461,
  [SMALL_STATE(99)] = 1474,
  [SMALL_STATE(100)] = 1485,
  [SMALL_STATE(101)] = 1494,
  [SMALL_STATE(102)] = 1503,
  [SMALL_STATE(103)] = 1512,
  [SMALL_STATE(104)] = 1521,
  [SMALL_STATE(105)] = 1532,
  [SMALL_STATE(106)] = 1541,
  [SMALL_STATE(107)] = 1550,
  [SMALL_STATE(108)] = 1559,
  [SMALL_STATE(109)] = 1572,
  [SMALL_STATE(110)] = 1583,
  [SMALL_STATE(111)] = 1592,
  [SMALL_STATE(112)] = 1601,
  [SMALL_STATE(113)] = 1610,
  [SMALL_STATE(114)] = 1623,
  [SMALL_STATE(115)] = 1632,
  [SMALL_STATE(116)] = 1645,
  [SMALL_STATE(117)] = 1658,
  [SMALL_STATE(118)] = 1667,
  [SMALL_STATE(119)] = 1676,
  [SMALL_STATE(120)] = 1685,
  [SMALL_STATE(121)] = 1694,
  [SMALL_STATE(122)] = 1705,
  [SMALL_STATE(123)] = 1714,
  [SMALL_STATE(124)] = 1722,
  [SMALL_STATE(125)] = 1730,
  [SMALL_STATE(126)] = 1740,
  [SMALL_STATE(127)] = 1748,
  [SMALL_STATE(128)] = 1756,
  [SMALL_STATE(129)] = 1764,
  [SMALL_STATE(130)] = 1772,
  [SMALL_STATE(131)] = 1780,
  [SMALL_STATE(132)] = 1788,
  [SMALL_STATE(133)] = 1796,
  [SMALL_STATE(134)] = 1804,
  [SMALL_STATE(135)] = 1812,
  [SMALL_STATE(136)] = 1820,
  [SMALL_STATE(137)] = 1828,
  [SMALL_STATE(138)] = 1836,
  [SMALL_STATE(139)] = 1844,
  [SMALL_STATE(140)] = 1852,
  [SMALL_STATE(141)] = 1860,
  [SMALL_STATE(142)] = 1868,
  [SMALL_STATE(143)] = 1876,
  [SMALL_STATE(144)] = 1886,
  [SMALL_STATE(145)] = 1894,
  [SMALL_STATE(146)] = 1902,
  [SMALL_STATE(147)] = 1910,
  [SMALL_STATE(148)] = 1920,
  [SMALL_STATE(149)] = 1928,
  [SMALL_STATE(150)] = 1936,
  [SMALL_STATE(151)] = 1944,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = false}}, SHIFT_EXTRA(),
  [5] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 0),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(88),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(45),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(44),
  [13] = {.entry = {.count = 1, .reusable = true}}, SHIFT_EXTRA(),
  [15] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1),
  [17] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2),
  [19] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(88),
  [22] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(45),
  [25] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(44),
  [28] = {.entry = {.count = 1, .reusable = false}}, SHIFT(62),
  [30] = {.entry = {.count = 1, .reusable = true}}, SHIFT(24),
  [32] = {.entry = {.count = 1, .reusable = true}}, SHIFT(25),
  [34] = {.entry = {.count = 1, .reusable = false}}, SHIFT(121),
  [36] = {.entry = {.count = 1, .reusable = true}}, SHIFT(38),
  [38] = {.entry = {.count = 1, .reusable = true}}, SHIFT(84),
  [40] = {.entry = {.count = 1, .reusable = true}}, SHIFT(145),
  [42] = {.entry = {.count = 1, .reusable = true}}, SHIFT(94),
  [44] = {.entry = {.count = 1, .reusable = false}}, SHIFT(74),
  [46] = {.entry = {.count = 1, .reusable = true}}, SHIFT(74),
  [48] = {.entry = {.count = 1, .reusable = true}}, SHIFT(133),
  [50] = {.entry = {.count = 1, .reusable = true}}, SHIFT(105),
  [52] = {.entry = {.count = 1, .reusable = false}}, SHIFT(111),
  [54] = {.entry = {.count = 1, .reusable = true}}, SHIFT(111),
  [56] = {.entry = {.count = 1, .reusable = true}}, SHIFT(112),
  [58] = {.entry = {.count = 1, .reusable = true}}, SHIFT(119),
  [60] = {.entry = {.count = 1, .reusable = true}}, SHIFT(122),
  [62] = {.entry = {.count = 1, .reusable = true}}, SHIFT(114),
  [64] = {.entry = {.count = 1, .reusable = true}}, SHIFT(103),
  [66] = {.entry = {.count = 1, .reusable = true}}, SHIFT(96),
  [68] = {.entry = {.count = 1, .reusable = true}}, SHIFT(78),
  [70] = {.entry = {.count = 1, .reusable = false}}, SHIFT(95),
  [72] = {.entry = {.count = 1, .reusable = true}}, SHIFT(86),
  [74] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_slide_obj, 4, .production_id = 6),
  [76] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_obj_inner, 5, .production_id = 12),
  [78] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_viewbox_inner, 2, .production_id = 8),
  [80] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_obj_inner, 6, .production_id = 15),
  [82] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_obj_inner, 6, .production_id = 12),
  [84] = {.entry = {.count = 1, .reusable = true}}, SHIFT(144),
  [86] = {.entry = {.count = 1, .reusable = true}}, SHIFT(36),
  [88] = {.entry = {.count = 1, .reusable = true}}, SHIFT(148),
  [90] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_slide, 2),
  [92] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_completion, 3),
  [94] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_viewbox_inner, 4, .production_id = 14),
  [96] = {.entry = {.count = 1, .reusable = true}}, SHIFT(42),
  [98] = {.entry = {.count = 1, .reusable = true}}, SHIFT(41),
  [100] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_register, 3, .production_id = 1),
  [102] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_viewbox_inner, 4, .production_id = 11),
  [104] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_action, 3),
  [106] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string_literal, 3),
  [108] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_action, 2),
  [110] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_obj_inner, 7, .production_id = 15),
  [112] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_viewbox_inner, 3, .production_id = 11),
  [114] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_viewbox_inner, 3, .production_id = 8),
  [116] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_slide_functions, 5),
  [118] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_index_parser, 3),
  [120] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_index_parser, 3),
  [122] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_viewbox_inner, 5, .production_id = 14),
  [124] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_slide_functions, 2),
  [126] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_obj_inner, 3),
  [128] = {.entry = {.count = 1, .reusable = true}}, SHIFT(50),
  [130] = {.entry = {.count = 1, .reusable = true}}, SHIFT(93),
  [132] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_slide_functions, 4),
  [134] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_obj, 4, .production_id = 3),
  [136] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string_literal, 2),
  [138] = {.entry = {.count = 1, .reusable = true}}, SHIFT(43),
  [140] = {.entry = {.count = 1, .reusable = true}}, SHIFT(28),
  [142] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_string_literal_repeat1, 2), SHIFT_REPEAT(43),
  [145] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_string_literal_repeat1, 2),
  [147] = {.entry = {.count = 1, .reusable = true}}, SHIFT(136),
  [149] = {.entry = {.count = 1, .reusable = true}}, SHIFT(134),
  [151] = {.entry = {.count = 1, .reusable = true}}, SHIFT(132),
  [153] = {.entry = {.count = 1, .reusable = true}}, SHIFT(124),
  [155] = {.entry = {.count = 1, .reusable = true}}, SHIFT(29),
  [157] = {.entry = {.count = 1, .reusable = true}}, SHIFT(141),
  [159] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_action, 4),
  [161] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_slide_functions, 3),
  [163] = {.entry = {.count = 1, .reusable = true}}, SHIFT(90),
  [165] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [167] = {.entry = {.count = 1, .reusable = true}}, SHIFT(146),
  [169] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_viewbox, 5, .production_id = 5),
  [171] = {.entry = {.count = 1, .reusable = true}}, SHIFT(92),
  [173] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_action, 5),
  [175] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_obj_inner, 2),
  [177] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_slide_function_repeat1, 2), SHIFT_REPEAT(11),
  [180] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_slide_function_repeat1, 2),
  [182] = {.entry = {.count = 1, .reusable = false}}, SHIFT(110),
  [184] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_slide_obj, 5, .production_id = 6),
  [186] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_slide_from, 4),
  [188] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_slide_from, 4),
  [190] = {.entry = {.count = 1, .reusable = true}}, SHIFT(27),
  [192] = {.entry = {.count = 1, .reusable = true}}, SHIFT(70),
  [194] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_obj_inner_repeat1, 2), SHIFT_REPEAT(143),
  [197] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_obj_inner_repeat1, 2),
  [199] = {.entry = {.count = 1, .reusable = true}}, SHIFT(51),
  [201] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_action_repeat1, 2),
  [203] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_action_repeat1, 2), SHIFT_REPEAT(116),
  [206] = {.entry = {.count = 1, .reusable = true}}, SHIFT(142),
  [208] = {.entry = {.count = 1, .reusable = true}}, SHIFT(77),
  [210] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_action_obj_repeat1, 2), SHIFT_REPEAT(13),
  [213] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_action_obj_repeat1, 2),
  [215] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__vb_identifier, 1),
  [217] = {.entry = {.count = 1, .reusable = true}}, SHIFT(81),
  [219] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_viewbox_inner_repeat1, 2),
  [221] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_viewbox_inner_repeat1, 2), SHIFT_REPEAT(104),
  [224] = {.entry = {.count = 1, .reusable = true}}, SHIFT(129),
  [226] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_slide_objects_repeat1, 2),
  [228] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_slide_objects_repeat1, 2), SHIFT_REPEAT(113),
  [231] = {.entry = {.count = 1, .reusable = true}}, SHIFT(39),
  [233] = {.entry = {.count = 1, .reusable = true}}, SHIFT(79),
  [235] = {.entry = {.count = 1, .reusable = true}}, SHIFT(35),
  [237] = {.entry = {.count = 1, .reusable = true}}, SHIFT(108),
  [239] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [241] = {.entry = {.count = 1, .reusable = true}}, SHIFT(46),
  [243] = {.entry = {.count = 1, .reusable = true}}, SHIFT(58),
  [245] = {.entry = {.count = 1, .reusable = false}}, SHIFT(89),
  [247] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
  [249] = {.entry = {.count = 1, .reusable = true}}, SHIFT(7),
  [251] = {.entry = {.count = 1, .reusable = true}}, SHIFT(97),
  [253] = {.entry = {.count = 1, .reusable = true}}, SHIFT(23),
  [255] = {.entry = {.count = 1, .reusable = true}}, SHIFT(68),
  [257] = {.entry = {.count = 1, .reusable = true}}, SHIFT(26),
  [259] = {.entry = {.count = 1, .reusable = true}}, SHIFT(130),
  [261] = {.entry = {.count = 1, .reusable = true}}, SHIFT(115),
  [263] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [265] = {.entry = {.count = 1, .reusable = true}}, SHIFT(33),
  [267] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_slide_functions_repeat1, 2),
  [269] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_slide_functions_repeat1, 2), SHIFT_REPEAT(98),
  [272] = {.entry = {.count = 1, .reusable = true}}, SHIFT(139),
  [274] = {.entry = {.count = 1, .reusable = true}}, SHIFT(137),
  [276] = {.entry = {.count = 1, .reusable = true}}, SHIFT(52),
  [278] = {.entry = {.count = 1, .reusable = true}}, SHIFT(47),
  [280] = {.entry = {.count = 1, .reusable = true}}, SHIFT(66),
  [282] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [284] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [286] = {.entry = {.count = 1, .reusable = true}}, SHIFT(106),
  [288] = {.entry = {.count = 1, .reusable = true}}, SHIFT(126),
  [290] = {.entry = {.count = 1, .reusable = true}}, SHIFT(64),
  [292] = {.entry = {.count = 1, .reusable = false}}, SHIFT(109),
  [294] = {.entry = {.count = 1, .reusable = true}}, SHIFT(31),
  [296] = {.entry = {.count = 1, .reusable = true}}, SHIFT(76),
  [298] = {.entry = {.count = 1, .reusable = true}}, SHIFT(147),
  [300] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [302] = {.entry = {.count = 1, .reusable = true}}, SHIFT(151),
  [304] = {.entry = {.count = 1, .reusable = true}}, SHIFT(138),
  [306] = {.entry = {.count = 1, .reusable = true}}, SHIFT(101),
  [308] = {.entry = {.count = 1, .reusable = true}}, SHIFT(20),
  [310] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_action_obj, 5, .production_id = 9),
  [312] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_obj_inner_repeat1, 4),
  [314] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_slide_function, 4, .production_id = 7),
  [316] = {.entry = {.count = 1, .reusable = true}}, SHIFT(48),
  [318] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_viewbox_obj, 2, .production_id = 10),
  [320] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_action_obj, 8, .production_id = 9),
  [322] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_slide_function, 3, .production_id = 7),
  [324] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_action_obj, 6, .production_id = 9),
  [326] = {.entry = {.count = 1, .reusable = true}}, SHIFT(131),
  [328] = {.entry = {.count = 1, .reusable = true}}, SHIFT(30),
  [330] = {.entry = {.count = 1, .reusable = true}}, SHIFT(123),
  [332] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_slide_obj, 6, .production_id = 6),
  [334] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_slide_function, 5, .production_id = 7),
  [336] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_slide_function, 6, .production_id = 7),
  [338] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [340] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_viewbox_obj, 3, .production_id = 13),
  [342] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_action_obj, 7, .production_id = 9),
  [344] = {.entry = {.count = 1, .reusable = true}}, SHIFT(127),
  [346] = {.entry = {.count = 1, .reusable = true}}, SHIFT(125),
  [348] = {.entry = {.count = 1, .reusable = true}}, SHIFT(140),
  [350] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_slide_objects, 4, .production_id = 4),
  [352] = {.entry = {.count = 1, .reusable = true}}, SHIFT(107),
  [354] = {.entry = {.count = 1, .reusable = true}}, SHIFT(55),
  [356] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_slide_objects, 5, .production_id = 4),
  [358] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_slide_objects, 4, .production_id = 2),
  [360] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [362] = {.entry = {.count = 1, .reusable = true}}, SHIFT(135),
  [364] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_slide_objects, 2),
  [366] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_slide_objects, 3),
  [368] = {.entry = {.count = 1, .reusable = true}}, SHIFT(72),
  [370] = {.entry = {.count = 1, .reusable = true}}, SHIFT(37),
  [372] = {.entry = {.count = 1, .reusable = true}}, SHIFT(118),
  [374] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [376] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [378] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_slide_objects, 3, .production_id = 2),
  [380] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
  [382] = {.entry = {.count = 1, .reusable = true}}, SHIFT(32),
  [384] = {.entry = {.count = 1, .reusable = true}}, SHIFT(22),
  [386] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [388] = {.entry = {.count = 1, .reusable = true}}, SHIFT(34),
  [390] = {.entry = {.count = 1, .reusable = true}}, SHIFT(150),
};

#ifdef __cplusplus
extern "C" {
#endif
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
    .primary_state_ids = ts_primary_state_ids,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
