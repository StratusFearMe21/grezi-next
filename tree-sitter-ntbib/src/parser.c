#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 13
#define LARGE_STATE_COUNT 4
#define SYMBOL_COUNT 11
#define ALIAS_COUNT 0
#define TOKEN_COUNT 6
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 0
#define MAX_ALIAS_SEQUENCE_LENGTH 3
#define PRODUCTION_ID_COUNT 1

enum ts_symbol_identifiers {
  sym_content = 1,
  anon_sym_LT = 2,
  anon_sym_GT = 3,
  anon_sym_LT_SLASH = 4,
  sym_tag = 5,
  sym_source_file = 6,
  sym_element = 7,
  sym_tag_start = 8,
  sym_tag_end = 9,
  aux_sym_source_file_repeat1 = 10,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [sym_content] = "content",
  [anon_sym_LT] = "<",
  [anon_sym_GT] = ">",
  [anon_sym_LT_SLASH] = "</",
  [sym_tag] = "tag",
  [sym_source_file] = "source_file",
  [sym_element] = "element",
  [sym_tag_start] = "tag_start",
  [sym_tag_end] = "tag_end",
  [aux_sym_source_file_repeat1] = "source_file_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [sym_content] = sym_content,
  [anon_sym_LT] = anon_sym_LT,
  [anon_sym_GT] = anon_sym_GT,
  [anon_sym_LT_SLASH] = anon_sym_LT_SLASH,
  [sym_tag] = sym_tag,
  [sym_source_file] = sym_source_file,
  [sym_element] = sym_element,
  [sym_tag_start] = sym_tag_start,
  [sym_tag_end] = sym_tag_end,
  [aux_sym_source_file_repeat1] = aux_sym_source_file_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [sym_content] = {
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
  [anon_sym_LT_SLASH] = {
    .visible = true,
    .named = false,
  },
  [sym_tag] = {
    .visible = true,
    .named = true,
  },
  [sym_source_file] = {
    .visible = true,
    .named = true,
  },
  [sym_element] = {
    .visible = true,
    .named = true,
  },
  [sym_tag_start] = {
    .visible = true,
    .named = true,
  },
  [sym_tag_end] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_source_file_repeat1] = {
    .visible = false,
    .named = false,
  },
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
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      ACCEPT_TOKEN(sym_content);
      if (eof) ADVANCE(2);
      if (lookahead == '/') ADVANCE(4);
      if (lookahead == '<') ADVANCE(6);
      if (lookahead == '>') ADVANCE(8);
      if (lookahead != 0) ADVANCE(3);
      END_STATE();
    case 1:
      if (eof) ADVANCE(2);
      if (lookahead == '<') ADVANCE(5);
      if (lookahead == '>') ADVANCE(7);
      END_STATE();
    case 2:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 3:
      ACCEPT_TOKEN(sym_content);
      if (lookahead == '<') ADVANCE(10);
      if (lookahead == '/' ||
          lookahead == '>') ADVANCE(4);
      if (lookahead != 0) ADVANCE(3);
      END_STATE();
    case 4:
      ACCEPT_TOKEN(sym_content);
      if (lookahead != 0 &&
          lookahead != '<') ADVANCE(4);
      END_STATE();
    case 5:
      ACCEPT_TOKEN(anon_sym_LT);
      if (lookahead == '/') ADVANCE(9);
      END_STATE();
    case 6:
      ACCEPT_TOKEN(anon_sym_LT);
      if (lookahead == '/') ADVANCE(9);
      if (lookahead != 0 &&
          lookahead != '>') ADVANCE(10);
      END_STATE();
    case 7:
      ACCEPT_TOKEN(anon_sym_GT);
      END_STATE();
    case 8:
      ACCEPT_TOKEN(anon_sym_GT);
      if (lookahead != 0 &&
          lookahead != '<') ADVANCE(4);
      END_STATE();
    case 9:
      ACCEPT_TOKEN(anon_sym_LT_SLASH);
      END_STATE();
    case 10:
      ACCEPT_TOKEN(sym_tag);
      if (lookahead != 0 &&
          lookahead != '/' &&
          lookahead != '>') ADVANCE(10);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 1},
  [2] = {.lex_state = 1},
  [3] = {.lex_state = 1},
  [4] = {.lex_state = 1},
  [5] = {.lex_state = 10},
  [6] = {.lex_state = 10},
  [7] = {.lex_state = 0},
  [8] = {.lex_state = 4},
  [9] = {.lex_state = 1},
  [10] = {.lex_state = 1},
  [11] = {.lex_state = 4},
  [12] = {.lex_state = 4},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [sym_content] = ACTIONS(1),
    [anon_sym_LT] = ACTIONS(1),
    [anon_sym_GT] = ACTIONS(1),
    [anon_sym_LT_SLASH] = ACTIONS(1),
    [sym_tag] = ACTIONS(1),
  },
  [1] = {
    [sym_source_file] = STATE(7),
    [sym_element] = STATE(2),
    [sym_tag_start] = STATE(8),
    [sym_tag_end] = STATE(8),
    [aux_sym_source_file_repeat1] = STATE(2),
    [ts_builtin_sym_end] = ACTIONS(3),
    [anon_sym_LT] = ACTIONS(5),
    [anon_sym_LT_SLASH] = ACTIONS(7),
  },
  [2] = {
    [sym_element] = STATE(3),
    [sym_tag_start] = STATE(8),
    [sym_tag_end] = STATE(8),
    [aux_sym_source_file_repeat1] = STATE(3),
    [ts_builtin_sym_end] = ACTIONS(9),
    [anon_sym_LT] = ACTIONS(5),
    [anon_sym_LT_SLASH] = ACTIONS(7),
  },
  [3] = {
    [sym_element] = STATE(3),
    [sym_tag_start] = STATE(8),
    [sym_tag_end] = STATE(8),
    [aux_sym_source_file_repeat1] = STATE(3),
    [ts_builtin_sym_end] = ACTIONS(11),
    [anon_sym_LT] = ACTIONS(13),
    [anon_sym_LT_SLASH] = ACTIONS(16),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 2,
    ACTIONS(21), 1,
      anon_sym_LT,
    ACTIONS(19), 2,
      ts_builtin_sym_end,
      anon_sym_LT_SLASH,
  [8] = 1,
    ACTIONS(23), 1,
      sym_tag,
  [12] = 1,
    ACTIONS(25), 1,
      sym_tag,
  [16] = 1,
    ACTIONS(27), 1,
      ts_builtin_sym_end,
  [20] = 1,
    ACTIONS(29), 1,
      sym_content,
  [24] = 1,
    ACTIONS(31), 1,
      anon_sym_GT,
  [28] = 1,
    ACTIONS(33), 1,
      anon_sym_GT,
  [32] = 1,
    ACTIONS(35), 1,
      sym_content,
  [36] = 1,
    ACTIONS(37), 1,
      sym_content,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(4)] = 0,
  [SMALL_STATE(5)] = 8,
  [SMALL_STATE(6)] = 12,
  [SMALL_STATE(7)] = 16,
  [SMALL_STATE(8)] = 20,
  [SMALL_STATE(9)] = 24,
  [SMALL_STATE(10)] = 28,
  [SMALL_STATE(11)] = 32,
  [SMALL_STATE(12)] = 36,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 0),
  [5] = {.entry = {.count = 1, .reusable = false}}, SHIFT(5),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
  [9] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1),
  [11] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2),
  [13] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(5),
  [16] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(6),
  [19] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_element, 2),
  [21] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_element, 2),
  [23] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [25] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
  [27] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [29] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [31] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [33] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [35] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tag_start, 3),
  [37] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tag_end, 3),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef TREE_SITTER_HIDE_SYMBOLS
#define TS_PUBLIC
#elif defined(_WIN32)
#define TS_PUBLIC __declspec(dllexport)
#else
#define TS_PUBLIC __attribute__((visibility("default")))
#endif

TS_PUBLIC const TSLanguage *tree_sitter_ntbib() {
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
    .symbol_metadata = ts_symbol_metadata,
    .public_symbol_map = ts_symbol_map,
    .alias_map = ts_non_terminal_alias_map,
    .alias_sequences = &ts_alias_sequences[0][0],
    .lex_modes = ts_lex_modes,
    .lex_fn = ts_lex,
    .primary_state_ids = ts_primary_state_ids,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
