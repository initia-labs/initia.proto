#!/bin/bash
set -o errexit -o nounset -o pipefail
command -v shellcheck >/dev/null && shellcheck "$0"

echo "install betterproto... pre-release for now. stable one has some issues"
#pip install --upgrade "avast.betterproto[compiler]"
pip install --upgrade "betterproto[compiler]" --pre
echo "install MarkupSafe==2.0.1 due to dependency"
pip install MarkupSafe==2.0.1

OUT_DIR="./initia_proto"

mkdir -p "$OUT_DIR"

echo "Processing initia proto files ..."
INITIA_DIR="../initia/proto"
COSMOS_DIR="../cosmos-sdk/proto"
COSMOS_PROTO_DIR="../cosmos-proto/proto"
IBC_DIR="../ibc-go/proto"
ICS_DIR="../ics23/proto"
THIRDPARTY_DIR="../third_party"


protoc \
  --proto_path="$INITIA_DIR" \
  --proto_path="$COSMOS_DIR" \
  --proto_path="$COSMOS_PROTO_DIR" \
  --proto_path="$IBC_DIR" \
  --proto_path="$ICS_DIR" \
  --proto_path="$THIRDPARTY_DIR" \
  --python_betterproto_out="${OUT_DIR}" \
  $(find ${INITIA_DIR} ${COSMOS_DIR} ${COSMOS_PROTO_DIR} ${IBC_DIR} ${ICS_DIR} ${THIRDPARTY_DIR} -path -prune -o -name '*.proto' -print0 | xargs -0)
