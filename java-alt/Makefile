all: init clean proto-gen build

init:
	@echo "Pull External Protofiles"
	@../scripts/prepare.sh java-alt
	@../scripts/update-submodule.sh `realpath ..`

proto-gen:
	@echo "Generating Protobuf files"
	./scripts/proto-gen.sh

clean:
	@echo "clean..."
	rm -rf src

build:
	@echo "build..."
	cd build; cmake .; make clean; make all

publish:
	@echo "publish..."
	cd build; make install

.PHONY: all proto-gen format init build publish
