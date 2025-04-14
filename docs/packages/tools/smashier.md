# Smashier Package

<p align="center">
  <img src="../../../smashlang_packages/tools/smashier/assets/logo.light.svg" alt="Smashier Package Logo" width="200" />
</p>

The Smashier package provides a powerful code formatter and linter for SmashLang. It helps maintain consistent code style, identifies potential issues, and automatically formats your code according to configurable rules.

## Installation

```bash
smashpkg install smashier
```

## Features

- Automatic code formatting
- Customizable formatting rules
- Code style enforcement
- Error detection and linting
- Integration with editors and IDEs
- Configuration via `.smashierrc` files
- Ignore specific files or patterns
- Fix code style issues automatically
- Format on save support
- Plugin system for custom rules
- Supports all SmashLang syntax features
- Preserves semantics while reformatting

## Basic Usage

### Command Line

```bash
# Format a file
smashier file.smash

# Format a file and save changes
smashier --write file.smash

# Format all .smash files in a directory
smashier --write ./src

# Check if files are formatted correctly (without modifying)
smashier --check ./src

# Format using a specific configuration file
smashier --config ./.smashierrc.json file.smash
```

### Programmatic Usage

```js
import { smashier } from "smashier";

// Format a string of code
const sourceCode = `
fn add(a,b){
return a+b;
}
`;

const formattedCode = smashier.format(sourceCode);
console.log(formattedCode);
// Output:
// fn add(a, b) {
//   return a + b;
// }

// Format with specific options
const customFormatted = smashier.format(sourceCode, {
  tabWidth: 4,
  useTabs: true,
  printWidth: 100,
  singleQuote: true
});

// Check if code is formatted correctly
const isFormatted = smashier.check(sourceCode);
console.log(isFormatted); // false

// Get formatting differences
const diff = smashier.diff(sourceCode);
console.log(diff);
```

## Configuration

Smashier can be configured using a `.smashierrc` file in JSON, YAML, or JavaScript format. You can also add a `smashier` field to your `package.json`.

### Example Configuration

```json
{
  "printWidth": 80,
  "tabWidth": 2,
  "useTabs": false,
  "semi": true,
  "singleQuote": false,
  "quoteProps": "as-needed",
  "trailingComma": "es5",
  "bracketSpacing": true,
  "arrowParens": "always",
  "endOfLine": "lf",
  "insertPragma": false,
  "requirePragma": false,
  "proseWrap": "preserve",
  "htmlWhitespaceSensitivity": "css",
  "embeddedLanguageFormatting": "auto"
}
```

### Configuration Options

#### `printWidth`
Line length where Smashier will try to wrap code.
- **Type**: Number
- **Default**: 80

#### `tabWidth`
Number of spaces per indentation level.
- **Type**: Number
- **Default**: 2

#### `useTabs`
Indent with tabs instead of spaces.
- **Type**: Boolean
- **Default**: false

#### `semi`
Add semicolons at the end of statements.
- **Type**: Boolean
- **Default**: true

#### `singleQuote`
Use single quotes instead of double quotes.
- **Type**: Boolean
- **Default**: false

#### `quoteProps`
Change when properties in objects are quoted.
- **Type**: String
- **Options**: "as-needed", "consistent", "preserve"
- **Default**: "as-needed"

#### `trailingComma`
Print trailing commas wherever possible.
- **Type**: String
- **Options**: "none", "es5", "all"
- **Default**: "es5"

#### `bracketSpacing`
Print spaces between brackets in object literals.
- **Type**: Boolean
- **Default**: true

#### `arrowParens`
Include parentheses around a sole arrow function parameter.
- **Type**: String
- **Options**: "always", "avoid"
- **Default**: "always"

## Advanced Usage

### Ignoring Files

Create a `.smashierignore` file to specify files that should be ignored:

```
# Ignore build files
build/
dist/

# Ignore specific files
src/generated.smash
src/vendor/**/*.smash

# Ignore files matching pattern
**/*.min.smash
```

### Editor Integration

#### VS Code

Install the Smashier extension for VS Code:

1. Open VS Code
2. Go to Extensions (Ctrl+Shift+X)
3. Search for "Smashier"
4. Install the extension
5. Configure it to format on save (optional):

```json
// settings.json
{
  "editor.defaultFormatter": "smashlang.smashier",
  "editor.formatOnSave": true,
  "[smash]": {
    "editor.formatOnSave": true
  }
}
```

#### Other Editors

Smashier provides integration with other popular editors:

- **Vim/Neovim**: Use the Smashier Vim plugin
- **Sublime Text**: Install the Smashier package
- **Atom**: Install the Smashier Atom package
- **JetBrains IDEs**: Use the Smashier plugin

### Custom Plugins

You can extend Smashier with custom plugins:

```js
// my-smashier-plugin.js
module.exports = {
  name: "smashier-plugin-custom-rules",
  rules: {
    "max-function-length": {
      create: (context) => {
        return {
          FunctionDeclaration: (node) => {
            const maxLines = context.options.maxLines || 50;
            const lines = node.loc.end.line - node.loc.start.line;
            
            if (lines > maxLines) {
              context.report({
                node,
                message: `Function is too long (${lines} lines). Maximum allowed is ${maxLines} lines.`
              });
            }
          }
        };
      }
    }
  }
};
```

Configure your plugin in `.smashierrc.js`:

```js
// .smashierrc.js
const myPlugin = require('./my-smashier-plugin');

module.exports = {
  plugins: [myPlugin],
  rules: {
    "max-function-length": {
      maxLines: 30
    }
  }
};
```

## API Reference

### Command Line Interface

#### `smashier [options] [file/dir...]`
Formats the specified files.

**Options**:
- `--write`, `-w`: Edit files in-place
- `--check`, `-c`: Check if files are formatted
- `--config`: Path to config file
- `--ignore-path`: Path to ignore file
- `--plugin`: Load a plugin
- `--list-different`, `-l`: Print filenames of unformatted files
- `--no-config`: Don't look for a config file
- `--no-ignore`: Don't respect ignore files
- `--stdin`: Read input from stdin
- `--stdin-filepath`: Path to the file to pretend stdin comes from
- `--version`, `-v`: Show version
- `--help`, `-h`: Show help

### Programmatic API

#### `smashier.format(source, options)`
Formats a string of code.
- **Parameters**: 
  - `source` (String): Source code to format
  - `options` (Object, optional): Formatting options
- **Returns**: (String) Formatted code

#### `smashier.check(source, options)`
Checks if code is formatted correctly.
- **Parameters**: 
  - `source` (String): Source code to check
  - `options` (Object, optional): Formatting options
- **Returns**: (Boolean) True if the code is formatted correctly

#### `smashier.diff(source, options)`
Gets formatting differences.
- **Parameters**: 
  - `source` (String): Source code to diff
  - `options` (Object, optional): Formatting options
- **Returns**: (String) Unified diff of formatting changes

#### `smashier.resolveConfig(filePath)`
Resolves configuration for a file.
- **Parameters**: 
  - `filePath` (String): Path to the file
- **Returns**: (Promise<Object>) Promise resolving to config object

#### `smashier.clearConfigCache()`
Clears the configuration cache.
- **Returns**: (void)

#### `smashier.getFileInfo(filePath)`
Gets information about a file.
- **Parameters**: 
  - `filePath` (String): Path to the file
- **Returns**: (Promise<Object>) Promise resolving to file info

## Examples

See the [examples directory](../../../smashlang_packages/tools/smashier/examples) for more detailed examples:

- [Basic Example](../../../smashlang_packages/tools/smashier/examples/basic.smash): Demonstrates basic formatting
- [Custom Config Example](../../../smashlang_packages/tools/smashier/examples/custom-config.smash): Shows custom configuration
- [Plugin Example](../../../smashlang_packages/tools/smashier/examples/plugin.smash): Demonstrates plugin usage

## Testing

The Smashier package includes comprehensive tests:

```bash
# Run all tests for the smashier package
smashtest smashlang_packages/tools/smashier/tests
```

## Contributing

Contributions to the Smashier package are welcome! Please follow these steps:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for your changes
5. Submit a pull request

## License

MIT