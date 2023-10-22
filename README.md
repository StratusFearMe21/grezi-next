# grezi-next
A re-done and re-factored verison of my old presentation software I wrote in 10th grade

Grezi allows you to write presentations using plain text. It's designed to make presentations that are very engaging, with little effort on your part.

## Features
- IDE Support:
    Grezi is designed to be used with an IDE ([Helix](https://github.com/helix-editor/helix) is what I've been using). Not only does the language have syntax highlighting and indentation hints thanks to the `tree-sitter` parser, but it also has a very basic language server that makes the process of making slideshows easier. Features of the LSP include
    - Rename symbol support
    - Goto declaration support
    - Goto reference support
    - Semantic tokens support (Syntax highlighting in non tree-sitter editors, eg. VSCode)
    - Errors and diagnostics support
    - Hot reloading on save
    - Snippet for copying down the previous slide and removing exited objects
    - Automatic "Hello World" boilerplate for empty files
    You can use the LSP by runing grezi with the `--lsp` argument
- Hot Reloading:
    Grezi allows you to hot reload your presentation *as* you create it. There are 2 hot reloading implementations
    - LSP (ideal): Just run the LSP
    - Watching: When not in LSP mode, Grezi will watch the open presentation for changes. This implementation is slower than using the LSP hot reloading, but if you don't have an IDE, this should work for you. 