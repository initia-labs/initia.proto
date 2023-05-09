#!/bin/bash
set -o nounset -o pipefail
command -v shellcheck >/dev/null && shellcheck "$0"

OUT_DIR="./src"

which grpc_cpp_plugin > /dev/null
if [ $? -ne 0 ]; then
  echo "grpc_cpp_plugin not found. Please install it first." 2>&1
  exit 1
fi

set -o errexit -o nounset -o pipefail

mkdir -p "$OUT_DIR"

echo "Processing initia proto files ..."
INITIA_DIR="../initia/proto"
INITIA_THIRD_PARTY_DIR="../initia/third_party/proto"

protoc \
  --plugin=protoc-gen-grpc=`which grpc_cpp_plugin` \
  --grpc_out="${OUT_DIR}" \
  --proto_path="$INITIA_DIR" \
  --proto_path="$INITIA_THIRD_PARTY_DIR" \
  $(find ${INITIA_DIR} ${INITIA_THIRD_PARTY_DIR} -path -prune -o -name '*.proto' -print0 | xargs -0)
protoc \
  --cpp_out="${OUT_DIR}" \
  --proto_path="$INITIA_DIR" \
  --proto_path="$INITIA_THIRD_PARTY_DIR" \
  $(find ${INITIA_DIR} ${INITIA_THIRD_PARTY_DIR} -path -prune -o -name '*.proto' -print0 | xargs -0)
