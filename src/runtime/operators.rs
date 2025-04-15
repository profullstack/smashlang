use std::collections::HashMap;
use crate::interpreter::{Value, Environment};

/// Implements the optional chaining operator (?.)
pub fn optional_chaining(object: &Value, property: &str) -> Value {
    match object {
        Value::Object(obj) => {
            if let Some(value) = obj.get(property) {
                value.clone()
            } else {
                Value::Undefined
            }
        },
        Value::ClassInstance(instance) => {
            if let Some(value) = instance.borrow().get_property(property) {
                value
            } else {
                Value::Undefined
            }
        },
        Value::Null | Value::Undefined => Value::Undefined,
        _ => Value::Undefined,
    }
}

/// Implements the optional computed property access operator (?.[])
pub fn optional_computed_property_access(object: &Value, property: &Value) -> Value {
    match object {
        Value::Object(obj) => {
            if let Value::String(key) = property {
                if let Some(value) = obj.get(key) {
                    value.clone()
                } else {
                    Value::Undefined
                }
            } else if let Value::Number(index) = property {
                if index.fract() == 0.0 && *index >= 0.0 {
                    let key = index.to_string();
                    if let Some(value) = obj.get(&key) {
                        value.clone()
                    } else {
                        Value::Undefined
                    }
                } else {
                    Value::Undefined
                }
            } else {
                Value::Undefined
            }
        },
        Value::Array(arr) => {
            if let Value::Number(index) = property {
                if index.fract() == 0.0 && *index >= 0.0 {
                    let idx = *index as usize;
                    if idx < arr.len() {
                        arr[idx].clone()
                    } else {
                        Value::Undefined
                    }
                } else {
                    Value::Undefined
                }
            } else {
                Value::Undefined
            }
        },
        Value::ClassInstance(instance) => {
            if let Value::String(key) = property {
                if let Some(value) = instance.borrow().get_property(key) {
                    value
                } else {
                    Value::Undefined
                }
            } else {
                Value::Undefined
            }
        },
        Value::Null | Value::Undefined => Value::Undefined,
        _ => Value::Undefined,
    }
}

/// Implements the optional method call operator (?.())
pub fn optional_method_call(object: &Value, method: &str, args: &[Value], env: &Environment) -> Result<Value, String> {
    match object {
        Value::Object(obj) => {
            if let Some(Value::Function(func)) = obj.get(method) {
                func.call(object.clone(), args, env)
            } else {
                Ok(Value::Undefined)
            }
        },
        Value::ClassInstance(instance) => {
            if let Some(method_value) = instance.borrow().get_property(method) {
                if let Value::Function(func) = method_value {
                    func.call(object.clone(), args, env)
                } else {
                    Ok(Value::Undefined)
                }
            } else {
                Ok(Value::Undefined)
            }
        },
        Value::Null | Value::Undefined => Ok(Value::Undefined),
        _ => Ok(Value::Undefined),
    }
}

/// Implements the nullish coalescing operator (??)
pub fn nullish_coalescing(left: &Value, right: &Value) -> Value {
    match left {
        Value::Null | Value::Undefined => right.clone(),
        _ => left.clone(),
    }
}

/// Implements the logical AND operator (&&)
pub fn logical_and(left: &Value, right: &Value, _env: &Environment) -> Value {
    if !left.is_truthy() {
        left.clone()
    } else {
        right.clone()
    }
}

/// Implements the logical OR operator (||)
pub fn logical_or(left: &Value, right: &Value, _env: &Environment) -> Value {
    if left.is_truthy() {
        left.clone()
    } else {
        right.clone()
    }
}

/// Implements the nullish assignment operator (??=)
pub fn nullish_assignment(target: &mut Value, value: &Value) -> Result<(), String> {
    match target {
        Value::Null | Value::Undefined => {
            *target = value.clone();
            Ok(())
        },
        _ => Ok(()),
    }
}

/// Implements the logical AND assignment operator (&&=)
pub fn logical_and_assignment(target: &mut Value, value: &Value) -> Result<(), String> {
    if target.is_truthy() {
        *target = value.clone();
    }
    Ok(())
}

/// Implements the logical OR assignment operator (||=)
pub fn logical_or_assignment(target: &mut Value, value: &Value) -> Result<(), String> {
    if !target.is_truthy() {
        *target = value.clone();
    }
    Ok(())
}

/// Implements the destructuring assignment for arrays
pub fn destructure_array(array: &Value, targets: &[Value]) -> Result<HashMap<String, Value>, String> {
    let mut result = HashMap::new();
    
    if let Value::Array(arr) = array {
        for (i, target) in targets.iter().enumerate() {
            match target {
                Value::Identifier(name) => {
                    if i < arr.len() {
                        result.insert(name.clone(), arr[i].clone());
                    } else {
                        result.insert(name.clone(), Value::Undefined);
                    }
                },
                Value::Object(obj) => {
                    if let Some(Value::Identifier(name)) = obj.get("name") {
                        if let Some(Value::Boolean(is_rest)) = obj.get("is_rest") {
                            if *is_rest {
                                // Rest element
                                let rest_array = arr.iter().skip(i).cloned().collect();
                                result.insert(name.clone(), Value::Array(rest_array));
                                break;
                            }
                        }
                        
                        if i < arr.len() {
                            let value = arr[i].clone();
                            
                            if let Some(Value::Object(default_value)) = obj.get("default_value") {
                                if let Value::Undefined = value {
                                    // Use default value
                                    if let Some(default) = default_value.get("value") {
                                        result.insert(name.clone(), default.clone());
                                    } else {
                                        result.insert(name.clone(), Value::Undefined);
                                    }
                                } else {
                                    result.insert(name.clone(), value);
                                }
                            } else {
                                result.insert(name.clone(), value);
                            }
                        } else if let Some(Value::Object(default_value)) = obj.get("default_value") {
                            // Use default value
                            if let Some(default) = default_value.get("value") {
                                result.insert(name.clone(), default.clone());
                            } else {
                                result.insert(name.clone(), Value::Undefined);
                            }
                        } else {
                            result.insert(name.clone(), Value::Undefined);
                        }
                    }
                },
                _ => {
                    return Err(format!("Invalid destructuring target: {:?}", target));
                }
            }
        }
    } else {
        return Err(format!("Cannot destructure non-array value: {:?}", array));
    }
    
    Ok(result)
}

/// Implements the destructuring assignment for objects
pub fn destructure_object(object: &Value, targets: &[Value]) -> Result<HashMap<String, Value>, String> {
    let mut result = HashMap::new();
    
    if let Value::Object(obj) = object {
        for target in targets {
            match target {
                Value::Identifier(name) => {
                    if let Some(value) = obj.get(name) {
                        result.insert(name.clone(), value.clone());
                    } else {
                        result.insert(name.clone(), Value::Undefined);
                    }
                },
                Value::Object(target_obj) => {
                    if let Some(Value::Identifier(name)) = target_obj.get("name") {
                        if let Some(Value::Boolean(is_rest)) = target_obj.get("is_rest") {
                            if *is_rest {
                                // Rest element
                                let mut rest_obj = HashMap::new();
                                
                                for (key, value) in obj {
                                    let skip = targets.iter().any(|t| {
                                        if let Value::Identifier(n) = t {
                                            n == key
                                        } else if let Value::Object(o) = t {
                                            if let Some(Value::Identifier(n)) = o.get("name") {
                                                n == key
                                            } else if let Some(Value::String(n)) = o.get("key") {
                                                n == key
                                            } else {
                                                false
                                            }
                                        } else {
                                            false
                                        }
                                    });
                                    
                                    if !skip {
                                        rest_obj.insert(key.clone(), value.clone());
                                    }
                                }
                                
                                result.insert(name.clone(), Value::Object(rest_obj));
                                continue;
                            }
                        }
                        
                        let key = if let Some(Value::String(key)) = target_obj.get("key") {
                            key.clone()
                        } else {
                            name.clone()
                        };
                        
                        if let Some(value) = obj.get(&key) {
                            result.insert(name.clone(), value.clone());
                        } else if let Some(Value::Object(default_value)) = target_obj.get("default_value") {
                            // Use default value
                            if let Some(default) = default_value.get("value") {
                                result.insert(name.clone(), default.clone());
                            } else {
                                result.insert(name.clone(), Value::Undefined);
                            }
                        } else {
                            result.insert(name.clone(), Value::Undefined);
                        }
                    }
                },
                _ => {
                    return Err(format!("Invalid destructuring target: {:?}", target));
                }
            }
        }
    } else {
        return Err(format!("Cannot destructure non-object value: {:?}", object));
    }
    
    Ok(result)
}

/// Implements the spread operator for arrays
pub fn spread_array(array: &Value) -> Result<Vec<Value>, String> {
    match array {
        Value::Array(arr) => Ok(arr.clone()),
        Value::String(s) => {
            let chars: Vec<Value> = s.chars().map(|c| Value::String(c.to_string())).collect();
            Ok(chars)
        },
        Value::Object(obj) => {
            if let Some(Value::Function(iter_fn)) = obj.get(Symbol::Iterator.as_str()) {
                // Object is iterable
                let iterator = iter_fn.call(array.clone(), &[], &Environment::new())?;
                
                if let Value::Object(iter_obj) = iterator {
                    if let Some(Value::Function(next_fn)) = iter_obj.get("next") {
                        let mut result = Vec::new();
                        
                        loop {
                            let next_result = next_fn.call(iterator.clone(), &[], &Environment::new())?;
                            
                            if let Value::Object(next_obj) = next_result {
                                if let Some(Value::Boolean(done)) = next_obj.get("done") {
                                    if *done {
                                        break;
                                    }
                                    
                                    if let Some(value) = next_obj.get("value") {
                                        result.push(value.clone());
                                    }
                                }
                            }
                        }
                        
                        Ok(result)
                    } else {
                        Err("Iterator has no next method".to_string())
                    }
                } else {
                    Err("Iterator is not an object".to_string())
                }
            } else {
                Err("Object is not iterable".to_string())
            }
        },
        _ => Err(format!("Cannot spread non-iterable value: {:?}", array)),
    }
}

/// Implements the spread operator for objects
pub fn spread_object(object: &Value) -> Result<HashMap<String, Value>, String> {
    match object {
        Value::Object(obj) => Ok(obj.clone()),
        _ => Err(format!("Cannot spread non-object value: {:?}", object)),
    }
}

/// Symbol enum for well-known symbols
pub enum Symbol {
    Iterator,
    // Add more symbols as needed
}

impl Symbol {
    pub fn as_str(&self) -> &'static str {
        match self {
            Symbol::Iterator => "Symbol.iterator",
            // Add more symbols as needed
        }
    }
}