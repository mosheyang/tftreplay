#!/usr/bin/env bash
# ABOUTME: Script to package the TFT Recorder as a macOS .app bundle
# ABOUTME: Handles building, bundling, and copying the Swift dylib

set -euo pipefail

echo "🔨 Building TFT Recorder.app..."

# Build the release binary and bundle it
cargo bundle --bin recorder --release

# The bundle might be named differently, check both possibilities
if [ -d "target/release/bundle/osx/TFT Recorder.app" ]; then
    APP="target/release/bundle/osx/TFT Recorder.app"
elif [ -d "target/release/bundle/osx/recorder_cli.app" ]; then
    APP="target/release/bundle/osx/recorder_cli.app"
    # Rename to the expected name
    mv "$APP" "target/release/bundle/osx/TFT Recorder.app"
    APP="target/release/bundle/osx/TFT Recorder.app"
else
    echo "❌ Error: Could not find app bundle"
    exit 1
fi

DYLIB="apple_capture/.build/$(uname -m)-apple-macosx/release/libAppleCapture.dylib"

# Ensure the Frameworks directory exists
mkdir -p "$APP/Contents/Frameworks"

# Copy the Swift dynamic library into the app bundle
if [ -f "$DYLIB" ]; then
    echo "📦 Copying libAppleCapture.dylib to app bundle..."
    cp "$DYLIB" "$APP/Contents/Frameworks/"
else
    echo "⚠️  Warning: libAppleCapture.dylib not found at $DYLIB"
    echo "   Make sure to build the Swift package first: cd apple_capture && swift build -c release"
fi

# Fix the library reference to use @rpath
if command -v install_name_tool &> /dev/null; then
    echo "🔧 Updating library paths..."
    install_name_tool -change "@rpath/libAppleCapture.dylib" "@executable_path/../Frameworks/libAppleCapture.dylib" "$APP/Contents/MacOS/recorder" 2>/dev/null || true
fi

echo "✅ Created $APP"
echo ""
echo "To run the app:"
echo "  open \"$APP\""
echo ""
echo "First run will prompt for Screen Recording permission."