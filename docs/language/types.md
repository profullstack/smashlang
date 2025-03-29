# Types and Type System in SmashLang

SmashLang features a dynamic type system with strong typing characteristics. This document explains the type system and the various data types available in SmashLang.

## Basic Types

SmashLang includes the following basic data types:

### Number

SmashLang has a single number type that represents both integers and floating-point values:

```js
let integer = 42;
let float = 3.14159;
let negative = -10;
let scientific = 2.5e6;  // 2,500,000
let binary = 0b1010;     // 10 in decimal
let octal = 0o744;       // 484 in decimal
let hex = 0xFF;          // 255 in decimal
```

Number operations:

```js
let sum = 5 + 10;        // Addition
let difference = 10 - 5;  // Subtraction
let product = 5 * 10;     // Multiplication
let quotient = 10 / 5;    // Division
let remainder = 10 % 3;   // Modulo (remainder)
let power = 2 ** 3;       // Exponentiation (2^3 = 8)
```

### String

Strings are sequences of characters, enclosed in single quotes (`'`), double quotes (`"`), or backticks (`` ` ``):

```js
let single = 'Single quotes';
let double = "Double quotes";
let backticks = `Backticks`;
```

Template strings (using backticks) support interpolation:

```js
let name = "Alice";
let greeting = `Hello, ${name}!`; // "Hello, Alice!"
```

String operations:

```js
let concatenated = "Hello, " + name + "!"; // Concatenation
let length = name.length;                  // String length
let uppercase = name.toUpperCase();        // "ALICE"
let lowercase = name.toLowerCase();        // "alice"
let substring = name.substring(1, 3);      // "li"
let charAt = name.charAt(0);               // "A"
```

### Boolean

Boolean values represent truth values: `true` or `false`:

```js
let isActive = true;
let isComplete = false;
```

Boolean operations:

```js
let and = true && false;  // Logical AND (false)
let or = true || false;   // Logical OR (true)
let not = !true;          // Logical NOT (false)
```

### Null

`null` represents the intentional absence of any object value:

```js
let empty = null;
```

### Undefined

`undefined` represents a value that hasn't been assigned:

```js
let notDefined;
print(notDefined); // undefined
```

## Complex Types

### Object

Objects are collections of key-value pairs:

```js
let person = {
  name: "Alice",
  age: 30,
  isActive: true,
  address: {
    city: "New York",
    country: "USA"
  }
};
```

Accessing object properties:

```js
let name = person.name;        // Dot notation
let age = person["age"];       // Bracket notation
let city = person.address.city; // Nested properties
```

Modifying objects:

```js
person.name = "Bob";            // Update a property
person.email = "bob@example.com"; // Add a new property
delete person.age;               // Remove a property
```

### Array

Arrays are ordered collections of values:

```js
let numbers = [1, 2, 3, 4, 5];
let mixed = [1, "two", true, { key: "value" }];
let empty = [];
```

Accessing array elements:

```js
let first = numbers[0];  // First element (index 0)
let last = numbers[numbers.length - 1];  // Last element
```

Array operations:

```js
numbers.push(6);         // Add to end: [1, 2, 3, 4, 5, 6]
numbers.pop();           // Remove from end: [1, 2, 3, 4, 5]
numbers.unshift(0);      // Add to beginning: [0, 1, 2, 3, 4, 5]
numbers.shift();         // Remove from beginning: [1, 2, 3, 4, 5]
let sliced = numbers.slice(1, 3);  // Extract [2, 3]
let spliced = numbers.splice(1, 2, 'a', 'b');  // Replace elements
let joined = numbers.join(", ");  // "1, 2, 3, 4, 5"
```

### Function

Functions are first-class objects in SmashLang:

```js
// Function declaration
fn add(a, b) {
  return a + b;
}

// Function expression
let multiply = fn(a, b) {
  return a * b;
};

// Arrow function
let subtract = (a, b) => a - b;
```

## Type Conversion

SmashLang performs automatic type conversion in many contexts:

### String Conversion

```js
let num = 42;
let str = "The answer is " + num;  // "The answer is 42"
```

### Numeric Conversion

```js
let str = "42";
let num = str * 1;  // 42
```

### Boolean Conversion

The following values are falsy (convert to `false`):
- `false`
- `0`
- `""` (empty string)
- `null`
- `undefined`
- `NaN`

All other values are truthy (convert to `true`).

```js
if ("") {
  // This won't execute (empty string is falsy)
}

if ("hello") {
  // This will execute (non-empty string is truthy)
}
```

## Type Checking

You can check the type of a value using the `typeof` operator:

```js
typeof 42;           // "number"
typeof "hello";      // "string"
typeof true;         // "boolean"
typeof undefined;    // "undefined"
typeof null;         // "object" (historical quirk)
typeof {};           // "object"
typeof [];           // "object"
typeof fn() {};      // "function"
```

For more precise array checking:

```js
Array.isArray([]);   // true
Array.isArray({});   // false
```

## Type Coercion

SmashLang performs type coercion in certain operations:

```js
"5" + 3;     // "53" (number is converted to string)
"5" - 3;     // 2 (string is converted to number)
"5" * "3";   // 15 (both strings are converted to numbers)
5 + true;    // 6 (true is converted to 1)
5 + false;   // 5 (false is converted to 0)
```

To avoid unexpected type coercion, use strict equality operators:

```js
5 == "5";    // true (with type coercion)
5 === "5";   // false (no type coercion)
```

## Best Practices

1. **Use strict equality operators** (`===` and `!==`) to avoid unexpected type coercion.

2. **Be explicit about type conversions** when needed:
   ```js
   let num = Number("42");    // Convert string to number
   let str = String(42);      // Convert number to string
   let bool = Boolean(0);     // Convert to boolean (false)
   ```

3. **Check for undefined or null** before accessing properties:
   ```js
   if (obj && obj.property) {
     // Safe to access obj.property
   }
   ```

4. **Use Array.isArray()** to check if a value is an array.

## See Also

- [Syntax Reference](./syntax.md)
- [Functions and Closures](./functions.md)
- [Object Enhancements](./object-enhancements.md)
