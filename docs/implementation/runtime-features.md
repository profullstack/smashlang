# Runtime Features Implementation for SmashLang

This document outlines the implementation plan for adding essential runtime features to SmashLang, focusing on timers, JSON handling, and other browser-like APIs.

## Overview

Modern JavaScript environments provide a set of runtime APIs that are essential for many applications. These include timers for scheduling code execution, JSON methods for data serialization, and various browser-like APIs. Implementing these features in SmashLang will enable more packages to work correctly without modification.

## Current Status

Based on code examination:
- There appears to be some code for setTimeout in codegen.rs.old, but it may be incomplete
- JSON handling may be partially implemented but not confirmed
- No implementation of browser-like APIs

## Implementation Goals

1. Implement Timers (setTimeout, clearTimeout, setInterval, clearInterval)
2. Implement JSON Methods (parse, stringify)
3. Implement Console API
4. Add basic Browser-like APIs
5. Add comprehensive tests

## Detailed Implementation Plan

### 1. Timers Implementation

Timers allow scheduling code execution after a delay or at regular intervals.

```javascript
// Target syntax
// One-time execution after delay
const timeoutId = setTimeout(() => {
  console.log('Executed after delay');
}, 1000);

// Cancel scheduled timeout
clearTimeout(timeoutId);

// Recurring execution at interval
const intervalId = setInterval(() => {
  console.log('Executed repeatedly');
}, 1000);

// Cancel interval
clearInterval(intervalId);

// Zero-delay execution (next event loop tick)
setTimeout(() => {
  console.log('Executed on next tick');
}, 0);
```

#### Implementation Steps:

1. **Runtime Implementation**
   - Create a timer system in the runtime
   - Implement task scheduling and execution
   - Support for cancellation

```rust
struct Timer {
    id: usize,
    callback: Value, // Function to call
    args: Vec<Value>, // Arguments to pass to the function
    delay: u64, // Delay in milliseconds
    is_interval: bool, // Whether this is an interval or timeout
    next_execution: u64, // Timestamp for next execution
}

struct TimerSystem {
    timers: HashMap<usize, Timer>,
    next_id: usize,
    current_time: u64,
}

impl TimerSystem {
    fn new() -> Self {
        Self {
            timers: HashMap::new(),
            next_id: 1,
            current_time: 0,
        }
    }
    
    fn set_timeout(&mut self, callback: Value, delay: u64, args: Vec<Value>) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        
        self.timers.insert(id, Timer {
            id,
            callback,
            args,
            delay,
            is_interval: false,
            next_execution: self.current_time + delay,
        });
        
        id
    }
    
    fn set_interval(&mut self, callback: Value, delay: u64, args: Vec<Value>) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        
        self.timers.insert(id, Timer {
            id,
            callback,
            args,
            delay,
            is_interval: true,
            next_execution: self.current_time + delay,
        });
        
        id
    }
    
    fn clear_timer(&mut self, id: usize) {
        self.timers.remove(&id);
    }
    
    fn update(&mut self, elapsed: u64) -> Vec<(Value, Vec<Value>)> {
        self.current_time += elapsed;
        let mut to_execute = Vec::new();
        let mut to_remove = Vec::new();
        
        for (id, timer) in &mut self.timers {
            if timer.next_execution <= self.current_time {
                to_execute.push((timer.callback.clone(), timer.args.clone()));
                
                if timer.is_interval {
                    // Schedule next execution
                    timer.next_execution = self.current_time + timer.delay;
                } else {
                    // One-time execution, remove after
                    to_remove.push(*id);
                }
            }
        }
        
        for id in to_remove {
            self.timers.remove(&id);
        }
        
        to_execute
    }
}
```

2. **Global Function Implementation**
   - Implement setTimeout, clearTimeout, setInterval, clearInterval as global functions

```rust
fn set_timeout(args: &[Value]) -> Result<Value, Error> {
    if args.len() < 2 {
        return Err(Error::TypeError("setTimeout requires at least 2 arguments".to_string()));
    }
    
    let callback = &args[0];
    if !callback.is_function() {
        return Err(Error::TypeError("First argument to setTimeout must be a function".to_string()));
    }
    
    let delay = match &args[1] {
        Value::Number(n) => *n as u64,
        _ => 0,
    };
    
    let timer_args = args[2..].to_vec();
    let id = TIMER_SYSTEM.lock().unwrap().set_timeout(callback.clone(), delay, timer_args);
    
    Ok(Value::Number(id as f64))
}

// Implement other timer functions similarly...
```

3. **Event Loop Integration**
   - Integrate timers with the event loop
   - Ensure timers are processed at appropriate times

```rust
fn run_event_loop() {
    let mut last_time = std::time::Instant::now();
    
    loop {
        let now = std::time::Instant::now();
        let elapsed = now.duration_since(last_time).as_millis() as u64;
        last_time = now;
        
        // Process timers
        let callbacks = TIMER_SYSTEM.lock().unwrap().update(elapsed);
        
        for (callback, args) in callbacks {
            // Execute callback
            if let Value::Function(func) = callback {
                let _ = func.call(Value::Undefined, &args);
            }
        }
        
        // Process other events (I/O, etc.)
        // ...
        
        // Sleep a bit to avoid busy waiting
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}
```

### 2. JSON Methods Implementation

JSON methods allow serializing JavaScript values to JSON strings and parsing JSON strings into JavaScript values.

```javascript
// Target syntax
// Stringify a value to JSON
const jsonString = JSON.stringify({ name: 'John', age: 30 });
const prettyJson = JSON.stringify({ name: 'John', age: 30 }, null, 2);
const filteredJson = JSON.stringify({ name: 'John', age: 30 }, ['name']);

// Parse JSON string to value
const value = JSON.parse('{"name":"John","age":30}');
const valueWithReviver = JSON.parse('{"date":"2023-01-01"}', (key, value) => {
  if (key === 'date') return new Date(value);
  return value;
});
```

#### Implementation Steps:

1. **JSON.stringify Implementation**
   - Implement serialization of JavaScript values to JSON strings
   - Support replacer function/array
   - Support space parameter for pretty printing

```rust
fn json_stringify(args: &[Value]) -> Result<Value, Error> {
    if args.is_empty() {
        return Ok(Value::String("undefined".to_string()));
    }
    
    let value = &args[0];
    let replacer = args.get(1);
    let space = args.get(2);
    
    // Implement serialization logic
    let json = serialize_to_json(value, replacer, space)?;
    
    Ok(Value::String(json))
}

fn serialize_to_json(value: &Value, replacer: Option<&Value>, space: Option<&Value>) -> Result<String, Error> {
    // Implement serialization based on value type
    match value {
        Value::Null => Ok("null".to_string()),
        Value::Boolean(b) => Ok(b.to_string()),
        Value::Number(n) => {
            if n.is_finite() {
                Ok(n.to_string())
            } else {
                Ok("null".to_string())
            }
        },
        Value::String(s) => Ok(format!("\"{}\"", escape_string(s))),
        Value::Array(arr) => {
            // Handle array serialization
            // ...
        },
        Value::Object(obj) => {
            // Handle object serialization
            // ...
        },
        _ => Ok("null".to_string()),
    }
}
```

2. **JSON.parse Implementation**
   - Implement parsing of JSON strings to JavaScript values
   - Support reviver function

```rust
fn json_parse(args: &[Value]) -> Result<Value, Error> {
    if args.is_empty() {
        return Err(Error::SyntaxError("JSON.parse requires at least 1 argument".to_string()));
    }
    
    let text = match &args[0] {
        Value::String(s) => s,
        _ => return Err(Error::SyntaxError("JSON.parse requires a string argument".to_string())),
    };
    
    let reviver = args.get(1);
    
    // Implement parsing logic
    let value = parse_json(text, reviver)?;
    
    Ok(value)
}

fn parse_json(text: &str, reviver: Option<&Value>) -> Result<Value, Error> {
    // Implement JSON parsing
    // ...
    
    // Apply reviver if provided
    if let Some(Value::Function(reviver_fn)) = reviver {
        // Apply reviver to parsed value
        // ...
    }
    
    // Return parsed value
    // ...
}
```

### 3. Console API Implementation

The console API provides logging and debugging functionality.

```javascript
// Target syntax
console.log('Hello, world!');
console.error('Something went wrong');
console.warn('Warning message');
console.info('Informational message');
console.debug('Debug message');

console.table([
  { name: 'John', age: 30 },
  { name: 'Jane', age: 25 }
]);

console.time('operation');
// Some operation
console.timeEnd('operation');

console.assert(false, 'Assertion failed');
console.trace('Trace message');
```

#### Implementation Steps:

1. **Console Object Implementation**
   - Create a global console object
   - Implement various logging methods

```rust
fn create_console_object() -> Value {
    let mut console = HashMap::new();
    
    console.insert("log".to_string(), Value::Function(Function::new(
        Some("log".to_string()),
        vec!["...args".to_string()],
        console_log,
    )));
    
    console.insert("error".to_string(), Value::Function(Function::new(
        Some("error".to_string()),
        vec!["...args".to_string()],
        console_error,
    )));
    
    // Implement other console methods...
    
    Value::Object(console)
}

fn console_log(this: Value, args: &[Value]) -> Result<Value, Error> {
    let mut output = String::new();
    
    for (i, arg) in args.iter().enumerate() {
        if i > 0 {
            output.push(' ');
        }
        output.push_str(&format!("{}", arg));
    }
    
    println!("{}", output);
    
    Ok(Value::Undefined)
}

// Implement other console methods similarly...
```

### 4. Basic Browser-like APIs

Implement basic browser-like APIs to support web applications.

```javascript
// Target syntax
// Fetch API
fetch('https://api.example.com/data')
  .then(response => response.json())
  .then(data => console.log(data))
  .catch(error => console.error('Error:', error));

// localStorage
localStorage.setItem('key', 'value');
const value = localStorage.getItem('key');
localStorage.removeItem('key');
localStorage.clear();

// URL and URLSearchParams
const url = new URL('https://example.com/path?query=value');
console.log(url.hostname); // example.com
console.log(url.pathname); // /path
console.log(url.searchParams.get('query')); // value
```

#### Implementation Steps:

1. **Fetch API Implementation**
   - Implement a basic fetch function that returns a Promise
   - Support for making HTTP requests

```rust
fn fetch(args: &[Value]) -> Result<Value, Error> {
    if args.is_empty() {
        return Err(Error::TypeError("fetch requires at least 1 argument".to_string()));
    }
    
    let url = match &args[0] {
        Value::String(s) => s,
        _ => return Err(Error::TypeError("fetch requires a string URL".to_string())),
    };
    
    let options = args.get(1);
    
    // Create a Promise
    let promise = create_promise();
    
    // In a real implementation, this would make an actual HTTP request
    // For this example, we'll simulate it
    std::thread::spawn(move || {
        // Simulate network request
        std::thread::sleep(std::time::Duration::from_millis(100));
        
        // Create a Response object
        let response = create_response_object(url, 200, "OK");
        
        // Resolve the Promise with the Response
        resolve_promise(promise, response);
    });
    
    Ok(promise)
}

fn create_response_object(url: &str, status: u16, status_text: &str) -> Value {
    // Create a Response object
    // ...
}
```

2. **localStorage Implementation**
   - Implement a global localStorage object
   - Support for storing and retrieving data

```rust
fn create_local_storage_object() -> Value {
    let mut local_storage = HashMap::new();
    
    local_storage.insert("setItem".to_string(), Value::Function(Function::new(
        Some("setItem".to_string()),
        vec!["key".to_string(), "value".to_string()],
        local_storage_set_item,
    )));
    
    // Implement other localStorage methods...
    
    Value::Object(local_storage)
}

fn local_storage_set_item(this: Value, args: &[Value]) -> Result<Value, Error> {
    if args.len() < 2 {
        return Err(Error::TypeError("setItem requires 2 arguments".to_string()));
    }
    
    let key = match &args[0] {
        Value::String(s) => s,
        _ => return Err(Error::TypeError("key must be a string".to_string())),
    };
    
    let value = match &args[1] {
        Value::String(s) => s.clone(),
        _ => format!("{}", args[1]),
    };
    
    // In a real implementation, this would store the data persistently
    // For this example, we'll use a global HashMap
    LOCAL_STORAGE.lock().unwrap().insert(key.clone(), value);
    
    Ok(Value::Undefined)
}

// Implement other localStorage methods similarly...
```

3. **URL and URLSearchParams Implementation**
   - Implement URL and URLSearchParams classes
   - Support for parsing and manipulating URLs

```rust
fn url_constructor(args: &[Value]) -> Result<Value, Error> {
    if args.is_empty() {
        return Err(Error::TypeError("URL constructor requires at least 1 argument".to_string()));
    }
    
    let url_str = match &args[0] {
        Value::String(s) => s,
        _ => return Err(Error::TypeError("URL constructor requires a string URL".to_string())),
    };
    
    let base_url = args.get(1).and_then(|v| {
        if let Value::String(s) = v {
            Some(s.as_str())
        } else {
            None
        }
    });
    
    // Parse the URL
    let parsed_url = parse_url(url_str, base_url)?;
    
    // Create a URL object
    let mut url_obj = HashMap::new();
    
    url_obj.insert("href".to_string(), Value::String(parsed_url.href));
    url_obj.insert("protocol".to_string(), Value::String(parsed_url.protocol));
    url_obj.insert("hostname".to_string(), Value::String(parsed_url.hostname));
    url_obj.insert("pathname".to_string(), Value::String(parsed_url.pathname));
    // Add other URL properties...
    
    // Create searchParams property
    url_obj.insert("searchParams".to_string(), create_url_search_params(parsed_url.search));
    
    Ok(Value::Object(url_obj))
}

// Implement URL parsing and URLSearchParams...
```

### 5. Testing

Create comprehensive tests for:
- Timer functionality (setTimeout, setInterval)
- JSON methods (parse, stringify)
- Console API
- Browser-like APIs (fetch, localStorage, URL)
- Edge cases and error conditions

## Implementation Details

### Timer Implementation Details

Timers require integration with an event loop to function correctly:

```rust
// Global timer system
lazy_static! {
    static ref TIMER_SYSTEM: Mutex<TimerSystem> = Mutex::new(TimerSystem::new());
}

// Event loop
fn start_event_loop() {
    std::thread::spawn(|| {
        run_event_loop();
    });
}
```

### JSON Serialization Details

JSON serialization needs to handle circular references and special values:

```rust
fn serialize_object(obj: &HashMap<String, Value>, replacer: Option<&Value>, space: Option<&Value>, visited: &mut HashSet<usize>) -> Result<String, Error> {
    // Check for circular references
    let obj_ptr = obj as *const _ as usize;
    if visited.contains(&obj_ptr) {
        return Err(Error::TypeError("Converting circular structure to JSON".to_string()));
    }
    visited.insert(obj_ptr);
    
    // Implement object serialization
    // ...
    
    visited.remove(&obj_ptr);
    Ok(result)
}
```

## Resources

- [MDN setTimeout Documentation](https://developer.mozilla.org/en-US/docs/Web/API/setTimeout)
- [MDN JSON Documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/JSON)
- [MDN Console API Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Console)
- [MDN Fetch API Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API)
- [MDN localStorage Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage)
- [MDN URL API Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URL)

## Timeline

1. **Week 1**: Implement timer functions (setTimeout, clearTimeout, setInterval, clearInterval)
2. **Week 2**: Implement JSON methods (parse, stringify)
3. **Week 3**: Implement Console API
4. **Week 4**: Implement basic Fetch API
5. **Week 5**: Implement localStorage and URL APIs
6. **Week 6**: Comprehensive testing and bug fixing