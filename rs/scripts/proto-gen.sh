#!/bin/bash
set -o errexit -o nounset -o pipefail
command -v shellcheck >/dev/null && shellcheck "$0"

export OUT_DIR="./src/protos"
mkdir -p "$OUT_DIR"

echo "Processing initia proto files ..."
export INITIA_DIR="../initia/proto"
export INITIA_THIRD_PARTY_DIR="../initia/third_party/proto"

## Path to this plugin, Note this must be an abolsute path on Windows (see #15)
#PROTOC_GEN_TS_PROTO_PATH="./node_modules/.bin/protoc-gen-ts_proto"
protoc \
    --prost_out=${OUT_DIR} \
    --proto_path="$INITIA_DIR" \
    --proto_path="$INITIA_THIRD_PARTY_DIR" \
    $(find ${INITIA_DIR} ${INITIA_THIRD_PARTY_DIR} -path -prune -o -name '*.proto' -print0 | xargs -0)