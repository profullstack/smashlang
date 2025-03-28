#!/bin/bash

SRC=${1:-example.smash}
OUT=${2:-build/out}
mkdir -p build

# Native build
echo "Building native..."
cargo run --release -- ${SRC} --out ${OUT}_native --emit exe

# Windows (requires x86_64-w64-mingw32 toolchain)
if command -v clang >/dev/null && clang --target=x86_64-w64-windows-gnu -v >/dev/null 2>&1; then
    echo "Building for Windows..."
    cargo run --release -- ${SRC} --out ${OUT}_windows --emit exe --target x86_64-w64-windows-gnu
else
    echo "Skipping Windows build (clang/mingw not available)"
fi

# Android (requires Android NDK)
if [ -n "$ANDROID_NDK_HOME" ]; then
    echo "Building for Android..."
    cargo run --release -- ${SRC} --out ${OUT}_android --emit exe --target aarch64-linux-android
else
    echo "Skipping Android build (ANDROID_NDK_HOME not set)"
fi

# macOS (requires osxcross or native mac)
if clang -v 2>&1 | grep -q "apple"; then
    echo "Building for macOS..."
    cargo run --release -- ${SRC} --out ${OUT}_macos --emit exe --target x86_64-apple-darwin
else
    echo "Skipping macOS build (no Apple clang detected)"
fi

# iOS (requires macOS + SDK)
if clang -v 2>&1 | grep -q "apple"; then
    echo "Building for iOS..."
    cargo run --release -- ${SRC} --out ${OUT}_ios --emit exe --target arm64-apple-ios
else
    echo "Skipping iOS build (not running on macOS)"
fi
