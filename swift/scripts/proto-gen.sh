#!/bin/bash
set -o errexit -o nounset -o pipefail
command -v shellcheck >/dev/null && shellcheck "$0"

OUT_DIR="./src"

mkdir -p "$OUT_DIR"

echo "Processing initia proto files ..."
INITIA_DIR="../initia/proto"
COSMOS_DIR="../cosmos-sdk/proto"
COSMOS_PROTO_DIR="../cosmos-proto/proto"
IBC_DIR="../ibc-go/proto"
ICS_DIR="../ics23/proto"
THIRDPARTY_DIR="../third_party"

# See https://github.com/apple/swift-protobuf/issues/83
protoc \
  --plugin=`which protoc-gen-swift` \
  --swift_out="${OUT_DIR}" \
  --swift_opt=Visibility=Public,FileNaming=PathToUnderscores \
  --proto_path="$INITIA_DIR" \
  --proto_path="$COSMOS_DIR" \
  --proto_path="$COSMOS_PROTO_DIR" \
  --proto_path="$IBC_DIR" \
  --proto_path="$ICS_DIR" \
  --proto_path="$THIRDPARTY_DIR" \
  $(find ${INITIA_DIR} ${COSMOS_DIR} ${COSMOS_PROTO_DIR} ${IBC_DIR} ${ICS_DIR} ${THIRDPARTY_DIR} -path -prune -o -name '*.proto' -print0 | xargs -0)

# See https://github.com/apple/swift-protobuf/issues/864 
rm "${OUT_DIR}/google_protobuf_any.pb.swift"

