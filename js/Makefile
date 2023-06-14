all: init proto-gen format

init:
	@echo "Pull External Protofiles"
	@../scripts/update-submodule.sh `realpath ..`

	@echo "Install dependencies"
	npm i

format:
	npm run format

proto-gen:
	@echo "Generating Protobuf files"
	@rm -rf ./src
	@./scripts/proto-gen.sh

build:
	npm run build

publish:
	npm publish

.PHONY: all proto-gen format init build publish
