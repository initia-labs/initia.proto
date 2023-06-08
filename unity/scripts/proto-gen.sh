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
INITIA_THIRD_PARTY_DIR="../initia/third_party/proto"

# copy proto files for protogen.
cp -fpr ${INITIA_DIR} ./proto
cp -fpr ${INITIA_THIRD_PARTY_DIR} ./third_party


PROTOS=""
for fname in `find ./proto -path -prune -o -name '*.proto'`; do
  PROTOS+="${fname#./proto/} "

done
for fname in `find ./third_party -path -prune -o -name '*.proto'`; do
  PROTOS+="${fname#./third_party/} "
done

protogen \
  +names=auto \
  --csharp_out="${OUT_DIR}" \
  --proto_path=third_party \
  --proto_path=proto \
  $PROTOS

# replace reserved keywords
find ./src -name '*.cs' | xargs sed -i '' 's/cosmos.base/cosmos.ibase/g'
find ./src -name '*.cs' | xargs sed -i '' 's/cosmos.params/cosmos.iparams/g'

# remove unnecessary files
rm -rf ./src/gogoproto ./src/google

# clean up
rm -rf ./proto ./third_party
