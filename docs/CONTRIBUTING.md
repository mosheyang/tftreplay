# Contributing to TFT Recorder

Thank you for your interest in contributing! This document provides guidelines and instructions for contributing to the TFT Recorder project.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/yourusername/tft-recorder.git`
3. Create a feature branch: `git checkout -b feature/your-feature-name`
4. Make your changes
5. Run tests: `cargo test && swift test`
6. Submit a pull request

## Development Setup

### Prerequisites

- macOS 12.0+
- Xcode 14+ with command line tools
- Rust 1.79+ (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
- Swift 5.10+
- Node.js 18+ (`brew install node`)

### Building

```bash
# Build everything
make all

# Or build components individually
cd apple_capture && swift build
cargo build --release
cd extension-host && npm install && npm run build
```

### Testing

```bash
# Run all tests
make test

# Or test individually
cargo test --workspace
cd apple_capture && swift test
cd extension-host && npm test
```

## Code Style

### Rust
- Follow standard Rust conventions
- Use `cargo fmt` before committing
- Run `cargo clippy` and fix warnings
- Document public APIs with `///` comments

### Swift
- Follow Swift API Design Guidelines
- Use SwiftLint configuration (if present)
- Document public APIs with `///` comments
- Prefer `guard` over nested `if` statements

### TypeScript
- Use ESLint configuration
- Prefer `const` over `let`
- Use async/await over callbacks
- Document exported functions with JSDoc

## Project Structure

```
tft-recorder/
├── recorder_core/      # Rust library (FFI bridge)
├── recorder_cli/       # Rust CLI binary
├── apple_capture/      # Swift screen capture
├── extension-host/     # Node.js plugin host
└── docs/              # Documentation
```

## Making Changes

### Adding Features

1. Discuss major features in an issue first
2. Keep changes focused and atomic
3. Update tests and documentation
4. Add entries to CHANGELOG.md

### Bug Fixes

1. Add a test that reproduces the bug
2. Fix the bug
3. Ensure all tests pass
4. Reference the issue in your commit

### Performance Improvements

1. Benchmark before and after
2. Document the improvement in the PR
3. Ensure no regression in functionality
4. Consider impact on binary size

## Pull Request Process

1. **Title**: Use conventional commits format (e.g., `feat:`, `fix:`, `docs:`)
2. **Description**: Explain what and why, not how
3. **Tests**: All tests must pass
4. **Documentation**: Update if needed
5. **Sign-off**: Sign your commits (`git commit -s`)

## Extension Development

See the [Extension SDK documentation](../extension-host/sdk/README.md) for:
- API reference
- Example extensions
- Publishing guidelines

## Debugging Tips

### Rust
```bash
RUST_LOG=debug cargo run -- record
RUST_BACKTRACE=1 cargo test
```

### Swift
Use Xcode for debugging:
```bash
cd apple_capture
swift package generate-xcodeproj
open *.xcodeproj
```

### Node.js
```bash
NODE_ENV=development node --inspect extension-host/dist/index.js
```

## Release Process

1. Update version in all `Cargo.toml` and `package.json` files
2. Update CHANGELOG.md
3. Create a tagged release
4. CI will build and upload artifacts

## Getting Help

- Check existing issues and discussions
- Join our Discord server (if applicable)
- Ask questions in issues with the "question" label

## Code of Conduct

- Be respectful and inclusive
- Focus on constructive criticism
- Help others learn and grow
- Report unacceptable behavior to maintainers

Thank you for contributing to TFT Recorder!