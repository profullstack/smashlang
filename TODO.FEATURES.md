# SmashLang Language Features TODO

This document outlines language features that are used in SmashLang packages but may not be fully implemented in the SmashLang language itself. After examining the codebase, it appears that many features have partial implementations, but may need completion or enhancement.

## Implementation Status Overview

| Feature | Status | Notes |
|---------|--------|-------|
| Async/Await | Partial | Grammar and AST support exists, runtime may need completion |
| Promises | Partial | Some implementation in codegen.rs.old |
| Classes | Missing | No grammar or AST support found |
| Arrow Functions | Implemented | Grammar and AST support exists |
| Template Literals | Implemented | Grammar and AST support exists |
| Destructuring | Partial | Basic support exists |
| Spread/Rest | Partial | AST node exists (SpreadElement) |
| ES Modules | Partial | Basic import statement exists |
| Map/Set | Missing | No dedicated implementation found |
| JSON Handling | Unknown | May exist but not confirmed |
| Regular Expressions | Implemented | Grammar and AST support exists |

## Core Language Features

### Promises and Asynchronous Programming

- [ ] **Promise Implementation Completion**
  - Complete the Promise constructor and methods
  - Example: `new Promise((resolve, reject) => { ... })`
  - Status: Partial implementation exists in codegen.rs.old

- [ ] **Async/Await Runtime Enhancement**
  - Ensure full runtime support for async functions and await expressions
  - Example: `async function fetchData() { const result = await api.get(); }`
  - Status: Grammar and AST support exists, runtime implementation may need completion

- [ ] **Promise Methods Completion**
  - Ensure all methods are fully implemented: `.then()`, `.catch()`, `.finally()`
  - Add static methods: `Promise.all()`, `Promise.race()`, `Promise.resolve()`, `Promise.reject()`
  - Status: Basic `.then()` and `.catch()` appear in codegen.rs.old

### Classes and Object-Oriented Programming

- [ ] **Class Syntax Implementation**
  - Add grammar and AST support for class declarations
  - Example: `class WebSocketClient { ... }`
  - Status: Not implemented

- [ ] **Constructor Methods**
  - Add support for class constructors
  - Example: `constructor(url, protocols = [], options = {}) { ... }`
  - Status: Not implemented

- [ ] **Instance and Static Methods**
  - Support for methods on class instances and static class methods
  - Example: `connect() { ... }` and `static parse(str) { ... }`
  - Status: Not implemented

- [ ] **Class Inheritance**
  - Support for extending classes
  - Example: `class CustomClient extends WebSocketClient { ... }`
  - Status: Not implemented

### Modern JavaScript Features

- [ ] **Arrow Function Enhancements**
  - Ensure proper lexical `this` binding and edge cases
  - Status: Basic implementation exists

- [ ] **Destructuring Enhancement**
  - Complete support for nested destructuring and default values
  - Example: `const { url, options: { timeout = 1000 } = {} } = config`
  - Status: Basic support exists

- [ ] **Spread/Rest Operator Enhancement**
  - Complete implementation for objects, arrays, and function parameters
  - Example: `const newObj = { ...obj1, ...obj2 }`
  - Status: Basic support exists (SpreadElement in AST)

- [ ] **Default Parameters**
  - Complete implementation for function parameters
  - Example: `function connect(url, options = {}) { ... }`
  - Status: May be partially implemented

- [ ] **Optional Chaining**
  - Add support for safely accessing nested properties
  - Example: `user?.address?.city`
  - Status: Not implemented

- [ ] **Nullish Coalescing**
  - Add support for providing fallbacks for null/undefined
  - Example: `const port = options.port ?? 8080`
  - Status: Not implemented

### Module System

- [ ] **ES Modules Enhancement**
  - Complete implementation of import/export functionality
  - Example: `import { WebSocketClient } from './websocket.js'`
  - Status: Basic import statement exists

- [ ] **Export Statements**
  - Add support for named and default exports
  - Example: `export const STATES = { ... }` and `export default websocket`
  - Status: Not implemented

## Data Handling and Manipulation

### Data Structures and Collections

- [ ] **Map and Set**
  - Add native implementations of Map and Set
  - Example: `const users = new Map()`
  - Status: Not implemented (though HashMap is used internally)

- [ ] **Enhanced Array Methods**
  - Ensure all modern array methods are implemented
  - Example: `array.flatMap()`, `array.find()`, etc.
  - Status: Some methods may be implemented

### Type System

- [ ] **Enhanced Type Checking**
  - Improve runtime type checking capabilities
  - Example: `typeof value === 'object' && value !== null`
  - Status: Basic type checking exists

### Reflection and Introspection

- [ ] **Object Reflection Methods**
  - Add support for Object.keys, Object.values, Object.entries
  - Example: `Object.entries(headers).forEach(([key, value]) => { ... })`
  - Status: May be partially implemented

## Browser and Runtime Features

### Timers and Scheduling

- [ ] **setTimeout/clearTimeout**
  - Complete implementation of timer functions
  - Example: `setTimeout(() => { ... }, 100)`
  - Status: Partial implementation in codegen.rs.old

- [ ] **setInterval/clearInterval**
  - Add support for recurring timers
  - Example: `setInterval(() => { ... }, 1000)`
  - Status: Not implemented

### JSON Handling

- [ ] **JSON Methods**
  - Ensure JSON.parse and JSON.stringify are fully implemented
  - Example: `JSON.parse(message)`, `JSON.stringify(data)`
  - Status: Unknown, may be partially implemented

### DOM and Browser APIs

- [ ] **Browser API Simulation**
  - Add simulated browser APIs for web development
  - Example: `document.createElement()`, `fetch()`, etc.
  - Status: Not implemented

## Implementation Plan

Based on the current state of implementation, here's a revised plan:

1. **Phase 1: Complete Core Async Support**
   - Finish Promise implementation
   - Ensure async/await works correctly in all cases
   - Add missing Promise methods

2. **Phase 2: Add Class Support**
   - Implement class syntax in grammar and parser
   - Add support for constructors and methods
   - Implement inheritance

3. **Phase 3: Enhance Modern JavaScript Features**
   - Complete destructuring implementation
   - Add optional chaining and nullish coalescing
   - Finish spread/rest operator support for all cases

4. **Phase 4: Complete Module System**
   - Add export statements
   - Enhance import functionality
   - Support module resolution

5. **Phase 5: Add Missing Data Structures**
   - Implement Map and Set
   - Complete array method implementations
   - Add Object reflection methods

6. **Phase 6: Add Runtime Features**
   - Complete timer implementations
   - Add JSON methods
   - Add browser API simulations as needed

## Testing Strategy

For each feature:

1. Create unit tests that verify the feature works as expected
2. Test edge cases and error conditions
3. Create integration tests with existing packages
4. Ensure backward compatibility

## Documentation

For each implemented feature:

1. Update language documentation
2. Provide examples of usage
3. Note any differences from JavaScript/TypeScript
4. Create tutorials for complex features