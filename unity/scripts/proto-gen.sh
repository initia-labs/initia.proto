#!/bin/bash
set -o nounset -o pipefail
command -v shellcheck >/dev/null && shellcheck "$0"

PROTOGEN=`which protogen`
if [ -z "$PROTOGEN" ]; then
  echo "protogen for protobuf-net not found. Please install protogen."
  exit 1
fi
OUT_DIR=./src

set -o errexit -o nounset -o pipefail

mkdir -p "$OUT_DIR"

echo "Processing initia proto files ..."
INITIA_DIR="../initia/proto"
COSMOS_DIR="../cosmos-sdk/proto"
COSMOS_PROTO_DIR="../cosmos-proto/proto"
IBC_DIR="../ibc-go/proto"
ICS_DIR="../ics23/proto"
THIRDPARTY_DIR="../third_party"

# copy proto files for protogen.
rm -rf ./proto; mkdir -p ./proto
cp -fpr ${INITIA_DIR} ./proto/initia
cp -fpr ${COSMOS_DIR} ./proto/cosmos
cp -fpr ${COSMOS_PROTO_DIR} ./proto/cosmos_proto
cp -fpr ${IBC_DIR} ./proto/ibc
cp -fpr ${ICS_DIR} ./proto/ics
cp -fpr ${THIRDPARTY_DIR} ./proto/third_party


PROTOS=""
for fname in `find ./proto/initia -path -prune -o -name '*.proto'`; do
  PROTOS+="${fname#./proto/initia/} " # trailing space is matter
done
for fname in `find ./proto/cosmos -path -prune -o -name '*.proto'`; do
  PROTOS+="${fname#./proto/cosmos/} " # trailing space is matter
done
for fname in `find ./proto/cosmos_proto -path -prune -o -name '*.proto'`; do
  PROTOS+="${fname#./proto/cosmos_proto/} " # trailing space is matter
done
for fname in `find ./proto/ibc -path -prune -o -name '*.proto'`; do
  PROTOS+="${fname#./proto/ibc/} " # trailing space is matter
done
for fname in `find ./proto/ics -path -prune -o -name '*.proto'`; do
  PROTOS+="${fname#./proto/ics/} " # trailing space is matter
done
for fname in `find ./proto/third_party -path -prune -o -name '*.proto'`; do
  PROTOS+="${fname#./proto/third_party/} " # trailing space is matter
done

protogen \
  +listset=yes \
  --csharp_out="${OUT_DIR}" \
  --proto_path=proto/initia \
  --proto_path=proto/cosmos \
  --proto_path=proto/cosmos_proto \
  --proto_path=proto/ibc \
  --proto_path=proto/ics \
  --proto_path=proto/third_party \
  $PROTOS

# replace reserved keywords
find ./src -name '*.cs' | xargs sed -i '' 's/cosmos.base/cosmos.ibase/g'
find ./src -name '*.cs' | xargs sed -i '' 's/cosmos.params/cosmos.iparams/g'

# remove unnecessary files
rm -rf ./src/gogoproto

# clean up
rm -rf ./proto
