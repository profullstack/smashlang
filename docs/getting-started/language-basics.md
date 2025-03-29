# SmashLang Language Basics

This guide introduces the fundamental concepts and syntax of SmashLang. If you're new to SmashLang, this is the perfect place to start.

## Hello World

Let's begin with the traditional "Hello World" program:

```js
// This is a comment
print("Hello, World!");
```

Save this to a file named `hello.smash` and run it with:

```bash
smash hello.smash
```

You should see `Hello, World!` printed to the console.

## Variables and Constants

SmashLang uses `let` for variables and `const` for constants, similar to JavaScript:

```js
// Variables (can be reassigned)
let name = "SmashLang";
let age = 1;
name = "SmashLang 1.0"; // Valid reassignment

// Constants (cannot be reassigned)
const PI = 3.14159;
const MAX_USERS = 100;
// PI = 3.14; // Error: Cannot reassign a constant
```

## Data Types

SmashLang has the following primitive data types:

### Numbers

```js
let integer = 42;
let float = 3.14;
let hex = 0xFF; // 255 in decimal
let binary = 0b1010; // 10 in decimal
let octal = 0o777; // 511 in decimal
let scientific = 1.5e3; // 1500
```

### Strings

```js
let single = 'Single quotes';
let double = "Double quotes";
let template = `Template string with ${single}`;

// Multi-line strings
let multiline = `
  This is a
  multi-line string
`;

// String methods
let length = single.length; // 14
let upper = single.toUpperCase(); // "SINGLE QUOTES"
let sub = single.substring(0, 6); // "Single"
```

### Booleans

```js
let isTrue = true;
let isFalse = false;

// Boolean operators
let and = isTrue && isFalse; // false
let or = isTrue || isFalse; // true
let not = !isTrue; // false
```

### Null and Undefined

```js
let nothing = null; // Explicitly no value
let notDefined; // Implicitly undefined
```

### Arrays

```js
// Array creation
let numbers = [1, 2, 3, 4, 5];
let mixed = [1, "two", true, null];
let empty = [];

// Accessing elements (zero-indexed)
let first = numbers[0]; // 1
let third = numbers[2]; // 3

// Array methods
numbers.push(6); // [1, 2, 3, 4, 5, 6]
numbers.pop(); // [1, 2, 3, 4, 5]
let sliced = numbers.slice(1, 3); // [2, 3]
let joined = numbers.join(", "); // "1, 2, 3, 4, 5"

// Array iteration
numbers.forEach(fn(num) {
  print(num);
});

let doubled = numbers.map(fn(num) => num * 2); // [2, 4, 6, 8, 10]
let evens = numbers.filter(fn(num) => num % 2 === 0); // [2, 4]
```

### Objects

```js
// Object creation
let person = {
  name: "John",
  age: 30,
  isEmployed: true,
  skills: ["JavaScript", "SmashLang", "Rust"],
  address: {
    city: "San Francisco",
    country: "USA"
  }
};

// Accessing properties
let personName = person.name; // "John"
let city = person.address.city; // "San Francisco"
let firstSkill = person.skills[0]; // "JavaScript"

// Property shorthand
let name = "Alice";
let age = 25;
let user = { name, age }; // Same as { name: name, age: age }

// Computed property names
let propName = "status";
let obj = {
  [propName]: "active"
}; // { status: "active" }
```

## Control Flow

### Conditionals

```js
// If statement
let age = 20;

if (age >= 18) {
  print("Adult");
} else if (age >= 13) {
  print("Teenager");
} else {
  print("Child");
}

// Ternary operator
let status = age >= 18 ? "Adult" : "Minor";
```

### Loops

```js
// For loop
for (let i = 0; i < 5; i++) {
  print(i);
}

// While loop
let count = 0;
while (count < 5) {
  print(count);
  count++;
}

// Do-while loop
let num = 0;
do {
  print(num);
  num++;
} while (num < 5);

// For-of loop (iterating over arrays)
let colors = ["red", "green", "blue"];
for (let color of colors) {
  print(color);
}

// For-in loop (iterating over object properties)
let person = { name: "John", age: 30 };
for (let key in person) {
  print(`${key}: ${person[key]}`);
}
```

## Functions

SmashLang supports multiple ways to define functions:

```js
// Function declaration
fn add(a, b) {
  return a + b;
}

// Function expression
let subtract = fn(a, b) {
  return a - b;
};

// Arrow function (concise syntax)
let multiply = fn(a, b) => a * b;

// Arrow function with block body
let divide = fn(a, b) => {
  if (b === 0) {
    throw new Error("Division by zero");
  }
  return a / b;
};

// Function with default parameters
fn greet(name = "Guest") {
  return `Hello, ${name}!`;
}

// Function with rest parameters
fn sum(...numbers) {
  return numbers.reduce((total, num) => total + num, 0);
}
```

## Modules and Imports

SmashLang uses a module system similar to ES modules:

```js
// Importing from the standard library
import { readFile, writeFile } from "std/fs";

// Importing a whole module
import * as math from "std/math";

// Importing from a local file
import { User } from "./user.smash";

// Importing from a package
import { parse } from "json";
```

Exporting from a module:

```js
// Named exports
export function add(a, b) {
  return a + b;
}

export const PI = 3.14159;

// Default export
export default function main() {
  print("Main function");
}
```

## Error Handling

SmashLang uses try-catch blocks for error handling:

```js
try {
  // Code that might throw an error
  let data = JSON.parse(invalidJson);
} catch (error) {
  // Handle the error
  print(`Error: ${error.message}`);
} finally {
  // This code always runs
  print("Cleanup code");
}
```

## Next Steps

Now that you understand the basics of SmashLang, you can:

- Try writing your [first SmashLang program](./first-program.md)
- Experiment with the [REPL](./repl.md)
- Learn about [pattern matching](../language/pattern-matching.md)
- Explore the [standard library](../standard-library/overview.md)

Happy coding with SmashLang!
