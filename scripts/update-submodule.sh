#!/bin/bash
set -o errexit -o nounset -o pipefail
command -v shellcheck >/dev/null && shellcheck "$0"

echo "INITIA-PROTO HOME: $1"
git submodule init
git submodule update --remote

# clone dependency proto files
COSMOS_URL=github.com/cosmos/cosmos-sdk
COSMOS_PROTO_URL=github.com/cosmos/cosmos-proto
IBC_URL=github.com/cosmos/ibc-go
ICS_URL=github.com/confio/ics23/go
IBC_V=v7

GOMOD=$1/initia/go.mod
AWK_SCRIPT=$1/scripts/getver.awk

COSMOS_SDK_VERSION=$(awk -f $AWK_SCRIPT -v expr='$COSMOS_URL' $GOMOD)
COSMOS_PROTO_VERSION=$(awk -f $AWK_SCRIPT -v expr='$COSMOS_PROTO_URL' $GOMOD)
IBC_VERSION=$(awk -f $AWK_SCRIPT -v expr='$IBC_URL' $GOMOD)
ICS_VERSION=$(awk -f $AWK_SCRIPT -v expr='$ICS_URL' $GOMOD)
#COSMOS_SDK_VERSION=$(grep "[^\/\/]$COSMOS_URL v" $1/initia/go.mod | sed -n -e "s/^.* //p")
#COSMOS_PROTO_VERSION=$(grep "[^\/\/]$COSMOS_PROTO_URL v" $1/initia/go.mod | sed -n -e "s/^.* //p")
#IBC_VERSION=$(grep "$IBC_URL/$IBC_V v" $1/initia/go.mod | sed -n -e "s/^.* //p")
#ICS_VERSION=$(grep "$ICS_URL v" $1/initia/go.mod | sed -n -e "s/^.* //p")

echo "ICS_VERSION: |$ICS_VERSION|"

# they don't have branches for their releases. use tags instead
pushd .
cd $1/cosmos-sdk; git checkout $COSMOS_SDK_VERSION
cd $1/cosmos-proto; git checkout $COSMOS_PROTO_VERSION
cd $1/ibc-go; git checkout $IBC_VERSION
cd $1/ics23; git checkout $ICS_VERSION
popd


