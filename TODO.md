# SmashLang TODO List

This document outlines the features, components, and architectural improvements needed for SmashLang.

## Architectural Refactoring

### Modular Architecture Migration

- [x] Create runtime module structure (`src/runtime/`)
- [ ] Migrate regex engine to runtime module (`src/runtime/regex.rs`)
- [ ] Create frontend module structure (`src/frontend/`)
  - [ ] Migrate lexer to frontend module
  - [ ] Migrate parser to frontend module with submodules for different language constructs
- [ ] Create backend module structure (`src/backend/`)
  - [ ] Migrate compiler to backend module
  - [ ] Create separate modules for different compilation targets
- [ ] Create tools module structure (`src/tools/`)
  - [ ] Migrate REPL to tools module
  - [ ] Migrate testing framework to tools module
  - [ ] Migrate package manager to tools module

### Code Organization Improvements

- [ ] Create clear interfaces between modules
- [ ] Implement proper error handling across module boundaries
- [ ] Add comprehensive documentation for module interfaces
- [ ] Create tests for module interfaces
- [ ] Implement proper dependency management between modules

## MVP v0.1.0

### Core Compiler
- [x] Complete basic LLVM IR code generation for essential AST node types
- [x] Implement simple error handling
- [x] Add support for basic binary operations
- [x] Support compilation for at least one target platform (Linux)
- [ ] Implement basic memory management (array allocation needs work)
- [x] Add unit tests for compiler components (lexer, parser, codegen)

### Language Features
- [x] Implement basic function definitions and calls
- [x] Add support for variables and basic data types
- [x] Implement simple control flow (if/else, loops, for...in, for...of)
- [ ] Complete module imports (parser implemented, codegen needed)
- [x] Implement simple error handling with try/catch

### Standard Library
- [x] Implement basic file I/O operations (via stdio packages)
- [x] Add simple string manipulation functions
- [ ] Complete array operations implementation
- [ ] Integrate HTTP client from networking/http package
- [ ] Integrate JSON parsing from core/json package
- [x] Add SmashTest tests for standard library functions

### Package Manager
- [x] Implement basic package installation
- [ ] Add simple dependency resolution
- [x] Implement package creation
- [ ] Add tests for package manager functionality

### Testing Framework
- [x] Implement basic test runner
- [x] Add essential assertions
- [x] Support simple test organization
- [x] Create example tests for language features

### Documentation
- [x] Create basic language reference
- [x] Add installation and getting started guides
- [ ] Document core library functions
- [ ] Add testing documentation

## Runtime Features

### Class System
- [x] Implement basic class definition structure
- [x] Support for inheritance
- [x] Instance and static methods
- [x] Private methods and properties
- [ ] Implement proper constructor chaining
- [ ] Add support for mixins/traits

### Promise Implementation
- [x] Basic Promise structure
- [x] Support for then/catch/finally
- [x] Static methods (all, race, etc.)
- [ ] Integration with async/await syntax
- [ ] Proper error propagation

### Collections
- [x] Implement Map and Set
- [x] Implement WeakMap and WeakSet
- [ ] Add additional collection methods
- [ ] Optimize collection performance

### JSON Handling
- [x] Basic JSON parsing and stringification
- [ ] Support for replacer and space parameters
- [ ] Handle circular references
- [ ] Optimize performance for large objects

### Module System
- [x] Basic module loading
- [ ] Support for dynamic imports
- [ ] Proper resolution of circular dependencies
- [ ] Integration with package system

### Browser API
- [ ] Implement DOM manipulation API
- [ ] Add event handling
- [ ] Support for fetch API
- [ ] Implement localStorage/sessionStorage

## BACKLOG

### Core Compiler
- [ ] Implement proper exception handling with landing pads
- [ ] Add support for all binary operations
- [ ] Implement pattern matching code generation
- [ ] Add proper type checking and semantic analysis
- [ ] Implement optimization passes
- [ ] Support cross-compilation for all target platforms
- [ ] Add debug information generation
- [ ] Improve test coverage for all compiler components

### Language Features
- [ ] Complete pattern matching implementation
- [ ] Implement proper async/await functionality
- [ ] Implement classes and inheritance
- [ ] Implement destructuring assignment
- [ ] Add support for generators and iterators
- [ ] Implement proper closures with lexical scoping
- [ ] Add support for decorators/annotations
- [ ] Add comprehensive tests for all language features

### Standard Library
- [ ] Complete the networking module (TCP/IP, WebSockets)
- [ ] Add date and time functionality
- [ ] Add regular expression support
- [ ] Implement process management functions
- [ ] Add cryptography functions
- [ ] Implement collections (maps, sets, etc.)
- [ ] Add math library functions
- [ ] Create tests for all standard library modules

### Hardware Interfaces
- [ ] Implement camera access API
- [ ] Add microphone and audio recording support
- [ ] Implement screen recording functionality
- [ ] Add support for input devices (keyboard, mouse)
- [ ] Implement Bluetooth device interaction
- [ ] Add USB device support
- [ ] Implement MIDI device support
- [ ] Add gamepad/controller support
- [ ] Implement system notifications
- [ ] Create tests for hardware interface modules

### Package Manager
- [ ] Implement package publishing
- [ ] Add version management
- [ ] Implement package verification and security
- [ ] Add support for private repositories
- [ ] Implement package updates and upgrades
- [ ] Add package documentation generation
- [ ] Create comprehensive tests for package manager

### WebAssembly Support
- [ ] Implement SmashLang to WebAssembly compilation
- [ ] Add direct DOM manipulation API
- [ ] Implement WebGL/Canvas rendering support
- [ ] Add Web Workers integration
- [ ] Improve debugging tools for WASM
- [ ] Optimize WASM binary size
- [ ] Add JavaScript interop layer
- [ ] Implement browser API bindings
- [ ] Add support for WASM modules
- [ ] Create tests for WebAssembly functionality

### Testing Framework
- [ ] Implement proper test result reporting
- [ ] Implement test fixtures and mocking
- [ ] Add support for parameterized tests
- [ ] Implement code coverage reporting
- [ ] Add performance benchmarking
- [ ] Implement snapshot testing
- [ ] Add support for test suites and organization
- [ ] Implement test filtering and tagging
- [ ] Create tests for the testing framework itself

### Development Tools
- [ ] Implement Language Server Protocol (LSP) support
- [ ] Add debugger support
- [ ] Implement code formatting tools
- [ ] Add linting and static analysis
- [ ] Implement documentation generation
- [ ] Add build system integration
- [ ] Implement project scaffolding
- [ ] Add IDE integration plugins
- [ ] Implement profiling tools
- [ ] Create tests for development tools

### Documentation
- [ ] Complete API reference documentation
- [ ] Add more code examples
- [ ] Create tutorials for common use cases
- [ ] Implement interactive documentation
- [ ] Add language specification
- [ ] Create contribution guidelines
- [ ] Implement documentation search
- [ ] Add internationalization support
- [ ] Create video tutorials
- [ ] Add testing documentation

### Infrastructure
- [ ] Set up continuous integration
- [ ] Implement automated testing
- [ ] Add release management
- [ ] Implement package registry
- [ ] Set up documentation hosting
- [ ] Add community forums
- [ ] Implement bug tracking
- [ ] Create roadmap and milestone tracking
- [ ] Set up security vulnerability reporting
- [ ] Implement test automation for all components

## Testing Guidelines

All code in SmashLang must have corresponding unit tests:

1. **Rust Components**: Use Rust's built-in testing framework for compiler components:
   - Lexer tests in `src/frontend/lexer/tests.rs`
   - Parser tests in `src/frontend/parser/tests.rs`
   - Codegen tests in `src/backend/codegen/tests.rs`
   - Runtime tests in `src/runtime/tests/`

2. **SmashLang Packages**: Use SmashTest for testing SmashLang packages:
   - Create test files in the `tests/` directory of each package
   - Name test files with `.test.smash` extension
   - Use the `assert` module for assertions

3. **Test Coverage Requirements**:
   - All new features must have corresponding tests
   - All bug fixes must include a test that would have caught the bug
   - Aim for at least 80% code coverage for all components

4. **Running Tests**:
   - Rust tests: `cargo test`
   - SmashLang tests: `smashtest path/to/test/file.test.smash`