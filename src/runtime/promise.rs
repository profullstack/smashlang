use std::cell::RefCell;
use std::collections::VecDeque;
use std::collections::HashMap;
use std::rc::Rc;
use crate::interpreter::{Value, Function, Environment};

/// Represents the state of a Promise
#[derive(Debug, Clone, PartialEq)]
pub enum PromiseState {
    Pending,
    Fulfilled,
    Rejected,
}

/// Represents a Promise in the SmashLang runtime
#[derive(Debug, Clone)]
pub struct Promise {
    /// Current state of the promise
    state: RefCell<PromiseState>,
    
    /// Value when fulfilled
    value: RefCell<Option<Value>>,
    
    /// Reason when rejected
    reason: RefCell<Option<Value>>,
    
    /// Callbacks to be called when the promise is fulfilled
    on_fulfill: RefCell<VecDeque<(Function, Rc<Promise>)>>,
    
    /// Callbacks to be called when the promise is rejected
    on_reject: RefCell<VecDeque<(Function, Rc<Promise>)>>,
}

impl Promise {
    /// Create a new pending Promise
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            state: RefCell::new(PromiseState::Pending),
            value: RefCell::new(None),
            reason: RefCell::new(None),
            on_fulfill: RefCell::new(VecDeque::new()),
            on_reject: RefCell::new(VecDeque::new()),
        })
    }
    
    /// Create a new Promise with an executor function
    pub fn with_executor(executor: &Function, env: &Environment) -> Rc<Self> {
        let promise = Self::new();
        
        // Create resolve and reject functions
        let resolve_promise = promise.clone();
        let resolve = Function::new_native(
            Some("resolve".to_string()),
            vec!["value".to_string()],
            move |_this, args, _env| {
                if let Some(value) = args.first() {
                    resolve_promise.resolve(value.clone());
                } else {
                    resolve_promise.resolve(Value::Undefined);
                }
                Ok(Value::Undefined)
            },
        );
        
        let reject_promise = promise.clone();
        let reject = Function::new_native(
            Some("reject".to_string()),
            vec!["reason".to_string()],
            move |_this, args, _env| {
                if let Some(reason) = args.first() {
                    reject_promise.reject(reason.clone());
                } else {
                    reject_promise.reject(Value::Undefined);
                }
                Ok(Value::Undefined)
            },
        );
        
        // Call the executor function
        let _ = executor.call(Value::Undefined, &[Value::Function(resolve), Value::Function(reject)], env);
        
        promise
    }
    
    /// Get the current state of the promise
    pub fn state(&self) -> PromiseState {
        self.state.borrow().clone()
    }
    
    /// Resolve the promise with a value
    pub fn resolve(&self, value: Value) {
        // Only resolve if the promise is still pending
        if *self.state.borrow() != PromiseState::Pending {
            return;
        }
        
        // Handle the case where the value is a Promise
        if let Value::Promise(other_promise) = &value {
            // If the value is a Promise, adopt its state
            let other_state = other_promise.state();
            
            match other_state {
                PromiseState::Pending => {
                    // Set up callbacks to resolve/reject this Promise when the other Promise settles
                    let this_promise_fulfilled = self.clone();
                    let this_promise_rejected = self.clone();
                    
                    // Create a callback for when the other promise is fulfilled
                    let on_fulfilled = Function::new_native(
                        None,
                        vec!["value".to_string()],
                        move |_this, args, _env| {
                            if let Some(value) = args.first() {
                                this_promise_fulfilled.resolve(value.clone());
                            } else {
                                this_promise_fulfilled.resolve(Value::Undefined);
                            }
                            Ok(Value::Undefined)
                        },
                    );
                    
                    // Create a callback for when the other promise is rejected
                    let on_rejected = Function::new_native(
                        None,
                        vec!["reason".to_string()],
                        move |_this, args, _env| {
                            if let Some(reason) = args.first() {
                                this_promise_rejected.reject(reason.clone());
                            } else {
                                this_promise_rejected.reject(Value::Undefined);
                            }
                            Ok(Value::Undefined)
                        },
                    );
                    
                    other_promise.then(on_fulfilled, on_rejected);
                },
                PromiseState::Fulfilled => {
                    // Resolve with the other Promise's value
                    if let Some(value) = other_promise.value.borrow().clone() {
                        self.resolve(value);
                    } else {
                        self.resolve(Value::Undefined);
                    }
                },
                PromiseState::Rejected => {
                    // Reject with the other Promise's reason
                    if let Some(reason) = other_promise.reason.borrow().clone() {
                        self.reject(reason);
                    } else {
                        self.reject(Value::Undefined);
                    }
                },
            }
            
            return;
        }
        
        // Set the state to fulfilled and store the value
        *self.state.borrow_mut() = PromiseState::Fulfilled;
        *self.value.borrow_mut() = Some(value);
        
        // Call all the fulfill callbacks
        let callbacks = self.on_fulfill.borrow_mut().drain(..).collect::<Vec<_>>();
        for (callback, promise) in callbacks {
            if let Some(value) = self.value.borrow().clone() {
                let result = callback.call(Value::Undefined, &[value], &Environment::new());
                
                match result {
                    Ok(result_value) => {
                        promise.resolve(result_value);
                    },
                    Err(err) => {
                        promise.reject(Value::String(err));
                    },
                }
            }
        }
    }
    
    /// Reject the promise with a reason
    pub fn reject(&self, reason: Value) {
        // Only reject if the promise is still pending
        if *self.state.borrow() != PromiseState::Pending {
            return;
        }
        
        // Set the state to rejected and store the reason
        *self.state.borrow_mut() = PromiseState::Rejected;
        *self.reason.borrow_mut() = Some(reason);
        
        // Call all the reject callbacks
        let callbacks = self.on_reject.borrow_mut().drain(..).collect::<Vec<_>>();
        for (callback, promise) in callbacks {
            if let Some(reason) = self.reason.borrow().clone() {
                let result = callback.call(Value::Undefined, &[reason], &Environment::new());
                
                match result {
                    Ok(result_value) => {
                        promise.resolve(result_value);
                    },
                    Err(err) => {
                        promise.reject(Value::String(err));
                    },
                }
            }
        }
    }
    
    /// Add callbacks to be called when the promise is settled
    pub fn then(&self, on_fulfilled: Function, on_rejected: Function) -> Rc<Promise> {
        let promise = Promise::new();
        
        match *self.state.borrow() {
            PromiseState::Pending => {
                // Add callbacks to be called when the promise is settled
                self.on_fulfill.borrow_mut().push_back((on_fulfilled, promise.clone()));
                self.on_reject.borrow_mut().push_back((on_rejected, promise.clone()));
            },
            PromiseState::Fulfilled => {
                // Call the fulfill callback immediately
                if let Some(value) = self.value.borrow().clone() {
                    let result = on_fulfilled.call(Value::Undefined, &[value], &Environment::new());
                    
                    match result {
                        Ok(result_value) => {
                            promise.resolve(result_value);
                        },
                        Err(err) => {
                            promise.reject(Value::String(err));
                        },
                    }
                }
            },
            PromiseState::Rejected => {
                // Call the reject callback immediately
                if let Some(reason) = self.reason.borrow().clone() {
                    let result = on_rejected.call(Value::Undefined, &[reason], &Environment::new());
                    
                    match result {
                        Ok(result_value) => {
                            promise.resolve(result_value);
                        },
                        Err(err) => {
                            promise.reject(Value::String(err));
                        },
                    }
                }
            },
        }
        
        promise
    }
    
    /// Add a callback to be called when the promise is fulfilled
    pub fn catch(&self, on_rejected: Function) -> Rc<Promise> {
        let on_fulfilled = Function::new_native(
            None,
            vec!["value".to_string()],
            |_this, args, _env| {
                if let Some(value) = args.first() {
                    Ok(value.clone())
                } else {
                    Ok(Value::Undefined)
                }
            },
        );
        
        self.then(on_fulfilled, on_rejected)
    }
    
    /// Add a callback to be called when the promise is settled
    pub fn finally(&self, on_finally: Function) -> Rc<Promise> {
        let promise = Promise::new();
        
        // Create a callback for when the promise is fulfilled
        let on_finally_fulfilled = on_finally.clone();
        let on_fulfilled = Function::new_native(
            None,
            vec!["value".to_string()],
            move |_this, args, env| {
                let value = args.first().cloned().unwrap_or(Value::Undefined);
                
                // Call the finally callback
                let result = on_finally_fulfilled.call(Value::Undefined, &[], env);
                
                match result {
                    Ok(_) => {
                        // Return the original value
                        Ok(value)
                    },
                    Err(err) => {
                        // Reject with the error
                        Err(err)
                    },
                }
            },
        );
        
        // Create a callback for when the promise is rejected
        let on_finally_rejected = on_finally;
        let on_rejected = Function::new_native(
            None,
            vec!["reason".to_string()],
            move |_this, args, env| {
                let reason = args.first().cloned().unwrap_or(Value::Undefined);
                
                // Call the finally callback
                let result = on_finally_rejected.call(Value::Undefined, &[], env);
                
                match result {
                    Ok(_) => {
                        // Re-throw the original reason
                        Err("Promise rejected".to_string())
                    },
                    Err(err) => {
                        // Reject with the error
                        Err(err)
                    },
                }
            },
        );
        
        // Chain the callbacks
        let result = self.then(on_fulfilled, on_rejected);
        
        // Forward the result to the promise
        promise.resolve(Value::Promise(result));
        
        promise
    }
    
    /// Create a Promise that is resolved with a given value
    pub fn resolve_with(value: Value) -> Rc<Self> {
        let promise = Self::new();
        promise.resolve(value);
        promise
    }
    
    /// Create a Promise that is rejected with a given reason
    pub fn reject_with(reason: Value) -> Rc<Self> {
        let promise = Self::new();
        promise.reject(reason);
        promise
    }
    
    /// Create a Promise that resolves when all of the promises in the iterable argument have resolved
    pub fn all(iterable: Value) -> Rc<Self> {
        let promise = Self::new();
        
        // Convert the iterable to an array
        let values = match iterable {
            Value::Array(values) => values,
            _ => Vec::new(),
        };
        
        if values.is_empty() {
            // Resolve with an empty array
            promise.resolve(Value::Array(Vec::new()));
            return promise;
        }
        
        // Create a shared state for tracking the results
        let results = Rc::new(RefCell::new(vec![Value::Undefined; values.len()]));
        let count = Rc::new(RefCell::new(0));
        
        // Process each value
        for (i, value) in values.iter().enumerate() {
            if let Value::Promise(p) = value {
                // Create a callback for when the promise is fulfilled
                let promise_clone = promise.clone();
                let results_clone = results.clone();
                let count_clone = count.clone();
                let values_len = values.len();
                
                let on_fulfilled = Function::new_native(
                    None,
                    vec!["value".to_string()],
                    move |_this, args, _env| {
                        let value = args.first().cloned().unwrap_or(Value::Undefined);
                        
                        // Store the result
                        results_clone.borrow_mut()[i] = value;
                        
                        // Increment the count
                        let mut count = count_clone.borrow_mut();
                        *count += 1;
                        
                        // If all promises have resolved, resolve the main promise
                        if *count == values_len {
                            let results = results_clone.borrow().clone();
                            promise_clone.resolve(Value::Array(results));
                        }
                        
                        Ok(Value::Undefined)
                    },
                );
                
                // Create a callback for when the promise is rejected
                let promise_clone = promise.clone();
                let on_rejected = Function::new_native(
                    None,
                    vec!["reason".to_string()],
                    move |_this, args, _env| {
                        let reason = args.first().cloned().unwrap_or(Value::Undefined);
                        
                        // Reject the main promise
                        promise_clone.reject(reason);
                        
                        Ok(Value::Undefined)
                    },
                );
                
                // Add the callbacks to the promise
                p.then(on_fulfilled, on_rejected);
            } else {
                // Store the value
                results.borrow_mut()[i] = value.clone();
                
                // Increment the count
                let mut count_val = count.borrow_mut();
                *count_val += 1;
                
                // If all values were not promises, resolve the main promise
                if *count_val == values.len() {
                    let results_val = results.borrow().clone();
                    promise.resolve(Value::Array(results_val));
                }
            }
        }
        
        promise
    }
    
    /// Create a Promise that resolves or rejects when one of the promises in the iterable resolves or rejects
    pub fn race(iterable: Value) -> Rc<Self> {
        let promise = Self::new();
        
        // Convert the iterable to an array
        let values = match iterable {
            Value::Array(values) => values,
            _ => Vec::new(),
        };
        
        if values.is_empty() {
            // Resolve with an empty array
            promise.resolve(Value::Array(Vec::new()));
            return promise;
        }
        
        // Process each value
        for value in values {
            if let Value::Promise(p) = &value {
                // Create a callback for when the promise is fulfilled
                let promise_clone = promise.clone();
                let on_fulfilled = Function::new_native(
                    None,
                    vec!["value".to_string()],
                    move |_this, args, _env| {
                        let value = args.first().cloned().unwrap_or(Value::Undefined);
                        
                        // Resolve the main promise
                        promise_clone.resolve(value);
                        
                        Ok(Value::Undefined)
                    },
                );
                
                // Create a callback for when the promise is rejected
                let promise_clone = promise.clone();
                let on_rejected = Function::new_native(
                    None,
                    vec!["reason".to_string()],
                    move |_this, args, _env| {
                        let reason = args.first().cloned().unwrap_or(Value::Undefined);
                        
                        // Reject the main promise
                        promise_clone.reject(reason);
                        
                        Ok(Value::Undefined)
                    },
                );
                
                // Add the callbacks to the promise
                p.then(on_fulfilled, on_rejected);
            } else {
                // Resolve with the value
                promise.resolve(value);
                break;
            }
        }
        
        promise
    }
    
    /// Create a Promise that resolves when all of the promises in the iterable have settled
    pub fn all_settled(iterable: Value) -> Rc<Self> {
        let promise = Self::new();
        
        // Convert the iterable to an array
        let values = match iterable {
            Value::Array(values) => values,
            _ => Vec::new(),
        };
        
        if values.is_empty() {
            // Resolve with an empty array
            promise.resolve(Value::Array(Vec::new()));
            return promise;
        }
        
        // Create a shared state for tracking the results
        let results = Rc::new(RefCell::new(vec![Value::Undefined; values.len()]));
        let count = Rc::new(RefCell::new(0));
        
        // Process each value
        for (i, value) in values.iter().enumerate() {
            if let Value::Promise(p) = value {
                // Create a callback for when the promise is fulfilled
                let promise_clone = promise.clone();
                let results_clone = results.clone();
                let count_clone = count.clone();
                let values_len = values.len();
                
                let on_fulfilled = Function::new_native(
                    None,
                    vec!["value".to_string()],
                    move |_this, args, _env| {
                        let value = args.first().cloned().unwrap_or(Value::Undefined);
                        
                        // Store the result
                        let mut result = HashMap::new();
                        result.insert("status".to_string(), Value::String("fulfilled".to_string()));
                        result.insert("value".to_string(), value);
                        results_clone.borrow_mut()[i] = Value::Object(result);
                        
                        // Increment the count
                        let mut count = count_clone.borrow_mut();
                        *count += 1;
                        
                        // If all promises have settled, resolve the main promise
                        if *count == values_len {
                            let results = results_clone.borrow().clone();
                            promise_clone.resolve(Value::Array(results));
                        }
                        
                        Ok(Value::Undefined)
                    },
                );
                
                // Create a callback for when the promise is rejected
                let promise_clone = promise.clone();
                let results_clone = results.clone();
                let count_clone = count.clone();
                let values_len = values.len();
                
                let on_rejected = Function::new_native(
                    None,
                    vec!["reason".to_string()],
                    move |_this, args, _env| {
                        let reason = args.first().cloned().unwrap_or(Value::Undefined);
                        
                        // Store the result
                        let mut result = HashMap::new();
                        result.insert("status".to_string(), Value::String("rejected".to_string()));
                        result.insert("reason".to_string(), reason);
                        results_clone.borrow_mut()[i] = Value::Object(result);
                        
                        // Increment the count
                        let mut count = count_clone.borrow_mut();
                        *count += 1;
                        
                        // If all promises have settled, resolve the main promise
                        if *count == values_len {
                            let results = results_clone.borrow().clone();
                            promise_clone.resolve(Value::Array(results));
                        }
                        
                        Ok(Value::Undefined)
                    },
                );
                
                // Add the callbacks to the promise
                p.then(on_fulfilled, on_rejected);
            } else {
                // Store the value
                let mut result = HashMap::new();
                result.insert("status".to_string(), Value::String("fulfilled".to_string()));
                result.insert("value".to_string(), value.clone());
                results.borrow_mut()[i] = Value::Object(result);
                
                // Increment the count
                let mut count_val = count.borrow_mut();
                *count_val += 1;
                
                // If all values were not promises, resolve the main promise
                if *count_val == values.len() {
                    let results_val = results.borrow().clone();
                    promise.resolve(Value::Array(results_val));
                }
            }
        }
        
        promise
    }
    
    /// Create a Promise that resolves when any of the promises in the iterable resolves
    pub fn any(iterable: Value) -> Rc<Self> {
        let promise = Self::new();
        
        // Convert the iterable to an array
        let values = match iterable {
            Value::Array(values) => values,
            _ => Vec::new(),
        };
        
        if values.is_empty() {
            // Reject with an AggregateError
            let mut error = HashMap::new();
            error.insert("name".to_string(), Value::String("AggregateError".to_string()));
            error.insert("message".to_string(), Value::String("All promises were rejected".to_string()));
            error.insert("errors".to_string(), Value::Array(Vec::new()));
            promise.reject(Value::Object(error));
            return promise;
        }
        
        // Create a shared state for tracking the errors
        let errors = Rc::new(RefCell::new(vec![Value::Undefined; values.len()]));
        let count = Rc::new(RefCell::new(0));
        
        // Process each value
        for (i, value) in values.iter().enumerate() {
            if let Value::Promise(p) = value {
                // Create a callback for when the promise is fulfilled
                let promise_clone = promise.clone();
                let on_fulfilled = Function::new_native(
                    None,
                    vec!["value".to_string()],
                    move |_this, args, _env| {
                        let value = args.first().cloned().unwrap_or(Value::Undefined);
                        
                        // Resolve the main promise
                        promise_clone.resolve(value);
                        
                        Ok(Value::Undefined)
                    },
                );
                
                // Create a callback for when the promise is rejected
                let promise_clone = promise.clone();
                let errors_clone = errors.clone();
                let count_clone = count.clone();
                let values_len = values.len();
                
                let on_rejected = Function::new_native(
                    None,
                    vec!["reason".to_string()],
                    move |_this, args, _env| {
                        let reason = args.first().cloned().unwrap_or(Value::Undefined);
                        
                        // Store the error
                        errors_clone.borrow_mut()[i] = reason;
                        
                        // Increment the count
                        let mut count = count_clone.borrow_mut();
                        *count += 1;
                        
                        // If all promises have rejected, reject the main promise
                        if *count == values_len {
                            let mut error = HashMap::new();
                            error.insert("name".to_string(), Value::String("AggregateError".to_string()));
                            error.insert("message".to_string(), Value::String("All promises were rejected".to_string()));
                            error.insert("errors".to_string(), Value::Array(errors_clone.borrow().clone()));
                            promise_clone.reject(Value::Object(error));
                        }
                        
                        Ok(Value::Undefined)
                    },
                );
                
                // Add the callbacks to the promise
                p.then(on_fulfilled, on_rejected);
            } else {
                // Resolve with the value
                promise.resolve(value.clone());
                break;
            }
        }
        
        promise
    }
}