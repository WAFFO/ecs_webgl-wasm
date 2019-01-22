#!/bin/bash

cd "$(dirname $0)"

cargo update;
cargo install -f wasm-bindgen-cli;
