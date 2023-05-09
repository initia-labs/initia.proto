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

GRPC_PROTOC_PLUGIN=${CSHARP_PLUGIN} \
INITIA_DIR=`realpath ../initia/proto` \
INITIA_THIRD_PARTY_DIR=`realpath ../initia/third_party/proto` \
  dotnet build