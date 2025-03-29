# Pattern Matching in SmashLang

Pattern matching is a powerful feature in SmashLang that allows you to match values against patterns and extract data from complex structures. It provides a concise and expressive way to handle different cases in your code.

## Basic Pattern Matching

The `match` expression in SmashLang allows you to compare a value against a series of patterns and execute code based on the first matching pattern:

```js
fn describeNumber(n) {
  return match (n) {
    0 => "Zero",
    1 => "One",
    2 => "Two",
    _ => `Number: ${n}`  // Default case (wildcard pattern)
  };
}

print(describeNumber(1)); // "One"
print(describeNumber(5)); // "Number: 5"
```

In this example, the `match` expression compares the value of `n` against each pattern in order. When a match is found, the corresponding expression is evaluated and returned.

## Pattern Types

SmashLang supports various types of patterns:

### Literal Patterns

Match against specific literal values:

```js
fn describeValue(value) {
  return match (value) {
    42 => "The answer",
    "hello" => "A greeting",
    true => "Boolean true",
    null => "Null value",
    _ => "Something else"
  };
}
```

### Variable Patterns

Bind the matched value to a variable:

```js
fn processValue(value) {
  return match (value) {
    x => `Got value: ${x}`  // Binds value to x
  };
}
```

### Wildcard Pattern

Match any value without binding it:

```js
fn processValue(value) {
  return match (value) {
    42 => "The answer",
    _ => "Not the answer"  // Matches anything else
  };
}
```

### Object Patterns

Match against object structures and extract properties:

```js
fn describeUser(user) {
  return match (user) {
    { name: "Alice", role: "Admin" } => "Alice the admin",
    { name: "Bob", role: "User" } => "Bob the user",
    { name, role: "Admin" } => `Admin: ${name}`,  // Extract name, match specific role
    { name, role } => `${name} has role ${role}`,  // Extract both properties
    _ => "Unknown user"
  };
}
```

### Array Patterns

Match against array structures and extract elements:

```js
fn describeArray(arr) {
  return match (arr) {
    [] => "Empty array",
    [x] => `Single element: ${x}`,
    [x, y] => `Two elements: ${x} and ${y}`,
    [first, ...rest] => `First: ${first}, Rest: ${rest}`,
    _ => "Something else"
  };
}
```

### Guard Patterns

Add conditions to patterns using the `when` clause:

```js
fn describeNumber(n) {
  return match (n) {
    x when x < 0 => "Negative number",
    x when x === 0 => "Zero",
    x when x % 2 === 0 => `Even number: ${x}`,
    x when x % 2 === 1 => `Odd number: ${x}`,
    _ => "Not a number"
  };
}
```

## Nested Patterns

Patterns can be nested to match complex data structures:

```js
fn processData(data) {
  return match (data) {
    { user: { name, address: { city } } } => `${name} is from ${city}`,
    { items: [first, ...rest] } => `First item: ${first}, ${rest.length} more items`,
    { error: { message } } => `Error: ${message}`,
    _ => "Unknown data structure"
  };
}
```

## Pattern Matching in Control Flow

Pattern matching can be used in control flow statements:

### Match in If Statements

```js
if (match (value) {
  { type: "success", data } => true,
  _ => false
}) {
  // Handle success case
  processData(value.data);
}
```

### Match in Assignments

```js
let description = match (status) {
  "pending" => "Waiting for processing",
  "processing" => "Currently processing",
  "completed" => "Processing complete",
  "failed" => "Processing failed",
  _ => "Unknown status"
};
```

## Exhaustiveness Checking

SmashLang's pattern matching includes exhaustiveness checking, which ensures that all possible cases are handled:

```js
// This will produce a warning because not all possible values are handled
fn describeBoolean(value) {
  return match (value) {
    true => "It's true"
    // Missing case for false
  };
}
```

To make the pattern matching exhaustive, either add all specific cases or include a wildcard pattern:

```js
fn describeBoolean(value) {
  return match (value) {
    true => "It's true",
    false => "It's false"
    // All cases covered
  };
}

// Or with a wildcard
fn describeBoolean(value) {
  return match (value) {
    true => "It's true",
    _ => "It's not true"
  };
}
```

## Pattern Matching with Enums

Pattern matching works well with enumerated types:

```js
const Result = {
  Ok: (value) => ({ type: "ok", value }),
  Err: (error) => ({ type: "error", error })
};

fn processResult(result) {
  return match (result) {
    { type: "ok", value } => `Success: ${value}`,
    { type: "error", error } => `Error: ${error}`
  };
}

let success = Result.Ok(42);
let failure = Result.Err("Something went wrong");

print(processResult(success)); // "Success: 42"
print(processResult(failure)); // "Error: Something went wrong"
```

## Best Practices

1. **Make patterns exhaustive** by including all possible cases or a wildcard pattern.

2. **Order patterns from most specific to least specific**, as patterns are matched in order.

3. **Use guard clauses** to add conditions to patterns when needed.

4. **Leverage destructuring** in patterns to extract and bind values.

5. **Keep pattern matching expressions focused** on a single concern.

## See Also

- [Control Flow](./control-flow.md)
- [Destructuring and Spread Operator](./destructuring.md)
- [Types and Type System](./types.md)
