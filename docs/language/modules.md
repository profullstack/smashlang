# Modules and Imports in SmashLang

SmashLang provides a modern module system that allows you to organize your code into reusable, maintainable units. This document explains how to create, import, and use modules in SmashLang.

## Module Basics

In SmashLang, any file can be a module. A module can export functions, objects, or primitive values that can be used by other modules.

## Exporting from Modules

### Named Exports

You can export individual items using the `export` keyword:

```js
// math.smash
export fn add(a, b) {
  return a + b;
}

export fn subtract(a, b) {
  return a - b;
}

export const PI = 3.14159;
```

You can also declare items first and then export them later:

```js
// utils.smash
fn formatDate(date) {
  // Implementation...
}

fn formatCurrency(amount) {
  // Implementation...
}

const VERSION = "1.0.0";

// Export multiple items at once
export { formatDate, formatCurrency, VERSION };
```

### Default Exports

Each module can have one default export, which is often used when a module primarily exports a single function or class:

```js
// calculator.smash
export default fn calculate(operation, a, b) {
  switch (operation) {
    case "add": return a + b;
    case "subtract": return a - b;
    case "multiply": return a * b;
    case "divide": return a / b;
    default: throw new Error("Unknown operation");
  }
}
```

You can also define the item first and then export it as default:

```js
// user.smash
const user = {
  name: "Default User",
  role: "Guest",
  permissions: ["read"]
};

export default user;
```

### Combining Named and Default Exports

A module can have both named exports and a default export:

```js
// api.smash
export fn get(url) {
  // Implementation...
}

export fn post(url, data) {
  // Implementation...
}

// Default export for the main functionality
export default fn request(method, url, data) {
  // Implementation...
}
```

## Importing from Modules

### Importing Named Exports

To import specific named exports from a module:

```js
// Import specific named exports
import { add, subtract } from "./math";

print(add(5, 3));      // 8
print(subtract(5, 3)); // 2
```

You can rename imports using the `as` keyword:

```js
// Rename imports to avoid naming conflicts
import { add as mathAdd, PI } from "./math";
import { add as arrayAdd } from "./array-utils";

print(mathAdd(1, 2));   // 3
print(arrayAdd([1, 2])); // [1, 2]
```

### Importing Default Exports

To import a default export:

```js
// Import a default export
import calculate from "./calculator";

print(calculate("add", 5, 3)); // 8
```

### Importing Both Named and Default Exports

You can import both the default export and named exports from a module:

```js
// Import both default and named exports
import request, { get, post } from "./api";

get("/users");
post("/users", { name: "Alice" });
request("PUT", "/users/1", { name: "Bob" });
```

### Importing All Exports

You can import all exports from a module as a namespace object:

```js
// Import everything as a namespace
import * as math from "./math";

print(math.add(5, 3));      // 8
print(math.subtract(5, 3)); // 2
print(math.PI);             // 3.14159
```

## Module Resolution

SmashLang resolves module specifiers in the following ways:

1. **Relative paths**: Paths starting with `./` or `../` are resolved relative to the importing file.
   ```js
   import { helper } from "./utils";  // Looks for utils.smash in the same directory
   import config from "../config";    // Looks for config.smash in the parent directory
   ```

2. **Absolute paths**: Paths starting with `/` are resolved from the project root.
   ```js
   import db from "/src/database";  // Looks for /src/database.smash from project root
   ```

3. **Package names**: Bare specifiers are resolved as packages from the `node_modules` directory or the SmashLang package registry.
   ```js
   import { useState } from "react";  // Looks for the react package
   ```

## Re-exporting

You can re-export items from another module:

```js
// Re-export specific named exports
export { add, subtract } from "./math";

// Re-export and rename
export { formatDate as formatDateTime } from "./utils";

// Re-export everything
export * from "./helpers";

// Re-export the default export
export { default } from "./calculator";

// Re-export the default export with a name
export { default as calc } from "./calculator";
```

## Dynamic Imports

SmashLang supports dynamic imports using the `import()` function, which returns a Promise:

```js
async fn loadModule() {
  try {
    // Dynamically import a module
    const math = await import("./math");
    
    return math.add(5, 3);
  } catch (error) {
    print("Failed to load module:", error);
  }
}
```

## Module Initialization

When a module is imported, its code is executed once, and the exports are cached. Subsequent imports of the same module will use the cached exports.

```js
// counter.smash
let count = 0;

export fn increment() {
  count += 1;
  return count;
}

export fn getCount() {
  return count;
}

print("Module initialized"); // This will be printed only once
```

```js
// main.smash
import { increment, getCount } from "./counter";
// "Module initialized" is printed

print(increment()); // 1
print(increment()); // 2

// Even when imported again in another module, the state is preserved
import { getCount } from "./counter";
// No initialization message is printed again

print(getCount()); // 2
```

## Best Practices

1. **One responsibility per module**: Each module should have a single, well-defined responsibility.

2. **Explicit exports**: Be explicit about what you're exporting from a module.

3. **Use named exports** for modules that export multiple items.

4. **Use default exports** for modules that primarily export a single function, class, or object.

5. **Avoid side effects** in modules when possible.

6. **Use consistent naming**: Use consistent naming conventions for your module files and exports.

## See Also

- [Functions and Closures](./functions.md)
- [Object Enhancements](./object-enhancements.md)
- [Error Handling](./error-handling.md)
