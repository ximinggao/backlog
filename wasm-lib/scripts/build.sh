#!/bin/bash

SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd $SCRIPTPATH/..
cargo build --release --target wasm32-wasip1
# cp ../target/wasm32-wasip1/release/wasm_lib.wasm ./wasm_lib.wasm
wasm2wat ../target/wasm32-wasip1/release/wasm_lib.wasm -o ../target/wasm32-wasip1/release/wasm_lib.wat
