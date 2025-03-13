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
- Errors and diagnostics support
- Hot reloading as you type
- Folding Ranges

You can use the LSP by runing grezi with the `--lsp` argument

### Note about VSCode
There's a vscode extension under the `grezi-vscode/` folder. Once you install it as a VSIX in your vscode instance, you need to point it at the location of the `grezi` binary for it to work.

### Cairo based exporting
Grezi allows you to export your presentation to PDF, PostScript, and many image formats using the `cairo` library. Just use the export flag like so
```sh
grezi -eo new.pdf new.grz 
grezi -eo new.ps new.grz 
grezi -eo new.png -i 6 new.grz 
grezi -eo new.png -i 6..8 new.grz 
grezi -eo new.png -i 6..8 -s 512 new.grz 
grezi -eo new.png -i 6..8 -s 1920x1080 new.grz 
```
Warning: You can also export to SVG, however, this method is not very reliable.

### Web Runtime (coming soon)
Grezi can export to various static formats, but you can also share the animated version of your presentations
via the WebAssembly runtime. Using the runtime is easy
1. Install these
  - [trunk](https://trunkrs.dev)
  - brotli
  - wasm-opt
  - GNU parallel
2. Build the WebAssembly runtime
```
trunk build --release
```
3. Export any slideshows you want to share to the `.slideshow` format (A custom format that packs your slideshow, all used fonts, and all used images together into one file)
```
grezi -eo new.slideshow new.grz
grezi -eo another.slideshow another.grz
```
4. Move the slideshows you want to share into the `dist/` directory that `trunk` created in step 2
```
mv new.slideshow dist/
mv another.slideshow dist/
```
5. Run the `opt.sh` script to compress everything and optimize the WASM binary
```
sh opt.sh
```
Optionally, pass a directory to `opt.sh` to move all the slideshow files in it to the dist directory
```
sh opt.sh ~/slideshows/
```
6. Serve the `dist/` directory with the server of your choice and go to this URL
```
https://{YOUR DOMAIN}/index.html#{A SLIDESHOW}.slideshow
```
