#!/bin/sh
file=""
copy="nil"

if [ -n "$1" ]; then
  copy="$1"
fi

if [ -n "$2" ]; then
  file="$2"
else
  file="dist/"
fi

echo "$file"
if [ -d "$file" ]; then
  if [ -d "$copy" ]; then
    cp "$copy"/*.slideshow "$copy"/*.pdf dist/
  fi

  find "$file" \
    -name "*.js" -o \
    -name "*.slideshow" -o \
    -name "*.pdf" -o \
    -name "*.ico" -o \
    -name "*.wasm" -o \
    -name "*.html" -o \
    -name "*.json" | parallel sh opt.sh "$copy" '{}'
else
  case `basename "$file"` in
  *.wasm)
      wasm-opt -O2 --fast-math --enable-simd "$file" -o "$file"
      ;;
  *)
  esac
  
  brotli -f "$file"
  rm -rf "$file"
fi
