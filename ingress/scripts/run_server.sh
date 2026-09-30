#!/usr/bin/env bash

SCRIPTPATH="$(
	cd "$(dirname "$0")"
	pwd -P
)"
cd $SCRIPTPATH
cd ../..
cd frontend
bun install
bun run build
cd ../ingress
# cargo clean
cargo run
