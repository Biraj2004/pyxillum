#!/usr/bin/env bash
set -euo pipefail

echo "=== Building Pyxillum macOS Universal Rust Dynamic Library & XCFramework ==="

# Build Apple Silicon (aarch64) and Intel (x86_64) cdylibs
cargo build --release --target aarch64-apple-darwin -p pyxillum-ffi
cargo build --release --target x86_64-apple-darwin -p pyxillum-ffi

mkdir -p target/universal-apple-darwin/release
lipo -create -output target/universal-apple-darwin/release/libpyxillum_ffi.dylib \
  target/aarch64-apple-darwin/release/libpyxillum_ffi.dylib \
  target/x86_64-apple-darwin/release/libpyxillum_ffi.dylib

echo "=== Universal Mach-O dynamic library created at target/universal-apple-darwin/release/libpyxillum_ffi.dylib ==="
