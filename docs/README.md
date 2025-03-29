# SmashLang Documentation

Welcome to the official SmashLang documentation! This guide provides comprehensive information about SmashLang, a bold, high-performance, JavaScript-inspired general-purpose programming language that compiles to native binaries.

## Table of Contents

### Getting Started
- [Installation Guide](./getting-started/installation.md)
- [Language Basics](./getting-started/language-basics.md)
- [Your First SmashLang Program](./getting-started/first-program.md)
- [Using the REPL](./getting-started/repl.md)

### Core Language
- [Syntax Reference](./language/syntax.md)
- [Types and Type System](./language/types.md)
- [Functions and Closures](./language/functions.md)
- [Control Flow](./language/control-flow.md)
- [Object Enhancements](./language/object-enhancements.md)
- [Destructuring and Spread Operator](./language/destructuring.md)
- [Error Handling](./language/error-handling.md)
- [Pattern Matching](./language/pattern-matching.md)
- [Modules and Imports](./language/modules.md)

### Standard Library
- [Overview](./standard-library/overview.md)
- [Core Types](./standard-library/core-types.md)
- [Collections](./standard-library/collections.md)
- [String Manipulation](./standard-library/strings.md)
- [Regular Expressions](./standard-library/regex.md)
- [Date and Time](./standard-library/datetime.md)
- [File System](./standard-library/filesystem.md)
- [Process Management](./standard-library/process.md)
- [OS Hooks](./standard-library/os-hooks.md)

### Networking
- [TCP/IP and UDP](./networking/tcp-udp.md)
- [HTTP/HTTPS Client](./networking/http-client.md)
- [WebSockets](./networking/websockets.md)

### Hardware Interfaces
- [Overview](./hardware/overview.md)
- [Camera Access](./hardware/camera.md)
- [Microphone Access](./hardware/microphone.md)
- [Screen Capture](./hardware/screen.md)
- [Input Devices](./hardware/input.md)
- [Cross-Platform Support](./hardware/cross-platform.md)

### Tools and Utilities
- [Compiler (smashc)](./tools/compiler.md)
- [Package Manager (smashpkg)](./tools/package-manager.md)
- [Test Runner (smashtest)](./tools/test-runner.md)
- [Language Server (smash-lang-server)](./tools/language-server.md)

### Advanced Topics
- [Memory Management](./advanced/memory-management.md)
- [Concurrency and Parallelism](./advanced/concurrency.md)
- [FFI (Foreign Function Interface)](./advanced/ffi.md)
- [WebAssembly Support](./advanced/wasm.md)
- [Performance Optimization](./advanced/optimization.md)

### Platform-Specific
- [Linux Development](./platforms/linux.md)
- [macOS Development](./platforms/macos.md)
- [Windows Development](./platforms/windows.md)
- [Android Development](./platforms/android.md)
- [iOS Development](./platforms/ios.md)

### Contributing
- [Development Setup](./contributing/setup.md)
- [Coding Standards](./contributing/coding-standards.md)
- [Architecture Overview](./contributing/architecture.md)
- [Testing Guidelines](./contributing/testing.md)

## Building the Documentation

To generate a static HTML site from these documentation files, run:

```bash
smash docs build
```

This will create a `docs-site` directory with the static HTML site that you can serve using any web server.

## Contributing to Documentation

We welcome contributions to the SmashLang documentation! If you find any errors, omissions, or areas that could be improved, please submit a pull request or open an issue on our GitHub repository.

When contributing to documentation, please follow these guidelines:

1. Use clear, concise language
2. Include code examples where appropriate
3. Follow the existing documentation structure
4. Test any code examples to ensure they work as expected

## License

ISC © 2025 SmashLang.com
