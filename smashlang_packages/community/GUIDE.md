# SmashLang Package Template Guide

This template provides a starting point for creating new SmashLang packages. Follow these steps to create your own package.

## Getting Started

1. Copy this `community` directory to create your new package:

   ```bash
   cp -r smashlang_packages/community smashlang_packages/[category]/[your_package_name]
   ```

   Replace `[category]` with one of: `core`, `networking`, `database`, or `community`.
   Replace `[your_package_name]` with your package's name (use lowercase with underscores).

2. Edit the `package.smash` file to include your package's information:
   - Update the name, version, description, and other metadata
   - Add your dependencies
   - Define your package's exports (functions, classes, objects)
   - Specify build and installation steps if needed

3. Update the `README.md` with documentation for your package:
   - Explain what your package does and why it's useful
   - Provide basic and advanced usage examples
   - Document your API (functions, classes, methods)

4. Create examples in the `examples/` directory:
   - The template includes `basic.smash` and `advanced.smash` starters
   - Update these with real examples of your package's functionality
   - Add more examples as needed

## Package Structure

```
[your_package_name]/
├── package.smash      # Package definition and metadata
├── README.md          # Documentation
├── GUIDE.md           # This guide (can be removed)
├── index.html         # Generated HTML documentation (created during build)
├── preinstall.smash   # Optional pre-installation script
├── postinstall.smash  # Optional post-installation script
├── assets/            # Package assets
│   ├── logo.svg         # Package logo (REQUIRED)
│   └── favicon.svg      # Package favicon (REQUIRED)
└── examples/          # Example code
    ├── basic.smash
    └── advanced.smash
```

> **IMPORTANT**: Each package MUST include both a logo file and a favicon file in SVG format in the assets directory. PNG format is allowed but not recommended. The logo will be displayed at the top of your package's README.md file, and the favicon can be used for web applications built with your package.
>
> During the build process, the README.md file will be automatically converted to index.html using pandoc (if installed). This HTML file will use the favicon.svg and have a clean, responsive design using Water.css. If pandoc is not installed, a warning will be displayed and the build will continue without generating the HTML file.
>
> Packages can include optional `preinstall.smash` and `postinstall.smash` scripts that will be executed before and after installation, respectively. These scripts can be used to perform setup tasks, install dependencies, or configure the environment.

## Package File Format

The `package.smash` file defines your package using a JSON-like format with comments. Here's what each field means:

- `name`: Your package's name (lowercase, underscores for multi-word names)
- `version`: Package version using semantic versioning (MAJOR.MINOR.PATCH)
- `description`: Brief description of what your package does
- `license`: License identifier (e.g., MIT, Apache-2.0, ISC)
- `authors`: List of package authors
- `maintainers`: GitHub usernames of package maintainers (optional)
- `dependencies`: Other SmashLang packages your package depends on
- `url`: URL to download the package source
- `sha256`: SHA-256 checksum of the package source
- `native_dependencies`: System dependencies required (optional)
- `build`: Commands to build the package (optional)
- `install`: Commands to install the package
- `test`: Commands to test the package (optional)
- `exports`: Functions, classes, and objects exported by your package
- `examples`: List of example files (optional)

## Exports

The `exports` field defines what your package makes available to users. This can include:

- Functions: `"functionName": "fn functionName(param1, param2) { /* code */ }"`
- Classes: `"ClassName": "class ClassName { /* code */ }"`
- Nested objects: 
  ```
  "nestedObject": {
    "nestedFunction": "fn nestedFunction() { /* code */ }"
  }
  ```

## Testing Your Package

Before submitting your package, test it locally:

```bash
# Test installation
smashpkg install --local smashlang_packages/[category]/[your_package_name]

# Test the examples
smash smashlang_packages/[category]/[your_package_name]/examples/basic.smash
```

## Submitting Your Package

Once your package is ready:

1. Ensure all tests pass
2. Remove this GUIDE.md file
3. Submit a pull request to the SmashLang repository

## Best Practices

- Use clear, descriptive names for functions and classes
- Document all public APIs in the README.md
- Include examples that demonstrate real-world usage
- Follow SmashLang coding conventions
- Keep dependencies minimal
- Use semantic versioning for your package version

## Need Help?

If you need assistance creating your package, check the [SmashLang documentation](https://smashlang.com/docs/packages) or join the [SmashLang community](https://discord.gg/smashlang).
