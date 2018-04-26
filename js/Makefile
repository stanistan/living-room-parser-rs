PACKAGE = parser
BUILD_DIR = ./target/wasm32-unknown-unknown/release
BUILD_FILES_JS := $(wildcard $(BUILD_DIR)/%.js)
BUILD_FILES_WASM := $(wildcard $(BUILD_DIR)/%.wasm)
TARGETS = pkg/src/$(PACKAGE).js pkg/src/$(PACKAGE).wasm

all: $(TARGETS)

clean:
	rm $(BUILD_DIR)/*.js $(BUILD_DIR)/*.wasm $(TARGETS)

test: $(TARGETS) pkg/node_modules
	cd pkg && npm test

pkg/node_modules:
	cd pkg && npm install

$(TARGETS): src/lib.rs
	cargo web build
	cp $(BUILD_DIR)/*.js pkg/src/$(PACKAGE).js
	cp $(BUILD_DIR)/*.wasm pkg/src/$(PACKAGE).wasm

