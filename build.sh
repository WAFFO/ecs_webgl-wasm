#!/bin/sh

set -ex
cd "$(dirname $0)"

rustup target add wasm32-unknown-unknown --toolchain stable

if ( ! command -v wasm-bindgen )
then
    cargo install wasm-bindgen-cli
fi

cargo build -p client --target wasm32-unknown-unknown
cargo build -p server

wasm-bindgen ./target/wasm32-unknown-unknown/debug/client.wasm --out-dir ./client/js/wasm
npm run --prefix ./client/js build

cp ./target/debug/server.exe ./www