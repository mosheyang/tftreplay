# TFT Recorder

Ultra-light screen recorder for Team Fight Tactics (TFT) with VS Code-style extensibility.

## Features

- **Ultra-light binary**: ≤ 40 MB single-file .app bundle
- **Hardware H.264 encoding**: 5-10× less CPU usage vs software encoding
- **Plugin ecosystem**: VS Code-style extension support
- **Mac-first design**: Built specifically for macOS using native APIs

## Quick Start

```bash
# Record TFT at 720p
recorder record

# Record with custom settings
recorder record --width 1920 --height 1080 --bitrate 8000000 --out gameplay.mp4

# Stop recording with Ctrl+C
```

## Installation

### Prerequisites

- macOS 12.0 or later
- Rust 1.79+
- Swift 5.10+
- Node.js 18+

### Building from source

```bash
# Clone the repository
git clone https://github.com/yourusername/tft-recorder.git
cd tft-recorder

# Build Swift package
cd apple_capture
swift build -c release
cd ..

# Build Rust binary
cargo build --release

# The binary will be at ./target/release/recorder
```

## Architecture

The recorder consists of three main components:

1. **Swift Capture Library** (`apple_capture/`): Handles screen capture using AVFoundation and hardware encoding via VideoToolbox
2. **Rust Core & CLI** (`recorder_core/` & `recorder_cli/`): Provides the main binary and FFI bridge to Swift
3. **Node.js Extension Host** (`extension-host/`): Manages plugins with VS Code-style extensibility

## Extension Development

Extensions follow the VS Code extension model. Create a `package.json` with:

```json
{
  "name": "my-extension",
  "version": "0.1.0",
  "main": "dist/index.js",
  "activationEvents": ["onRecordingStart"],
  "contributes": {
    "commands": [{
      "command": "myExtension.doSomething",
      "title": "Do Something Cool"
    }]
  }
}
```

Place extensions in `~/.tft-recorder/extensions/`.

## Performance

- Cold start: < 50ms
- CPU usage during 1080p/60fps capture: < 150%
- Memory usage: < 100MB
- File size: ~1MB/minute at 720p/4Mbps

## Contributing

See [CONTRIBUTING.md](docs/CONTRIBUTING.md) for development guidelines.

## License

MIT License - see LICENSE file for details.