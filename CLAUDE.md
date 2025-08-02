# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Documentation Location

The main architecture documentation for Skycrane is maintained in:
`~/projects/misc/obsidian-vault/projects/skycrane/`

Key documents:
- `Architecture.md` - System design, components, GitOps architecture
- `Timeline.md` - Development roadmap and milestones
- `Gitless-GitOps.md` - OCI-based GitOps design
- `README.md` - Project overview and navigation

- Note: Use `~/projects/misc/obsidian-vault/cloudflavor/projects/skycrane/` for all docs

## Project Overview

Skycrane is an Infrastructure as Code (IaC) tool that uses WASI plugins to manage cloud resources across multiple providers. It uses Starlark (Python-like configuration language) for resource definitions.

## Build and Development Commands

```bash
# Build release binary
make build
# or
cargo build --release

# Development build with clippy checks
make dev
# or
cargo clippy && cargo build

# Run clippy with strict warnings
make clippy
# or
cargo clippy --all-targets --all-features -- -D warnings

# Run tests
make test
# or
cargo test

# Clean build artifacts
make clean

# Documentation
make docs        # Build docs with mdbook
make docs-serve  # Serve docs locally
```

## Architecture

### Core Components

1. **CLI Entry Point**: `src/bin/skycrane.rs` - Main binary entry point
2. **Command Structure**: `src/lib.rs` - Defines CLI commands (init, plan, apply, reconcile, destroy)
3. **WASM Plugin System**: 
   - `src/wasm/engine.rs` - WASI runtime initialization using wasmtime
   - `src/wasm/plugins.rs` - Plugin loading and management
4. **Configuration Loading**:
   - `src/config/loader.rs` - Starlark file parsing and module extraction
   - Reads `.star` files from specified directories
   - Extracts module declarations to determine required plugins

### Key Dependencies

- **wasmtime**: WebAssembly runtime for executing WASI plugins
- **starlark**: Configuration language parser and evaluator
- **tokio**: Async runtime
- **WIT Interface**: WebAssembly Interface Types definitions symlinked from skyforge repo

### Plugin Architecture

1. Plugins are WASI components loaded from `~/.config/skycrane/plugins/`
2. Each plugin implements the SkyforgeApi interface defined in WIT
3. Plugin specifications maintained in separate [Skyforge](https://github.com/cloudflavor/skyforge) repository
4. Starlark configurations specify which plugin to use via `module(name, version)` declarations

### Development Workflow

1. **WIT Dependency**: The `wit` directory is a symlink to skyforge repository's spec/wit
2. **Environment Variables**:
   - `SKYCRANE_CONFIG_PATH`: Override default config path (default: ~/.config/skycrane)
   - `SKYCRANE_LOG_LEVEL`: Set log level (trace, debug, info, warn, error)
3. **Testing**: Run `cargo test` for unit tests

### Important Notes

- Currently in active development, not production-ready
- Uses bindgen to generate Rust bindings from WIT interface definitions
- Plugin system allows extending functionality without modifying core codebase
- Configuration files use `.star` extension and are parsed as Starlark