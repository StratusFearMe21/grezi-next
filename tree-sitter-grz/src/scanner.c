#include "tree_sitter/parser.h"

enum TokenType {
  STRING_CONTENT,
  RAW_STRING_CONTENT,
  OBJ_OTHER,
};

void *tree_sitter_grz_external_scanner_create() { return NULL; }
void tree_sitter_grz_external_scanner_destroy(void *p) {}
void tree_sitter_grz_external_scanner_reset(void *p) {}
unsigned tree_sitter_grz_external_scanner_serialize(void *p, char *buffer) {
  return 0;
}
void tree_sitter_grz_external_scanner_deserialize(void *p, const char *b,
                                                  unsigned n) {}

static void advance(TSLexer *lexer) { lexer->advance(lexer, false); }

bool tree_sitter_grz_external_scanner_scan(void *payload, TSLexer *lexer,
                                           const bool *valid_symbols) {
  if (valid_symbols[STRING_CONTENT]) {
    bool has_content = false;
    for (;;) {
      if (lexer->lookahead == '\"' || lexer->lookahead == '\\') {
        break;
      } else if (lexer->lookahead == 0) {
        return false;
      }
      has_content = true;
      advance(lexer);
    }
    lexer->result_symbol = STRING_CONTENT;
    return has_content;
  }
  if (valid_symbols[OBJ_OTHER]) {
    switch (lexer->lookahead) {
    case 'r':
      advance(lexer);
      if (lexer->lookahead == '#') {
        return false;
      }
      break;
    case '0':
    case '1':
    case '2':
    case '3':
    case '4':
    case '5':
    case '6':
    case '7':
    case '8':
    case '9':
    case '\"':
    case '\t':
    case '\n':
    case '\r':
    case ' ':
      return false;
    }
    int delimeters = 0;
    bool in_whitespace = false;
    lexer->result_symbol = OBJ_OTHER;
    for (;;) {
      switch (lexer->lookahead) {
      case '\t':
      case '\n':
      case '\r':
      case ' ':
        if (!in_whitespace) {
          lexer->mark_end(lexer);
          in_whitespace = true;
        }
        advance(lexer);
        continue;
      case '(':
      case '[':
      case '{':
        delimeters += 1;
        break;
      case ')':
      case ']':
      case '}':
        delimeters -= 1;
        if (delimeters == -1) {
          if (!in_whitespace)
            lexer->mark_end(lexer);
          return true;
        }
        in_whitespace = false;
        advance(lexer);
        lexer->mark_end(lexer);
        continue;
      case ',':
      case '>':
        if (delimeters <= 0) {
          if (!in_whitespace)
            lexer->mark_end(lexer);
          return true;
        }
        break;
      case 0:
        return false;
      }
      in_whitespace = false;
      advance(lexer);
    }
  }
  if (valid_symbols[RAW_STRING_CONTENT]) {
    bool has_content = false;
    for (;;) {
      if (lexer->lookahead == 0) {
        return false;
      } else if (lexer->lookahead == '"') {
        lexer->mark_end(lexer);
        advance(lexer);
        if (lexer->lookahead == '#') {
          break;
        } else {
          advance(lexer);
        }
      }
      has_content = true;
      advance(lexer);
    }
    lexer->result_symbol = RAW_STRING_CONTENT;
    return has_content;
  } else {
    return false;
  }
}
