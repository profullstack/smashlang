# Control Flow in SmashLang

Control flow statements allow you to control the execution path of your program based on conditions, repeat blocks of code, and handle exceptions. This document covers the various control flow mechanisms available in SmashLang.

## Conditional Statements

### If Statement

The `if` statement executes a block of code if a specified condition is true:

```js
let temperature = 22;

if (temperature > 30) {
  print("It's hot outside!");
}
```

### If-Else Statement

The `if-else` statement executes one block of code if a condition is true and another block if the condition is false:

```js
let temperature = 22;

if (temperature > 30) {
  print("It's hot outside!");
} else {
  print("It's not hot outside.");
}
```

### If-Else If-Else Statement

The `if-else if-else` statement allows you to check multiple conditions:

```js
let temperature = 22;

if (temperature > 30) {
  print("It's hot outside!");
} else if (temperature > 20) {
  print("It's warm outside.");
} else if (temperature > 10) {
  print("It's cool outside.");
} else {
  print("It's cold outside.");
}
```

### Ternary Operator

The ternary operator is a shorthand for the `if-else` statement:

```js
let temperature = 22;
let weather = temperature > 30 ? "hot" : "not hot";
print(`It's ${weather} outside.`);
```

## Switch Statement

The `switch` statement evaluates an expression and executes the code block associated with the matching case:

```js
let day = "Monday";

switch (day) {
  case "Monday":
    print("Start of the work week.");
    break;
  case "Tuesday":
  case "Wednesday":
  case "Thursday":
    print("Middle of the work week.");
    break;
  case "Friday":
    print("End of the work week.");
    break;
  case "Saturday":
  case "Sunday":
    print("Weekend!");
    break;
  default:
    print("Invalid day.");
}
```

The `break` statement is used to exit the switch block. Without it, execution would "fall through" to the next case.

## Loops

### For Loop

The `for` loop repeats a block of code a specified number of times:

```js
// Basic for loop
for (let i = 0; i < 5; i++) {
  print(`Iteration ${i}`);
}

// For loop with multiple variables
for (let i = 0, j = 10; i < 5; i++, j--) {
  print(`i = ${i}, j = ${j}`);
}
```

### While Loop

The `while` loop repeats a block of code as long as a specified condition is true:

```js
let count = 0;

while (count < 5) {
  print(`Count: ${count}`);
  count++;
}
```

### Do-While Loop

The `do-while` loop is similar to the `while` loop, but it always executes the block of code at least once before checking the condition:

```js
let count = 0;

do {
  print(`Count: ${count}`);
  count++;
} while (count < 5);
```

### For-Of Loop

The `for-of` loop iterates over the values in an iterable object (like an array):

```js
let fruits = ["apple", "banana", "cherry"];

for (let fruit of fruits) {
  print(fruit);
}
```

### For-In Loop

The `for-in` loop iterates over the enumerable properties of an object:

```js
let person = { name: "Alice", age: 30, city: "New York" };

for (let key in person) {
  print(`${key}: ${person[key]}`);
}
```

## Loop Control

### Break Statement

The `break` statement terminates the current loop or switch statement:

```js
for (let i = 0; i < 10; i++) {
  if (i === 5) {
    break; // Exit the loop when i is 5
  }
  print(i);
}
// Output: 0, 1, 2, 3, 4
```

### Continue Statement

The `continue` statement skips the current iteration of a loop and continues with the next iteration:

```js
for (let i = 0; i < 10; i++) {
  if (i % 2 === 0) {
    continue; // Skip even numbers
  }
  print(i);
}
// Output: 1, 3, 5, 7, 9
```

## Error Handling

### Try-Catch Statement

The `try-catch` statement allows you to handle exceptions (runtime errors):

```js
try {
  // Code that might throw an error
  let result = someUndefinedFunction();
} catch (error) {
  // Code to handle the error
  print(`An error occurred: ${error.message}`);
}
```

### Try-Catch-Finally Statement

The `finally` block contains code that always executes, regardless of whether an exception was thrown or caught:

```js
try {
  // Code that might throw an error
  let result = someUndefinedFunction();
} catch (error) {
  // Code to handle the error
  print(`An error occurred: ${error.message}`);
} finally {
  // Code that always executes
  print("This always runs");
}
```

### Throwing Exceptions

You can throw your own exceptions using the `throw` statement:

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

## Short-Circuit Evaluation

Logical operators (`&&` and `||`) use short-circuit evaluation, which can be used for conditional execution:

### Logical AND (&&)

The `&&` operator returns the first falsy value or the last value if all are truthy:

```js
// Only call the function if user exists
let user = getUser();
user && processUser(user);

// Equivalent to:
if (user) {
  processUser(user);
}
```

### Logical OR (||)

The `||` operator returns the first truthy value or the last value if all are falsy:

```js
// Use a default value if the first value is falsy
let name = getUserName() || "Anonymous";

// Equivalent to:
let name = getUserName();
if (!name) {
  name = "Anonymous";
}
```

### Nullish Coalescing (??)

The `??` operator returns the right-hand operand when the left-hand operand is `null` or `undefined`:

```js
let count = getCount() ?? 0;

// Equivalent to:
let count = (getCount() !== null && getCount() !== undefined) ? getCount() : 0;
```

## Best Practices

1. **Keep conditions simple** and easy to understand.

2. **Avoid deep nesting** of conditional statements; consider refactoring complex conditions into separate functions.

3. **Use meaningful variable names** in loops to make your code more readable.

4. **Always include a default case** in switch statements to handle unexpected values.

5. **Use try-catch blocks** to handle potential errors gracefully.

6. **Consider using early returns** to reduce nesting and improve readability.

## See Also

- [Functions and Closures](./functions.md)
- [Error Handling](./error-handling.md)
- [Pattern Matching](./pattern-matching.md)
