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
        let resolve = {
            let resolve_promise = resolve_promise.clone();
            Function::new_native(
                Some("resolve".to_string()),
                vec!["value".to_string()],
                move |_this, args: &[Value], _env| {
                    if let Some(value) = args.first() {
                        resolve_promise.resolve(value.clone());
                    } else {
                        resolve_promise.resolve(Value::Undefined);
                    }
                    Ok(Value::Undefined)
                },
            )
        };
        let reject_promise = promise.clone();
        let reject = {
            let reject_promise = reject_promise.clone();
            Function::new_native(
                Some("reject".to_string()),
                vec!["reason".to_string()],
                move |_this, args: &[Value], _env| {
                    if let Some(reason) = args.first() {
                        reject_promise.reject(reason.clone());
                    } else {
                        reject_promise.reject(Value::Undefined);
                    }
                    Ok(Value::Undefined)
                },
            )
        };

        
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
                        Function::new_native(
                            None,
                            vec!["value".to_string()],
                            move |_this, args: &[Value], _env| {
                                if let Some(value) = args.first() {
                                    this_promise.resolve(value.clone());
                                } else {
                                    this_promise.resolve(Value::Undefined);
                                }
                                Ok(Value::Undefined)
                            },
                        ),
                        Function::new_native(
                            None,
                            vec!["reason".to_string()],
                            move |_this, args: &[Value], _env| {
                                if let Some(reason) = args.first() {
                                    this_promise.reject(reason.clone());
                                } else {
                                    this_promise.reject(Value::Undefined);
                                }
                                Ok(Value::Undefined)
                            },
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
        let callbacks = self.on_reject.borrow_mut().drain(..).collect::<Vec<_>>();
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
                let _on_fulfilled_clone = on_fulfilled.clone();
                let _result_promise_clone = result_promise.clone();
                
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
                let _on_rejected_clone = on_rejected.clone();
                let _result_promise_clone = result_promise.clone();
                
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
        let on_fulfilled = Function::new_native(
            None,
            vec!["value".to_string()],
            |_, args: &[Value], _| {
                if let Some(value) = args.first() {
                    Ok(value.clone())
                } else {
                    Ok(Value::Undefined)
                }
            },
        );
        
        self.then(on_fulfilled, on_rejected)
    }
    
    /// Add a callback to be called when the Promise is settled
    pub fn finally(&self, on_finally: Function) -> Rc<Promise> {
        let result_promise = Promise::new();
        
        // Create on_fulfilled and on_rejected callbacks that call on_finally
        let on_finally_fulfilled = on_finally.clone();
        let on_fulfilled = Function::new_native(
            None,
            vec!["value".to_string()],
            move |_, args: &[Value], env| {
                let value = args.first().cloned().unwrap_or(Value::Undefined);
                match on_finally_fulfilled.call(Value::Undefined, &[], env) {
                    Ok(_) => Ok(value),
                    Err(error) => Err(error),
                }
            },
        );
        let on_finally_rejected = on_finally;
        let on_rejected = Function::new_native(
            None,
            vec!["reason".to_string()],
            move |_, args: &[Value], env| {
                let reason = args.first().cloned().unwrap_or(Value::Undefined);
                match on_finally_rejected.call(Value::Undefined, &[], env) {
                    Ok(_) => Err(format!("{}", reason)),
                    Err(error) => Err(error),
                }
            },
        );
        self.then(on_fulfilled, on_rejected)
    }

    /// Create a Promise that resolves when all the Promises in the iterable resolve
    pub fn all(iterable: Value) -> Rc<Promise> {
        let result_promise = Promise::new();
        let promises = match iterable {
            Value::Array(arr) => arr,
            _ => {
                result_promise.reject(Value::String("Promise.all requires an array".to_string()));
                return result_promise;
            },
        };
        if promises.is_empty() {
            result_promise.resolve(Value::Array(vec![]));
            return result_promise;
        }
        use std::rc::Rc;
        use std::cell::RefCell;
        let results = Rc::new(RefCell::new(vec![Value::Undefined; promises.len()]));
        let remaining = Rc::new(RefCell::new(promises.len()));
        for (i, promise) in promises.iter().enumerate() {
            if let Value::Promise(p) = promise {
                let result_promise_clone = result_promise.clone();
                let results = results.clone();
                let remaining = remaining.clone();
                p.then(
                    Function::new_native(
                        None,
                        vec!["value".to_string()],
                        {
                            let results = results.clone();
                            let remaining = remaining.clone();
                            let result_promise_clone = result_promise_clone.clone();
                            move |_, args: &[Value], _| {
                                if let Some(value) = args.first() {
                                    results.borrow_mut()[i] = value.clone();
                                    let mut rem = remaining.borrow_mut();
                                    *rem -= 1;
                                    if *rem == 0 {
                                        result_promise_clone.resolve(Value::Array(results.borrow().clone()));
                                    }
                                }
                                Ok(Value::Undefined)
                            }
                        },
                    ),
                    Function::new_native(
                        None,
                        vec!["reason".to_string()],
                        {
                            let result_promise_clone = result_promise_clone.clone();
                            move |_, args: &[Value], _| {
                                if let Some(reason) = args.first() {
                                    result_promise_clone.reject(reason.clone());
                                }
                                Ok(Value::Undefined)
                            }
                        },
                    ),
                );
            } else {
                results.borrow_mut()[i] = promise.clone();
                let mut rem = remaining.borrow_mut();
                *rem -= 1;
                if *rem == 0 {
                    result_promise.resolve(Value::Array(results.borrow().clone()));
                }
            }
        }
        result_promise
    }

    /// Create a Promise that resolves or rejects as soon as one of the Promises in the iterable resolves or rejects
    pub fn race(iterable: Value) -> Rc<Promise> {
        let result_promise = Promise::new();
        let promises = match iterable {
            Value::Array(arr) => arr,
            _ => {
                result_promise.reject(Value::String("Promise.race requires an array".to_string()));
                return result_promise;
            },
        };
        if promises.is_empty() {
            result_promise.resolve(Value::Undefined);
            return result_promise;
        }
        let settled = Rc::new(RefCell::new(false));
        for promise in promises.iter() {
            if let Value::Promise(p) = promise {
                let result_promise_clone = result_promise.clone();
                let settled = settled.clone();
                p.then(
                    Function::new_native(
                        None,
                        vec!["value".to_string()],
                        {
                            let result_promise_clone = result_promise_clone.clone();
                            let settled = settled.clone();
                            move |_, args: &[Value], _| {
                                let mut done = settled.borrow_mut();
                                if !*done {
                                    *done = true;
                                    if let Some(value) = args.first() {
                                        result_promise_clone.resolve(value.clone());
                                    } else {
                                        result_promise_clone.resolve(Value::Undefined);
                                    }
                                }
                                Ok(Value::Undefined)
                            }
                        },
                    ),
                    Function::new_native(
                        None,
                        vec!["reason".to_string()],
                        {
                            let result_promise_clone = result_promise_clone.clone();
                            let settled = settled.clone();
                            move |_, args: &[Value], _| {
                                let mut done = settled.borrow_mut();
                                if !*done {
                                    *done = true;
                                    if let Some(reason) = args.first() {
                                        result_promise_clone.reject(reason.clone());
                                    } else {
                                        result_promise_clone.reject(Value::Undefined);
                                    }
                                }
                                Ok(Value::Undefined)
                            }
                        },
                    ),
                );
            } else {
                let mut done = settled.borrow_mut();
                if !*done {
                    *done = true;
                    result_promise.resolve(promise.clone());
                }
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
        
        use std::rc::Rc;
        use std::cell::RefCell;
        let results = Rc::new(RefCell::new(vec![Value::Undefined; promises.len()]));
        let remaining = Rc::new(RefCell::new(promises.len()));
        
        for (i, promise) in promises.iter().enumerate() {
            if let Value::Promise(p) = promise {
                let result_promise_clone = result_promise.clone();
                let results = results.clone();
                let remaining = remaining.clone();
                p.then(
                    Function::new_native(
                        None,
                        vec!["value".to_string()],
                        {
                            let results = results.clone();
                            let remaining = remaining.clone();
                            let result_promise_clone = result_promise_clone.clone();
                            move |_, args: &[Value], _| {
                                if let Some(value) = args.first() {
                                    let mut result_obj = HashMap::new();
                                    result_obj.insert("status".to_string(), Value::String("fulfilled".to_string()));
                                    result_obj.insert("value".to_string(), value.clone());
                                    results.borrow_mut()[i] = Value::Object(result_obj);
                                    let mut rem = remaining.borrow_mut();
                                    *rem -= 1;
                                    if *rem == 0 {
                                        result_promise_clone.resolve(Value::Array(results.borrow().clone()));
                                    }
                                }
                                Ok(Value::Undefined)
                            }
                        },
                    ),
                    Function::new_native(
                        None,
                        vec!["reason".to_string()],
                        {
                            let results = results.clone();
                            let remaining = remaining.clone();
                            let result_promise_clone = result_promise_clone.clone();
                            move |_, args: &[Value], _| {
                                if let Some(reason) = args.first() {
                                    let mut result_obj = HashMap::new();
                                    result_obj.insert("status".to_string(), Value::String("rejected".to_string()));
                                    result_obj.insert("reason".to_string(), reason.clone());
                                    results.borrow_mut()[i] = Value::Object(result_obj);
                                    let mut rem = remaining.borrow_mut();
                                    *rem -= 1;
                                    if *rem == 0 {
                                        result_promise_clone.resolve(Value::Array(results.borrow().clone()));
                                    }
                                }
                                Ok(Value::Undefined)
                            }
                        },
                    ),
                );
            } else {
                let mut result_obj = HashMap::new();
                result_obj.insert("status".to_string(), Value::String("fulfilled".to_string()));
                result_obj.insert("value".to_string(), promise.clone());
                results.borrow_mut()[i] = Value::Object(result_obj);
                let mut rem = remaining.borrow_mut();
                *rem -= 1;
                if *rem == 0 {
                    result_promise.resolve(Value::Array(results.borrow().clone()));
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
        
        use std::rc::Rc;
        use std::cell::RefCell;
        let errors = Rc::new(RefCell::new(vec![Value::Undefined; promises.len()]));
        let remaining = Rc::new(RefCell::new(promises.len()));
        
        for (i, promise) in promises.iter().enumerate() {
            if let Value::Promise(p) = promise {
                let result_promise_clone = result_promise.clone();
                let errors = errors.clone();
                let remaining = remaining.clone();
                p.then(
                    Function::new_native(
                        None,
                        vec!["value".to_string()],
                        {
                            let result_promise_clone = result_promise_clone.clone();
                            move |_, args: &[Value], _| {
                                if let Some(value) = args.first() {
                                    result_promise_clone.resolve(value.clone());
                                }
                                Ok(Value::Undefined)
                            }
                        },
                    ),
                    Function::new_native(
                        None,
                        vec!["reason".to_string()],
                        {
                            let errors = errors.clone();
                            let remaining = remaining.clone();
                            let result_promise_clone = result_promise_clone.clone();
                            move |_, args: &[Value], _| {
                                if let Some(reason) = args.first() {
                                    errors.borrow_mut()[i] = reason.clone();
                                    let mut rem = remaining.borrow_mut();
                                    *rem -= 1;
                                    if *rem == 0 {
                                        let mut error = HashMap::new();
                                        error.insert("name".to_string(), Value::String("AggregateError".to_string()));
                                        error.insert("message".to_string(), Value::String("All promises were rejected".to_string()));
                                        error.insert("errors".to_string(), Value::Array(errors.borrow().clone()));
                                        result_promise_clone.reject(Value::Object(error));
                                    }
                                }
                                Ok(Value::Undefined)
                            }
                        },
                    ),
                );
            } else {
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