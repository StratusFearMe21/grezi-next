#!/bin/sh
wasm=dist/grezi-*_bg.wasm
js=dist/grezi-*.js
wasm-opt -O2 --fast-math $wasm -o $wasm
sed -i "s/grezi.js/$(basename $js)/g; s/grezi_bg.wasm/$(basename $wasm)/g" dist/sw.js
find dist/ \
  -name "*.js" -o \
  -name "*.slideshow" -o \
  -name "*.ico" -o \
  -name "*.wasm" -o \
  -name "*.html" -o \
  -name "*.json" | parallel brotli -f
