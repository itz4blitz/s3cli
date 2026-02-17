# s3cli

CLI-first S3 storage for developers and AI agents.

## Install

```bash
cargo install s3cli
```

## Quick Start

```bash
# Initialize config
s3cli config init

# Upload files
s3cli push demo.mp4
s3cli push screenshot.png --public

# Download
s3cli pull abc123
s3cli pull abc123 -o myfile.mp4

# Share with expiring links
s3cli share abc123
s3cli share abc123 --expires 7d --copy

# List your files
s3cli ls
s3cli ls --long

# Get file info
s3cli info abc123

# Delete files
s3cli rm abc123
```

## Configuration

```bash
# Interactive setup
s3cli config init

# Or manually edit ~/.s3cli/config.toml
```

Supported providers: AWS S3, Cloudflare R2, Backblaze B2, MinIO, Local filesystem

## Features

- **Push** - Upload files (stdin support with `-`)
- **Pull** - Download files by ID or key
- **Ls** - List files with pattern filtering
- **Rm** - Delete files
- **Share** - Generate presigned URLs (auto-copy to clipboard with `--copy`)
- **Info** - Show file metadata
- **Copy/Move** - Organize files within bucket
- **Cat** - Stream file to stdout

## Architecture

- **CLI**: Rust with clap
- **Storage**: Any S3-compatible provider (AWS, R2, Backblaze, MinIO)
- **Auth**: Your API keys - stored locally in config
