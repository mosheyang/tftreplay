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
    # Delete the stale bundle to avoid confusion
    rm -rf "target/release/bundle/osx/recorder_cli.app"
else
    echo "❌ Error: Could not find app bundle"
    exit 1
fi

DYLIB="apple_capture/.build/$(uname -m)-apple-macosx/release/libAppleCapture.dylib"

# Re-build the Swift package so the dylib is always fresh
echo "🔨 Building Swift package (release)..."
(cd apple_capture && swift build -c release)

# Frameworks and Resources folders
mkdir -p "$APP/Contents/Frameworks" "$APP/Contents/Resources"

if [ -f "resources/tft.icns" ]; then
    # >1 kB = real icon, placeholder is only ~70 B
    ICON_SIZE=$(stat -f%z resources/tft.icns)
    if [ $ICON_SIZE -lt 1024 ]; then
        echo "❌ resources/tft.icns is still a placeholder. Please replace it with a real icon."
        exit 1
    fi
    echo "📦 Copying app icon ($ICON_SIZE bytes)..."
    cp "resources/tft.icns" "$APP/Contents/Resources/"
fi

# Copy the Swift dynamic library into the app bundle
if [ -f "$DYLIB" ]; then
    echo "📦 Copying libAppleCapture.dylib to app bundle..."
    cp "$DYLIB" "$APP/Contents/Frameworks/"
else
    echo "❌ libAppleCapture.dylib not found at $DYLIB"
    exit 1
fi

# Fix the library references to use @rpath correctly
if command -v install_name_tool &> /dev/null; then
    echo "🔧 Updating library paths..."
    # Set the dylib's install name to @rpath-relative
    install_name_tool -id "@rpath/libAppleCapture.dylib" \
        "$APP/Contents/Frameworks/libAppleCapture.dylib"
    
    # Ensure the binary references the dylib via @rpath
    install_name_tool -change "@rpath/libAppleCapture.dylib" \
        "@rpath/libAppleCapture.dylib" \
        "$APP/Contents/MacOS/recorder"
fi

echo "✅ Created $APP"
echo ""
echo "To run the app:"
echo "  open \"$APP\""
echo ""
echo "First run will prompt for Screen Recording permission."