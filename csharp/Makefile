all: init proto-gen #format

init:
	@echo "Pull External Protofiles"
	git submodule init
	git submodule update --remote

format:
	npm run format

proto-gen:
	@echo "Generating Protobuf files"
	./scripts/proto-gen.sh

build:
	dotnet build

publish:
	dotnet publish

.PHONY: all proto-gen format init build publish
