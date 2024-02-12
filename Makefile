.PHONY: all build dev  build-plugin-scaleway build-plugin-scaleway-dev copy-plugin-scaleway docs docs-serve

CURRENT_DIR=$(shell pwd)

export SKYCRANE_ASSETS_PATH=${CURRENT_DIR}/assets

all: build build-plugin-scaleway


clippy:
	@echo "Running clippy"
	@cargo clippy --all-targets --all-features -- -D warnings

build:  build-plugin-scaleway copy-plugin-scaleway
	@echo "Building binary"
	@cargo build --release

build-dev: build-plugin-scaleway-dev copy-plugin-scaleway
	@echo "Building binary"
	@cargo build

dev: build-plugin-scaleway-dev copy-plugin-scaleway
	@echo "Building binary"
	@cargo clippy
	@cargo build

clean: 
	@echo "Cleaning"
	@cargo clean
	@rm -rf assets/plugins/scaleway.wasm || true
	@rm -rf target || true
	@rm -rf creates/skycrane/target || true
	@rm -rf creates/plugins/scaleway/target || true

build-plugin-scaleway:
	@echo "Building plugin: scaleway"
	@cd crates/plugins/scaleway && cargo build --release

build-plugin-scaleway-dev:
	@echo "Building plugin: scaleway"
	@cd crates/plugins/scaleway && cargo build

copy-plugin-scaleway-dev:
	@echo "Copying plugin: scaleway"
	@cp target/wasm32-unknown-unknown/debug/scaleway.wasm assets/plugins/

copy-plugin-scaleway:
	@echo "Copying plugin: scaleway"
	@cp target/wasm32-unknown-unknown/release/scaleway.wasm assets/plugins/

docs:
	@echo "Building docs"
	@cd docs && mdbook build

docs-serve:
	@echo "Serving docs"
	@cd docs && mdbook serve

test:
	@echo "Running tests"
	@cargo test