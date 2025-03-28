# SmashLang Packages

<p align="center">
  <img src="../assets/logo.svg" alt="SmashLang logo" width="200" />
</p>

## Overview

This directory contains all official and community packages for [SmashLang](https://github.com/smashlang/smashlang), a high-performance JavaScript-inspired programming language that compiles to native binaries.

All packages are organized in directories with a consistent structure that includes the package definition, examples, and documentation.

## Directory Structure

```
smashlang_packages/
├── __package__template/  # Template for creating new packages
├── core/               # Essential libraries maintained by the SmashLang team
│   ├── math/            # Mathematics library
│   ├── json/            # JSON parsing and serialization
│   └── crypto/          # Cryptography library
├── networking/         # Libraries for HTTP, WebSockets, and other network protocols
│   ├── http/            # HTTP client library
│   └── websocket/       # WebSocket client and server
├── database/           # Database drivers and ORM tools
│   ├── sqlite/          # SQLite database bindings
│   ├── postgres/        # PostgreSQL database bindings
│   ├── redis/           # Redis client
│   └── pocketbase/      # PocketBase client
├── community/          # Third-party packages contributed by the community
│   ├── game_engine/     # 2D game engine
│   └── ml/              # Machine learning library
└── README.md           # This file
```

Each package directory contains:

```
package_name/
├── package.smash    # Package definition and metadata
├── README.md        # Documentation
└── examples/        # Example code
    ├── basic.smash
    └── advanced.smash
```

## Using Packages

To install a package, use the `smashpkg` command:

```bash
# Install a package
smashpkg install math

# Install a specific version
smashpkg install sqlite@3.36.0

# Install multiple packages
smashpkg install math crypto json
```

Once installed, you can import packages in your SmashLang code:

```js
// Import the entire package
import "math";

// Use the package
let result = math.sin(0.5) + math.cos(0.5);
print(result);
```

## Available Packages

### Core

- **math**: Advanced mathematics library with trigonometric, statistical, and matrix operations
- **json**: Fast JSON parsing and serialization
- **crypto**: Cryptography library with modern algorithms

### Networking

- **http**: Advanced HTTP client library with support for modern protocols
- **websocket**: WebSocket client and server implementation

### Database

- **sqlite**: SQLite database bindings
- **postgres**: PostgreSQL database bindings

### Community

- **game_engine**: Fast 2D game engine with physics and rendering capabilities
- **ml**: Machine learning library with neural networks and more

## Package Formula Structure

Each package is defined by a formula file written in SmashLang. Here's a basic example:

```js
// math.smash - Math library formula
{
  "name": "math",
  "version": "1.0.0",
  "description": "Advanced mathematics library for SmashLang",
  "license": "MIT",
  "authors": ["SmashLang Team"],
  "dependencies": [],
  "url": "https://smashlang.com/packages/math-1.0.0.tar.gz",
  "sha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
  "install": [
    "cp -r src/* #{prefix}/"
  ],
  "test": [
    "smash test/run.smash"
  ],
  "exports": {
    "sin": "fn sin(x) { /* Implementation */ }",
    "cos": "fn cos(x) { /* Implementation */ }",
    "tan": "fn tan(x) { /* Implementation */ }"
  }
}
```

## Contributing Packages

To contribute a new package:

1. Create a new formula file in the appropriate directory
2. Test your formula locally
3. Submit a pull request

## License

ISC © 2025 SmashLang.com

Individual packages may have their own licenses as specified in their formula files.
