all: init proto-gen

init:
	@echo "Pull External Protofiles"
	@../scripts/prepare.sh swift
	@../scripts/update-submodule.sh `realpath ..`

proto-gen:
	@echo "Generating Protobuf files"
	./scripts/proto-gen.sh

build:
	@echo "building..."

publish:
	@echo "publishing..."

.PHONY: all proto-gen format init build publish
