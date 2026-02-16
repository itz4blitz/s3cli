# s3cli - Technical Specification

> CLI-first S3 storage for developers and AI agents.

---

## 1. Technology Stack & Tooling

### 1.1 Core Technologies

| Component | Technology | Version | Rationale |
|-----------|-----------|---------|-----------|
| Language | Rust | 1.75+ (stable) | Memory safety, performance, binary distribution |
| CLI Parser | clap | 4.5+ | Industry standard, derive macros, subcommands |
| Async Runtime | tokio | 1.35+ | AWS SDK requires async, full features |
| S3 SDK | aws-sdk-s3 | 1.x | Official AWS SDK, async, modular |
| HTTP Client | aws-smithy | via SDK | Built into SDK |
| Config | figment | 0.10+ | Layered config (env, file, CLI args) |
| Logging | tracing | 0.1+ | Structured logging, ecosystem standard |
| Error Handling | anyhow | 1.0+ | Context-aware errors for CLI |
| Serialization | serde | 1.0+ | Config & data serialization |
| UUID | uuid | 1.0+ | Short IDs for file references |

### 1.2 Build & Release

| Tool | Purpose |
|------|---------|
| cargo-binstall | Fast binary installs |
| cargo-dist | Cross-platform releases |
| rustup | Toolchain management |
| cargo-audit | Security vulnerability scanning |
| cargo-udeps | Unused dependency detection |
| cargo-hack | Feature permutation testing |

### 1.3 MSRV (Minimum Supported Rust Version)

```
MSRV: 1.75.0
Rationale: Stable async closures, const generics, good ecosystem support
```

---

## 2. Project Structure

```
s3cli/
├── Cargo.toml              # Workspace root (optional, single crate for v1)
├── rust-toolchain.toml     # MSRV definition
├── .cargo/
│   └── config.toml         # Build configuration
├── claude.md               # Project context for AI assistants
├── src/
│   ├── main.rs            # Entry point
│   ├── lib.rs             # Library root
│   ├── cli/
│   │   ├── mod.rs
│   │   ├── commands/      # Subcommand implementations
│   │   │   ├── mod.rs
│   │   │   ├── push.rs
│   │   │   ├── pull.rs
│   │   │   ├── ls.rs
│   │   │   ├── rm.rs
│   │   │   ├── share.rs
│   │   │   ├── copy.rs
│   │   │   ├── mv.rs
│   │   │   ├── info.rs
│   │   │   └── cat.rs
│   │   └── args.rs        # CLI argument definitions
│   ├── config/
│   │   ├── mod.rs
│   │   ├── settings.rs    # Configuration structs
│   │   ├── providers.rs  # Config sources (env, file, CLI)
│   │   └── storage.rs    # Storage backend config
│   ├── storage/
│   │   ├── mod.rs
│   │   ├── s3.rs         # S3 client wrapper
│   │   ├── local.rs       # Local filesystem backend
│   │   └── traits.rs      # Storage抽象 traits
│   ├── models/
│   │   ├── mod.rs
│   │   ├── file_entry.rs # File metadata
│   │   └── signed_url.rs  # URL generation
│   ├── errors.rs          # Error types
│   └── output.rs          # Output formatting
├── tests/
│   └── integration/       # Integration tests
├── scripts/
│   └── release.sh          # Release automation
└── docs/
    └── specification.md   # This file
```

---

## 3. CLI Design

### 3.1 Command Structure

Following Unix conventions (kebab-case, subcommands):

```bash
s3cli <COMMAND> [OPTIONS] [ARGS]
```

### 3.2 Global Options

| Flag | Description | Default |
|------|-------------|---------|
| `-c, --config <PATH>` | Config file path | `~/.s3cli/config.toml` |
| `-p, --profile <NAME>` | Named profile | `default` |
| `-v, --verbose` | Increase verbosity | 0 (silent) |
| `--color <WHEN>` | Color output | `auto` |
| `--no-verify-ssl` | Disable SSL verification | false |
| `--endpoint <URL>` | Custom S3 endpoint | provider default |

### 3.3 Commands

#### Core Commands

| Command | Alias | Description |
|---------|-------|-------------|
| `push` | `up`, `upload` | Upload file(s) to S3 |
| `pull` | `down`, `download` | Download file by ID or key |
| `ls` | `list` | List stored files |
| `rm` | `delete`, `del` | Remove file(s) |
| `info` | `stat` | Show file metadata |

#### Extended Commands

| Command | Alias | Description |
|---------|-------|-------------|
| `share` | `link` | Generate presigned URL |
| `copy` | `cp` | Copy within bucket |
| `move` | `mv` | Move within bucket |
| `cat` | - | Stream file to stdout |
| `sync` | - | Bidirectional sync |
| `bucket` | `bucket` | Bucket operations |

#### Management Commands

| Command | Description |
|---------|-------------|
| `config` | Manage configuration |
| `login` | OAuth login (hosted) |
| `whoami` | Show current user |

### 3.4 Command Details

#### `s3cli push <FILE> [OPTIONS]`

```
Upload a file to S3 storage.

Usage:
  s3cli push <FILE> [--public] [--expires <DURATION>] [--metadata <JSON>]

Arguments:
  <FILE>                  File to upload (use - for stdin)

Options:
  -p, --public           Make file publicly accessible
  -e, --expires <DURATION>  Expiration for file (e.g., 7d, 24h)
  -m, --metadata <JSON>  JSON metadata to attach
  -t, --content-type <TYPE>  Set Content-Type
  --cache-control <VAL>  Set Cache-Control header
  -o, --output <FORMAT>  Output format (short, json, yaml)

Examples:
  s3cli push video.mp4
  s3cli push screenshot.png --public
  s3cli push data.json --expires 30d --metadata '{"source": "cli"}'
  cat file.txt | s3cli push - --content-type text/plain
```

**Output:**
```
✓ Uploaded: abc123def456 (video.mp4, 150MB)
https://cdn.example.com/abc123def456
```

#### `s3cli pull <ID> [OPTIONS]`

```
Download a file from S3 storage.

Usage:
  s3cli pull <ID> [-o <PATH>]

Arguments:
  <ID>                   File ID or S3 key

Options:
  -o, --output <PATH>   Output path (default: current directory)
  --force               Overwrite existing file

Examples:
  s3cli pull abc123
  s3cli pull my-bucket/path/file.txt -o ./output.txt
```

#### `s3cli ls [OPTIONS]`

```
List stored files.

Usage:
  s3cli ls [PATTERN] [--sort <FIELD>] [--order <ORDER>]

Arguments:
  PATTERN                Filter by key pattern (glob)

Options:
  -l, --long            Long format with details
  --sort <FIELD>        Sort by (name, size, date) [default: date]
  --order <ORDER>       Order (asc, desc) [default: desc]
  --limit <N>           Maximum results [default: 100]
  --offset <N>          Pagination offset

Examples:
  s3cli ls
  s3cli ls "*.mp4" --sort size --order desc
  s3cli ls --long --limit 10
```

#### `s3cli share <ID> [OPTIONS]`

```
Generate a presigned URL for a file.

Usage:
  s3cli share <ID> [--expires <DURATION>] [--download]

Arguments:
  ID                     File ID or S3 key

Options:
  -e, --expires <DURATION>  Expiration time [default: 7d]
  -d, --download           Force download (Content-Disposition)
  -p, --public            Generate public URL (if public file)

Examples:
  s3cli share abc123
  s3cli share abc123 --expires 30d
  s3cli share abc123 --download -e 1h
```

#### `s3cli config [OPTIONS]`

```
Manage s3cli configuration.

Usage:
  s3cli config <SUBCOMMAND>

Subcommands:
  get <KEY>          Get config value
  set <KEY> <VALUE>  Set config value
  list               Show all config
  init               Initialize new config
  import <PATH>      Import from environment

Examples:
  s3cli config set provider r2
  s3cli config set bucket my-files
  s3cli config get provider
  s3cli config list
```

---

## 4. Configuration System

### 4.1 Config File Format (TOML)

```toml
# ~/.s3cli/config.toml

[default]
provider = "r2"              # r2, s3, backblaze, minio, local
endpoint = "https://abc123.r2.cloudflarestorage.com"
region = "auto"
bucket = "my-files"
access_key = "..."
secret_key = "..."

# Custom profile
[profile.dev]
provider = "minio"
endpoint = "http://localhost:9000"
region = "us-east-1"
bucket = "dev-files"
access_key = "minioadmin"
secret_key = "minioadmin"

[profile.local]
provider = "local"
path = "/tmp/s3cli-storage"

# Global settings
[settings]
color = "auto"
verify_ssl = true
default_expiry = "7d"
upload_concurrency = 4
download_concurrency = 4
part_size = "8MB"

# Aliases for quick access
[aliases]
videos = "prefix:videos/*"
screenshots = "prefix:screenshots/*"
```

### 4.2 Environment Variables

| Variable | Description |
|----------|-------------|
| `S3CLI_PROVIDER` | Storage provider |
| `S3CLI_ENDPOINT` | Custom endpoint URL |
| `S3CLI_REGION` | Region |
| `S3CLI_BUCKET` | Bucket name |
| `S3CLI_ACCESS_KEY` | Access key |
| `S3CLI_SECRET_KEY` | Secret key |
| `S3CLI_CONFIG` | Config file path |
| `S3CLI_PROFILE` | Named profile |

### 4.3 Config Precedence

```
CLI flags > Environment variables > Profile config > Default config
```

---

## 5. Storage Architecture

### 5.1 Provider Support

| Provider | Status | Endpoint | Notes |
|----------|--------|----------|-------|
| AWS S3 | ✅ | Standard | Full support |
| Cloudflare R2 | ✅ | Standard | Zero egress fees |
| Backblaze B2 | ✅ | Standard | S3 compatible |
| MinIO | ✅ | Custom | Self-hosted |
| Local | ✅ | N/A | Development/testing |
| Wasabi | ⚙️ | Standard | S3 compatible |
| DigitalOcean Spaces | ⚙️ | Custom | S3 compatible |

### 5.2 Storage Abstraction

```rust
// src/storage/traits.rs

#[async_trait]
pub trait Storage: Send + Sync {
    /// Upload a file
    async fn put(&self, key: &str, data: impl Read + Send + Sync, metadata: &FileMetadata) -> Result<FileEntry, StorageError>;
    
    /// Download a file
    async fn get(&self, key: &str) -> Result<StreamedData, StorageError>;
    
    /// Delete a file
    async fn delete(&self, key: &str) -> Result<(), StorageError>;
    
    /// List files
    async fn list(&self, prefix: Option<&str>, pagination: &Pagination) -> Result<Vec<FileEntry>, StorageError>;
    
    /// Generate presigned URL
    async fn presign(&self, key: &str, expires: Duration) -> Result<String, StorageError>;
    
    /// Get file metadata
    async fn head(&self, key: &str) -> Result<FileMetadata, StorageError>;
    
    /// Copy file
    async fn copy(&self, src: &str, dest: &str) -> Result<(), StorageError>;
    
    /// Move file
    async fn move_to(&self, src: &str, dest: &str) -> Result<(), StorageError>;
}
```

### 5.3 Key Generation Strategy

```
ID format: 12 char alphanumeric ( Crockford Base32)
Example: abc123def456

Key format: {prefix}/{id}.{extension}
Example: uploads/abc123def456.mp4

Short ID lookup via metadata index
```

---

## 6. Data Models

### 6.1 FileEntry

```rust
pub struct FileEntry {
    pub id: String,           // Short ID (abc123def456)
    pub key: String,          // Full S3 key
    pub size: u64,            // Size in bytes
    pub etag: String,        // S3 ETag
    pub content_type: String, // MIME type
    pub metadata: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub storage_class: Option<String>,
}
```

### 6.2 FileMetadata (upload)

```rust
pub struct FileMetadata {
    pub original_name: String,
    pub content_type: Option<String>,
    pub cache_control: Option<String>,
    pub content_disposition: Option<String>,
    pub metadata: HashMap<String, String>,
    pub expires_after: Option<Duration>,
    pub is_public: bool,
}
```

---

## 7. Error Handling

### 7.1 Error Types

```rust
pub enum Error {
    // Configuration errors
    ConfigNotFound(PathBuf),
    ConfigParse(String),
    InvalidProvider(String),
    MissingCredentials,
    
    // Storage errors
    Storage(StorageError),
    ObjectNotFound(String),
    PermissionDenied(String),
    
    // Network errors
    Network(String),
    Timeout,
    
    // CLI errors
    InvalidInput(String),
    FileNotFound(PathBuf),
    
    // Internal errors
    Internal(String),
}
```

### 7.2 Error Display

```
Error: Object not found
  ├─ ID: abc123
  └─ Help: The file may have been deleted or the ID is incorrect
```

---

## 8. Design Patterns

### 8.1 Architectural Patterns

| Pattern | Usage |
|---------|-------|
| **Command** | Each CLI subcommand as separate module |
| **Builder** | S3 client configuration |
| **Trait Objects** | Storage backend abstraction |
| **Result Chaining** | Error context propagation |
| **Layered Config** | figment for config composition |

### 8.2 Code Organization

```
┌─────────────────────────────────────────────────────────┐
│                    CLI Layer (clap)                     │
│  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐   │
│  │  push   │  │  pull   │  │   ls    │  │ config  │   │
│  └────┬────┘  └────┬────┘  └────┬────┘  └────┬────┘   │
└───────┼────────────┼────────────┼────────────┼─────────┘
        │            │            │            │
┌───────▼────────────▼────────────▼────────────▼─────────┐
│                  Service Layer                         │
│  ┌─────────────────────────────────────────────────┐  │
│  │            StorageManager                        │  │
│  └────────────────────┬────────────────────────────┘  │
└──────────────────────┼───────────────────────────────┘
                       │
┌──────────────────────▼───────────────────────────────┐
│                  Storage Layer                        │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐          │
│  │ S3Client │  │  Local   │  │  Mock    │          │
│  └──────────┘  └──────────┘  └──────────┘          │
└─────────────────────────────────────────────────────┘
```

---

## 9. Features & Roadmap

### 9.1 MVP (v0.1.0)

- [ ] Basic CLI structure with clap
- [ ] Config file management
- [ ] S3 client (AWS SDK)
- [ ] `push` - upload files
- [ ] `pull` - download files  
- [ ] `ls` - list files
- [ ] `rm` - delete files
- [ ] `share` - presigned URLs
- [ ] Basic error handling

### 9.2 v0.2.0 - Enhanced Operations

- [ ] `copy` / `move` operations
- [ ] `info` - file metadata
- [ ] `cat` - stream to stdout
- [ ] Glob patterns for filtering
- [ ] Progress bars for transfers
- [ ] Multiple file upload

### 9.3 v0.3.0 - Advanced Features

- [ ] `sync` command
- [ ] Bucket operations
- [ ] Custom metadata
- [ ] Storage class selection
- [ ] Parallel uploads/downloads

### 9.4 v1.0.0 - Production

- [ ] Comprehensive tests
- [ ] Performance optimization
- [ ] CI/CD pipeline
- [ ] Release binaries
- [ ] Documentation

### 9.5 Future Considerations

- OAuth integration (hosted version)
- Web dashboard
- Bucket lifecycle policies
- Multipart upload resume
- Encryption (client-side)
- Webhooks

---

## 10. Testing Strategy

### 10.1 Test Types

| Type | Location | Purpose |
|------|----------|---------|
| Unit | `src/**/*.rs` | Individual function testing |
| Integration | `tests/integration/` | Command-level testing |
| Mock | `tests/mocks/` | Storage mocking |

### 10.2 Testing Commands

```bash
# Unit tests
cargo test

# Integration tests
cargo test --test integration

# With coverage
cargo test --cov

# Fuzzing
cargo fuzz run storage
```

---

## 11. CI/CD Pipeline

### 11.1 GitHub Actions

```yaml
# .github/workflows/ci.yml
on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo test
      - run: cargo clippy -- -D warnings
      - run: cargo fmt --check

  release:
    needs: test
    if: startsWith(github.ref, 'refs/tags/')
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: cargo-binstall/cargo-binstall@latest
      - run: cargo binstall cargo-dist -y
      - run: cargo dist build
```

---

## 12. Release Strategy

### 12.1 Artifacts

| Platform | Format | Tool |
|----------|--------|------|
| macOS (Intel) | .tar.gz | cargo-binstall |
| macOS (Apple Silicon) | .tar.gz | cargo-binstall |
| Linux (x86_64) | .tar.gz | cargo-binstall |
| Linux (ARM64) | .tar.gz | cargo-binstall |
| Windows | .zip | cargo-binstall |

### 12.2 Installation

```bash
# Via cargo-binstall
cargo binstall s3cli

# Via Homebrew
brew install s3cli

# Via npm
npm install -g s3cli

# Manual
curl -L https://s3cli/install | sh
```

---

## 13. Key Implementation Notes

### 13.1 Performance Considerations

1. **Reuse S3 Client** - Share client/config across operations
2. **Concurrent Transfers** - Tokio for parallel uploads/downloads
3. **Streaming** - Don't load entire files into memory
4. **Connection Pooling** - HTTP connection reuse

### 13.2 Security Considerations

1. **Credentials** - Never log or expose credentials
2. **HTTPS** - Default to TLS, warn if disabled
3. **Local Storage** - Encrypt at rest option
4. **URL Expiry** - Reasonable defaults (7d), warn on long expiries

### 13.3 User Experience

1. **Idiomatic CLI** - Follow Unix conventions (pipes, flags)
2. **Help Text** - Comprehensive, with examples
3. **Error Messages** - Actionable, with suggestions
4. **Progress** - Show progress for long operations
5. **Confirmation** - Confirm destructive operations

---

## 14. Dependencies (Cargo.toml)

```toml
[package]
name = "s3cli"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <you@example.com>"]
description = "CLI-first S3 storage for developers and AI agents"
license = "MIT OR Apache-2.0"
rust-version = "1.75"

[dependencies]
# CLI
clap = { version = "4.5", features = ["derive", "usage"] }
clap_complete = "4.5"

# Async
tokio = { version = "1.35", features = ["full"] }
async-trait = "0.1"

# AWS SDK
aws-config = "1.1"
aws-sdk-s3 = "1.28"
aws-smithy-types = "0.60"

# Config
figment = { version = "0.10", features = ["toml", "env"] }
serde = { version = "1.0", features = ["derive"] }
toml = "0.8"

# Errors & Logging
anyhow = "1.0"
thiserror = "1.0"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

# Utils
uuid = { version = "1.0", features = ["v4"] }
chrono = { version = "0.4", features = ["serde"] }
crockford-base32 = "0.2"
dirs = "5.0"
indicatif = "0.17"  # Progress bars
bytesize = "1.0"
mime_guess = "2.0"

[dev-dependencies]
tempfile = "3.8"
mockall = "0.12"
tokio-test = "0.4"

[features]
default = ["dotenv"]
dotenv = ["dep:dotenv"]

[profile.release]
lto = true
codegen-units = 1
panic = "abort"
strip = true
```

---

## 15. Quick Start Implementation

### 15.1 Phase 1: CLI Skeleton

1. Initialize project with `cargo new s3cli`
2. Add clap dependencies
3. Define `Cli` struct with subcommands
4. Implement basic main.rs

### 15.2 Phase 2: Config System

1. Create config module with figment
2. Implement config file read/write
3. Add environment variable support
4. Create provider abstraction

### 15.3 Phase 3: Storage Layer

1. Implement S3 client wrapper
2. Add storage trait
3. Implement local filesystem backend
4. Add error handling

### 15.4 Phase 4: Commands

1. Implement `push` command
2. Implement `pull` command
3. Implement `ls` command
4. Implement `share` command
5. Add error handling and output formatting

---

*Specification Version: 1.0*
*Last Updated: 2024*
