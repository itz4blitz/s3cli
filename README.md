# s3cli

CLI-first S3 storage for developers and AI agents.

**Website**: https://s3cli.com

## Concept

```bash
# Upload files
s3cli push demo.mp4
s3cli push screenshot.png

# Download
s3cli pull abc123

# Share with expiring links
s3cli share abc123 --expires 7d

# List your files
s3cli ls

# Configure storage
s3cli config --provider r2 --key X --secret Y --bucket my-files
```

## Architecture

- **CLI**: Rust-based with clap
- **Storage**: BYO S3-compatible (R2, Backblaze, AWS, MinIO, local)
- **Auth**: OAuth (GitHub, Google) - for hosted version
- **Landing page**: Next.js with OAuth login (optional)

## Why s3cli?

- No API keys to manage for your AI agents
- Unified CLI across any S3-compatible storage
- AI-friendly: simple URLs for embedding in agents
- Bring your own storage - you control your data

## Roadmap

- [ ] MVP: Rust CLI with local/S3 storage
- [ ] OAuth integration (for hosted version)
- [ ] Landing page
- [ ] Pre-signed URL generation
- [ ] File metadata and sharing
