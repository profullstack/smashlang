# SmashLang Architecture and Development Guidelines

## Overview

SmashLang is a JavaScript-inspired programming language that compiles to native binaries. It aims to provide the clarity and ease of use of JavaScript with the performance of languages like C and Rust, without the bloat. This document outlines the architecture, coding standards, and contribution guidelines for SmashLang.

## Project Structure

```
smashlang/
├── src/               # Source code
│   ├── lexer.rs      # Tokenization of source code
│   ├── parser.rs     # Parsing tokens into AST
│   ├── compiler.rs   # Compiling AST to LLVM IR
│   ├── repl.rs       # Interactive REPL implementation
│   ├── main.rs       # Main entry point for the `smash` binary
│   ├── smashpkg.rs   # Package manager implementation
│   ├── smashc.rs     # Standalone compiler implementation
│   └── lib.rs        # Library exports
├── assets/           # Static assets (logos, etc.)
├── scripts/          # Utility scripts
├── install.sh        # Cross-platform installer
└── Cargo.toml        # Rust package definition
```

## Architecture

SmashLang follows a traditional compiler pipeline architecture:

1. **Lexical Analysis (Lexer)**: Converts source code into tokens
2. **Parsing (Parser)**: Converts tokens into an Abstract Syntax Tree (AST)
3. **Semantic Analysis**: Validates the AST and performs type checking
4. **Optimization**: Optimizes the AST for better performance
5. **Code Generation (Compiler)**: Generates LLVM IR from the AST
6. **Binary Generation**: Uses LLVM to generate native binaries

The REPL provides an interactive environment for testing SmashLang code without compiling to binaries.

## Coding Standards

### Rust Code Style

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `rustfmt` for consistent formatting
- Use `clippy` for linting
- Aim for zero warnings in the codebase
- Use meaningful variable and function names
- Add comments for complex logic, but prefer self-documenting code

### Documentation

- Document all public functions, structs, and enums with doc comments
- Include examples in doc comments where appropriate
- Keep the README.md up-to-date with installation and usage instructions
- Document language features in the docs/ directory

### Testing

- Write unit tests for all components
- Include integration tests for the compiler pipeline
- Add regression tests for fixed bugs
- Aim for high test coverage, especially in the lexer and parser

### Error Handling

- Use `Result` and `Option` types for error handling
- Provide meaningful error messages with source locations
- Avoid panics in library code
- Use the `anyhow` crate for error propagation

## Language Design Principles

1. **JavaScript-inspired syntax**: Familiar to web developers
2. **Static typing with inference**: Type safety without verbosity
3. **Performance-focused**: Compile to efficient native code
4. **Modern features**: Support for modern programming paradigms
5. **Cross-platform**: Run on Linux, macOS, and Windows

## Contribution Guidelines

### Pull Request Process

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests and ensure they pass
5. Submit a pull request

### Commit Message Format

Use the following format for commit messages:

```
<type>(<scope>): <subject>

<body>

<footer>
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Formatting changes
- `refactor`: Code refactoring
- `test`: Adding or modifying tests
- `chore`: Maintenance tasks

### Code Review Process

- All code must be reviewed before merging
- Address all review comments
- Ensure tests pass before merging
- Keep pull requests focused on a single feature or fix

## Development Environment Setup

### Requirements

- Rust (latest stable version)
- LLVM 15.0 (for the compiler)
- Git

### Command-Line Tools

- `smash`: Interactive REPL and script runner
- `smashc`: Standalone compiler for building executables and libraries
- `smashpkg`: Package manager for installing and managing dependencies

### Setup Steps

1. Clone the repository
2. Install dependencies
3. Build with `cargo build`
4. Run tests with `cargo test`

## Binary Targets

- `smash`: The main language binary (REPL and runner)
- `smashpkg`: The package manager
- `smashc`: The standalone compiler for developers

## Feature Implementation Guidelines

### Adding Language Features

1. Update the lexer to recognize new tokens if needed
2. Update the parser to handle new syntax
3. Add AST nodes for new constructs
4. Implement evaluation in the REPL
5. Implement code generation in the compiler
6. Add tests for the new feature
7. Update documentation

### Adding Standard Library Features

1. Define the API in the standard library
2. Implement the functionality
3. Add bindings to the compiler
4. Add tests for the new functionality
5. Update documentation

## Cross-Platform Considerations

- Use platform-agnostic APIs when possible
- Test on all supported platforms (Linux, macOS, Windows)
- Handle platform-specific paths and behaviors
- Use conditional compilation for platform-specific code

## Performance Considerations

- Profile code to identify bottlenecks
- Optimize hot paths
- Use efficient data structures
- Minimize memory allocations
- Consider parallelism for CPU-intensive tasks

## Security Considerations

- Validate all user input
- Avoid unsafe code when possible
- Follow secure coding practices
- Keep dependencies up-to-date

## Versioning and Releases

- Follow Semantic Versioning (SemVer)
- Maintain a changelog
- Tag releases in Git
- Publish binaries for all supported platforms

## Conclusion

By following these guidelines, we can maintain a high-quality codebase and create a powerful, user-friendly programming language. SmashLang aims to combine the best aspects of JavaScript with the performance of native code, providing a compelling alternative for developers across various domains.
