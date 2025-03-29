# SmashLang Syntax Reference

This document provides a comprehensive reference for SmashLang's syntax.

## Comments

SmashLang supports single-line comments:

```js
// This is a single-line comment
let x = 5; // This is an end-of-line comment
```

## Variables and Constants

### Variable Declaration

Variables are declared using the `let` keyword:

```js
let name = "Alice";
let age = 30;
let isActive = true;
```

### Constant Declaration

Constants are declared using the `const` keyword:

```js
const PI = 3.14159;
const MAX_USERS = 100;
```

Constants cannot be reassigned after declaration.

## Data Types

SmashLang has the following primitive data types:

### Numbers

```js
let integer = 42;
let float = 3.14;
let negative = -10;
```

### Strings

```js
let singleQuotes = 'Hello';
let doubleQuotes = "World";
let template = `Hello, ${name}!`; // Template strings with interpolation
```

### Booleans

```js
let isTrue = true;
let isFalse = false;
```

### Null

```js
let empty = null;
```

## Operators

### Arithmetic Operators

```js
let sum = a + b;        // Addition
let difference = a - b; // Subtraction
let product = a * b;    // Multiplication
let quotient = a / b;   // Division
let remainder = a % b;  // Modulo
```

### Assignment Operators

```js
x = y;    // Basic assignment
x += y;   // Addition assignment (x = x + y)
x -= y;   // Subtraction assignment (x = x - y)
x *= y;   // Multiplication assignment (x = x * y)
x /= y;   // Division assignment (x = x / y)
```

### Comparison Operators

```js
x == y;   // Equal to
x != y;   // Not equal to
x === y;  // Strict equal to (value and type)
x !== y;  // Strict not equal to (value and type)
x > y;    // Greater than
x < y;    // Less than
x >= y;   // Greater than or equal to
x <= y;   // Less than or equal to
```

### Logical Operators

```js
x && y;   // Logical AND
x || y;   // Logical OR
!x;       // Logical NOT
```

## Control Flow

### Conditional Statements

```js
// If statement
if (condition) {
  // Code to execute if condition is true
}

// If-else statement
if (condition) {
  // Code to execute if condition is true
} else {
  // Code to execute if condition is false
}

// If-else if-else statement
if (condition1) {
  // Code to execute if condition1 is true
} else if (condition2) {
  // Code to execute if condition2 is true
} else {
  // Code to execute if all conditions are false
}
```

### Switch Statement

```js
switch (expression) {
  case value1:
    // Code to execute if expression === value1
    break;
  case value2:
    // Code to execute if expression === value2
    break;
  default:
    // Code to execute if no case matches
}
```

### Loops

```js
// For loop
for (let i = 0; i < 10; i++) {
  // Code to repeat
}

// While loop
while (condition) {
  // Code to repeat as long as condition is true
}

// For...of loop (iterating over arrays)
for (let item of array) {
  // Code to execute for each item
}

// For...in loop (iterating over object properties)
for (let key in object) {
  // Code to execute for each property
}
```

## Functions

### Function Declaration

```js
fn add(a, b) {
  return a + b;
}
```

### Arrow Functions

```js
let add = (a, b) => a + b;

let greet = (name) => {
  return `Hello, ${name}!`;
};
```

### Function Expressions

```js
let add = fn(a, b) {
  return a + b;
};
```

## Objects

### Object Literals

```js
let person = {
  name: "Alice",
  age: 30,
  greet() {
    return `Hello, my name is ${this.name}`;
  }
};
```

### Accessing Object Properties

```js
let name = person.name;        // Dot notation
let age = person["age"];       // Bracket notation
```

### Shorthand Property Names

```js
let name = "Alice";
let age = 30;

// Instead of {name: name, age: age}
let person = { name, age };
```

### Spread Operator with Objects

```js
let defaults = { theme: "dark", fontSize: 12 };
let userPrefs = { theme: "light" };

// Combine objects
let settings = { ...defaults, ...userPrefs };
```

## Arrays

### Array Literals

```js
let numbers = [1, 2, 3, 4, 5];
let mixed = [1, "two", true, null];
```

### Accessing Array Elements

```js
let first = numbers[0];  // First element (index 0)
let last = numbers[numbers.length - 1];  // Last element
```

### Array Methods

```js
numbers.push(6);         // Add to end
numbers.pop();           // Remove from end
numbers.unshift(0);      // Add to beginning
numbers.shift();         // Remove from beginning
numbers.slice(1, 3);     // Extract a section
numbers.splice(1, 2);    // Remove elements
```

### Spread Operator with Arrays

```js
let arr1 = [1, 2, 3];
let arr2 = [4, 5, 6];

// Combine arrays
let combined = [...arr1, ...arr2];
```

## Destructuring

### Array Destructuring

```js
let [a, b, c] = [1, 2, 3];

// Skip elements
let [first, , third] = [1, 2, 3];

// Rest pattern
let [head, ...tail] = [1, 2, 3, 4];
```

### Object Destructuring

```js
let { name, age } = person;

// Assign to different variable names
let { name: personName, age: personAge } = person;

// Default values
let { name, role = "User" } = person;

// Rest pattern
let { name, ...rest } = person;
```

## Modules

### Importing

```js
import { functionName } from "./module";
import { functionName as alias } from "./module";
import * as module from "./module";
```

### Exporting

```js
// Named exports
export fn add(a, b) {
  return a + b;
}

export const PI = 3.14159;

// Default export
export default fn multiply(a, b) {
  return a * b;
}
```

## Error Handling

```js
try {
  // Code that might throw an error
} catch (error) {
  // Code to handle the error
} finally {
  // Code that always executes
}
```

## Regular Expressions

```js
let regex = /pattern/;
let regexWithFlags = /pattern/gi;

// Testing
if (regex.test(string)) {
  // Pattern matches
}

// Matching
let matches = string.match(regex);
```

## See Also

- [Types and Type System](./types.md)
- [Functions and Closures](./functions.md)
- [Object Enhancements](./object-enhancements.md)
- [Destructuring and Spread Operator](./destructuring.md)
