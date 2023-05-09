#!/bin/bash
set -o nounset -o pipefail
command -v shellcheck >/dev/null && shellcheck "$0"

OUT_DIR="./src"
CSHARP_PLUGIN=`which grpc_csharp_plugin`
if [ -z "$CSHARP_PLUGIN" ]; then
  echo "grpc_csharp_plugin not found. Please install it first."
  exit 1
fi

set -o errexit -o nounset -o pipefail

mkdir -p "$OUT_DIR"

echo "Processing initia proto files ..."
INITIA_DIR="../initia/proto"
INITIA_THIRD_PARTY_DIR="../initia/third_party/proto"

protoc \
  --plugin=protoc-gen-grpc=${CSHARP_PLUGIN} \
  --csharp_out="${OUT_DIR}" \
  --csharp_opt=file_extension=.g.cs,serializable \
  --grpc_out="${OUT_DIR}" \
  --grpc_opt=no_server \
  --proto_path="$INITIA_DIR" \
  --proto_path="$INITIA_THIRD_PARTY_DIR" \
  $(find ${INITIA_DIR} ${INITIA_THIRD_PARTY_DIR} -path -prune -o -name '*.proto' -print0 | xargs -0)
