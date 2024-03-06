#!/bin/bash
set -o errexit -o nounset -o pipefail
command -v shellcheck >/dev/null && shellcheck "$0"


# chcek realpath is existing
if ! command -v realpath &> /dev/null
then
	echo "realpath could not be found: install coreutils first."
	exit 1
fi

case $1 in
	"js") PREREQUISITES="protoc npm";;
	"python") PREREQUISITES="protoc pip";;
	"rust") PREREQUISITES="cargo";;
	"java") PREREQUISITES="java gradle";;
	"java-alt") PREREQUISITES="protoc-gen-quickbuf";;
	"swift") PREREQUISITES="protoc protoc-gen-swift";;
	"csharp") PREREQUISITES="dotnet grpc_csharp_plugin";; 
	"unity") PREREQUISITES="protogen";; 
	"cpp") PREREQUISITES="protoc cmake";;
	"c") PREREQUISITES="protoc-c";;
esac
PREREQUISITES="$PREREQUISITES go"

for PREREQUISITE in $PREREQUISITES; do
	if ! command -v $PREREQUISITE &> /dev/null
	then
		echo "$PREREQUISITE could not be found: install it or set PATH first."
		exit 1
	fi
done
