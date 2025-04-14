# SmashLang Packages Documentation

This directory contains comprehensive documentation for all SmashLang packages. The packages are organized by category, with each package having its own documentation file.

## Package Categories

- [Core](./core/): Fundamental packages for basic functionality
- [Networking](./networking/): Packages for network communication and protocols
- [Database](./database/): Packages for database connectivity and operations
- [Hardware](./hardware/): Packages for hardware access and control
- [Tools](./tools/): Utility packages and development tools

## Documentation Structure

Each package documentation follows a consistent structure:

1. **Overview**: Brief description of the package and its purpose
2. **Installation**: Instructions for installing the package
3. **Features**: List of key features and capabilities
4. **Basic Usage**: Simple examples to get started
5. **Advanced Usage**: More complex examples demonstrating advanced features
6. **API Reference**: Detailed documentation of all functions, classes, and methods
7. **Examples**: Links to example code
8. **Testing**: Information about package tests
9. **Contributing**: Guidelines for contributing to the package
10. **License**: License information

## Example Packages

We've created detailed documentation for one package in each category as examples:

- **Core**: [Math](./core/math.md) - Mathematical functions and utilities
- **Networking**: [HTTP](./networking/http.md) - HTTP client for web requests
- **Database**: [PostgreSQL](./database/postgres.md) - PostgreSQL database client
- **Hardware**: [Camera](./hardware/camera.md) - Camera access and control
- **Tools**: [Smashier](./tools/smashier.md) - Code formatter and linter

## Package Implementation Status

For information about the implementation status of all packages and what needs to be done, see the [TODO-PKGS.md](../../TODO-PKGS.md) file in the project root.

## Creating Package Documentation

When creating documentation for a new package, use the existing examples as templates. Each documentation file should:

1. Include a package logo at the top
2. Provide clear installation instructions
3. List all features
4. Include basic and advanced usage examples with code snippets
5. Document the complete API
6. Link to example code in the package directory
7. Provide information about testing and contributing

## Package Development

For information about developing new packages, see the [Package Template Guide](../../smashlang_packages/__package__template/GUIDE.md).

## Package Directory Structure

Each package follows a standard directory structure:

```
package_name/
├── package.smash      # Package definition and metadata
├── package_config.json # Theme and presentation configuration
├── README.md          # Package documentation
├── assets/            # Package assets (logos, icons)
├── src/               # Source code
├── examples/          # Example code
└── tests/             # Test files
```

## Using Packages in Your Code

To use a package in your SmashLang code, first install it:

```bash
smashpkg install package_name
```

Then import it in your code:

```javascript
// Import the entire package
import "package_name";

// Import specific exports
import { function1, Class1 } from "package_name";

// Import with alias
import * as pkg from "package_name";
```

## Contributing to Package Documentation

If you find errors or want to improve the package documentation:

1. Fork the repository
2. Make your changes
3. Submit a pull request

Please follow the existing documentation structure and style.