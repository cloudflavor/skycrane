<div align="center">
    <a href="https://skycrane.io">https://skycrane.io</a>
</div>
<br>

<div align="center">
    <a href="https://docs.skycrane.io">
        <img src="https://img.shields.io/badge/web-docs-blue" alt="Static Badge"/>
    </a>
    <img alt="GitHub Actions Workflow Status" src="https://img.shields.io/github/actions/workflow/status/cloudflavor/skycrane/rust.yml">
</div>
<br>

<div align="center">
  <img src="assets/images/skycrane.svg" width="160" alt="skycrane">
</div>

<div align="center">
  <strong>Skycrane automates infrastructure</strong>
</div>

---

[Skycrane](https://skycrane.io) is an Infrastructure as Code (IaC) tool that uses WASI
plugins to securely and efficiently manage cloud resources across multiple providers.  

It allows users to define resources using
[Starlark](https://github.com/bazelbuild/starlark), a Python-like configuration language.

Resource specifications for each plugin are maintained in the
Skyforge repository, enabling a standardized
approach to resource definition across different cloud platforms.  

Drawing inspiration from kustomize, Skycrane provides a streamlined way to manage
different environments such as dev, staging and production, allowing for easy
customization and overlay of configurations.  

## Status

Skycrane is currently in development and is not yet ready for production use!

## Installation

```shell
$ cargo install skycrane
...
```

## Init

Initialize a new repository with a plugin specification.
It will also validate that the plugin is loaded successfully.

```shell
$ skycrane init ~/projects/rust/skyforge/crates/plugins/hetzner/spec
2024-07-28T19:48:35.649086Z  INFO skycrane::commands: Initializing new repository at "/path/to/skyforge/crates/plugins/hetzner/spec"
2024-07-28T19:48:35.837392Z  INFO skycrane::wasm::engine: Plugin hetzner.wasm loaded successfully!
```

# Skyforge

A registry of WASI plugins for infrastructure provisioning

## Building

All plugins are built using the wasm component of the Rust toolchain. To build the plugins, run the following command:

e.g. for the Hetzner plugin:

```sh
$ cargo install cargo-component --locked
...
$ cargo component build && cp ../../../target/wasm32-wasi/debug/hetzner.wasm ~/.config/skycrane/plugins/
...
```

This will build the plugin and copy the resulting wasm file to the
`~/.config/skycrane/plugins/` directory. The manual copy is necessary because we don't
have a registry in place at the moment. Once the registry is in place, skycrane will
download the plugin automatically.


cargo component build --target wasm32-wasip1 && cp ../../../target/wasm32-wasip1/debug/hetzner.wasm ~/.config/skycrane/plugins
