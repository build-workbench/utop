# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Project standardization with comprehensive documentation
- CONTRIBUTING.md with contribution guidelines
- CODE_OF_CONDUCT.md following Contributor Covenant v2.1
- SECURITY.md with security policy
- GitHub issue and PR templates
- Enhanced CI/CD pipeline with cross-platform builds
- Automated release workflow
- docs/ directory with architecture and comparison documentation

### Changed

- Enhanced README.md with badges, architecture diagram, and better structure
- Improved .editorconfig with comprehensive settings
- Added rustfmt.toml for Rust code formatting
- Added .golangci.yml for Go linting

## [0.1.0] - 2025-01-07

### Added

- Initial release
- **dos2unix**: Rust implementation of CRLF to LF converter
  - In-place file conversion
  - stdin/stdout support
  - Check mode for detecting CRLF
  - Quiet mode option
- **gzip**: Multi-language gzip implementation
  - Go implementation (gzip-go)
  - Rust implementation (rgzip)
  - Basic compression and decompression
- **htop**: Cross-platform system monitor
  - Unix/Rust implementation
  - Windows/Rust implementation
  - Windows/Go implementation
  - Real-time process monitoring
  - CPU and memory usage display

### Infrastructure

- Cargo workspace for Rust projects
- Go workspace configuration
- GitHub Actions CI pipeline
- Makefile for unified build commands

---

## Sub-project Changelogs

For detailed changes in each sub-project, see:

- [dos2unix/changelog/](./dos2unix/changelog/)
- [gzip/changelog/](./gzip/changelog/)
- [gzip/go/changelog/](./gzip/go/changelog/)
- [gzip/rust/changelog/](./gzip/rust/changelog/)
- [htop/changelog/](./htop/changelog/)
- [htop/unix/rust/changelog/](./htop/unix/rust/changelog/)
- [htop/win/go/changelog/](./htop/win/go/changelog/)
