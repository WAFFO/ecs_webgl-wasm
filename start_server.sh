#!/bin/sh

set -ex
cd "$(dirname $0)/www"

unameOut="$(uname -s)";
case "${unameOut}" in
    Linux*)     machine=Linux;;
    Darwin*)    machine=Mac;;
    CYGWIN*)    machine=Cygwin;;
    MINGW*)     machine=MinGw;;
    *)          machine="UNKNOWN:${unameOut}"
esac

function cleanup {
    if [[ "${machine}" == "MinGw" ]] || [[ "${machine}" == "Cygwin" ]]; then
        taskkill -F -IM node.exe
    fi;
    kill %1
    kill %2
}

trap cleanup SIGINT
cargo run --bin server 2>&1 | sed -e 's/^/[Cargo] /' \
  & npm --prefix ../client/js/ run serve 2>&1 | sed -e 's/^/[Webpack] /'

cd "$(dirname $0)"