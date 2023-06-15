all: init clean proto-gen

init:
	@echo "Pull External Protofiles"
	@../scripts/prepare.sh java
	@../scripts/update-submodule.sh `realpath ..`

proto-gen:
	@echo "Generating Protobuf files"
	./gradlew cleanProto generateProto

publish:
	@echo "Publishing..."
	./gradlew publish

clean:
	./gradlew cleanProto

.PHONY: all proto-gen format init publish
