# Functions and Closures in SmashLang

Functions are a fundamental building block in SmashLang. They allow you to encapsulate reusable code, create abstractions, and build modular programs.

## Function Declaration

In SmashLang, functions are declared using the `fn` keyword:

```js
fn greet(name) {
  return `Hello, ${name}!`;
}

// Calling the function
print(greet("World")); // Output: Hello, World!
```

## Function Parameters

Functions can accept multiple parameters, which are listed inside the parentheses:

```js
fn add(a, b) {
  return a + b;
}

print(add(5, 3)); // Output: 8
```

### Default Parameters

You can specify default values for parameters, which will be used if the argument is not provided:

```js
fn greet(name, greeting = "Hello") {
  return `${greeting}, ${name}!`;
}

print(greet("World")); // Output: Hello, World!
print(greet("World", "Hi")); // Output: Hi, World!
```

### Rest Parameters

The rest parameter syntax allows a function to accept an indefinite number of arguments as an array:

```js
fn sum(...numbers) {
  let total = 0;
  for (let num of numbers) {
    total += num;
  }
  return total;
}

print(sum(1, 2, 3, 4)); // Output: 10
```

## Arrow Functions

SmashLang supports arrow functions, which provide a more concise syntax for writing functions:

```js
// Traditional function
fn double(x) {
  return x * 2;
}

// Arrow function equivalent
let double = (x) => x * 2;

// Arrow function with multiple parameters
let add = (a, b) => a + b;

// Arrow function with block body
let greet = (name) => {
  let message = `Hello, ${name}!`;
  return message;
};
```

## Function Expressions

Functions can be assigned to variables, passed as arguments, and returned from other functions:

```js
// Function expression
let greet = fn(name) {
  return `Hello, ${name}!`;
};

// Passing a function as an argument
fn executeFunction(func, value) {
  return func(value);
}

print(executeFunction(greet, "World")); // Output: Hello, World!
```

## Closures

SmashLang supports closures, which are functions that remember the environment in which they were created:

```js
fn createCounter() {
  let count = 0;
  
  return fn() {
    count += 1;
    return count;
  };
}

let counter = createCounter();
print(counter()); // Output: 1
print(counter()); // Output: 2
print(counter()); // Output: 3
```

In this example, the inner function maintains access to the `count` variable from its parent scope, even after the parent function has finished executing.

## Higher-Order Functions

SmashLang supports higher-order functions, which are functions that take other functions as arguments or return functions:

```js
// Function that returns a function
fn multiplier(factor) {
  return fn(number) {
    return number * factor;
  };
}

let double = multiplier(2);
let triple = multiplier(3);

print(double(5)); // Output: 10
print(triple(5)); // Output: 15
```

## Method Shorthand Syntax

When defining methods in objects, you can use a shorthand syntax:

```js
// Traditional method definition
let person = {
  name: "Alice",
  greet: fn() {
    return `Hello, my name is ${this.name}`;
  }
};

// Method shorthand syntax
let person = {
  name: "Alice",
  greet() {
    return `Hello, my name is ${this.name}`;
  }
};
```

## Destructuring Parameters

You can use destructuring in function parameters to extract values from objects and arrays:

```js
// Object destructuring in parameters
fn printUserInfo({ name, age, email }) {
  print(`Name: ${name}, Age: ${age}, Email: ${email}`);
}

let user = { name: "Alice", age: 30, email: "alice@example.com" };
printUserInfo(user);

// Array destructuring in parameters
fn printCoordinates([x, y, z]) {
  print(`X: ${x}, Y: ${y}, Z: ${z}`);
}

let point = [10, 20, 30];
printCoordinates(point);
```

## Best Practices

1. **Use descriptive function names** that indicate what the function does.

2. **Keep functions small and focused** on a single task.

3. **Use arrow functions** for short, simple operations and callbacks.

4. **Avoid side effects** when possible, making functions more predictable and easier to test.

5. **Use default parameters** instead of conditionally setting values inside the function body.

6. **Document complex functions** with comments explaining their purpose, parameters, and return values.

## See Also

- [Object Enhancements](./object-enhancements.md)
- [Destructuring and Spread Operator](./destructuring.md)
- [Modules and Imports](./modules.md)
