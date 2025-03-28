# SmashLang Packages

<p align="center">
  <img src="https://raw.githubusercontent.com/smashlang/smashlang/main/assets/logo.svg" alt="SmashLang logo" width="200" />
</p>

## Overview

This repository hosts the official package registry for [SmashLang](https://github.com/smashlang/smashlang), a high-performance JavaScript-inspired programming language that compiles to native binaries.

Similar to Homebrew's model, all SmashLang packages are defined in this central GitHub repository, making it easy to contribute, review, and maintain packages.

## Using Packages

To install a package, use the `smashpkg` command:

```bash
# Install a package
smashpkg install math

# Install a specific version
smashpkg install sqlite@3.35.0

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

The repository is organized into several categories:

- **Core**: Essential libraries maintained by the SmashLang team
- **Networking**: Libraries for HTTP, WebSockets, and other network protocols
- **Database**: Database drivers and ORM tools
- **Community**: Third-party packages contributed by the community

Browse the [Formula](./Formula) directory to see all available packages.

## Contributing Packages

We welcome contributions from the community! To add a new package:

1. Fork this repository
2. Create a new formula file in the appropriate directory
3. Test the formula locally using `smashpkg test <formula>`
4. Submit a pull request

Please read our [CONTRIBUTING.md](./CONTRIBUTING.md) guide for detailed instructions and package formula requirements.

## Package Formula Structure

Each package is defined by a formula file written in SmashLang. Here's a basic example:

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

## API

This repository also serves as a package registry API that can be consumed by tools and IDEs:

- Package list: [api/v1/packages.json](./api/v1/packages.json)
- Search: [api/v1/search.json?q=database](./api/v1/search.json?q=database)
- Package info: [api/v1/info/sqlite.json](./api/v1/info/sqlite.json)

## License

ISC © 2025 SmashLang.com

Individual packages may have their own licenses as specified in their formula files.
