.PHONY: all build dev build_scaleway 

all: build build_plugin_scaleway


clippy:
	@echo "Running clippy"
	@cargo clippy --all-targets --all-features -- -D warnings

build: 
	@echo "Building binary"
	@cargo build --release

dev: 
	@echo "Building binary"
	@cargo clippy
	@cargo build

build_plugin_scaleway:
	@echo "Building plugin: scaleway"
	@cd crates/plugins/scaleway && cargo build --release

