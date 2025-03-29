# Error Handling in SmashLang

Error handling is a critical aspect of writing robust applications. SmashLang provides several mechanisms for detecting, throwing, and handling errors during program execution.

## Try-Catch-Finally

The primary mechanism for handling errors in SmashLang is the `try-catch-finally` statement:

```js
try {
  // Code that might throw an error
  let result = riskyOperation();
} catch (error) {
  // Code to handle the error
  print(`An error occurred: ${error.message}`);
} finally {
  // Code that always executes, regardless of whether an error occurred
  cleanupResources();
}
```

### Try Block

The `try` block contains the code that might throw an error. If an error occurs within this block, execution immediately jumps to the `catch` block.

### Catch Block

The `catch` block receives the error object that was thrown and allows you to handle the error gracefully. The error parameter (commonly named `error`, `err`, or `e`) contains information about the error, such as its message and stack trace.

### Finally Block

The `finally` block contains code that always executes, regardless of whether an error was thrown or caught. This is useful for cleanup operations that should happen regardless of the outcome, such as closing files or network connections.

The `finally` block is optional. You can have a `try-catch` without a `finally`:

```js
try {
  // Code that might throw an error
} catch (error) {
  // Code to handle the error
}
```

## Throwing Errors

You can throw your own errors using the `throw` statement. SmashLang allows you to throw any value as an error, but it's best practice to throw an `Error` object or one of its subclasses:

```js
fn divide(a, b) {
  if (b === 0) {
    throw new Error("Division by zero");
  }
  return a / b;
}

try {
  let result = divide(10, 0);
} catch (error) {
  print(error.message); // "Division by zero"
}
```

## Error Types

SmashLang provides several built-in error types:

### Error

The base error type for all errors:

```js
throw new Error("Something went wrong");
```

### TypeError

Indicates that a value is not of the expected type:

```js
fn processArray(arr) {
  if (!Array.isArray(arr)) {
    throw new TypeError("Expected an array");
  }
  // Process the array
}
```

### ReferenceError

Indicates that a reference to an undeclared variable was attempted:

```js
try {
  let value = undefinedVariable; // This variable doesn't exist
} catch (error) {
  print(error instanceof ReferenceError); // true
}
```

### SyntaxError

Indicates a syntax error in the code (these are usually caught during parsing, before execution):

```js
try {
  eval("let x = ;"); // Invalid syntax
} catch (error) {
  print(error instanceof SyntaxError); // true
}
```

### RangeError

Indicates that a value is not within the expected range:

```js
fn createArray(size) {
  if (size < 0) {
    throw new RangeError("Array size cannot be negative");
  }
  return new Array(size);
}
```

## Custom Error Types

You can create custom error types by extending the built-in `Error` class:

```js
class ValidationError extends Error {
  constructor(message) {
    super(message);
    this.name = "ValidationError";
  }
}

fn validateUser(user) {
  if (!user.name) {
    throw new ValidationError("User name is required");
  }
  if (!user.email) {
    throw new ValidationError("User email is required");
  }
}

try {
  validateUser({ name: "Alice" }); // Missing email
} catch (error) {
  if (error instanceof ValidationError) {
    print(`Validation error: ${error.message}`);
  } else {
    print(`Unexpected error: ${error.message}`);
  }
}
```

## Error Properties

Error objects in SmashLang have several properties:

### message

A human-readable description of the error:

```js
try {
  throw new Error("Something went wrong");
} catch (error) {
  print(error.message); // "Something went wrong"
}
```

### name

The name of the error type:

```js
try {
  throw new TypeError("Expected a string");
} catch (error) {
  print(error.name); // "TypeError"
}
```

### stack

A stack trace that shows where the error was thrown:

```js
try {
  throw new Error("Something went wrong");
} catch (error) {
  print(error.stack);
  // Error: Something went wrong
  //     at <anonymous>:2:9
  //     at ...
}
```

## Error Handling Patterns

### Selective Catching

You can selectively catch different types of errors:

```js
try {
  let result = riskyOperation();
} catch (error) {
  if (error instanceof TypeError) {
    // Handle type errors
  } else if (error instanceof RangeError) {
    // Handle range errors
  } else {
    // Handle other errors
  }
}
```

### Error Propagation

You can catch an error, perform some action, and then rethrow it:

```js
try {
  processData();
} catch (error) {
  // Log the error
  logError(error);
  
  // Rethrow the error
  throw error;
}
```

### Error Transformation

You can catch an error and throw a different one:

```js
try {
  accessDatabase();
} catch (error) {
  // Transform the error
  throw new ApplicationError("Database access failed", { cause: error });
}
```

## Async Error Handling

When working with asynchronous code, you can use `try-catch` with `async/await`:

```js
async fn fetchData() {
  try {
    let response = await fetch("https://api.example.com/data");
    let data = await response.json();
    return data;
  } catch (error) {
    print(`Failed to fetch data: ${error.message}`);
    return null;
  }
}
```

## Best Practices

1. **Be specific about what you catch**: Catch only the errors you can handle, and let others propagate.

2. **Use descriptive error messages**: Error messages should clearly indicate what went wrong.

3. **Clean up resources in finally blocks**: Use `finally` blocks to ensure resources are properly released.

4. **Create custom error types** for different categories of errors in your application.

5. **Avoid empty catch blocks**: Always do something meaningful when catching an error, even if it's just logging it.

6. **Consider the scope of your try blocks**: Keep `try` blocks as small as possible to catch errors at the right level.

## See Also

- [Control Flow](./control-flow.md)
- [Async Programming](./async.md)
- [Functions and Closures](./functions.md)
