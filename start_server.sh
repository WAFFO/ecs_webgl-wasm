#!/bin/sh

set -ex
cd "$(dirname $0)/www"

trap 'kill %1' SIGINT
cargo run --bin server 2>&1 | sed -e 's/^/[Cargo] /' \
  & npm --prefix ../client/js/ run serve 2>&1 | sed -e 's/^/[Webpack] /'

cd "$(dirname $0)"
