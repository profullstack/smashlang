# SmashLang Packages Repository Structure

## Overview

The `smashlang_packages` repository follows a structure similar to Homebrew's model, where all packages are hosted in a central GitHub repository. This document outlines the structure and workflow for managing SmashLang packages.

## Repository Structure

```
smashlang_packages/
├── README.md                 # Repository documentation
├── CONTRIBUTING.md          # Guidelines for contributing packages
├── Formula/                 # Directory containing all package formulas
│   ├── core/                # Core/official packages
│   │   ├── math.smash       # Math library formula
│   │   ├── crypto.smash     # Cryptography library formula
│   │   └── ...              # Other core packages
│   ├── networking/          # Networking-related packages
│   │   ├── http.smash       # HTTP client library formula
│   │   ├── websocket.smash  # WebSocket library formula
│   │   └── ...              # Other networking packages
│   ├── database/            # Database-related packages
│   │   ├── sqlite.smash     # SQLite binding formula
│   │   ├── postgres.smash   # PostgreSQL binding formula
│   │   └── ...              # Other database packages
│   └── community/           # Community-contributed packages
│       ├── game_engine.smash # Game engine library formula
│       ├── ml.smash         # Machine learning library formula
│       └── ...              # Other community packages
├── scripts/                 # Utility scripts for repository management
│   ├── lint.sh              # Script to lint package formulas
│   ├── test.sh              # Script to test package formulas
│   └── ...                  # Other utility scripts
└── api/                     # API endpoints for package registry
    ├── v1/                  # API version 1
    │   ├── packages.json    # List of all packages
    │   └── search.json      # Search endpoint
    └── ...                  # Other API versions
```

## Package Formula Structure

Each package formula is a SmashLang file that defines how to install the package. Here's an example formula structure:

```js
// math.smash - Math library formula
{
  "name": "math",
  "version": "1.0.0",
  "description": "Advanced mathematics library for SmashLang",
  "homepage": "https://github.com/smashlang/math",
  "license": "MIT",
  "authors": ["SmashLang Team"],
  "dependencies": [],
  "url": "https://github.com/smashlang/math/archive/v1.0.0.tar.gz",
  "sha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
  "install": [
    "cp -r src/* #{prefix}/"
  ],
  "test": [
    "smash test/run.smash"
  ]
}
```

## Workflow for Adding a New Package

1. Fork the `smashlang_packages` repository
2. Create a new formula file in the appropriate directory
3. Test the formula locally using `smashpkg test <formula>`
4. Submit a pull request to the main repository
5. Wait for review and approval
6. Once merged, the package becomes available to all SmashLang users

## Package Installation Process

When a user runs `smashpkg install <package>`, the following happens:

1. The package manager fetches the latest package list from the GitHub repository
2. It locates the formula for the requested package
3. It downloads the package source from the URL specified in the formula
4. It verifies the checksum to ensure integrity
5. It installs the package according to the instructions in the formula
6. It registers the package as installed in the local system

## API Endpoints

The repository provides JSON API endpoints that can be consumed by the package manager and other tools:

- `/api/v1/packages.json`: List of all available packages
- `/api/v1/search.json?q=<query>`: Search for packages by name or description
- `/api/v1/info/<package>.json`: Detailed information about a specific package
