# Compiling SmashLang Programs for Linux

This document provides information about compiling SmashLang programs specifically for Linux platforms.

## Overview

SmashLang supports compiling programs for various Linux distributions and architectures. The compiler uses LLVM and Clang to generate native Linux binaries that can take advantage of Linux-specific features and libraries.

## Target Platforms

When compiling for Linux, you can specify the target platform using the `--target` flag. The following Linux targets are supported:

- `x86_64-unknown-linux-gnu` - 64-bit Intel/AMD Linux (most common)
- `aarch64-unknown-linux-gnu` - 64-bit ARM Linux (e.g., Raspberry Pi 4, many servers)
- `arm-unknown-linux-gnueabihf` - 32-bit ARM Linux (e.g., older Raspberry Pi models)

If no target is specified and you're compiling on a Linux system, the compiler will target your current platform.

## Command Line Examples

### Basic Compilation

To compile a SmashLang program for the current Linux platform:

```bash
smashc hello.smash
```

### Targeting Specific Linux Platforms

To compile for a specific Linux platform:

```bash
smashc hello.smash --target x86_64-unknown-linux-gnu
```

For ARM64 Linux (e.g., Raspberry Pi 4):

```bash
smashc hello.smash --target aarch64-unknown-linux-gnu
```

For 32-bit ARM Linux:

```bash
smashc hello.smash --target arm-unknown-linux-gnueabihf
```

### Specifying Output File

To specify the output file name:

```bash
smashc hello.smash -o hello_program
```

### Release Mode

For optimized builds:

```bash
smashc hello.smash --release
```

## Linux-Specific Features

When compiling for Linux, the SmashLang compiler automatically:

1. Links against standard Linux libraries (`-lm`, `-ldl`, `-lpthread`)
2. Sets appropriate architecture-specific flags
3. Configures the executable format for Linux (ELF)

## Required Libraries

To compile SmashLang programs for Linux, you need the following dependencies installed:

- LLVM (version 15.0 or later)
- Clang (version 15.0 or later)
- Standard C libraries and headers

On Debian/Ubuntu systems, you can install these with:

```bash
sudo apt-get install llvm-15 clang-15 libclang-15-dev
```

On Fedora/RHEL systems:

```bash
sudo dnf install llvm clang
```

## Troubleshooting

### Common Issues

1. **Missing Libraries**: If you encounter "missing library" errors during linking, ensure you have the required development libraries installed.

   ```bash
   sudo apt-get install build-essential
   ```

2. **Cross-Compilation Issues**: When cross-compiling for a different architecture, you may need to install the appropriate cross-compilation toolchain.

   ```bash
   # For ARM64 target on Debian/Ubuntu
   sudo apt-get install gcc-aarch64-linux-gnu
   ```

3. **Permission Issues**: If you can't execute the compiled binary, ensure it has execute permissions:

   ```bash
   chmod +x ./your_program
   ```

### Debugging

For debugging issues with Linux compilation, you can use:

```bash
smashc hello.smash --debug
```

This will include debug information in the binary, making it easier to debug with tools like GDB.

## Advanced Topics

### Static Linking

To create a statically linked binary (useful for distribution):

```bash
smashc hello.smash --static
```

### Using Linux-Specific Features

SmashLang provides access to Linux-specific features through the standard library. For example:

```javascript
import "std/os";

// Use Linux-specific features
if (std.os.platform === "linux") {
  // Linux-specific code
}
```

## Further Reading

- [SmashLang Standard Library Documentation](./std_library.md)
- [Cross-Compilation Guide](./cross_compilation.md)
- [Performance Optimization](./optimization.md)