all: clean init proto-gen build

init:
	@echo "Pull External Protofiles"
	@../scripts/prepare.sh csharp
	@../scripts/update-submodule.sh `realpath ..`

proto-gen:
	@echo "Generating Protobuf files"
	./scripts/proto-gen.sh

clean:
	@rm -rf ./bin ./obj

build:
	@echo "build..."
	@INITIA_DIR=`realpath ../initia/proto` \
	COSMOS_DIR=`realpath ../cosmos-sdk/proto` \
	COSMOS_PROTO_DIR=`realpath ../initia/cosmos-proto/proto` \
	IBC_DIR=`realpath ../initia/ibc-go/proto` \
	ICS_DIR=`realpath ../initia/ics23/proto` \
	THIRD_PARTY_DIR=`realpath ../initia/third_party` \
	dotnet pack --include-source --include-symbols --configuration Release

publish:
	@echo "publishing..."
	@INITIA_DIR=`realpath ../initia/proto` \
	INITIA_THIRD_PARTY_DIR=`realpath ../initia/third_party/proto` \
	dotnet publish --configuration Release --no-build --framework "netstandard2.0" --output "./bin/Release/netstandard2.0"
	@INITIA_DIR=`realpath ../initia/proto` \
	INITIA_THIRD_PARTY_DIR=`realpath ../initia/third_party/proto` \
	dotnet publish --configuration Release --no-build --framework "net6.0" --output "./bin/Release/net6.0"
	@INITIA_DIR=`realpath ../initia/proto` \
	INITIA_THIRD_PARTY_DIR=`realpath ../initia/third_party/proto` \
	dotnet publish --configuration Release --no-build --framework "net461" --output "./bin/Release/net461"

push:
	@echo "pushing..."
	@INITIA_DIR=`realpath ../initia/proto` \
	INITIA_THIRD_PARTY_DIR=`realpath ../initia/third_party/proto` \
	dotnet nuget push ./bin/Release/initia-proto.${RELEASE_VERSION}.nupkg --source https://api.nuget.org/v3/index.json --api-key $(NUGET_API_KEY)

.PHONY: all proto-gen format init build publish
