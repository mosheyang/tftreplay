# Architecture

The TFT Recorder is designed as a modular, extensible screen recording system optimized for Team Fight Tactics gameplay capture on macOS.

## Overview

```
┌─────────────────┐     ┌──────────────────┐     ┌─────────────────┐
│   CLI (Rust)    │────▶│  Core (Rust)     │────▶│ Swift Capture   │
│  recorder bin   │     │  FFI Bridge      │     │  AVFoundation   │
└─────────────────┘     └──────────────────┘     └─────────────────┘
         │                                                 │
         │                                                 │
         ▼                                                 ▼
┌─────────────────┐                            ┌─────────────────┐
│   GUI (Rust)    │  (egui) – lists recordings & buttons
└─────────────────┘
┌─────────────────┐                            ┌─────────────────┐
│ Extension Host  │                            │ Hardware H.264  │
│   (Node.js)     │                            │   Encoder       │
└─────────────────┘                            └─────────────────┘
```

## Core Components

### 1. Swift Capture Library (`apple_capture/`)

**Purpose**: Direct interface with macOS screen capture APIs

**Key Classes**:
- `CaptureSession`: Manages AVCaptureSession lifecycle
- `Encoder`: Hardware H.264 encoding via VideoToolbox
- `FrameRingBuffer`: Circular buffer for instant replay features

**Design Decisions**:
- Uses AVFoundation for maximum compatibility
- Hardware encoding to minimize CPU usage
- Direct CoreVideo buffer handling for zero-copy performance

### 2. Rust Core (`recorder_core/`)

**Purpose**: Safe FFI bridge and core recording logic

**Key Components**:
- FFI module: Manual C bindings to Swift (future: cxx for type safety)
- Recorder struct: Thread-safe recording state management
- Platform abstraction: Allows future Linux support

### 3. CLI & GUI (`recorder_cli/`)

**Purpose**: User-facing interfaces

**CLI Subcommands**:
- `record`: Start recording with specified parameters
- `host`: Launch extension host (internal)
- `daemon`: Run as background service for IPC

**GUI**: Auto-launch when bundled as `.app` (egui front-end)

### 4. Extension Host (`extension-host/`)

**Purpose**: VS Code-style plugin system

**Features**:
- Dynamic extension loading from `~/.tft-recorder/extensions/`
- gRPC IPC for recorder control
- Event-based activation (onRecordingStart, onCommand, etc.)
- TypeScript SDK for extension development

## Data Flow

1. **Recording Start**:
   ```
   CLI --[start cmd]--> Rust Core --[FFI]--> Swift --[AVCapture]--> Screen
   ```

2. **Frame Processing**:
   ```
   Screen --[CVPixelBuffer]--> Encoder --[H.264]--> MP4 Writer
                                  |
                                  └--[Ring Buffer]--> Extensions
   ```

3. **Extension Communication**:
   ```
   Extension <--[gRPC/Unix Socket]--> Daemon <--[Events]--> Recorder
   ```

## Key Design Principles

1. **Minimal Overhead**: Every component optimized for low latency
2. **Extensibility First**: Plugin architecture from day one
3. **Native Performance**: Platform-specific optimizations
4. **Type Safety**: Rust core with strong FFI contracts
5. **User Experience**: Simple CLI with sensible defaults

## Future Considerations

- **Linux Support**: Wayland capture via PipeWire
- **Windows Support**: Desktop Duplication API
- **GPU Encoding**: Metal/CUDA accelerated filters
- **Live Streaming**: RTMP output module
- **Cloud Sync**: Automatic highlight upload