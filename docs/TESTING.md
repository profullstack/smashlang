# SmashLang Testing Guide

## Running Tests with the Installer

When installing SmashLang from the master branch using the `install.sh` script with the `--master` flag, the installer will automatically run a comprehensive test suite to ensure the codebase is functioning correctly. This provides an additional layer of verification before the compiler is built and installed.

The installer runs the following tests:

1. Main crate tests (`cargo test`)
2. Tests for all workspace packages (`cargo test --all`)
3. Tests with all features enabled (`cargo test --all-features`)
4. Example tests from the `docs/getting-started` directory
5. Tests for all packages in the `smashlang_packages` directory

```bash
# Install from master branch with comprehensive tests
./install.sh --master
```

## Manual Testing

### Running Unit Tests

SmashLang has a comprehensive test suite that can be run manually using Cargo:

```bash
# Run all tests in the main crate
cargo test

# Run tests with verbose output
cargo test -- --nocapture

# Run a specific test
cargo test test_name

# Run tests for all workspace packages
cargo test --all

# Run tests with all features enabled
cargo test --all-features

# Run tests for a specific package in the smashlang_packages directory
cd smashlang_packages/package_name && cargo test
```

### Testing Examples

The `docs/getting-started` directory contains example SmashLang programs that can be used to test the compiler. A test script is provided to run all examples automatically:

```bash
# Run all examples
./docs/getting-started/run_all_examples.sh
```

## Test Coverage

The SmashLang test suite includes tests for:

1. **Lexer**: Tests for tokenizing source code
2. **Parser**: Tests for parsing tokens into an AST
3. **Code Generation**: Tests for generating C code from the AST
4. **Compiler**: End-to-end tests for the complete compilation process

## Adding New Tests

When adding new features to SmashLang, it's important to also add corresponding tests. Tests should be placed in the `tests` directory and follow the existing naming conventions:

- `lexer_parser_tests.rs`: Tests for the lexer and parser
- `codegen_tests.rs`: Tests for code generation
- `compiler_tests.rs`: End-to-end tests for the compiler

## Continuous Integration

In a CI/CD environment, tests can be run automatically using:

```bash
# Run all tests and examples
cargo test && ./docs/getting-started/run_all_examples.sh
```

This ensures that all changes to the codebase are properly tested before being merged.
