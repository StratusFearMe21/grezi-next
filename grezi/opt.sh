#!/bin/sh
wasm-opt -O4 --enable-simd --enable-nontrapping-float-to-int --enable-relaxed-simd --enable-threads --enable-reference-types --enable-extended-const --enable-bulk-memory dist/grezi-*_bg.wasm -o dist/grezi-*_bg.wasm
