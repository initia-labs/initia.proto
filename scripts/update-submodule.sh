#!/bin/bash
set -o errexit -o nounset -o pipefail
command -v shellcheck >/dev/null && shellcheck "$0"

git submodule init
git submodule update --remote

# clone dependency proto files
COSMOS_URL=github.com/cosmos/cosmos-sdk
COSMOS_PROTO_URL=github.com/cosmos/cosmos-proto
IBC_URL=github.com/cosmos/ibc-go
ICS_URL=github.com/confio/ics23/go
IBC_V=v7
POB_URL=github.com/skip-mev/pob
WASM_URL=github.com/CosmWasm/wasmd

GOMOD=$1/initia/go.mod
MINIGOMOD=$1/miniwasm/go.mod
AWK_SCRIPT=$1/scripts/getver.awk

COSMOS_SDK_VERSION=$(awk -f $AWK_SCRIPT -v expr="$COSMOS_URL" $GOMOD)
COSMOS_PROTO_VERSION=$(awk -f $AWK_SCRIPT -v expr="$COSMOS_PROTO_URL" $GOMOD)
IBC_VERSION=$(awk -f $AWK_SCRIPT -v expr="$IBC_URL/$IBC_V" $GOMOD)
ICS_VERSION=$(awk -f $AWK_SCRIPT -v expr="$ICS_URL" $GOMOD)
POB_VERSION=$(awk -f $AWK_SCRIPT -v expr="$POB_URL" $GOMOD)
WASM_VERSION=$(awk -f $AWK_SCRIPT -v expr="$WASM_URL" $MINIGOMOD)

# if ICS_VERSION is v0.9.0, forced to set v0.10.0
if [[ "$ICS_VERSION" == "v0.9.0" ]]; then
	ICS_VERSION="v0.10.0"
	echo "FORCED TO SET ICS_VERSION TO v0.10.0 instad of v0.9.0! v0.9.0 has an invalid proto structure"
fi


echo "COSMOS_SDK_VERSION: $COSMOS_SDK_VERSION"
echo "COSMOS_PROTO_VERSION: $COSMOS_PROTO_VERSION"
echo "IBC_VERSION: $IBC_VERSION"
echo "ICS_VERSION: $ICS_VERSION"
echo "POB_VERSION: $POB_VERSION"
echo "WASM_VERSION: $WASM_VERSION"

# they don't have branches for their releases. use tags instead
pushd .
cd $1/cosmos-sdk; git checkout $COSMOS_SDK_VERSION
cd $1/cosmos-proto; git checkout $COSMOS_PROTO_VERSION
cd $1/ibc-go; git checkout $IBC_VERSION
cd $1/ics23; git checkout go/$ICS_VERSION    # ics23's tag have a prefix "go/"
cd $1/pob; git checkout $POB_VERSION
cd $1/wasm; git checkout $WASM_VERSION
popd
