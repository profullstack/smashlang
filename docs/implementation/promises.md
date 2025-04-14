# Promise Implementation for SmashLang

This document outlines the implementation plan for adding full Promise support to SmashLang.

## Overview

Promises are a core feature of modern JavaScript that allow for better handling of asynchronous operations. A Promise represents a value that may not be available yet but will be resolved at some point in the future. Promises are essential for features like async/await and are heavily used in network operations, file I/O, and other asynchronous tasks.

## Current Status

Based on code examination:
- Grammar includes `async` and `await` keywords
- AST has nodes for `AwaitExpr` and `is_async` flags for functions
- There's partial implementation in `codegen.rs.old` for Promise creation and methods like `.then()` and `.catch()`
- The runtime implementation may be incomplete

## Implementation Goals

1. Implement the Promise constructor
2. Implement Promise instance methods (then, catch, finally)
3. Implement Promise static methods (all, race, resolve, reject, allSettled, any)
4. Ensure proper integration with async/await
5. Add comprehensive tests

## Detailed Implementation Plan

### 1. Promise Constructor

```javascript
// Target syntax
new Promise((resolve, reject) => {
  // Asynchronous operation
  if (success) {
    resolve(value);
  } else {
    reject(error);
  }
});
```

#### Implementation Steps:

1. **Grammar Updates**
   - Ensure `Promise` is recognized as a constructor in the grammar
   - Update the `new_expression` rule if needed

2. **AST Updates**
   - Add or update `NewExpr` node to handle Promise constructor specifically
   - Ensure the executor function is properly parsed

3. **Runtime Implementation**
   - Create a `Promise` class/struct in the runtime
   - Implement the constructor that takes an executor function
   - Implement internal state management (pending, fulfilled, rejected)
   - Implement resolve and reject functions

### 2. Promise Instance Methods

```javascript
// Target syntax
promise
  .then(value => {
    // Handle fulfilled state
    return transformedValue;
  })
  .catch(error => {
    // Handle rejected state
    return recoveryValue;
  })
  .finally(() => {
    // Always executed
    cleanup();
  });
```

#### Implementation Steps:

1. **Grammar Updates**
   - No specific grammar updates needed for method calls

2. **Runtime Implementation**
   - Implement `.then(onFulfilled, onRejected)` method
     - Returns a new Promise
     - Handles both synchronous and asynchronous onFulfilled/onRejected functions
     - Implements proper promise chaining
   - Implement `.catch(onRejected)` method
     - Syntactic sugar for `.then(undefined, onRejected)`
   - Implement `.finally(onFinally)` method
     - Executes callback regardless of promise state
     - Passes through the result/error to the next promise

### 3. Promise Static Methods

```javascript
// Target syntax
Promise.resolve(value);
Promise.reject(error);
Promise.all([promise1, promise2]);
Promise.race([promise1, promise2]);
Promise.allSettled([promise1, promise2]);
Promise.any([promise1, promise2]);
```

#### Implementation Steps:

1. **Runtime Implementation**
   - Implement `Promise.resolve(value)` 
     - Returns a promise that is resolved with the given value
     - Handles the case when value is a promise (returns the promise)
   - Implement `Promise.reject(reason)`
     - Returns a promise that is rejected with the given reason
   - Implement `Promise.all(iterable)`
     - Returns a promise that resolves when all promises in the iterable resolve
     - Rejects if any promise rejects
   - Implement `Promise.race(iterable)`
     - Returns a promise that resolves/rejects as soon as one promise in the iterable resolves/rejects
   - Implement `Promise.allSettled(iterable)`
     - Returns a promise that resolves when all promises have settled
     - Result is an array of objects with status and value/reason
   - Implement `Promise.any(iterable)`
     - Returns a promise that resolves as soon as one promise in the iterable resolves
     - Rejects only if all promises reject

### 4. Async/Await Integration

```javascript
// Target syntax
async function fetchData() {
  try {
    const response = await fetch(url);
    const data = await response.json();
    return data;
  } catch (error) {
    console.error("Error fetching data:", error);
    throw error;
  }
}
```

#### Implementation Steps:

1. **Runtime Implementation**
   - Ensure async functions return promises
   - Implement await expression to properly suspend and resume execution
   - Handle exceptions in async functions (convert to rejected promises)
   - Implement proper promise unwrapping for await expressions

### 5. Testing

Create comprehensive tests for:
- Promise constructor
- Promise state transitions
- Promise chaining
- Error handling
- Each static method
- Async/await with promises
- Edge cases (e.g., awaiting non-promises, rejections in async functions)

## Implementation Details

### Promise States

A Promise can be in one of three states:
- **Pending**: Initial state, neither fulfilled nor rejected
- **Fulfilled**: The operation completed successfully
- **Rejected**: The operation failed

### Promise Internal Structure

```rust
struct Promise {
    state: PromiseState,
    value: Option<Value>,
    reason: Option<Value>,
    on_fulfill: Vec<(FunctionValue, Option<Promise>)>,
    on_reject: Vec<(FunctionValue, Option<Promise>)>,
}

enum PromiseState {
    Pending,
    Fulfilled,
    Rejected,
}
```

### Promise Resolution Procedure

The Promise Resolution Procedure is a complex algorithm that handles different cases when resolving a promise:
1. If the value is the same promise, reject with TypeError
2. If the value is another promise, adopt its state
3. If the value is an object or function with a `then` method, call it with resolve and reject
4. Otherwise, fulfill the promise with the value

## Resources

- [Promises/A+ Specification](https://promisesaplus.com/)
- [MDN Promise Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise)
- [ECMAScript Promise Specification](https://tc39.es/ecma262/#sec-promise-objects)

## Timeline

1. **Week 1**: Implement Promise constructor and basic state management
2. **Week 2**: Implement then, catch, and finally methods
3. **Week 3**: Implement static methods (resolve, reject, all, race)
4. **Week 4**: Implement remaining static methods and ensure async/await integration
5. **Week 5**: Comprehensive testing and bug fixing