use std::collections::HashMap;
use crate::parser::AstNode;
use crate::runtime::promise::Promise;
use crate::runtime::timers::{TimerManager, create_set_timeout_function, create_set_interval_function, create_clear_timeout_function, create_clear_interval_function};
use crate::runtime::json::create_json_object;
use crate::runtime::browser::BrowserEnvironment;
use crate::runtime::collections::{Map, Set, WeakMap, WeakSet};
use crate::interpreter::value::Value;
use crate::interpreter::function::Function;
use crate::interpreter::environment::Environment;

/// Interpreter for SmashLang
pub struct Interpreter {
    environment: Environment,
    timer_manager: TimerManager,
    browser_environment: BrowserEnvironment,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut env = Environment::new();
        
        // Create timer manager
        let timer_manager = TimerManager::new(env.clone());
        
        // Create browser environment
        let browser_environment = BrowserEnvironment::new();
        
        // Define global functions and objects
        env.define("console", Value::Object({
            let mut console = HashMap::new();
            console.insert("log".to_string(), Value::Function(Function::new_native(
                Some("log".to_string()),
                vec!["...args".to_string()],
                |_this, args, _env| {
                    // Print all arguments
                    for (i, arg) in args.iter().enumerate() {
                        if i > 0 {
                            print!(" ");
                        }
                        print!("{}", arg);
                    }
                    println!();
                    Ok(Value::Undefined)
                },
            )));
            console
        }));
        
        // Define Promise constructor and static methods
        env.define("Promise", Value::Object({
            let mut promise_obj = HashMap::new();
            
            // Promise constructor
            promise_obj.insert("constructor".to_string(), Value::Function(Function::new_native(
                Some("Promise".to_string()),
                vec!["executor".to_string()],
                |_this, args, env| {
                    if args.is_empty() {
                        return Err("Promise constructor requires an executor function".to_string());
                    }
                    
                    if let Value::Function(executor) = &args[0] {
                        let promise = Promise::with_executor(executor, env);
                        Ok(Value::Promise(promise))
                    } else {
                        Err("Promise executor must be a function".to_string())
                    }
                },
            )));
            
            // Promise.resolve
            promise_obj.insert("resolve".to_string(), Value::Function(Function::new_native(
                Some("resolve".to_string()),
                vec!["value".to_string()],
                |_this, args, _env| {
                    let value = args.first().cloned().unwrap_or(Value::Undefined);
                    let promise = Promise::resolve_with(value);
                    Ok(Value::Promise(promise))
                },
            )));
            
            // Promise.reject
            promise_obj.insert("reject".to_string(), Value::Function(Function::new_native(
                Some("reject".to_string()),
                vec!["reason".to_string()],
                |_this, args, _env| {
                    let reason = args.first().cloned().unwrap_or(Value::Undefined);
                    let promise = Promise::reject_with(reason);
                    Ok(Value::Promise(promise))
                },
            )));
            
            // Promise.all
            promise_obj.insert("all".to_string(), Value::Function(Function::new_native(
                Some("all".to_string()),
                vec!["iterable".to_string()],
                |_this, args, _env| {
                    let iterable = args.first().cloned().unwrap_or(Value::Undefined);
                    let promise = Promise::all(iterable);
                    Ok(Value::Promise(promise))
                },
            )));
            
            // Promise.race
            promise_obj.insert("race".to_string(), Value::Function(Function::new_native(
                Some("race".to_string()),
                vec!["iterable".to_string()],
                |_this, args, _env| {
                    let iterable = args.first().cloned().unwrap_or(Value::Undefined);
                    let promise = Promise::race(iterable);
                    Ok(Value::Promise(promise))
                },
            )));
            
            // Promise.allSettled
            promise_obj.insert("allSettled".to_string(), Value::Function(Function::new_native(
                Some("allSettled".to_string()),
                vec!["iterable".to_string()],
                |_this, args, _env| {
                    let iterable = args.first().cloned().unwrap_or(Value::Undefined);
                    let promise = Promise::all_settled(iterable);
                    Ok(Value::Promise(promise))
                },
            )));
            
            // Promise.any
            promise_obj.insert("any".to_string(), Value::Function(Function::new_native(
                Some("any".to_string()),
                vec!["iterable".to_string()],
                |_this, args, _env| {
                    let iterable = args.first().cloned().unwrap_or(Value::Undefined);
                    let promise = Promise::any(iterable);
                    Ok(Value::Promise(promise))
                },
            )));
            
            promise_obj
        }));
        
        // Define Map constructor
        env.define("Map", Value::Function(Function::new_native(
            Some("Map".to_string()),
            vec!["iterable".to_string()],
            |_this, args, _env| {
                if args.is_empty() {
                    Ok(Value::Map(Map::new()))
                } else {
                    let iterable = &args[0];
                    match Map::from_iterable(iterable) {
                        Ok(map) => Ok(Value::Map(map)),
                        Err(err) => Err(err),
                    }
                }
            },
        )));
        
        // Define Set constructor
        env.define("Set", Value::Function(Function::new_native(
            Some("Set".to_string()),
            vec!["iterable".to_string()],
            |_this, args, _env| {
                if args.is_empty() {
                    Ok(Value::Set(Set::new()))
                } else {
                    let iterable = &args[0];
                    match Set::from_iterable(iterable) {
                        Ok(set) => Ok(Value::Set(set)),
                        Err(err) => Err(err),
                    }
                }
            },
        )));
        
        // Define WeakMap constructor
        env.define("WeakMap", Value::Function(Function::new_native(
            Some("WeakMap".to_string()),
            vec!["iterable".to_string()],
            |_this, args, _env| {
                if args.is_empty() {
                    Ok(Value::WeakMap(WeakMap::new()))
                } else {
                    let iterable = &args[0];
                    match WeakMap::from_iterable(iterable) {
                        Ok(weak_map) => Ok(Value::WeakMap(weak_map)),
                        Err(err) => Err(err),
                    }
                }
            },
        )));
        
        // Define WeakSet constructor
        env.define("WeakSet", Value::Function(Function::new_native(
            Some("WeakSet".to_string()),
            vec!["iterable".to_string()],
            |_this, args, _env| {
                if args.is_empty() {
                    Ok(Value::WeakSet(WeakSet::new()))
                } else {
                    let iterable = &args[0];
                    match WeakSet::from_iterable(iterable) {
                        Ok(weak_set) => Ok(Value::WeakSet(weak_set)),
                        Err(err) => Err(err),
                    }
                }
            },
        )));
        
        // Define timer functions
        env.define("setTimeout", Value::Function(create_set_timeout_function(timer_manager.clone())));
        env.define("setInterval", Value::Function(create_set_interval_function(timer_manager.clone())));
        env.define("clearTimeout", Value::Function(create_clear_timeout_function(timer_manager.clone())));
        env.define("clearInterval", Value::Function(create_clear_interval_function(timer_manager.clone())));
        
        // Define JSON object
        env.define("JSON", create_json_object());
        
        // Define browser globals
        let window = browser_environment.create_window();
        env.define("window", window.clone());
        
        // Extract properties from window object to global scope
        if let Value::Object(window_obj) = window {
            for (key, value) in window_obj {
                env.define(&key, value);
            }
        }
        
        Self {
            environment: env,
            timer_manager,
            browser_environment,
        }
    }
    
    pub fn evaluate(&self, _node: &AstNode) -> Result<Value, String> {
        // For now, just return a simple value
        Ok(Value::Number(42.0))
    }
}