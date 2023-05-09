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

publish:
	@echo "publish..."

.PHONY: all proto-gen format init build publish
