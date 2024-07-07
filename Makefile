.PHONY: all build dev docs docs-serve

CURRENT_DIR=$(shell pwd)

export SKYCRANE_SPEC_PATH=${CURRENT_DIR}/spec

all: build

clippy:
	@echo "Running clippy"
	@cargo clippy --all-targets --all-features -- -D warnings

build:
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

docs:
	@echo "Building docs"
	@cd docs && mdbook build

docs-serve:
	@echo "Serving docs"
	@cd docs && mdbook serve

test:
	@echo "Running tests"
	@cargo test