# Modern JavaScript Features Implementation for SmashLang

This document outlines the implementation plan for enhancing SmashLang with modern JavaScript features that are either partially implemented or missing.

## Overview

Modern JavaScript has evolved significantly with ECMAScript 2015 (ES6) and subsequent versions, introducing features that make code more concise, readable, and powerful. While SmashLang has implemented some of these features (like arrow functions and template literals), others are missing or incomplete.

## Current Status

Based on code examination:
- Arrow functions are implemented in the grammar and AST
- Template literals are implemented
- Basic destructuring is supported
- Spread/rest operators have AST nodes
- Optional chaining is not implemented
- Nullish coalescing is not implemented
- Default parameters may be partially implemented

## Implementation Goals

1. Enhance destructuring assignment
2. Implement optional chaining
3. Implement nullish coalescing
4. Complete spread/rest operator implementation
5. Enhance default parameters
6. Add comprehensive tests

## Detailed Implementation Plan

### 1. Enhanced Destructuring Assignment

```javascript
// Target syntax
// Object destructuring
const { url, options = {}, options: { timeout = 1000 } } = config;

// Array destructuring
const [first, second, ...rest] = array;

// Nested destructuring
const { users: [{ name, age }] } = response;
```

#### Implementation Steps:

1. **Grammar Updates**
   - Enhance destructuring rules to support nested patterns and defaults

```pest
destructuring_assignment = { array_destructuring | object_destructuring }

array_destructuring = { 
    "[" ~ array_destructuring_target ~ ("," ~ array_destructuring_target)* ~ "]" ~ "=" ~ expression 
}

object_destructuring = { 
    "{" ~ object_destructuring_target ~ ("," ~ object_destructuring_target)* ~ "}" ~ "=" ~ expression 
}

array_destructuring_target = {
    rest_element |
    destructuring_assignment |
    (identifier ~ ("=" ~ expression)?)
}

object_destructuring_target = {
    rest_element |
    (identifier ~ ("=" ~ expression)?) |
    (identifier ~ ":" ~ (destructuring_assignment | identifier) ~ ("=" ~ expression)?)
}

rest_element = { "..." ~ identifier }
```

2. **AST Updates**
   - Enhance destructuring nodes to support nested patterns

```rust
ArrayDestructuring {
    targets: Vec<DestructuringTarget>,
    value: Box<AstNode>,
}

ObjectDestructuring {
    targets: Vec<DestructuringTarget>,
    value: Box<AstNode>,
}

enum DestructuringTarget {
    Identifier {
        name: String,
        default_value: Option<Box<AstNode>>,
    },
    ObjectPattern {
        properties: Vec<DestructuringProperty>,
    },
    ArrayPattern {
        elements: Vec<DestructuringTarget>,
    },
    RestElement {
        argument: Box<DestructuringTarget>,
    },
}

struct DestructuringProperty {
    key: String,
    value: DestructuringTarget,
}
```

3. **Runtime Implementation**
   - Implement destructuring algorithm for objects and arrays
   - Handle nested patterns
   - Apply default values when source is undefined
   - Handle rest elements

### 2. Optional Chaining

```javascript
// Target syntax
const city = user?.address?.city;
const name = user?.['first' + 'Name'];
const processedData = response?.data?.process?.();
```

#### Implementation Steps:

1. **Grammar Updates**
   - Add optional chaining to property access and method calls

```pest
optional_property_access = { "?." ~ identifier }
optional_computed_property_access = { "?." ~ "[" ~ expression ~ "]" }
optional_method_call = { "?." ~ identifier ~ arguments }

member_access = { 
    "." ~ identifier | 
    "[" ~ expression ~ "]" |
    optional_property_access |
    optional_computed_property_access |
    optional_method_call
}
```

2. **AST Updates**
   - Add optional chaining nodes to the AST

```rust
OptionalPropertyAccess {
    object: Box<AstNode>,
    property: String,
}

OptionalComputedPropertyAccess {
    object: Box<AstNode>,
    property: Box<AstNode>,
}

OptionalMethodCall {
    object: Box<AstNode>,
    method: String,
    args: Vec<AstNode>,
}
```

3. **Runtime Implementation**
   - Check if the object is null or undefined before accessing properties
   - Return undefined if the object is null or undefined
   - Otherwise, proceed with the property access or method call

### 3. Nullish Coalescing

```javascript
// Target syntax
const port = options.port ?? 8080;
const timeout = config?.timeout ?? defaultTimeout ?? 1000;
```

#### Implementation Steps:

1. **Grammar Updates**
   - Add nullish coalescing operator to expressions

```pest
nullish_coalescing_expression = { 
    logical_or_expression ~ ("??" ~ logical_or_expression)*
}

// Update the expression hierarchy
conditional_expression = { 
    nullish_coalescing_expression ~ ("?" ~ expression ~ ":" ~ expression)?
}
```

2. **AST Updates**
   - Add nullish coalescing node to the AST

```rust
NullishCoalescing {
    left: Box<AstNode>,
    right: Box<AstNode>,
}
```

3. **Runtime Implementation**
   - Evaluate the left operand
   - If it's null or undefined, evaluate and return the right operand
   - Otherwise, return the left operand

### 4. Enhanced Spread/Rest Operators

```javascript
// Target syntax
// Rest parameters
function sum(...numbers) {
  return numbers.reduce((total, n) => total + n, 0);
}

// Spread in arrays
const combined = [...array1, ...array2];

// Spread in objects
const merged = { ...obj1, ...obj2, additionalProp: value };
```

#### Implementation Steps:

1. **Grammar Updates**
   - Enhance spread/rest syntax for functions, arrays, and objects

```pest
rest_parameter = { "..." ~ identifier }
parameter_list = { (identifier | rest_parameter) ~ ("," ~ (identifier | rest_parameter))* }

spread_element = { "..." ~ expression }
array_literal = { "[" ~ (expression | spread_element) ~ ("," ~ (expression | spread_element))* ~ "]" }

spread_property = { "..." ~ expression }
property = { (identifier | string_literal) ~ ":" ~ expression | spread_property }
```

2. **AST Updates**
   - Enhance spread/rest nodes in the AST

```rust
RestParameter {
    name: String,
}

SpreadElement {
    expression: Box<AstNode>,
}

SpreadProperty {
    expression: Box<AstNode>,
}
```

3. **Runtime Implementation**
   - Implement rest parameter collection in functions
   - Implement array spreading algorithm
   - Implement object spreading algorithm

### 5. Enhanced Default Parameters

```javascript
// Target syntax
function connect(url, options = { timeout: 1000, retries: 3 }) {
  // Function body
}

function fetchData(endpoint, { headers = {}, method = 'GET' } = {}) {
  // Function body
}
```

#### Implementation Steps:

1. **Grammar Updates**
   - Enhance parameter list to support default values

```pest
parameter_with_default = { identifier ~ "=" ~ expression }
parameter_list = { 
    (identifier | parameter_with_default | rest_parameter) ~ 
    ("," ~ (identifier | parameter_with_default | rest_parameter))* 
}
```

2. **AST Updates**
   - Enhance parameter nodes to include default values

```rust
Parameter {
    name: String,
    default_value: Option<Box<AstNode>>,
    is_rest: bool,
}
```

3. **Runtime Implementation**
   - Check if arguments are provided for parameters
   - If not, evaluate and use the default value
   - Handle complex default values (objects, arrays, etc.)

### 6. Testing

Create comprehensive tests for:
- Nested destructuring with default values
- Optional chaining with various object structures
- Nullish coalescing with different values
- Spread/rest operators in different contexts
- Default parameters with complex values
- Edge cases and error conditions

## Implementation Details

### Destructuring Algorithm

1. Evaluate the right-hand side expression
2. For array destructuring:
   - Iterate through the array elements and assign to corresponding targets
   - Apply default values for undefined elements
   - Collect remaining elements for rest patterns
3. For object destructuring:
   - Extract properties by name
   - Apply default values for undefined properties
   - Handle nested patterns recursively
   - Collect remaining properties for rest patterns

### Optional Chaining Evaluation

1. Evaluate the object expression
2. If the result is null or undefined, return undefined
3. Otherwise, proceed with the property access or method call

### Nullish Coalescing Evaluation

1. Evaluate the left operand
2. If the result is null or undefined, evaluate and return the right operand
3. Otherwise, return the left operand

## Resources

- [MDN Optional Chaining](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Optional_chaining)
- [MDN Nullish Coalescing](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Nullish_coalescing_operator)
- [MDN Destructuring Assignment](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Destructuring_assignment)
- [MDN Spread Syntax](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Spread_syntax)
- [MDN Rest Parameters](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Functions/rest_parameters)
- [MDN Default Parameters](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Functions/Default_parameters)

## Timeline

1. **Week 1**: Enhance destructuring assignment
2. **Week 2**: Implement optional chaining
3. **Week 3**: Implement nullish coalescing
4. **Week 4**: Complete spread/rest operator implementation
5. **Week 5**: Enhance default parameters
6. **Week 6**: Comprehensive testing and bug fixing