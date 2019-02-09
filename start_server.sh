#!/bin/sh

set -ex
cd "$(dirname $0)/www"

./server.exe

cd "$(dirname $0)"
