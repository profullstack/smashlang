# SmashLang Testing Framework

This document provides comprehensive documentation for the SmashLang testing framework. The framework is designed to be familiar to developers who have used Jest, Mocha, or similar JavaScript testing libraries.

## Table of Contents

- [Getting Started](#getting-started)
- [Test Runner](#test-runner)
- [Writing Tests](#writing-tests)
  - [Basic Tests](#basic-tests)
  - [Test Groups](#test-groups)
  - [Setup and Teardown](#setup-and-teardown)
  - [Async Tests](#async-tests)
  - [Test Tags](#test-tags)
- [Assertions](#assertions)
  - [Basic Assertions](#basic-assertions)
  - [Boolean Assertions](#boolean-assertions)
  - [Type Assertions](#type-assertions)
  - [Error Assertions](#error-assertions)
  - [Null and Undefined Assertions](#null-and-undefined-assertions)
- [Running Tests](#running-tests)
  - [Command Line Options](#command-line-options)
  - [Filtering Tests](#filtering-tests)
  - [Output Formats](#output-formats)
- [Integration with Packages](#integration-with-packages)
- [Best Practices](#best-practices)
- [Examples](#examples)

## Getting Started

To use the SmashLang testing framework, you need to import the necessary functions from the standard library:

```javascript
import { test, describe, expect, beforeEach, afterEach } from 'std/testing';
```

Create a test file with a `.test.smash` extension or a filename ending with `_test.smash`.

## Test Runner

The `smashtest` command is used to run tests:

```bash
# Run all tests in a directory
smashtest ./tests

# Run a specific test file
smashtest ./tests/unit.test.smash

# Run tests with a specific tag
smashtest ./tests --tag=unit
```

## Writing Tests

### Basic Tests

The simplest way to write a test is using the `test` function:

```javascript
test('addition works correctly', () => {
  expect(2 + 2).toBe(4);
});
```

The first argument is a description of the test, and the second is a function containing the test code.

### Test Groups

You can group related tests using the `describe` function:

```javascript
describe('Math operations', () => {
  test('addition works correctly', () => {
    expect(2 + 2).toBe(4);
  });
  
  test('subtraction works correctly', () => {
    expect(5 - 3).toBe(2);
  });
});
```

Test groups can be nested for better organization:

```javascript
describe('Math operations', () => {
  describe('Basic operations', () => {
    test('addition works correctly', () => {
      expect(2 + 2).toBe(4);
    });
  });
});
```

### Setup and Teardown

You can use `beforeEach` and `afterEach` to run code before and after each test in a group:

```javascript
describe('Counter', () => {
  let counter;
  
  beforeEach(() => {
    counter = createCounter(0);
  });
  
  afterEach(() => {
    counter = null;
  });
  
  test('increments correctly', () => {
    counter.increment();
    expect(counter.getValue()).toBe(1);
  });
  
  test('decrements correctly', () => {
    counter.decrement();
    expect(counter.getValue()).toBe(-1);
  });
});
```

### Async Tests

You can test asynchronous code using async/await:

```javascript
test('async operations work', async () => {
  const result = await fetchData();
  expect(result.success).toBeTrue();
});
```

### Test Tags

You can add tags to tests to categorize them and run specific subsets:

```javascript
test('addition works correctly', ['math', 'unit'], () => {
  expect(2 + 2).toBe(4);
});
```

Then run only tests with a specific tag:

```bash
smashtest ./tests --tag=unit
```

## Assertions

The `expect` function is used to make assertions about values.

### Basic Assertions

```javascript
expect(value).toBe(expected);      // Strict equality (===)
expect(value).not.toBe(expected);  // Strict inequality (!==)
```

### Boolean Assertions

```javascript
expect(value).toBeTrue();   // Assert that value is true
expect(value).toBeFalse();  // Assert that value is false
```

### Type Assertions

```javascript
expect(typeof value).toBe('string');  // Check the type of a value
expect(Array.isArray(value)).toBeTrue();  // Check if value is an array
```

### Error Assertions

```javascript
expect(() => {
  throw new Error('Test error');
}).toThrow();
```

### Null and Undefined Assertions

```javascript
expect(value).toBeNull();      // Assert that value is null
expect(value).toBeUndefined(); // Assert that value is undefined
expect(value).toBeDefined();   // Assert that value is not undefined
```

## Running Tests

### Command Line Options

The `smashtest` command supports several options:

```bash
smashtest <path> [options]
```

Options:
- `--tag=<tag>`: Run tests with a specific tag
- `--verbose`: Show detailed test output
- `--format=<format>`: Output format (default: pretty, options: json, tap)
- `--help`, `-h`: Show help message
- `--version`, `-v`: Show version information

### Filtering Tests

You can filter tests in several ways:

1. By file path:
   ```bash
   smashtest ./tests/unit.test.smash
   ```

2. By tag:
   ```bash
   smashtest ./tests --tag=unit
   ```

### Output Formats

The test runner supports multiple output formats:

1. Pretty (default): Colorful, human-readable output
2. JSON: Machine-readable JSON format
3. TAP: Test Anything Protocol format for integration with other tools

```bash
smashtest ./tests --format=json
```

## Integration with Packages

When you create a new package with `smashpkg create`, it automatically includes test files that work with the testing framework:

```bash
smashpkg create my-package
```

This creates a package with the following test structure:

```
my-package/
├── tests/
│   └── index.test.smash
```

You can run the tests for your package with:

```bash
smashtest ./smashlang_packages/my-package/tests
```

## Best Practices

1. **Organize tests logically**: Use `describe` blocks to group related tests.
2. **Write descriptive test names**: Test descriptions should clearly state what's being tested.
3. **One assertion per test**: Each test should focus on a single behavior.
4. **Use setup and teardown**: Use `beforeEach` and `afterEach` to avoid repetitive code.
5. **Test edge cases**: Include tests for boundary conditions and error cases.
6. **Keep tests independent**: Tests should not depend on the state from other tests.
7. **Use tags for categorization**: Add tags to organize tests by type (unit, integration, etc.).

## Examples

Here's a complete example demonstrating various testing features:

```javascript
import { test, describe, expect, beforeEach, afterEach } from 'std/testing';
import { stringUtils } from '../src/utils/string.smash';

// Basic test
test('capitalize function works correctly', () => {
  expect(stringUtils.capitalize('hello')).toBe('Hello');
});

// Test group with setup and teardown
describe('String utilities', () => {
  let testString;
  
  beforeEach(() => {
    testString = 'hello world';
  });
  
  afterEach(() => {
    testString = null;
  });
  
  test('capitalize works correctly', () => {
    const result = stringUtils.capitalize(testString);
    expect(result).toBe('Hello world');
  });
  
  test('reverse works correctly', () => {
    const result = stringUtils.reverse(testString);
    expect(result).toBe('dlrow olleh');
  });
});

// Async test
test('async operations work', async () => {
  const result = await Promise.resolve('test');
  expect(result).toBe('test');
});

// Test with tags
test('tagged test example', ['unit', 'string'], () => {
  expect('SmashLang'.length).toBe(9);
});
```

For more examples, see the [examples directory](./examples/testing) and the [template package](./smashlang_packages/__package__template/tests).
