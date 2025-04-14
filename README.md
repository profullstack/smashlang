# SmashLang: A JS-like Programming Language in Rust

<p align="center">
  <img src="assets/logo.png" alt="SmashLang Logo" width="200">
</p>

<p align="center">
  <a href="https://github.com/yourusername/smashlang/actions"><img src="https://github.com/yourusername/smashlang/workflows/CI/badge.svg" alt="Build Status"></a>
  <a href="https://crates.io/crates/smashlang"><img src="https://img.shields.io/crates/v/smashlang.svg" alt="Version"></a>
  <a href="https://github.com/yourusername/smashlang/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License: MIT"></a>
  <a href="https://docs.rs/smashlang"><img src="https://docs.rs/smashlang/badge.svg" alt="Documentation"></a>
</p>

SmashLang is a JavaScript-inspired programming language implemented in Rust that compiles to native binaries across all major platforms (desktop, mobile, server, WebAssembly, etc.). The language supports dynamic typing, native date/time, regular expressions, and control flow constructs like `if`, `for`, and `while`.

## 🚀 Features

- **JavaScript-like Syntax**: Familiar syntax for JavaScript developers
- **Dynamic Typing**: Flexible type system with runtime type checking
- **Native Compilation**: Compiles to native binaries for all major platforms
- **WebAssembly Support**: Compile to WebAssembly for web applications
- **Cross-Platform**: Works on Linux, macOS, Windows, iOS, Android, and more
- **Standard Library**: Built-in support for common operations
- **Regular Expressions**: Native regex support
- **Date/Time Handling**: Comprehensive date and time functionality
- **Error Handling**: Try/catch/finally mechanism
- **Modern Language Features**: Destructuring, pattern matching, async/await, and more

## 🧰 Implementation

SmashLang is built using modern Rust crates:

- **Lexer**: Uses [logos](https://crates.io/crates/logos) for efficient tokenization
- **Parser**: Uses [pest](https://crates.io/crates/pest) for parsing with PEG grammar
- **Interpreter**: Custom interpreter with dynamic typing
- **Compiler**: Native code generation using [cranelift](https://crates.io/crates/cranelift)
- **Standard Library**: Implemented using Rust's ecosystem (chrono, regex, etc.)

## 📦 Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/yourusername/smashlang.git
cd smashlang

# Build the project
cargo build --release

# Install the binaries
cargo install --path .
```

### Using the Install Script

```bash
curl -sSL https://raw.githubusercontent.com/yourusername/smashlang/main/install.sh | bash
```

## 🚀 Quick Start

### Hello World

Create a file named `hello.smash`:

```javascript
// hello.smash
print("Hello, SmashLang!");
```

Run it:

```bash
smash run hello.smash
```

### Compile to Native Binary

```bash
smashc hello.smash -o hello
./hello
```

### Compile to WebAssembly

```bash
smashc hello.smash --wasm -o hello.wasm
```

## 📚 Documentation

Comprehensive documentation is available in the `docs` directory:

- [Getting Started Guide](docs/getting-started/README.md)
- [Language Reference](docs/language/README.md)
- [Standard Library](docs/std/README.md)
- [WebAssembly Support](docs/wasm_support.md)
- [OS Hooks](docs/std_os_hooks.md)
- [Process Management](docs/std_process.md)

## 🧪 Examples

SmashLang comes with a variety of examples to help you learn:

### Language Features

- [Control Flow](docs/language/examples/control-flow.smash)
- [Error Handling](docs/language/examples/error-handling.smash)
- [Functions](docs/language/examples/functions.smash)
- [Modules](docs/language/examples/modules.smash)
- [Pattern Matching](docs/language/examples/pattern-matching.smash)
- [Syntax](docs/language/examples/syntax.smash)
- [Types](docs/language/examples/types.smash)
- [Destructuring](docs/language/examples/destructuring.smash)
- [Object Enhancements](docs/language/examples/object-enhancements.smash)

### OS Integration

- [File System](docs/os_hooks/examples/file_system.smash)
- [Process Management](docs/os_hooks/examples/process_management.smash)

### WebAssembly

- [Hello WASM](docs/wasm/examples/hello_wasm.smash)

### Hardware Integration

- [Input Tester](docs/hardware/examples/input_tester.smash)
- [Keyboard Monitor](docs/hardware/examples/keyboard_monitor.smash)
- [Mouse Tracker](docs/hardware/examples/mouse_tracker.smash)
- [Touch Visualizer](docs/hardware/examples/touch_visualizer.smash)

## 🧪 Testing

Run all examples:

```bash
cd docs
./test_all_examples.sh
```

Run the test suite:

```bash
cargo test
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.