all: init proto-gen

init:
	@echo "Pull External Protofiles"
	git submodule init
	git submodule update --remote

proto-gen:
	@echo "Generating Protobuf files"
	./scripts/proto-gen.sh

clean:
	@echo "clean..."
	rm -rf src

build:
	@echo "build..."
	cd cmake; cmake .; make clean; make all

publish:
	@echo "publish..."
	cd cmake; make install

.PHONY: all proto-gen format init build publish
