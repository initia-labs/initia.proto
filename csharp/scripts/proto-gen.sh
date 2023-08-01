#!/bin/bash
set -o errexit -o nounset -o pipefail
command -v shellcheck >/dev/null && shellcheck "$0"

OUT_DIR="./src"

mkdir -p "$OUT_DIR"

echo "Processing initia proto files ..."

#INITIA_DIR=`realpath ../initia/proto` \
#INITIA_THIRD_PARTY_DIR=`realpath ../initia/third_party/proto` \
#  dotnet build

INITIA_DIR="../initia/proto"
COSMOS_DIR="../cosmos-sdk/proto"
COSMOS_PROTO_DIR="../cosmos-proto/proto"
IBC_DIR="../ibc-go/proto"
ICS_DIR="../ics23/proto"
POB_DIR="../pob/proto"
THIRDPARTY_DIR="../third_party"

CSHARP_PLUGIN=`which grpc_csharp_plugin`
protoc \
  --plugin=protoc-gen-grpc=${CSHARP_PLUGIN} \
  --csharp_out="${OUT_DIR}" \
  --csharp_opt=file_extension=.g.cs,base_namespace=,serializable \
  --proto_path="$INITIA_DIR" \
  --proto_path="$COSMOS_DIR" \
  --proto_path="$COSMOS_PROTO_DIR" \
  --proto_path="$IBC_DIR" \
  --proto_path="$ICS_DIR" \
  --proto_path="$POB_DIR" \
  --proto_path="$THIRDPARTY_DIR" \
  $(find ${INITIA_DIR} ${COSMOS_DIR} ${COSMOS_PROTO_DIR} ${IBC_DIR} ${ICS_DIR} ${POB_DIR} ${THIRDPARTY_DIR} -path -prune -o -name '*.proto' -print0 | xargs -0)

echo "Removing Google/Protobuf: conflicts with Grpc.Protobuf"
rm -rf "${OUT_DIR}/Google/Protobuf"
