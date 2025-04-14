#!/bin/bash
# Cross-compilation script for SmashLang
# Usage: ./scripts/cross_compile.sh <target> <source_file>
# Example: ./scripts/cross_compile.sh x86_64-pc-windows-gnu examples/hello.smash

set -e

if [ $# -lt 2 ]; then
    echo "Usage: $0 <target> <source_file>"
    echo "Available targets:"
    echo "  x86_64-pc-windows-gnu       - Windows (64-bit)"
    echo "  x86_64-unknown-linux-gnu    - Linux (64-bit)"
    echo "  x86_64-apple-darwin         - macOS (64-bit)"
    echo "  aarch64-apple-ios           - iOS (ARM64)"
    echo "  aarch64-linux-android       - Android (ARM64)"
    echo "  wasm32-unknown-unknown      - WebAssembly"
    exit 1
fi

TARGET=$1
SOURCE_FILE=$2
FILENAME=$(basename "$SOURCE_FILE" .smash)
OUTPUT_DIR="target/$TARGET/release"

# Ensure the output directory exists
mkdir -p "$OUTPUT_DIR"

# Build the compiler for the target
echo "Building SmashLang compiler for $TARGET..."
cargo build --release --target "$TARGET"

# Compile the source file
echo "Compiling $SOURCE_FILE for $TARGET..."
case "$TARGET" in
    wasm32-unknown-unknown)
        # For WebAssembly, we need to use wasm-pack
        if ! command -v wasm-pack &> /dev/null; then
            echo "wasm-pack not found. Installing..."
            cargo install wasm-pack
        fi
        
        # Create a temporary directory for the wasm project
        TEMP_DIR=$(mktemp -d)
        cp "$SOURCE_FILE" "$TEMP_DIR/input.smash"
        
        # Create a simple wasm project
        cat > "$TEMP_DIR/Cargo.toml" << EOF
[package]
name = "smashlang-wasm"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
smashlang = { path = "../../" }
wasm-bindgen = "0.2"
EOF
        
        # Create the lib.rs file
        mkdir -p "$TEMP_DIR/src"
        cat > "$TEMP_DIR/src/lib.rs" << EOF
use wasm_bindgen::prelude::*;
use std::fs;

#[wasm_bindgen]
pub fn run() -> String {
    let source = include_str!("../input.smash");
    match smashlang::execute(source) {
        Ok(value) => value.to_string(),
        Err(err) => format!("Error: {}", err),
    }
}
EOF
        
        # Build the wasm module
        (cd "$TEMP_DIR" && wasm-pack build --target web)
        
        # Copy the output
        cp "$TEMP_DIR/pkg/smashlang_wasm_bg.wasm" "$OUTPUT_DIR/$FILENAME.wasm"
        cp "$TEMP_DIR/pkg/smashlang_wasm.js" "$OUTPUT_DIR/$FILENAME.js"
        
        # Create an HTML file to run the wasm
        cat > "$OUTPUT_DIR/$FILENAME.html" << EOF
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>SmashLang WebAssembly</title>
</head>
<body>
    <h1>SmashLang WebAssembly</h1>
    <div id="output"></div>
    <script type="module">
        import init, { run } from './$FILENAME.js';
        
        async function runWasm() {
            await init();
            const result = run();
            document.getElementById('output').textContent = result;
        }
        
        runWasm();
    </script>
</body>
</html>
EOF
        
        echo "WebAssembly output: $OUTPUT_DIR/$FILENAME.wasm"
        echo "JavaScript wrapper: $OUTPUT_DIR/$FILENAME.js"
        echo "HTML runner: $OUTPUT_DIR/$FILENAME.html"
        ;;
    *)
        # For native targets, use the SmashLang compiler
        EXTENSION=""
        if [[ "$TARGET" == *windows* ]]; then
            EXTENSION=".exe"
        fi
        
        # Use the SmashLang compiler to compile the source file
        "$OUTPUT_DIR/smash$EXTENSION" compile "$SOURCE_FILE" -o "$OUTPUT_DIR/$FILENAME$EXTENSION"
        
        echo "Output: $OUTPUT_DIR/$FILENAME$EXTENSION"
        ;;
esac

echo "Cross-compilation complete!"