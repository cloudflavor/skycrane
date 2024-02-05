.PHONY: all build dev  build-plugin-scaleway build-plugin-scaleway-dev copy-plugin-scaleway docs docs-serve
SKYCRANE_ASSETS_PATH := ./assets

all: build build-plugin-scaleway


clippy:
	@echo "Running clippy"
	@cargo clippy --all-targets --all-features -- -D warnings

build: 
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
	@rm -rf assets/config/plugins/scaleway.wasm

build-plugin-scaleway:
	@echo "Building plugin: scaleway"
	@cd crates/plugins/scaleway && cargo build --release

build-plugin-scaleway-dev:
	@echo "Building plugin: scaleway"
	@cd crates/plugins/scaleway && cargo build

copy_plugin_scaleway:
	@echo "Copying plugin: scaleway"
	@cp target/wasm32-unknown-unknown/debug/scaleway.wasm assets/config/plugins/scaleway.wasm

docs:
	@echo "Building docs"
	@cd docs && mdbook build

docs-serve:
	@echo "Serving docs"
	@cd docs && mdbook serve
