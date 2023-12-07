#!/bin/bash
set -o errexit -o nounset -o pipefail
command -v shellcheck >/dev/null && shellcheck "$0"

# regex pattern to check if the version is including commit hash or not
regex_pattern="v[0-9]+.[0-9]+.[0-9]+-[0-9.]+-[a-fA-F0-9]+"

function getver {
	pushd . >/dev/null
	cd $2

	# try to get replace version first, if not, get version
	val=$(go list -f "{{.Replace.Version}}" -m $1 2>/dev/null)
	if [ $? -ne 0 ]; then
		val=$(go list -f "{{.Version}}" -m $1 2>/dev/null)
	fi

	# if val includes a commit, get the commit hash from val
	if [[ $val =~ $regex_pattern ]]; then
		val=$(echo $val | cut -d'-' -f3)
	fi

	popd >/dev/null
	echo $val 
}

# update submodules
git submodule init
git submodule update --remote

# clone dependency proto files
COSMOS_URL=github.com/cosmos/cosmos-sdk
COSMOS_PROTO_URL=github.com/cosmos/cosmos-proto
IBC_URL=github.com/cosmos/ibc-go
ICS_URL=github.com/confio/ics23/go
IBC_V=v7
WASMD_URL=github.com/CosmWasm/wasmd
BLOCK_SDK_URL=github.com/skip-mev/block-sdk

GOMOD_PATH=$1/initia
MINIWASM_GOMOD_PATH=$1/miniwasm

# get versions
COSMOS_SDK_VERSION=`getver $COSMOS_URL $GOMOD_PATH`
COSMOS_PROTO_VERSION=`getver $COSMOS_PROTO_URL $GOMOD_PATH`
IBC_VERSION=`getver $IBC_URL/$IBC_V $GOMOD_PATH`
ICS_VERSION=`getver $ICS_URL $GOMOD_PATH`
BLOCK_SDK_VERSION=`getver $BLOCK_SDK_URL $GOMOD_PATH`
WASMD_VERSION=`getver $WASMD_URL $MINIWASM_GOMOD_PATH`

# if ICS_VERSION is v0.9.0, forced to set v0.10.0
if [[ "$ICS_VERSION" == "v0.9.0" ]]; then
	ICS_VERSION="v0.10.0"
	echo "FORCED TO SET ICS_VERSION TO v0.10.0 instad of v0.9.0! v0.9.0 has an invalid proto structure"
fi


echo "COSMOS_SDK_VERSION: $COSMOS_SDK_VERSION"
echo "COSMOS_PROTO_VERSION: $COSMOS_PROTO_VERSION"
echo "IBC_VERSION: $IBC_VERSION"
echo "ICS_VERSION: $ICS_VERSION"
echo "WASMD_VERSION: $WASMD_VERSION"
echo "BLOCK_SDK_VERSION: $BLOCK_SDK_VERSION"

# they don't have branches for their releases. use tags instead
pushd .
cd $1/cosmos-sdk; git checkout $COSMOS_SDK_VERSION
cd $1/cosmos-proto; git checkout $COSMOS_PROTO_VERSION
cd $1/ibc-go; git checkout $IBC_VERSION
cd $1/ics23; git checkout go/$ICS_VERSION    # ics23's tag have a prefix "go/"
cd $1/wasmd; git checkout $WASMD_VERSION
cd $1/block-sdk; git checkout $BLOCK_SDK_VERSION
popd
