# SmashLang Compiler Guide

## Introduction

SmashLang is a modern programming language designed for simplicity and expressiveness. This guide will help you get started with the SmashLang compiler and run your first programs.

## Prerequisites

Before you begin, make sure you have the following installed:

- Rust and Cargo (for building the compiler)
- Clang (required for the compilation process)

## Building the Compiler

To build the SmashLang compiler from source:

```bash
# Clone the repository (if you haven't already)
git clone https://github.com/profullstack/smashlang.git
cd smashlang

# Build the compiler
cargo build --release
```

This will create the `smashc` executable in the `target/release` directory.

## Using the Compiler

The SmashLang compiler (`smashc`) takes a SmashLang source file and compiles it to an executable:

```bash
# Basic usage
smashc input.smash -o output

# Specify a target platform
smashc input.smash -o output --target linux
```

Available targets include:
- `linux` (x86_64)
- `linux-arm64` (ARM64)
- `macos` (x86_64)
- `macos-arm64` (Apple Silicon)
- `windows` (x86_64)

## Running the Examples

This directory contains several example programs that demonstrate SmashLang features. To run them:

```bash
# Compile an example
smashc docs/getting-started/01_hello_world.smash -o hello

# Run the compiled program
./hello
```

## Current Implementation Status

The SmashLang compiler is under active development. The current version:

- Parses the full SmashLang syntax
- Generates C code as an intermediate representation
- Uses Clang to compile the C code to an executable

Some language features are parsed but not yet fully implemented in the code generation phase. The examples in this directory are designed to work with the current implementation, with comments indicating future capabilities.

## Developing with SmashLang

When writing SmashLang programs, remember these key points:

1. All statements must end with a semicolon (`;`)
2. Variables are declared using `let` and constants with `const`
3. Functions are defined using the `fn` keyword

Refer to the example programs for more details on syntax and features.

## Troubleshooting

If you encounter issues:

- Make sure Clang is installed and available in your PATH
- Check that your SmashLang syntax is correct (all statements end with semicolons)
- Verify that you're using features supported by the current implementation

For more help, refer to the project documentation or open an issue on GitHub.
