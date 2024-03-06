#!/bin/bash
set -o nounset -o pipefail
command -v shellcheck >/dev/null && shellcheck "$0"

OUT_DIR="./src"

set -o errexit -o nounset -o pipefail

mkdir -p "$OUT_DIR"

echo "Processing initia proto files ..."
INITIA_DIR="../initia/proto"
COSMOS_DIR="../cosmos-sdk/proto"
COSMOS_PROTO_DIR="../cosmos-proto/proto"
IBC_DIR="../ibc-go/proto"
ICS_DIR="../ics23/proto"
WASMD_DIR="../wasmd/proto"
SDK_DIR="../block-sdk/proto"
SLINKY_DIR="../slinky/proto"
THIRDPARTY_DIR="../third_party"

protoc \
  --plugin="`which protoc-gen-quickbuf`" \
  --quickbuf_opt="allocation=lazy" \
  --quickbuf_out="${OUT_DIR}" \
  --proto_path="$INITIA_DIR" \
  --proto_path="$COSMOS_DIR" \
  --proto_path="$COSMOS_PROTO_DIR" \
  --proto_path="$IBC_DIR" \
  --proto_path="$ICS_DIR" \
  --proto_path="$WASMD_DIR" \
  --proto_path="$SDK_DIR" \
  --proto_path="$SLINKY_DIR" \
  --proto_path="$THIRDPARTY_DIR" \
  $(find ${INITIA_DIR} ${COSMOS_DIR} ${COSMOS_PROTO_DIR} ${IBC_DIR} ${ICS_DIR} ${WASMD_DIR} ${SDK_DIR} ${SLINKY_DIR} ${THIRDPARTY_DIR} -path -prune -o -name '*.proto' -print0 | xargs -0)
