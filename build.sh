#! /bin/bash

set -exuo pipefail

TARGET=wasm32-unknown-unknown
BINARY=target/$TARGET/release/wasm_julia.wasm

cargo build --target $TARGET --release
wasm-strip $BINARY
mkdir -p www
wasm-opt -o www/wasm_julia.wasm -Oz $BINARY
ls -lh $BINARY www/wasm_julia.wasm
