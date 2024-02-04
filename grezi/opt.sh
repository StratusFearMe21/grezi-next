#!/bin/sh
wasm=dist/grezi-*_bg.wasm
js=dist/grezi-*.js
if [ -d $1 ]; then
  cp $1/*.slideshow $1/*.pdf dist/
fi
wasm-opt -O2 --fast-math $wasm -o $wasm
find dist/ \
  -name "*.js" -o \
  -name "*.slideshow" -o \
  -name "*.pdf" -o \
  -name "*.ico" -o \
  -name "*.wasm" -o \
  -name "*.html" -o \
  -name "*.json" | parallel brotli -f
