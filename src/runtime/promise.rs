use std::cell::RefCell;
use std::collections::VecDeque;
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
        let resolve = Function::new(
            Some("resolve".to_string()),
            vec!["value".to_string()],
            vec![],
            env.clone(),
            Box::new(move |_this, args, _env| {
                if let Some(value) = args.first() {
                    resolve_promise.resolve(value.clone());
                } else {
                    resolve_promise.resolve(Value::Undefined);
                }
                Ok(Value::Undefined)
            }),
        );
        
        let reject_promise = promise.clone();
        let reject = Function::new(
            Some("reject".to_string()),
            vec!["reason".to_string()],
            vec![],
            env.clone(),
            Box::new(move |_this, args, _env| {
                if let Some(reason) = args.first() {
                    reject_promise.reject(reason.clone());
                } else {
                    reject_promise.reject(Value::Undefined);
                }
                Ok(Value::Undefined)
            }),
        );
        
        // Call the executor with resolve and reject
        let _ = executor.call(
            Value::Undefined,
            &[Value::Function(resolve), Value::Function(reject)],
            env,
        );
        
        promise
    }
    
    /// Create a Promise that is already fulfilled with a value
    pub fn resolve_with(value: Value) -> Rc<Self> {
        let promise = Self::new();
        promise.resolve(value);
        promise
    }
    
    /// Create a Promise that is already rejected with a reason
    pub fn reject_with(reason: Value) -> Rc<Self> {
        let promise = Self::new();
        promise.reject(reason);
        promise
    }
    
    /// Get the current state of the Promise
    pub fn state(&self) -> PromiseState {
        self.state.borrow().clone()
    }
    
    /// Resolve the Promise with a value
    pub fn resolve(&self, value: Value) {
        // Only resolve if the Promise is still pending
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
                    let this_promise = self.clone();
                    other_promise.then(
                        Function::new(
                            None,
                            vec!["value".to_string()],
                            vec![],
                            Environment::new(),
                            Box::new(move |_this, args, _env| {
                                if let Some(value) = args.first() {
                                    this_promise.resolve(value.clone());
                                } else {
                                    this_promise.resolve(Value::Undefined);
                                }
                                Ok(Value::Undefined)
                            }),
                        ),
                        Function::new(
                            None,
                            vec!["reason".to_string()],
                            vec![],
                            Environment::new(),
                            Box::new(move |_this, args, _env| {
                                if let Some(reason) = args.first() {
                                    this_promise.reject(reason.clone());
                                } else {
                                    this_promise.reject(Value::Undefined);
                                }
                                Ok(Value::Undefined)
                            }),
                        ),
                    );
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
        
        // Call all the on_fulfill callbacks
        let mut callbacks = self.on_fulfill.borrow_mut().drain(..).collect::<Vec<_>>();
        for (callback, result_promise) in callbacks {
            let value = self.value.borrow().clone().unwrap_or(Value::Undefined);
            
            // Call the callback with the value
            match callback.call(Value::Undefined, &[value], &Environment::new()) {
                Ok(result) => {
                    // Resolve the result Promise with the callback's result
                    result_promise.resolve(result);
                },
                Err(error) => {
                    // Reject the result Promise with the error
                    result_promise.reject(Value::String(error));
                },
            }
        }
    }
    
    /// Reject the Promise with a reason
    pub fn reject(&self, reason: Value) {
        // Only reject if the Promise is still pending
        if *self.state.borrow() != PromiseState::Pending {
            return;
        }
        
        // Set the state to rejected and store the reason
        *self.state.borrow_mut() = PromiseState::Rejected;
        *self.reason.borrow_mut() = Some(reason);
        
        // Call all the on_reject callbacks
        let mut callbacks = self.on_reject.borrow_mut().drain(..).collect::<Vec<_>>();
        for (callback, result_promise) in callbacks {
            let reason = self.reason.borrow().clone().unwrap_or(Value::Undefined);
            
            // Call the callback with the reason
            match callback.call(Value::Undefined, &[reason], &Environment::new()) {
                Ok(result) => {
                    // Resolve the result Promise with the callback's result
                    result_promise.resolve(result);
                },
                Err(error) => {
                    // Reject the result Promise with the error
                    result_promise.reject(Value::String(error));
                },
            }
        }
    }
    
    /// Add callbacks to be called when the Promise is settled
    pub fn then(&self, on_fulfilled: Function, on_rejected: Function) -> Rc<Promise> {
        let result_promise = Promise::new();
        
        match *self.state.borrow() {
            PromiseState::Pending => {
                // Add callbacks to be called when the Promise is settled
                self.on_fulfill.borrow_mut().push_back((on_fulfilled, result_promise.clone()));
                self.on_reject.borrow_mut().push_back((on_rejected, result_promise.clone()));
            },
            PromiseState::Fulfilled => {
                // Call the on_fulfilled callback immediately
                let value = self.value.borrow().clone().unwrap_or(Value::Undefined);
                
                // Schedule the callback to be called asynchronously
                let on_fulfilled_clone = on_fulfilled.clone();
                let result_promise_clone = result_promise.clone();
                
                // In a real implementation, this would be scheduled on the event loop
                // For simplicity, we'll call it directly
                match on_fulfilled.call(Value::Undefined, &[value], &Environment::new()) {
                    Ok(result) => {
                        result_promise.resolve(result);
                    },
                    Err(error) => {
                        result_promise.reject(Value::String(error));
                    },
                }
            },
            PromiseState::Rejected => {
                // Call the on_rejected callback immediately
                let reason = self.reason.borrow().clone().unwrap_or(Value::Undefined);
                
                // Schedule the callback to be called asynchronously
                let on_rejected_clone = on_rejected.clone();
                let result_promise_clone = result_promise.clone();
                
                // In a real implementation, this would be scheduled on the event loop
                // For simplicity, we'll call it directly
                match on_rejected.call(Value::Undefined, &[reason], &Environment::new()) {
                    Ok(result) => {
                        result_promise.resolve(result);
                    },
                    Err(error) => {
                        result_promise.reject(Value::String(error));
                    },
                }
            },
        }
        
        result_promise
    }
    
    /// Add a callback to be called when the Promise is rejected
    pub fn catch(&self, on_rejected: Function) -> Rc<Promise> {
        // Create a dummy on_fulfilled callback that just passes through the value
        let on_fulfilled = Function::new(
            None,
            vec!["value".to_string()],
            vec![],
            Environment::new(),
            Box::new(|_this, args, _env| {
                if let Some(value) = args.first() {
                    Ok(value.clone())
                } else {
                    Ok(Value::Undefined)
                }
            }),
        );
        
        self.then(on_fulfilled, on_rejected)
    }
    
    /// Add a callback to be called when the Promise is settled
    pub fn finally(&self, on_finally: Function) -> Rc<Promise> {
        let result_promise = Promise::new();
        
        // Create on_fulfilled and on_rejected callbacks that call on_finally
        let on_fulfilled = Function::new(
            None,
            vec!["value".to_string()],
            vec![],
            Environment::new(),
            Box::new(move |_this, args, env| {
                let value = args.first().cloned().unwrap_or(Value::Undefined);
                
                // Call on_finally
                match on_finally.call(Value::Undefined, &[], env) {
                    Ok(_) => {
                        // Return the original value
                        Ok(value)
                    },
                    Err(error) => {
                        // Reject with the error
                        Err(error)
                    },
                }
            }),
        );
        
        let on_rejected = Function::new(
            None,
            vec!["reason".to_string()],
            vec![],
            Environment::new(),
            Box::new(move |_this, args, env| {
                let reason = args.first().cloned().unwrap_or(Value::Undefined);
                
                // Call on_finally
                match on_finally.call(Value::Undefined, &[], env) {
                    Ok(_) => {
                        // Re-throw the original reason
                        Err(format!("{}", reason))
                    },
                    Err(error) => {
                        // Reject with the error
                        Err(error)
                    },
                }
            }),
        );
        
        self.then(on_fulfilled, on_rejected)
    }
    
    /// Create a Promise that resolves when all the Promises in the iterable resolve
    pub fn all(iterable: Value) -> Rc<Promise> {
        let result_promise = Promise::new();
        
        // Convert the iterable to an array
        let promises = match iterable {
            Value::Array(arr) => arr,
            _ => {
                // If the iterable is not an array, reject the Promise
                result_promise.reject(Value::String("Promise.all requires an array".to_string()));
                return result_promise;
            },
        };
        
        if promises.is_empty() {
            // If the array is empty, resolve with an empty array
            result_promise.resolve(Value::Array(vec![]));
            return result_promise;
        }
        
        let mut results = vec![Value::Undefined; promises.len()];
        let mut remaining = promises.len();
        
        // Set up callbacks for each Promise
        for (i, promise) in promises.iter().enumerate() {
            if let Value::Promise(p) = promise {
                let result_promise_clone = result_promise.clone();
                let results_clone = results.clone();
                
                p.then(
                    Function::new(
                        None,
                        vec!["value".to_string()],
                        vec![],
                        Environment::new(),
                        Box::new(move |_this, args, _env| {
                            if let Some(value) = args.first() {
                                // Store the result
                                results[i] = value.clone();
                                
                                // Decrement the remaining count
                                remaining -= 1;
                                
                                // If all Promises have resolved, resolve the result Promise
                                if remaining == 0 {
                                    result_promise_clone.resolve(Value::Array(results_clone.clone()));
                                }
                            }
                            
                            Ok(Value::Undefined)
                        }),
                    ),
                    Function::new(
                        None,
                        vec!["reason".to_string()],
                        vec![],
                        Environment::new(),
                        Box::new(move |_this, args, _env| {
                            if let Some(reason) = args.first() {
                                // Reject the result Promise with the first rejection reason
                                result_promise_clone.reject(reason.clone());
                            }
                            
                            Ok(Value::Undefined)
                        }),
                    ),
                );
            } else {
                // If the value is not a Promise, treat it as already resolved
                results[i] = promise.clone();
                remaining -= 1;
                
                // If all Promises have resolved, resolve the result Promise
                if remaining == 0 {
                    result_promise.resolve(Value::Array(results.clone()));
                }
            }
        }
        
        result_promise
    }
    
    /// Create a Promise that resolves or rejects when the first Promise in the iterable resolves or rejects
    pub fn race(iterable: Value) -> Rc<Promise> {
        let result_promise = Promise::new();
        
        // Convert the iterable to an array
        let promises = match iterable {
            Value::Array(arr) => arr,
            _ => {
                // If the iterable is not an array, reject the Promise
                result_promise.reject(Value::String("Promise.race requires an array".to_string()));
                return result_promise;
            },
        };
        
        if promises.is_empty() {
            // If the array is empty, the Promise will never settle
            return result_promise;
        }
        
        // Set up callbacks for each Promise
        for promise in promises {
            if let Value::Promise(p) = promise {
                let result_promise_clone = result_promise.clone();
                
                p.then(
                    Function::new(
                        None,
                        vec!["value".to_string()],
                        vec![],
                        Environment::new(),
                        Box::new(move |_this, args, _env| {
                            if let Some(value) = args.first() {
                                // Resolve the result Promise with the first resolved value
                                result_promise_clone.resolve(value.clone());
                            }
                            
                            Ok(Value::Undefined)
                        }),
                    ),
                    Function::new(
                        None,
                        vec!["reason".to_string()],
                        vec![],
                        Environment::new(),
                        Box::new(move |_this, args, _env| {
                            if let Some(reason) = args.first() {
                                // Reject the result Promise with the first rejection reason
                                result_promise_clone.reject(reason.clone());
                            }
                            
                            Ok(Value::Undefined)
                        }),
                    ),
                );
            } else {
                // If the value is not a Promise, treat it as already resolved
                result_promise.resolve(promise);
                break;
            }
        }
        
        result_promise
    }
    
    /// Create a Promise that resolves when all the Promises in the iterable settle
    pub fn all_settled(iterable: Value) -> Rc<Promise> {
        let result_promise = Promise::new();
        
        // Convert the iterable to an array
        let promises = match iterable {
            Value::Array(arr) => arr,
            _ => {
                // If the iterable is not an array, reject the Promise
                result_promise.reject(Value::String("Promise.allSettled requires an array".to_string()));
                return result_promise;
            },
        };
        
        if promises.is_empty() {
            // If the array is empty, resolve with an empty array
            result_promise.resolve(Value::Array(vec![]));
            return result_promise;
        }
        
        let mut results = vec![Value::Undefined; promises.len()];
        let mut remaining = promises.len();
        
        // Set up callbacks for each Promise
        for (i, promise) in promises.iter().enumerate() {
            if let Value::Promise(p) = promise {
                let result_promise_clone = result_promise.clone();
                let results_clone = results.clone();
                
                p.then(
                    Function::new(
                        None,
                        vec!["value".to_string()],
                        vec![],
                        Environment::new(),
                        Box::new(move |_this, args, _env| {
                            if let Some(value) = args.first() {
                                // Create a fulfilled result object
                                let mut result_obj = HashMap::new();
                                result_obj.insert("status".to_string(), Value::String("fulfilled".to_string()));
                                result_obj.insert("value".to_string(), value.clone());
                                
                                // Store the result
                                results[i] = Value::Object(result_obj);
                                
                                // Decrement the remaining count
                                remaining -= 1;
                                
                                // If all Promises have settled, resolve the result Promise
                                if remaining == 0 {
                                    result_promise_clone.resolve(Value::Array(results_clone.clone()));
                                }
                            }
                            
                            Ok(Value::Undefined)
                        }),
                    ),
                    Function::new(
                        None,
                        vec!["reason".to_string()],
                        vec![],
                        Environment::new(),
                        Box::new(move |_this, args, _env| {
                            if let Some(reason) = args.first() {
                                // Create a rejected result object
                                let mut result_obj = HashMap::new();
                                result_obj.insert("status".to_string(), Value::String("rejected".to_string()));
                                result_obj.insert("reason".to_string(), reason.clone());
                                
                                // Store the result
                                results[i] = Value::Object(result_obj);
                                
                                // Decrement the remaining count
                                remaining -= 1;
                                
                                // If all Promises have settled, resolve the result Promise
                                if remaining == 0 {
                                    result_promise_clone.resolve(Value::Array(results_clone.clone()));
                                }
                            }
                            
                            Ok(Value::Undefined)
                        }),
                    ),
                );
            } else {
                // If the value is not a Promise, treat it as already resolved
                let mut result_obj = HashMap::new();
                result_obj.insert("status".to_string(), Value::String("fulfilled".to_string()));
                result_obj.insert("value".to_string(), promise.clone());
                
                results[i] = Value::Object(result_obj);
                remaining -= 1;
                
                // If all Promises have settled, resolve the result Promise
                if remaining == 0 {
                    result_promise.resolve(Value::Array(results.clone()));
                }
            }
        }
        
        result_promise
    }
    
    /// Create a Promise that resolves when any of the Promises in the iterable resolves
    pub fn any(iterable: Value) -> Rc<Promise> {
        let result_promise = Promise::new();
        
        // Convert the iterable to an array
        let promises = match iterable {
            Value::Array(arr) => arr,
            _ => {
                // If the iterable is not an array, reject the Promise
                result_promise.reject(Value::String("Promise.any requires an array".to_string()));
                return result_promise;
            },
        };
        
        if promises.is_empty() {
            // If the array is empty, reject with AggregateError
            let mut error = HashMap::new();
            error.insert("name".to_string(), Value::String("AggregateError".to_string()));
            error.insert("message".to_string(), Value::String("All promises were rejected".to_string()));
            error.insert("errors".to_string(), Value::Array(vec![]));
            
            result_promise.reject(Value::Object(error));
            return result_promise;
        }
        
        let mut errors = vec![Value::Undefined; promises.len()];
        let mut remaining = promises.len();
        
        // Set up callbacks for each Promise
        for (i, promise) in promises.iter().enumerate() {
            if let Value::Promise(p) = promise {
                let result_promise_clone = result_promise.clone();
                let errors_clone = errors.clone();
                
                p.then(
                    Function::new(
                        None,
                        vec!["value".to_string()],
                        vec![],
                        Environment::new(),
                        Box::new(move |_this, args, _env| {
                            if let Some(value) = args.first() {
                                // Resolve the result Promise with the first resolved value
                                result_promise_clone.resolve(value.clone());
                            }
                            
                            Ok(Value::Undefined)
                        }),
                    ),
                    Function::new(
                        None,
                        vec!["reason".to_string()],
                        vec![],
                        Environment::new(),
                        Box::new(move |_this, args, _env| {
                            if let Some(reason) = args.first() {
                                // Store the error
                                errors[i] = reason.clone();
                                
                                // Decrement the remaining count
                                remaining -= 1;
                                
                                // If all Promises have rejected, reject the result Promise with AggregateError
                                if remaining == 0 {
                                    let mut error = HashMap::new();
                                    error.insert("name".to_string(), Value::String("AggregateError".to_string()));
                                    error.insert("message".to_string(), Value::String("All promises were rejected".to_string()));
                                    error.insert("errors".to_string(), Value::Array(errors_clone.clone()));
                                    
                                    result_promise_clone.reject(Value::Object(error));
                                }
                            }
                            
                            Ok(Value::Undefined)
                        }),
                    ),
                );
            } else {
                // If the value is not a Promise, treat it as already resolved
                result_promise.resolve(promise);
                break;
            }
        }
        
        result_promise
    }
}

// Add Promise to the Value enum
impl Value {
    pub fn is_promise(&self) -> bool {
        matches!(self, Value::Promise(_))
    }
}