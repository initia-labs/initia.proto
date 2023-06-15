#!/bin/bash
set -o errexit -o nounset -o pipefail
command -v shellcheck >/dev/null && shellcheck "$0"

OUT_DIR="./src"

mkdir -p "$OUT_DIR"

echo "Processing initia proto files ..."

#INITIA_DIR=`realpath ../initia/proto` \
#INITIA_THIRD_PARTY_DIR=`realpath ../initia/third_party/proto` \
#  dotnet build

INITIA_DIR=`realpath ../initia/proto`
INITIA_THIRD_PARTY_DIR=`realpath ../initia/third_party/proto`
protoc \
  --plugin=protoc-gen-grpc=${CSHARP_PLUGIN} \
  --csharp_out="${OUT_DIR}" \
  --csharp_opt=file_extension=.g.cs,base_namespace=,serializable \
  --proto_path="$INITIA_DIR" \
  --proto_path="$INITIA_THIRD_PARTY_DIR" \
  $(find ${INITIA_DIR} ${INITIA_THIRD_PARTY_DIR} -path -prune -o -name '*.proto' -print0 | xargs -0)

echo "Removing Google/Protobuf: conflicts with Grpc.Protobuf"
rm -rf "${OUT_DIR}/Google/Protobuf"
