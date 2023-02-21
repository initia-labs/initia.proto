all: init proto-gen

init:
	@echo "Pull External Protofiles"
	git submodule init
	git submodule update --remote

	@echo "Install proto compilers..."
	cargo install protoc-gen-prost protoc-gen-tonic


proto-gen: 
	@echo "Generating Protobuf files"
	export OUT_DIR="./src/proto"
	@export INITIA_DIR="../initia/proto"
	@export INITIA_THIRD_PARTY_DIR="../initia/third_party/proto"
	cargo build

build: 
	cargo build

publish:
	cargo publish

.PHONY: all proto-gen init build publish
