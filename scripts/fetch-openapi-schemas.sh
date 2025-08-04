#!/usr/bin/env bash

# Fetch OpenAPI v3 schemas for cloud providers

# Create base directory for schemas
SCHEMA_DIR="crates/skyforge/plugins"

# Hetzner Cloud API
echo "Fetching Hetzner Cloud API schema..."
mkdir -p "$SCHEMA_DIR/hetzner/api"
wget -O "$SCHEMA_DIR/hetzner/api/hetzner-openapiv3.json" https://docs.hetzner.cloud/cloud.spec.json

# Exoscale Cloud API
echo "Fetching Exoscale Cloud API Schema..."
mkdir -p "$SCHEMA_DIR/exoscale/api"
wget -O "$SCHEMA_DIR/exoscale/api/exoscale-openapiv3.json" https://openapi-v2.exoscale.com/source.json

echo "Done fetching schemas."

