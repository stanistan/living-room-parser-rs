PACKAGE = parser
BUILD_DIR = ./target/wasm32-unknown-unknown/release
BUILD_FILES_JS := $(wildcard $(BUILD_DIR)/%.js)
BUILD_FILES_WASM := $(wildcard $(BUILD_DIR)/%.wasm)
TARGETS = $(PACKAGE).js $(PACKAGE).wasm

all: $(TARGETS)

clean:
	rm $(BUILD_DIR)/*.js $(BUILD_DIR)/*.wasm $(TARGETS)

test: $(TARGETS)
	npm test

$(TARGETS): src/lib.rs
	cargo web build
	cp $(BUILD_DIR)/*.js $(PACKAGE).js
	cp $(BUILD_DIR)/*.wasm $(PACKAGE).wasm

