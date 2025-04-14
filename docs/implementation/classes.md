# Class Implementation for SmashLang

This document outlines the implementation plan for adding class support to SmashLang.

## Overview

Classes are a fundamental feature of object-oriented programming that provide a clear and concise syntax for creating objects with shared properties and methods. They are heavily used in modern JavaScript applications and frameworks, making them essential for SmashLang to support the packages we've developed.

## Current Status

Based on code examination:
- No grammar rules for class declarations
- No AST nodes for classes, constructors, or inheritance
- No runtime support for class-based object creation

## Implementation Goals

1. Implement class declaration syntax
2. Implement constructor methods
3. Implement instance methods and properties
4. Implement static methods and properties
5. Implement inheritance (extends)
6. Implement private fields and methods
7. Add comprehensive tests

## Detailed Implementation Plan

### 1. Class Declaration Syntax

```javascript
// Target syntax
class WebSocketClient {
  // Class body
}
```

#### Implementation Steps:

1. **Grammar Updates**
   - Add `class_declaration` rule to the grammar
   - Add `class_body` rule for the class contents
   - Update `statement` rule to include class declarations

```pest
// Example grammar addition
class_declaration = { "class" ~ identifier ~ ("extends" ~ identifier)? ~ class_body }
class_body = { "{" ~ class_member* ~ "}" }
class_member = { constructor_method | method_definition | property_definition }
```

2. **AST Updates**
   - Add `ClassDeclaration` node to the AST
   - Include fields for class name, parent class (if any), and body

```rust
// Example AST node
ClassDeclaration {
    name: String,
    parent: Option<String>,
    body: Vec<ClassMember>,
}

enum ClassMember {
    Constructor(ConstructorMethod),
    Method(MethodDefinition),
    Property(PropertyDefinition),
}
```

3. **Runtime Implementation**
   - Create a class representation in the runtime
   - Implement class registration in the environment
   - Implement basic instantiation mechanism

### 2. Constructor Methods

```javascript
// Target syntax
class WebSocketClient {
  constructor(url, protocols = [], options = {}) {
    this.url = url;
    this.protocols = protocols;
    this.options = options;
  }
}
```

#### Implementation Steps:

1. **Grammar Updates**
   - Add `constructor_method` rule to the grammar
   - Ensure it handles parameter lists

```pest
constructor_method = { "constructor" ~ "(" ~ parameter_list? ~ ")" ~ block }
```

2. **AST Updates**
   - Add `ConstructorMethod` node to the AST
   - Include fields for parameters and body

```rust
ConstructorMethod {
    params: Vec<String>,
    body: Vec<AstNode>,
}
```

3. **Runtime Implementation**
   - Implement constructor execution during object instantiation
   - Ensure `this` is properly bound to the new instance
   - Handle default parameters and rest parameters

### 3. Instance Methods and Properties

```javascript
// Target syntax
class WebSocketClient {
  // Instance property with initializer
  state = 'closed';
  
  // Instance method
  connect() {
    this.state = 'connecting';
    // Method implementation
  }
}
```

#### Implementation Steps:

1. **Grammar Updates**
   - Add `method_definition` rule to the grammar
   - Add `property_definition` rule for instance properties

```pest
method_definition = { async_modifier? ~ identifier ~ "(" ~ parameter_list? ~ ")" ~ block }
property_definition = { identifier ~ "=" ~ expression ~ ";" }
```

2. **AST Updates**
   - Add `MethodDefinition` node to the AST
   - Add `PropertyDefinition` node for properties

```rust
MethodDefinition {
    name: String,
    params: Vec<String>,
    body: Vec<AstNode>,
    is_async: bool,
}

PropertyDefinition {
    name: String,
    value: Box<AstNode>,
}
```

3. **Runtime Implementation**
   - Implement method binding to instances
   - Ensure `this` is properly bound in method calls
   - Implement property initialization during instantiation

### 4. Static Methods and Properties

```javascript
// Target syntax
class WebSocketClient {
  // Static property
  static STATES = {
    CONNECTING: 0,
    OPEN: 1,
    CLOSING: 2,
    CLOSED: 3
  };
  
  // Static method
  static isValidUrl(url) {
    return url.startsWith('ws://') || url.startsWith('wss://');
  }
}
```

#### Implementation Steps:

1. **Grammar Updates**
   - Update `method_definition` and `property_definition` to handle `static` keyword

```pest
method_definition = { static_modifier? ~ async_modifier? ~ identifier ~ "(" ~ parameter_list? ~ ")" ~ block }
property_definition = { static_modifier? ~ identifier ~ "=" ~ expression ~ ";" }
static_modifier = { "static" }
```

2. **AST Updates**
   - Update `MethodDefinition` and `PropertyDefinition` to include `is_static` flag

```rust
MethodDefinition {
    name: String,
    params: Vec<String>,
    body: Vec<AstNode>,
    is_async: bool,
    is_static: bool,
}

PropertyDefinition {
    name: String,
    value: Box<AstNode>,
    is_static: bool,
}
```

3. **Runtime Implementation**
   - Implement static method and property storage on the class itself
   - Ensure static methods and properties are accessible without instantiation

### 5. Inheritance (extends)

```javascript
// Target syntax
class CustomClient extends WebSocketClient {
  constructor(url, options) {
    super(url, [], options);
    this.customProperty = true;
  }
  
  customMethod() {
    // Method implementation
  }
}
```

#### Implementation Steps:

1. **Grammar Updates**
   - Ensure `class_declaration` handles the `extends` clause
   - Add support for `super` calls in constructors and methods

```pest
super_call = { "super" ~ "(" ~ (expression ~ ("," ~ expression)*)? ~ ")" }
super_method_call = { "super" ~ "." ~ identifier ~ "(" ~ (expression ~ ("," ~ expression)*)? ~ ")" }
```

2. **AST Updates**
   - Add `SuperCall` and `SuperMethodCall` nodes to the AST

```rust
SuperCall {
    args: Vec<AstNode>,
}

SuperMethodCall {
    method: String,
    args: Vec<AstNode>,
}
```

3. **Runtime Implementation**
   - Implement prototype chain for inheritance
   - Ensure parent constructor is called with `super()`
   - Implement method overriding and parent method access

### 6. Private Fields and Methods

```javascript
// Target syntax
class WebSocketClient {
  // Private field
  #socket = null;
  
  // Private method
  #connect() {
    // Private implementation
  }
  
  // Public method that uses private members
  connect() {
    this.#socket = new WebSocket(this.url);
    this.#connect();
  }
}
```

#### Implementation Steps:

1. **Grammar Updates**
   - Update `property_definition` and `method_definition` to handle private identifiers

```pest
private_identifier = @{ "#" ~ (ASCII_ALPHA | "_") ~ (ASCII_ALPHANUMERIC | "_")* }
method_definition = { static_modifier? ~ async_modifier? ~ (identifier | private_identifier) ~ "(" ~ parameter_list? ~ ")" ~ block }
property_definition = { static_modifier? ~ (identifier | private_identifier) ~ "=" ~ expression ~ ";" }
```

2. **AST Updates**
   - Update `MethodDefinition` and `PropertyDefinition` to handle private members

```rust
MethodDefinition {
    name: String,
    params: Vec<String>,
    body: Vec<AstNode>,
    is_async: bool,
    is_static: bool,
    is_private: bool,
}

PropertyDefinition {
    name: String,
    value: Box<AstNode>,
    is_static: bool,
    is_private: bool,
}
```

3. **Runtime Implementation**
   - Implement private field and method storage
   - Enforce privacy (only accessible within class methods)
   - Handle error cases for invalid access

### 7. Testing

Create comprehensive tests for:
- Class declaration and instantiation
- Constructor execution
- Instance methods and properties
- Static methods and properties
- Inheritance and method overriding
- Super calls
- Private fields and methods
- Edge cases (e.g., accessing private members, inheritance chains)

## Implementation Details

### Class Internal Structure

```rust
struct Class {
    name: String,
    parent: Option<Box<Class>>,
    constructor: Option<Function>,
    instance_methods: HashMap<String, Function>,
    static_methods: HashMap<String, Function>,
    instance_properties: HashMap<String, Value>,
    static_properties: HashMap<String, Value>,
    private_methods: HashMap<String, Function>,
    private_properties: HashMap<String, Value>,
}
```

### Instance Creation Process

1. Create a new object
2. Set up the prototype chain based on the class hierarchy
3. Initialize instance properties from class definition
4. Execute the constructor with the new object as `this`
5. Return the initialized object

### Method Resolution

1. Check if the method exists on the instance
2. If not, check the prototype chain
3. Bind `this` to the instance when calling the method

## Resources

- [MDN Classes Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Classes)
- [ECMAScript Class Specification](https://tc39.es/ecma262/#sec-class-definitions)
- [Private Class Features Proposal](https://github.com/tc39/proposal-private-methods)

## Timeline

1. **Week 1**: Implement basic class declaration and constructor syntax
2. **Week 2**: Implement instance methods and properties
3. **Week 3**: Implement static methods and properties
4. **Week 4**: Implement inheritance and super calls
5. **Week 5**: Implement private fields and methods
6. **Week 6**: Comprehensive testing and bug fixing