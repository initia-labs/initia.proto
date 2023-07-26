all: init clean proto-gen build

init:
	@echo "Pull External Protofiles"
	@../scripts/prepare.sh c
	@../scripts/update-submodule.sh `realpath ..`

proto-gen:
	@echo "Generating Protobuf files"
	./scripts/proto-gen.sh

clean:
	@echo "clean..."
	rm -rf src

build:
	@echo "build..."
	@echo "================="
	@echo "== UNSUPPORTED =="
	@echo "================="
	exit 1
	#cd build; cmake .; make clean; make all

publish:
	@echo "publish..."
	@echo "================="
	@echo "== UNSUPPORTED =="
	@echo "================="
	exit 1
	#cd build; make install

.PHONY: all proto-gen format init build publish
