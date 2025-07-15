# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.1] - 2025-07-15

### Added
- Minimalist macOS GUI (`recorder_cli::gui`) bundled in `.app`
- Auto-generated rpath (`build.rs`) so `libAppleCapture.dylib` loads without `install_name_tool`

### Fixed
- Crash at launch "no LC_RPATH's found" on macOS 15.1
- Duplicate bundle (`recorder_cli.app`) now removed by `scripts/package_app.sh`

### Changed
- Single-command build: `cargo bundle --release`
- Docs updated for new toolchain minimums (Rust 1.87, macOS 13+)

## [0.1.0] - 2025-07-14

### Added
- Initial release with core recording functionality
- Hardware-accelerated H.264 encoding via VideoToolbox
- Swift-C-Rust FFI integration
- VS Code-style extension system
- macOS app bundle support
- CLI interface with record/host/daemon commands

### Features
- Ultra-light binary (< 1MB)
- Low CPU usage during recording
- Extensible plugin architecture
- Native macOS integration