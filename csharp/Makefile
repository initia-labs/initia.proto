all: init proto-gen

init:
	@echo "Pull External Protofiles"
	git submodule init
	git submodule update --remote

proto-gen:
	@echo "Generating Protobuf files"
	./scripts/proto-gen.sh

build:
	@echo "build..."
	#dotnet pack --include-symbols --include-source --no-build
	dotnet pack --no-build

publish:
	dotnet publish

.PHONY: all proto-gen format init build publish
