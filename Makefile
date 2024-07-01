.PHONY: all build dev docs docs-serve

CURRENT_DIR=$(shell pwd)

export SKYCRANE_SPEC_PATH=${CURRENT_DIR}/spec

all: build

clippy:
	@echo "Running clippy"
	@cargo clippy --all-targets --all-features -- -D warnings

get-spec:
	@echo "Downloading spec"
	@wget --quiet https://raw.githubusercontent.com/cloudflavor/skyforge/main/spec/plugins-interface.json -O spec/plugins-interface.json

build: get-spec
	@echo "Building binary"
	@cargo build --release

build-dev: get-spec
	@echo "Building binary"
	@cargo build

dev: get-spec
	@echo "Building binary"
	@cargo clippy
	@cargo build

clean: 
	@echo "Cleaning"
	@cargo clean
	@rm -rf target || true
	@rm -rf spec/plugins-interface.json || true

docs:
	@echo "Building docs"
	@cd docs && mdbook build

docs-serve:
	@echo "Serving docs"
	@cd docs && mdbook serve

test:
	@echo "Running tests"
	@cargo test