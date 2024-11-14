#!/bin/bash
set -o errexit -o nounset -o pipefail
command -v shellcheck >/dev/null && shellcheck "$0"

OUT_DIR="./src"

# Path to this plugin, Note this must be an abolsute path on Windows (see #15)
PROTOC_GEN_TS_PROTO_PATH="./node_modules/.bin/protoc-gen-ts_proto"

mkdir -p "$OUT_DIR"

echo "Processing initia proto files ..."
INITIA_DIR="../initia/proto"
MINIWASM_DIR="../miniwasm/proto"
MINIEVM_DIR="../minievm/proto"
COSMOS_DIR="../cosmos-sdk/proto"
COSMOS_PROTO_DIR="../cosmos-proto/proto"
IBC_DIR="../ibc-go/proto"
ICS_DIR="../ics23/proto"
WASMD_DIR="../wasmd/proto"
SDK_DIR="../block-sdk/proto"
CONNECT_DIR="../connect/proto"
FORWARDING_DIR="../forwarding/proto"
THIRDPARTY_DIR="../third_party"

protoc \
  --plugin="protoc-gen-ts_proto=${PROTOC_GEN_TS_PROTO_PATH}" \
  --ts_proto_out="${OUT_DIR}" \
  --ts_proto_opt="useMapType=true,useExactTypes=false,esModuleInterop=true,forceLong=bigint,useOptionals=true,outputClientImpl=grpc-web,globalThisPolyfill=true" \
  --proto_path="$INITIA_DIR" \
  --proto_path="$MINIWASM_DIR" \
  --proto_path="$MINIEVM_DIR" \
  --proto_path="$COSMOS_DIR" \
  --proto_path="$COSMOS_PROTO_DIR" \
  --proto_path="$IBC_DIR" \
  --proto_path="$ICS_DIR" \
  --proto_path="$WASMD_DIR" \
  --proto_path="$SDK_DIR" \
  --proto_path="$CONNECT_DIR" \
  --proto_path="$FORWARDING_DIR" \
  --proto_path="$THIRDPARTY_DIR" \
  $(find ${INITIA_DIR} ${MINIWASM_DIR} ${MINIEVM_DIR} ${COSMOS_DIR} ${COSMOS_PROTO_DIR} ${IBC_DIR} ${ICS_DIR} ${WASMD_DIR} ${SDK_DIR} ${CONNECT_DIR} ${FORWARDING_DIR} ${THIRDPARTY_DIR} -path -prune -o -name '*.proto' -print0 | xargs -0)
