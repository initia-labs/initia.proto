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
	npm build

publish:
	npm publish

.PHONY: all proto-gen format init build publish
