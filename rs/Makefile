all: init proto-gen format

init:
	@echo "Pull External Protofiles"
	git submodule init
	git submodule update --remote

	@echo "Install proto compilers..."
	cargo install protoc-gen-prost protoc-gen-tonic


proto-gen: 
	@echo "Generating Protobuf files"
	./scripts/proto-gen.sh

build: 
	cargo build

publish:
	cargo publish

.PHONY: all proto-gen format init build publish
