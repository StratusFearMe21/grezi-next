package tree_sitter_grz_test

import (
	"testing"

	tree_sitter "github.com/tree-sitter/go-tree-sitter"
	tree_sitter_grz "github.com/stratusfearme21/tree-sitter-grz/bindings/go"
)

func TestCanLoadGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_grz.Language())
	if language == nil {
		t.Errorf("Error loading Grz grammar")
	}
}
