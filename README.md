# grezi-next
A re-done and re-factored verison of my old presentation software I wrote in 10th grade

Grezi allows you to write presentations using plain text. It's designed to make presentations that are very engaging, with little effort on your part.

**Note that:** Grezi depends on the Helix text editor for syntax highlighting. If you don't have Helix, you can find a binary [here](https://github.com/helix-editor/helix/releases/latest)

## Features

### IDE Support:
Grezi is designed to be used with an IDE ([Helix](https://github.com/helix-editor/helix) is what I've been using). Not only does the language have syntax highlighting and indentation hints thanks to the `tree-sitter` parser, but it also has an embedded language server that makes the process of making slideshows easier. Features of the LSP include
- Rename symbol support
- Goto declaration support
- Goto reference support
- Code formatting
- Semantic tokens support (Syntax highlighting in non tree-sitter editors, eg. VSCode)
- Inlay hints for edges and slide numbers
- Errors and diagnostics support
- Hot reloading as you type
- Snippet for copying down the previous slide and removing exited objects
- Automatic "Hello World" boilerplate for empty files

You can use the LSP by runing grezi with the `--lsp` argument

### Note about VSCode
After making a VSCode extension, I found that everything in the LSP works except for completions. For now, until
I figure that out, Grezi's LSP doesn't officially support VSCode.
Try [Helix](https://github.com/helix-editor/helix)

### Hot Reloading:
Grezi allows you to hot reload your presentation *as* you create it. There are 2 hot reloading implementations
- LSP (ideal): Just run the LSP, every keystroke you make will be reflected in the slideshow window.
- Watching: When not in LSP mode, Grezi will watch the open presentation for changes. This implementation is slower than using the LSP hot reloading, but if you don't have an IDE, this should work for you. 

### Cairo based exporting
Grezi allows you to export your presentation to PDF/PostScript using the `cairo` library. Just use the export flag
```sh
grezi -eo new.pdf new.grz 
grezi -eo new.ps new.grz 
grezi -eo new.png -i 6 new.grz 
grezi -eo new.png -i 6..8 new.grz 
grezi -eo new.png -i 6..8 -s 512 new.grz 
grezi -eo new.png -i 6..8 -s 1920x1080 new.grz 
```
Warning: You can also export to SVG, however, this method is not very reliable.
