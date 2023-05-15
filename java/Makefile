all: init clean proto-gen

init:
	@echo "Pull External Protofiles"
	git submodule init
	git submodule update --remote

proto-gen:
	@echo "Generating Protobuf files"
	./gradlew cleanProto generateProto

publish:
	@echo "Publishing..."
	./gradlew publish

clean:
	./gradlew cleanProto

.PHONY: all proto-gen format init publish
