# Data Structures Implementation for SmashLang

This document outlines the implementation plan for adding modern JavaScript data structures to SmashLang, specifically Map, Set, WeakMap, and WeakSet.

## Overview

Modern JavaScript includes several built-in data structures beyond arrays and objects that provide specialized functionality for different use cases. These data structures are essential for efficient data manipulation and are widely used in modern applications.

## Current Status

Based on code examination:
- Basic arrays and objects are implemented
- HashMap is used internally but not exposed as a language feature
- No implementation of Map, Set, WeakMap, or WeakSet

## Implementation Goals

1. Implement Map
2. Implement Set
3. Implement WeakMap
4. Implement WeakSet
5. Add comprehensive tests

## Detailed Implementation Plan

### 1. Map Implementation

Map is a collection of key-value pairs where keys can be of any type (unlike objects, which only allow string and symbol keys).

```javascript
// Target syntax
const userMap = new Map();
userMap.set('john', { name: 'John', age: 30 });
userMap.set('jane', { name: 'Jane', age: 25 });

const user = userMap.get('john');
const hasUser = userMap.has('john');
userMap.delete('john');
const size = userMap.size;

// Iteration
for (const [key, value] of userMap) {
  console.log(key, value);
}

// Methods
const keys = [...userMap.keys()];
const values = [...userMap.values()];
const entries = [...userMap.entries()];
userMap.forEach((value, key) => {
  console.log(key, value);
});

// Constructor with initial entries
const initialMap = new Map([
  ['key1', 'value1'],
  ['key2', 'value2']
]);

// Clear all entries
userMap.clear();
```

#### Implementation Steps:

1. **Runtime Implementation**
   - Create a `Map` class in the runtime
   - Implement internal storage using a HashMap
   - Implement key equality based on value equality, not just reference equality
   - Implement all methods and properties

```rust
struct MapObject {
    entries: HashMap<Value, Value>,
}

impl MapObject {
    fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }
    
    fn set(&mut self, key: Value, value: Value) -> Value {
        self.entries.insert(key, value);
        Value::Map(self.clone()) // Return the map for chaining
    }
    
    fn get(&self, key: &Value) -> Value {
        match self.entries.get(key) {
            Some(value) => value.clone(),
            None => Value::Undefined,
        }
    }
    
    // Implement other methods...
}
```

2. **Constructor Implementation**
   - Implement the `Map` constructor
   - Handle initialization with an iterable

```rust
fn map_constructor(args: &[Value]) -> Result<Value, Error> {
    let map = MapObject::new();
    
    if !args.is_empty() {
        let iterable = &args[0];
        // Initialize from iterable
        if let Value::Array(items) = iterable {
            for item in items {
                if let Value::Array(entry) = item {
                    if entry.len() >= 2 {
                        map.set(entry[0].clone(), entry[1].clone());
                    }
                }
            }
        }
    }
    
    Ok(Value::Map(map))
}
```

3. **Method Implementation**
   - Implement all Map methods
   - Ensure proper behavior for edge cases

```rust
fn map_set(this: &Value, args: &[Value]) -> Result<Value, Error> {
    if let Value::Map(map) = this {
        if args.len() >= 2 {
            let key = &args[0];
            let value = &args[1];
            return Ok(map.set(key.clone(), value.clone()));
        }
    }
    Err(Error::TypeError("set called on non-Map object".to_string()))
}

// Implement other methods similarly...
```

4. **Iterator Implementation**
   - Implement iterator protocol for Map
   - Support for keys(), values(), entries(), and for...of loops

```rust
fn map_entries(this: &Value, _args: &[Value]) -> Result<Value, Error> {
    if let Value::Map(map) = this {
        let entries: Vec<Value> = map.entries.iter()
            .map(|(k, v)| {
                let entry = vec![k.clone(), v.clone()];
                Value::Array(entry)
            })
            .collect();
        return Ok(Value::Iterator(entries));
    }
    Err(Error::TypeError("entries called on non-Map object".to_string()))
}

// Implement other iterator methods similarly...
```

### 2. Set Implementation

Set is a collection of unique values of any type.

```javascript
// Target syntax
const uniqueUsers = new Set();
uniqueUsers.add('john');
uniqueUsers.add('jane');
uniqueUsers.add('john'); // Ignored, already exists

const hasUser = uniqueUsers.has('john');
uniqueUsers.delete('john');
const size = uniqueUsers.size;

// Iteration
for (const user of uniqueUsers) {
  console.log(user);
}

// Methods
const values = [...uniqueUsers.values()];
const entries = [...uniqueUsers.entries()]; // [value, value] pairs
uniqueUsers.forEach(value => {
  console.log(value);
});

// Constructor with initial values
const initialSet = new Set(['value1', 'value2', 'value1']);

// Clear all entries
uniqueUsers.clear();
```

#### Implementation Steps:

1. **Runtime Implementation**
   - Create a `Set` class in the runtime
   - Implement internal storage using a HashMap (with values as keys and dummy values)
   - Implement all methods and properties

```rust
struct SetObject {
    values: HashMap<Value, ()>,
}

impl SetObject {
    fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }
    
    fn add(&mut self, value: Value) -> Value {
        self.values.insert(value, ());
        Value::Set(self.clone()) // Return the set for chaining
    }
    
    fn has(&self, value: &Value) -> bool {
        self.values.contains_key(value)
    }
    
    // Implement other methods...
}
```

2. **Constructor Implementation**
   - Implement the `Set` constructor
   - Handle initialization with an iterable

```rust
fn set_constructor(args: &[Value]) -> Result<Value, Error> {
    let set = SetObject::new();
    
    if !args.is_empty() {
        let iterable = &args[0];
        // Initialize from iterable
        if let Value::Array(items) = iterable {
            for item in items {
                set.add(item.clone());
            }
        }
    }
    
    Ok(Value::Set(set))
}
```

3. **Method Implementation**
   - Implement all Set methods
   - Ensure proper behavior for edge cases

```rust
fn set_add(this: &Value, args: &[Value]) -> Result<Value, Error> {
    if let Value::Set(set) = this {
        if !args.is_empty() {
            let value = &args[0];
            return Ok(set.add(value.clone()));
        }
    }
    Err(Error::TypeError("add called on non-Set object".to_string()))
}

// Implement other methods similarly...
```

4. **Iterator Implementation**
   - Implement iterator protocol for Set
   - Support for values(), entries(), and for...of loops

```rust
fn set_values(this: &Value, _args: &[Value]) -> Result<Value, Error> {
    if let Value::Set(set) = this {
        let values: Vec<Value> = set.values.keys().cloned().collect();
        return Ok(Value::Iterator(values));
    }
    Err(Error::TypeError("values called on non-Set object".to_string()))
}

// Implement other iterator methods similarly...
```

### 3. WeakMap Implementation

WeakMap is a collection of key-value pairs where the keys are objects and are held "weakly", meaning they don't prevent garbage collection.

```javascript
// Target syntax
const weakMap = new WeakMap();
const obj1 = {};
const obj2 = {};

weakMap.set(obj1, 'value1');
weakMap.set(obj2, 'value2');

const value = weakMap.get(obj1);
const hasObj = weakMap.has(obj1);
weakMap.delete(obj1);
```

#### Implementation Steps:

1. **Runtime Implementation**
   - Create a `WeakMap` class in the runtime
   - Implement internal storage using a specialized weak reference collection
   - Ensure keys are only objects
   - Implement all methods

```rust
struct WeakMapObject {
    // This would require a specialized implementation with weak references
    // For simplicity, we'll use a regular HashMap here, but in a real implementation
    // this would need to use weak references
    entries: HashMap<Value, Value>,
}

impl WeakMapObject {
    fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }
    
    fn set(&mut self, key: Value, value: Value) -> Result<Value, Error> {
        // Ensure key is an object
        if !key.is_object() {
            return Err(Error::TypeError("WeakMap key must be an object".to_string()));
        }
        
        self.entries.insert(key, value);
        Ok(Value::WeakMap(self.clone())) // Return the map for chaining
    }
    
    // Implement other methods...
}
```

2. **Constructor Implementation**
   - Implement the `WeakMap` constructor
   - Handle initialization with an iterable
   - Validate that keys are objects

3. **Method Implementation**
   - Implement all WeakMap methods
   - Ensure proper behavior for edge cases
   - Validate that keys are objects

### 4. WeakSet Implementation

WeakSet is a collection of objects that are held "weakly", meaning they don't prevent garbage collection.

```javascript
// Target syntax
const weakSet = new WeakSet();
const obj1 = {};
const obj2 = {};

weakSet.add(obj1);
weakSet.add(obj2);

const hasObj = weakSet.has(obj1);
weakSet.delete(obj1);
```

#### Implementation Steps:

1. **Runtime Implementation**
   - Create a `WeakSet` class in the runtime
   - Implement internal storage using a specialized weak reference collection
   - Ensure values are only objects
   - Implement all methods

```rust
struct WeakSetObject {
    // This would require a specialized implementation with weak references
    // For simplicity, we'll use a regular HashMap here, but in a real implementation
    // this would need to use weak references
    values: HashMap<Value, ()>,
}

impl WeakSetObject {
    fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }
    
    fn add(&mut self, value: Value) -> Result<Value, Error> {
        // Ensure value is an object
        if !value.is_object() {
            return Err(Error::TypeError("WeakSet value must be an object".to_string()));
        }
        
        self.values.insert(value, ());
        Ok(Value::WeakSet(self.clone())) // Return the set for chaining
    }
    
    // Implement other methods...
}
```

2. **Constructor Implementation**
   - Implement the `WeakSet` constructor
   - Handle initialization with an iterable
   - Validate that values are objects

3. **Method Implementation**
   - Implement all WeakSet methods
   - Ensure proper behavior for edge cases
   - Validate that values are objects

### 5. Testing

Create comprehensive tests for:
- Map creation and basic operations
- Set creation and basic operations
- WeakMap and WeakSet behavior
- Edge cases (e.g., non-object keys in WeakMap)
- Iterator behavior
- Performance with large collections

## Implementation Details

### Value Equality for Map and Set

Map and Set use value equality for keys/values, which is different from reference equality:

```rust
fn values_equal(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Number(a), Value::Number(b)) => a == b,
        (Value::String(a), Value::String(b)) => a == b,
        (Value::Boolean(a), Value::Boolean(b)) => a == b,
        (Value::Null, Value::Null) => true,
        (Value::Undefined, Value::Undefined) => true,
        // For objects, arrays, and functions, use reference equality
        _ => std::ptr::eq(a, b),
    }
}
```

### Weak References

For WeakMap and WeakSet, we need to implement weak references, which don't prevent garbage collection:

```rust
// This is a simplified example. In practice, this would require integration
// with the garbage collector and use of weak pointers.
struct WeakRef<T> {
    ptr: *const T,
}

impl<T> WeakRef<T> {
    fn new(value: &T) -> Self {
        Self {
            ptr: value as *const T,
        }
    }
    
    fn upgrade(&self) -> Option<&T> {
        // Check if the object still exists
        // This is simplified; in practice, this would involve checking
        // with the garbage collector
        if self.ptr.is_null() {
            None
        } else {
            unsafe { Some(&*self.ptr) }
        }
    }
}
```

### Iterator Protocol

To support iteration, we need to implement the iterator protocol:

```rust
struct Iterator {
    values: Vec<Value>,
    index: usize,
}

impl Iterator {
    fn new(values: Vec<Value>) -> Self {
        Self {
            values,
            index: 0,
        }
    }
    
    fn next(&mut self) -> Value {
        if self.index < self.values.len() {
            let value = self.values[self.index].clone();
            self.index += 1;
            Value::Object({
                let mut obj = HashMap::new();
                obj.insert("value".to_string(), value);
                obj.insert("done".to_string(), Value::Boolean(false));
                obj
            })
        } else {
            Value::Object({
                let mut obj = HashMap::new();
                obj.insert("done".to_string(), Value::Boolean(true));
                obj
            })
        }
    }
}
```

## Resources

- [MDN Map Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map)
- [MDN Set Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set)
- [MDN WeakMap Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WeakMap)
- [MDN WeakSet Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WeakSet)
- [ECMAScript Map Specification](https://tc39.es/ecma262/#sec-map-objects)
- [ECMAScript Set Specification](https://tc39.es/ecma262/#sec-set-objects)

## Timeline

1. **Week 1**: Implement Map class and methods
2. **Week 2**: Implement Set class and methods
3. **Week 3**: Implement WeakMap class and methods
4. **Week 4**: Implement WeakSet class and methods
5. **Week 5**: Implement iterator protocol for collections
6. **Week 6**: Comprehensive testing and bug fixing