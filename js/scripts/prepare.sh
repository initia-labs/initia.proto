#!/bin/bash
set -o errexit -o nounset -o pipefail
command -v shellcheck >/dev/null && shellcheck "$0"

DIRS="amino cosmos cosmos_proto gogoproto google ibc initia tendermint cosmwasm sdk connect miniwasm minievm celestia noble"

for dir in $DIRS; do
  rm -rf "$dir"
  cp -R "./build/$dir" ./
done
