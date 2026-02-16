# Building s3cli - A CLI-First S3 Storage Tool

> A journey from zero to production: Building a modern Rust CLI tool for S3 storage.

## Table of Contents

1. [Introduction](#introduction)
2. [Project Setup](#project-setup)
3. [TDD Journey](#tdd-journey)
4. [Implementation Notes](#implementation-notes)
5. [Lessons Learned](#lessons-learned)

---

## Introduction

This document tracks the development of **s3cli** - a CLI-first S3 storage tool for developers and AI agents. The goal is to build a production-ready CLI that works with any S3-compatible storage (AWS S3, Cloudflare R2, Backblaze B2, MinIO, local filesystem).

### Why This Project?

- **No API key management** for AI agents
- **Unified CLI** across any S3-compatible storage
- **Developer-friendly** with sensible defaults
- **Production-ready** with proper testing and error handling

### Technology Stack

| Component | Technology |
|-----------|------------|
| Language | Rust 1.75+ |
| CLI Parser | clap 4.5 |
| Async Runtime | tokio |
| S3 SDK | aws-sdk-s3 |
| Config | figment |
| Testing | Built-in Rust testing |

---

## Project Setup

### Initial Requirements

- Rust 1.75+ installed via rustup
- Cargo package manager
- Git for version control

### Repository Structure

```
s3cli/
├── Cargo.toml
├── src/
│   ├── main.rs
│   └── lib.rs
├── tests/
│   └── integration/
└── SPEC.md
```

---

## TDD Journey

### Phase 1: Project Initialization

**Date**: 2024

**Goal**: Set up the Rust project with dependencies and verify it compiles.

**Steps**:

1. Created Cargo.toml with all dependencies
2. Set up basic main.rs
3. Verified compilation with `cargo build`

**Commit**: `feat: Initialize Rust project with dependencies`

---

### Phase 2: CLI Structure

**Date**: 2024

**Goal**: Define CLI commands and structure.

**Test-First Approach**:

```rust
// We want to support:
// s3cli push <file>
// s3cli pull <id>
// s3cli ls
// s3cli share <id>
```

**Implementation**:
- Defined Cli enum with subcommands
- Added clap derive macros
- Implemented help/version

**Commit**: `feat: Add CLI structure with clap`

---

### Phase 3: Configuration System

**Date**: 2024

**Goal**: Manage configuration from file, env, and CLI args.

**Test-First Approach**:

```rust
#[test]
fn test_config_from_file() {
    // Should read config from ~/.s3cli/config.toml
}

#[test]
fn test_config_from_env() {
    // Should read from S3CLI_* environment variables
}
```

**Implementation**:
- Created Config struct
- Implemented figment-based config loading
- Added provider support (S3, R2, MinIO, local)

**Commit**: `feat: Implement configuration system`

---

### Phase 4: Storage Abstraction

**Date**: 2024

**Goal**: Abstract storage behind a trait for multiple backends.

**Test-First Approach**:

```rust
#[tokio::test]
async fn test_s3_upload() {
    // Should upload file to S3
}

#[tokio::test]
async fn test_local_storage() {
    // Should upload to local filesystem
}
```

**Implementation**:
- Created Storage trait
- Implemented S3Client and LocalStorage
- Added error handling

**Commit**: `feat: Add storage abstraction layer`

---

### Phase 5: Push Command

**Date**: 2024

**Goal**: Upload files to S3 storage.

**Test-First Approach**:

```rust
#[test]
fn test_push_command_parses_args() {
    // Should parse push <file> arguments
}

#[tokio::test]
async fn test_push_file_to_s3() {
    // Should upload file and return ID
}
```

**Implementation**:
- Implemented push subcommand
- Added file reading and metadata extraction
- Generated short IDs

**Commit**: `feat: Implement push command`

---

### Phase 6: Pull Command

**Date**: 2024

**Goal**: Download files from S3 storage.

**Test-First Approach**:

```rust
#[test]
fn test_pull_command_parses_args() {
    // Should parse pull <id> arguments
}
```

**Implementation**:
- Implemented pull subcommand
- Added streaming download
- Handle file writing

**Commit**: `feat: Implement pull command`

---

### Phase 7: List Command

**Date**: 2024

**Goal**: List stored files.

**Implementation**:
- Implemented ls subcommand
- Added sorting and filtering
- Table output formatting

**Commit**: `feat: Implement ls command`

---

### Phase 8: Share Command

**Date**: 2024

**Goal**: Generate presigned URLs.

**Implementation**:
- Implemented share subcommand
- Added expiration support
- Custom endpoint support

**Commit**: `feat: Implement share command`

---

### Phase 9: Additional Commands

**Date**: 2024

**Goal**: Complete the command set.

**Implementation**:
- rm (delete)
- copy/move
- info (metadata)
- cat (stream to stdout)
- sync (bidirectional)

**Commit**: `feat: Add remaining commands`

---

### Phase 10: Integration Testing

**Date**: 2024

**Goal**: End-to-end testing.

**Implementation**:
- Integration tests for each command
- Mock storage for testing
- Test fixtures

**Commit**: `test: Add integration tests`

---

### Phase 11: CI/CD

**Date**: 2024

**Goal**: Automated testing and releases.

**Implementation**:
- GitHub Actions workflow
- cargo-dist for releases
- Cross-platform builds

**Commit**: `ci: Add GitHub Actions workflow`

---

### Phase 12: Release

**Date**: 2024

**Goal**: Production release.

**Implementation**:
- Release binaries for all platforms
- Homebrew formula
- Installation scripts

**Commit**: `chore: Release v1.0.0`

---

## Implementation Notes

### Key Design Decisions

1. **Config Precedence**: CLI > Env > File > Default
2. **Storage Trait**: Allows easy testing and multiple backends
3. **Short IDs**: Crockford Base32 for human-readable IDs
4. **Streaming**: Never load entire files into memory

### Error Handling Strategy

- Use `anyhow` for application errors
- Use `thiserror` for library errors
- Provide actionable error messages with suggestions

### Performance Considerations

- Reuse S3 client across operations
- Use tokio for concurrent transfers
- Stream large files instead of loading into memory

---

## Lessons Learned

*(To be filled as we progress through the project)*

---

## Progress Summary

| Phase | Status | Date |
|-------|--------|------|
| Project Setup | ✅ Complete | - |
| CLI Structure | ⏳ In Progress | - |
| Config System | ⏳ Pending | - |
| Storage Layer | ⏳ Pending | - |
| Push Command | ⏳ Pending | - |
| Pull Command | ⏳ Pending | - |
| List Command | ⏳ Pending | - |
| Share Command | ⏳ Pending | - |
| Additional Commands | ⏳ Pending | - |
| Integration Tests | ⏳ Pending | - |
| CI/CD | ⏳ Pending | - |
| Release | ⏳ Pending | - |

---

*Last Updated: 2024*
*Version: 0.1.0*
