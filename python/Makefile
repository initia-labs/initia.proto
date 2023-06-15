all: init proto-gen build

init:
	@echo "Pull External Protofiles"
	@../scripts/prepare.sh python
	@../scripts/update-submodule.sh `realpath ..`

# do not format. we uses just auto-generated ones.
#format: 
#	npm run format

proto-gen: 
	@echo "Generating Protobuf files"
	./scripts/proto-gen.sh

clean: 
	rm -rf initia-proto

build: 
	poetry build

publish:
	poetry publish

.PHONY: all proto-gen format init build publish
