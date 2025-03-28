# Contributing to SmashLang Packages

Thank you for your interest in contributing to the SmashLang package ecosystem! This guide will help you create and submit new package formulas to the repository.

## Getting Started

### Prerequisites

- [SmashLang](https://github.com/smashlang/smashlang) installed on your system
- Git and a GitHub account
- Basic understanding of SmashLang syntax
- Knowledge of the package you want to add

### Development Workflow

1. Fork the [smashlang_packages](https://github.com/smashlang/smashlang_packages) repository
2. Clone your fork to your local machine
3. Create a new branch for your package
4. Create and test your package formula
5. Submit a pull request

## Creating a Package Formula

### Formula Structure

Package formulas are SmashLang files that define how to install a package. Here's the basic structure:

```js
{
  "name": "package_name",              // Required: Package name (lowercase, underscores)
  "version": "1.0.0",                // Required: Package version (semver)
  "description": "Package description", // Required: Brief description
  "homepage": "https://example.com",   // Required: Project homepage
  "license": "MIT",                   // Required: License identifier
  "authors": ["Author Name"],         // Required: List of authors
  "maintainers": ["@github_username"], // Optional: GitHub usernames of maintainers
  "dependencies": [],                 // Optional: Other SmashLang packages required
  "url": "https://example.com/file.tar.gz", // Required: Source URL
  "sha256": "hash",                   // Required: SHA256 checksum of the source
  
  // Optional: System dependencies required
  "native_dependencies": [
    {
      "name": "dependency_name",
      "version": ">=1.0.0",
      "debian": "package-name",
      "fedora": "package-name",
      "arch": "package-name",
      "macos": "package-name",
      "windows": "package-name"
    }
  ],
  
  // Optional: Build steps (for packages that need compilation)
  "build": [
    "./configure --prefix=#{prefix}",
    "make",
    "make install"
  ],
  
  // Required: Installation steps
  "install": [
    "cp -r src/* #{prefix}/"
  ],
  
  // Optional but recommended: Test commands
  "test": [
    "smash test/run.smash"
  ],
  
  // Optional: Example files
  "examples": [
    "examples/example1.smash",
    "examples/example2.smash"
  ]
}
```

### Formula Placement

Place your formula in the appropriate directory based on its category:

- `Formula/core/` - Core libraries maintained by the SmashLang team
- `Formula/networking/` - Networking-related packages
- `Formula/database/` - Database-related packages
- `Formula/community/` - Community-contributed packages

### Formula Naming

Formula files should be named `<package_name>.smash` and use lowercase with underscores for multi-word names.

## Testing Your Formula

Before submitting, test your formula locally:

```bash
# Test formula installation
smashpkg test ./Formula/category/package_name.smash

# Install from the formula
smashpkg install --formula ./Formula/category/package_name.smash

# Test the installed package
smash -e 'import "package_name"; // Test code here'
```

## Submission Guidelines

### Pull Request Process

1. Ensure your formula passes all tests
2. Create a pull request against the main repository
3. Fill out the pull request template with all required information
4. Wait for review from maintainers

### Review Criteria

Package formulas are reviewed based on:

- Adherence to the formula structure
- Code quality and correctness
- Security considerations
- Documentation quality
- Test coverage

### After Submission

After your pull request is submitted:

1. Automated tests will run on your formula
2. Maintainers will review your submission
3. You may be asked to make changes
4. Once approved, your formula will be merged

## Package Maintenance

As a package contributor, you're encouraged to maintain your packages by:

- Updating versions when new releases are available
- Fixing bugs and addressing issues
- Improving documentation and examples
- Responding to user feedback

## Code of Conduct

Please note that this project is released with a [Contributor Code of Conduct](./CODE_OF_CONDUCT.md). By participating in this project, you agree to abide by its terms.

## Need Help?

If you need help with creating or submitting a package formula, you can:

- Open an issue in the repository
- Ask in the [SmashLang Discord](https://discord.gg/smashlang)
- Email the maintainers at packages@smashlang.com

Thank you for contributing to the SmashLang ecosystem!
