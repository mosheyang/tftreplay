# TFT Recorder

Ultra-light screen recorder for Team Fight Tactics (TFT) with VS Code-style extensibility.

## Features

- **Ultra-light bundle**: ≤ 40 MB self-contained `.app`
- **Hardware H.264 encoding**: 5-10× lower CPU than software encoding
- **Plugin ecosystem**: VS Code-style extension support
- **Minimalist GUI**: "TFT Recorder.app" lists existing captures & one-click record/stop

## Quick Start

```bash
# Launch the macOS app (preferred)
open /Applications/TFT\ Recorder.app          # or double-click in Finder

# CLI fallback (headless / CI)
recorder record --window Finder --duration 10 --out ~/Movies/tft.mp4

# Stop recording with Ctrl+C
```

## Installation

### Prerequisites

- macOS 13.0+ (Ventura)
- Rust 1.87+
- Swift 5.10+
- Node.js 18+

### Building from source

```bash
# Clone & bundle
git clone https://github.com/yourusername/tft-recorder.git
cd tft-recorder

# Release bundle with one command
cargo install cargo-bundle            # first time only
cargo bundle --bin recorder --release # creates target/release/bundle/osx/TFT\ Recorder.app
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