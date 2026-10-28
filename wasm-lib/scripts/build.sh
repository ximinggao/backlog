#!/bin/bash

SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd $SCRIPTPATH/..
cargo build --release --target wasm32-wasip1
cp ../target/wasm32-wasip1/release/wasm_lib.wasm ./wasm_lib.wasm
wasm2wat ./wasm_lib.wasm -o ./wasm_lib.wat
