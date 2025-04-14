# SmashLang Language Features TODO

This document outlines language features that are used in SmashLang packages but may not be fully implemented in the SmashLang language itself. These features need to be implemented to ensure that all packages work correctly.

## Core Language Features

### Promises and Asynchronous Programming

- [ ] **Promise Implementation**
  - Complete implementation of Promise constructor and methods
  - Example: `new Promise((resolve, reject) => { ... })`
  - Status: Grammar has async/await keywords, but runtime implementation may be incomplete

- [ ] **Async/Await Runtime Support**
  - Full runtime support for async functions and await expressions
  - Example: `async function fetchData() { const result = await api.get(); }`
  - Status: Grammar supports syntax, but runtime implementation may be incomplete

- [ ] **Promise Methods**
  - `.then()`, `.catch()`, `.finally()`
  - `Promise.all()`, `Promise.race()`, `Promise.resolve()`, `Promise.reject()`
  - Status: Not implemented

### Classes and Object-Oriented Programming

- [ ] **Class Implementation**
  - Complete implementation of class syntax and behavior
  - Example: `class WebSocketClient { ... }`
  - Status: Not in grammar, needs to be added

- [ ] **Constructor Methods**
  - Initialize class instances
  - Example: `constructor(url, protocols = [], options = {}) { ... }`
  - Status: Not implemented

- [ ] **Instance Methods**
  - Methods that operate on class instances
  - Example: `connect() { ... }`
  - Status: Not implemented as part of class syntax

- [ ] **Static Methods**
  - Methods that belong to the class itself, not instances
  - Example: `static parse(str) { ... }`
  - Status: Not implemented

- [ ] **Private Methods and Properties**
  - Methods and properties that are not accessible outside the class
  - Example: `#privateMethod() { ... }`
  - Status: Not implemented

- [ ] **Class Inheritance**
  - Extend classes to inherit functionality
  - Example: `class CustomClient extends WebSocketClient { ... }`
  - Status: Not implemented

### Modern JavaScript Features

- [ ] **Arrow Function Enhancements**
  - Complete implementation of arrow functions with lexical `this`
  - Example: `(a, b) => a + b`
  - Status: Basic syntax in grammar, may need runtime improvements

- [ ] **Template Literal Enhancements**
  - Tagged templates and raw templates
  - Example: ``String.raw`This is a raw string: \n` ``
  - Status: Basic template strings in grammar, advanced features missing

- [ ] **Destructuring Assignment Enhancements**
  - Complete implementation of array and object destructuring
  - Example: `const { url, options = {} } = config`
  - Status: Basic destructuring in grammar, may need runtime improvements

- [ ] **Spread/Rest Operators**
  - Complete implementation for arrays, objects, and function parameters
  - Example: `function add(...numbers) { ... }`
  - Status: Rest element in grammar, may need runtime improvements

- [ ] **Default Parameters**
  - Default values for function parameters
  - Example: `function connect(url, options = {}) { ... }`
  - Status: Not fully implemented

- [ ] **Optional Chaining**
  - Access nested properties without checking each level
  - Example: `user?.address?.city`
  - Status: Not in grammar

- [ ] **Nullish Coalescing**
  - Provide fallbacks for null or undefined values
  - Example: `const port = options.port ?? 8080`
  - Status: Not in grammar

### Module System

- [ ] **ES Modules Enhancement**
  - Complete implementation of import/export functionality
  - Example: `import { WebSocketClient } from './websocket.js'`
  - Status: Basic import statement in grammar, needs enhancement

- [ ] **Default Exports**
  - Export a single value as the default
  - Example: `export default websocket`
  - Status: Not fully implemented

- [ ] **Named Exports**
  - Export multiple values from a module
  - Example: `export const STATES = { ... }`
  - Status: Not fully implemented

## Data Handling and Manipulation

### Error Handling

- [ ] **Enhanced Try/Catch**
  - Improved exception handling
  - Example: `try { ... } catch (error) { ... } finally { ... }`
  - Status: Basic syntax in grammar, may need runtime improvements

- [ ] **Custom Error Types**
  - Define application-specific error classes
  - Example: `class ConnectionError extends Error { ... }`
  - Status: Not implemented

### Data Structures and Collections

- [ ] **Map and Set**
  - Key-value collections and unique value collections
  - Example: `const users = new Map()`
  - Status: Not implemented

- [ ] **WeakMap and WeakSet**
  - Collections that don't prevent garbage collection
  - Example: `const cache = new WeakMap()`
  - Status: Not implemented

- [ ] **Enhanced Array Methods**
  - Complete implementation of functional array operations
  - Example: `args.reduce((sum, value) => sum + value, 0)`
  - Status: Basic array support exists, advanced methods may be incomplete

### Type System

- [ ] **Runtime Type Checking**
  - Improved type checking at runtime
  - Example: `if (typeof callback !== 'function') { ... }`
  - Status: Basic type checking exists, may need enhancement

- [ ] **Type Annotations**
  - Support for JSDoc-style type annotations
  - Example: `@param {string} url - WebSocket server URL`
  - Status: Not implemented

### Reflection and Introspection

- [ ] **Enhanced Type Operators**
  - Improved type checking at runtime
  - Example: `typeof data === 'object'`
  - Status: Basic support exists, may need enhancement

- [ ] **Instance Checking**
  - Check if an object is an instance of a class
  - Example: `data instanceof ArrayBuffer`
  - Status: Not fully implemented

- [ ] **Object Reflection Methods**
  - Methods to inspect object properties
  - Example: `Object.keys(options)`, `Object.entries(headers)`
  - Status: Not fully implemented

## Browser and Runtime Features

### Timers and Scheduling

- [ ] **setTimeout/clearTimeout**
  - Execute code after a delay
  - Example: `setTimeout(() => { ... }, 100)`
  - Status: Not implemented in core language

- [ ] **setInterval/clearInterval**
  - Execute code repeatedly
  - Example: `setInterval(() => { ... }, 1000)`
  - Status: Not implemented in core language

### JSON Handling

- [ ] **Enhanced JSON Support**
  - Complete implementation of JSON parsing and stringification
  - Example: `JSON.parse(message)`, `JSON.stringify(data)`
  - Status: Basic support may exist, needs enhancement

### Regular Expressions

- [ ] **Enhanced RegExp Support**
  - Complete implementation of regular expressions
  - Example: `const pattern = new RegExp('^ws://')`
  - Status: Basic syntax in grammar, may need runtime improvements

- [ ] **RegExp Methods**
  - Complete implementation of RegExp methods
  - Example: `pattern.test(url)`, `string.match(pattern)`
  - Status: Not fully implemented

### DOM and Browser APIs

- [ ] **Document Object Model**
  - API for interacting with HTML documents
  - Example: `document.createElement('div')`
  - Status: Not implemented

- [ ] **Browser Event System**
  - Register and handle browser events
  - Example: `element.addEventListener('click', handler)`
  - Status: Not implemented

- [ ] **Fetch API**
  - Make HTTP requests
  - Example: `fetch(url).then(response => response.json())`
  - Status: Not implemented

- [ ] **WebSocket API**
  - Native WebSocket support
  - Example: `new WebSocket('ws://example.com')`
  - Status: Not implemented

## Implementation Priority

1. **High Priority**
   - Promises and Async/Await (complete implementation)
   - Classes and OOP
   - Enhanced ES Modules
   - Complete Error Handling
   - JSON Handling

2. **Medium Priority**
   - Modern JavaScript Features (optional chaining, nullish coalescing)
   - Data Structures and Collections (Map, Set)
   - Enhanced Type System
   - Reflection and Introspection

3. **Lower Priority**
   - Timers and Scheduling
   - Enhanced Regular Expressions
   - DOM and Browser APIs

## Implementation Plan

1. **Phase 1: Core Language Features**
   - Complete promises and async/await implementation
   - Add full class support with inheritance
   - Enhance ES modules support

2. **Phase 2: Modern JavaScript Features**
   - Improve arrow functions
   - Enhance template literals
   - Complete destructuring, spread/rest operators
   - Add optional chaining and nullish coalescing

3. **Phase 3: Data Handling**
   - Enhance error handling
   - Add Map, Set, and complete array methods
   - Improve JSON utilities

4. **Phase 4: Browser and Runtime Features**
   - Add timers and scheduling
   - Enhance regular expressions
   - Add DOM and browser API support

## Testing Strategy

For each implemented feature:

1. Create unit tests that verify the feature works as expected
2. Create integration tests that use the feature in realistic scenarios
3. Test the feature with existing packages to ensure compatibility
4. Document any edge cases or limitations

## Documentation Plan

For each implemented feature:

1. Update language documentation with syntax and examples
2. Provide migration guides for existing code
3. Create tutorials for common use cases
4. Document any differences from JavaScript/TypeScript implementations