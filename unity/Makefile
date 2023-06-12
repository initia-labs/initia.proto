all: clean init proto-gen build

init:
	@echo "Pull External Protofiles"
	git submodule init
	git submodule update --remote

clean:
	@rm -rf ./bin ./obj ./src


proto-gen:
	@echo "build..."
	./scripts/proto-gen.sh


build:
	@echo "build..."
	@INITIA_DIR=`realpath ../initia/proto` \
	INITIA_THIRD_PARTY_DIR=`realpath ../initia/third_party/proto` \
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
	dotnet nuget push ./bin/Release/initia-unity-proto.${RELEASE_VERSION}.nupkg --source https://api.nuget.org/v3/index.json --api-key $(NUGET_API_KEY)

.PHONY: all proto-gen format init build publish
