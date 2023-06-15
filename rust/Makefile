all: clean init proto-gen

init:
	@echo "Pull External Protofiles"
	@../scripts/prepare.sh rust
	@../scripts/update-submodule.sh `realpath ..`
	cargo install protoc-gen-prost protoc-gen-tonic

clean:
	@echo "Clean up previously built protos..."
	@-rm -rf ./proto ./initia-proto/src/proto


proto-gen: 
	@echo "Generating Protobuf files"
	cargo build -p initia-proto-builder
	cargo build -p initia-proto

build: init proto-gen

publish:
	cargo publish --allow-dirty

.PHONY: all proto-gen init build publish
