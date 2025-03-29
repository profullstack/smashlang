# Package Name

<p align="center">
  <img src="./assets/logo.svg" alt="Package Logo" width="200" />
</p>

A brief description of what your package does and why it's useful.

## Installation

```bash
smashpkg install package_name
```

## Features

- Feature 1: Brief description
- Feature 2: Brief description
- Feature 3: Brief description

## Basic Usage

```js
import "package_name";

// Basic example code
const result = package_name.functionName(param1, param2);
console.log(result);

// Class example
const instance = new package_name.ClassName();
instance.method();
```

## Advanced Usage

```js
// More complex example demonstrating advanced features
async fn advancedExample() {
  // Code example here
}
```

## API Reference

### Functions

#### `functionName(param1, param2)`

Description of what the function does.

**Parameters:**
- `param1` (Type): Description of parameter
- `param2` (Type): Description of parameter

**Returns:**
- (ReturnType): Description of return value

**Example:**
```js
const result = package_name.functionName("value1", 42);
```

### Classes

#### `ClassName`

Description of what the class does.

**Constructor:**
```js
new package_name.ClassName(options)
```

**Parameters:**
- `options` (Object): Configuration options
  - `option1` (Type): Description of option
  - `option2` (Type): Description of option

**Methods:**

##### `instance.method(param)`

Description of what the method does.

**Parameters:**
- `param` (Type): Description of parameter

**Returns:**
- (ReturnType): Description of return value

## Examples

See the [examples directory](./examples) for more detailed examples:

- [Basic Example](./examples/basic.smash): Description of what this example demonstrates
- [Advanced Example](./examples/advanced.smash): Description of what this example demonstrates
- [Testing Example](./examples/testing.smash): Demonstrates how to use the testing framework

## Testing

This package includes comprehensive tests using SmashLang's built-in testing framework.

### Running Tests

```bash
# Run all tests for this package
smashtest ./tests

# Run a specific test file
smashtest ./tests/utils/string.test.smash

# Run tests with a specific tag
smashtest ./tests --tag=unit
```

### Test Structure

Tests are organized in the following structure:

```
tests/
├── index.test.smash       # Main test file for the package
└── utils/                 # Tests for utility modules
    ├── string.test.smash  # Tests for string utilities
    ├── math.test.smash    # Tests for math utilities
    └── validator.test.smash # Tests for validation utilities
```

### Writing Tests

Tests use the SmashLang testing framework with a syntax inspired by Jest and Mocha:

```javascript
import { test, describe, expect, beforeEach, afterEach } from 'std/testing';

// Simple test
test('addition works correctly', () => {
  expect(2 + 2).toBe(4);
});

// Grouped tests with setup and teardown
describe('Feature group', () => {
  let testData;
  
  beforeEach(() => {
    testData = setupTestData();
  });
  
  test('feature works as expected', () => {
    expect(testData.process()).toBeTrue();
  });
});
```

## Contributing

Instructions for contributing to this package.

## License

MIT (or your chosen license)
