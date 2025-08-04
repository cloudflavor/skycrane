# Skycrane Plugin Installation Strategies

## The Problem
Not everyone has or wants an OCI registry, especially when starting out. We need multiple ways to get plugins.

## Installation Methods (in order of preference)

### 1. **Direct URL Download** (Simplest)
```bash
skycrane init --plugin-url https://github.com/cloudflavor/skycrane-plugins/releases/download/v0.1.0/hetzner-0.1.0.wasm
```
- Download directly from GitHub releases, S3, or any HTTP server
- Can verify with SHA256 checksum
- Works immediately, no infrastructure needed

### 2. **Git Repository** (Developer-friendly)
```bash
skycrane init --plugin-repo https://github.com/cloudflavor/skycrane-plugins.git
```
- Clone a git repo with pre-built plugins
- Can pin to specific commit/tag/branch
- Great for internal plugins

### 3. **Local File Path** (Development/Testing)
```bash
skycrane init --plugin-path ../my-plugin/target/wasm32-wasip1/release/
```
- Use plugins from local filesystem
- Perfect for development
- Can be relative or absolute path

### 4. **Embedded Plugins** (Zero-dependency)
```bash
# Plugins embedded in the skycrane binary itself
skycrane init --use-embedded
```
- Bundle common plugins directly in the CLI
- Works offline immediately
- Increases binary size but guarantees availability

### 5. **Plugin Registry File** (Decentralized)
```yaml
# .skycrane/registry.yaml
plugins:
  hetzner:
    0.1.0:
      url: https://example.com/hetzner-0.1.0.wasm
      sha256: abc123...
    0.2.0:
      git: https://github.com/org/plugin.git
      ref: v0.2.0
  aws:
    1.0.0:
      path: /opt/skycrane/plugins/aws-1.0.0.wasm
```
- Simple YAML/TOML file listing available plugins
- Can be hosted anywhere (GitHub, S3, local file)
- Community can maintain their own registries

### 6. **IPFS/BitTorrent** (Distributed)
```bash
skycrane init --ipfs QmYwAPJzv5CZsnA625s3Xf2nemtYgPpHdWEz79ojWnPbdG
```
- Decentralized, censorship-resistant
- Content-addressed for integrity
- Good for regions with limited connectivity

### 7. **OCI Registry** (Enterprise)
```bash
skycrane init --registry registry.example.com/skycrane/plugins
```
- When you're ready for enterprise features
- Supports private registries
- Full versioning and signing

## Implementation in init Command

```toml
# .skycrane/config.toml
[plugin_sources]
# Try these in order until one succeeds
sources = [
  { type = "local", path = "./plugins" },
  { type = "url", base = "https://github.com/cloudflavor/plugins/releases/download" },
  { type = "git", repo = "https://github.com/cloudflavor/skycrane-plugins.git" },
  { type = "oci", registry = "registry.example.com", optional = true }
]

[plugin_cache]
path = "~/.config/skycrane/cache"
max_size = "1GB"
ttl = "30d"
```

## Benefits Over Terraform

1. **No Registry Lock-in**: Terraform requires registry.terraform.io or complex mirrors
2. **Works Offline**: Can use local/embedded plugins immediately
3. **Git-native**: Developers can use familiar git workflows
4. **Progressive Enhancement**: Start simple, add OCI later if needed
5. **Community-friendly**: Anyone can host plugins without infrastructure

## Security Considerations

- Always verify SHA256 checksums
- Support GPG signatures for high-security environments
- Show clear warnings for unverified plugins
- Allow but track "trusted sources" list

## Example Workflow

```bash
# First time user - use GitHub releases
skycrane init --plugin-url https://github.com/cloudflavor/plugins/releases/latest/

# Developer - use local build
skycrane init --plugin-path ../my-plugin/target/

# Enterprise - use private OCI
skycrane init --registry internal.company.com/skycrane

# Offline environment - use embedded
skycrane init --use-embedded

# Community plugin - use git
skycrane init --plugin-repo https://github.com/community/awesome-plugin.git
```

This gives users choice and doesn't force them into any particular infrastructure requirement!