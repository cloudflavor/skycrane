#!/usr/bin/env bash

# Update WIT dependencies
wit-deps -m spec/wit/deps.toml -d spec/wit/deps -l spec/wit/deps.lock update

