package tree_sitter_grz_test

import (
	"testing"

	tree_sitter "github.com/smacker/go-tree-sitter"
	"github.com/tree-sitter/tree-sitter-grz"
)

func TestCanLoadGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_grz.Language())
	if language == nil {
		t.Errorf("Error loading Grz grammar")
	}
}
