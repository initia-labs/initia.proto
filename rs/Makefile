all: init proto-gen format

init:
	@echo "Pull External Protofiles"
	git submodule init
	git submodule update --remote

	@echo "Install proto compilers..."
	cargo install protoc-gen-prost protoc-gen-tonic


proto-gen: 
	@echo "Generating Protobuf files"
	export OUT_DIR="./src/proto"
	echo "Processing initia proto files ..."
	@export INITIA_DIR="../initia/proto"
	@export INITIA_THIRD_PARTY_DIR="../initia/third_party/proto"
	cargo build
	#./scripts/proto-gen.sh

build: 
	cargo build

publish:
	cargo publish

.PHONY: all proto-gen format init build publish
