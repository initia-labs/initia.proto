all: init proto-gen build

init:
	@echo "Pull External Protofiles"
	git submodule init
	git submodule update --remote

proto-gen:
	@echo "Generating Protobuf files"
	./scripts/proto-gen.sh

build:
	@echo "build..."
	@INITIA_DIR=`realpath ../initia/proto` \
	INITIA_THIRD_PARTY_DIR=`realpath ../initia/third_party/proto` \
	dotnet pack --configuration Release

publish:
	@INITIA_DIR=`realpath ../initia/proto` \
	INITIA_THIRD_PARTY_DIR=`realpath ../initia/third_party/proto` \
	dotnet publish --configuration Release --no-build 
	@INITIA_DIR=`realpath ../initia/proto` \
	INITIA_THIRD_PARTY_DIR=`realpath ../initia/third_party/proto` \
	dotnet nuget push ./bin/Release/initia.proto.*.nupkg --source https://api.nuget.org/v3/index.json --api-key $(NUGET_API_KEY)

.PHONY: all proto-gen format init build publish
