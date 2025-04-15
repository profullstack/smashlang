use std::collections::HashMap;
use crate::interpreter::{Value, Function, Environment};
use serde_json::{self, json, Value as JsonValue};

/// Convert a SmashLang Value to a serde_json Value
pub fn value_to_json(value: &Value) -> Result<JsonValue, String> {
    match value {
        Value::Identifier(_) => todo!("Identifier serialization not implemented"),
        Value::Null => Ok(JsonValue::Null),
        Value::Undefined => Ok(JsonValue::Null),
        Value::Boolean(b) => Ok(JsonValue::Bool(*b)),
        Value::Number(n) => {
            if n.is_nan() {
                Ok(JsonValue::Null)
            } else if n.is_infinite() {
                if n.is_sign_positive() {
                    Ok(json!("Infinity"))
                } else {
                    Ok(json!("-Infinity"))
                }
            } else {
                Ok(JsonValue::Number(serde_json::Number::from_f64(*n).unwrap_or(serde_json::Number::from(0))))
            }
        },
        Value::String(s) => Ok(JsonValue::String(s.clone())),
        Value::Array(arr) => {
            let mut json_arr = Vec::new();
            for item in arr {
                json_arr.push(value_to_json(item)?);
            }
            Ok(JsonValue::Array(json_arr))
        },
        Value::Object(obj) => {
            let mut json_obj = serde_json::Map::new();
            for (key, val) in obj {
                json_obj.insert(key.clone(), value_to_json(val)?);
            }
            Ok(JsonValue::Object(json_obj))
        },
        Value::Function(_) => Ok(JsonValue::Null),
        Value::Promise(_) => Ok(JsonValue::Null),
        Value::Class(_) => Ok(JsonValue::Null),
        Value::ClassInstance(_) => {
            // Try to convert class instance to object
            let mut json_obj = serde_json::Map::new();
            if let Value::ClassInstance(instance) = value {
                let properties = &instance.borrow().properties;
                for (key, val) in properties {
                    json_obj.insert(key.clone(), value_to_json(val)?);
                }
            }
            Ok(JsonValue::Object(json_obj))
        },
        Value::Map(map) => {
            // Convert Map to object
            let mut json_obj = serde_json::Map::new();
            for (key, val) in map.entries() {
                if let Value::String(key_str) = key {
                    json_obj.insert(key_str, value_to_json(&val)?);
                }
            }
            Ok(JsonValue::Object(json_obj))
        },
        Value::Set(set) => {
            // Convert Set to array
            let mut json_arr = Vec::new();
            for item in set.values() {
                json_arr.push(value_to_json(&item)?);
            }
            Ok(JsonValue::Array(json_arr))
        },
        Value::WeakMap(_) => Ok(JsonValue::Object(serde_json::Map::new())),
        Value::WeakSet(_) => Ok(JsonValue::Array(Vec::new())),
    }
}

/// Convert a serde_json Value to a SmashLang Value
pub fn json_to_value(json: &JsonValue) -> Value {
    match json {
        JsonValue::Null => Value::Null,
        JsonValue::Bool(b) => Value::Boolean(*b),
        JsonValue::Number(n) => {
            if let Some(f) = n.as_f64() {
                Value::Number(f)
            } else {
                Value::Number(0.0)
            }
        },
        JsonValue::String(s) => {
            if s == "Infinity" {
                Value::Number(f64::INFINITY)
            } else if s == "-Infinity" {
                Value::Number(f64::NEG_INFINITY)
            } else {
                Value::String(s.clone())
            }
        },
        JsonValue::Array(arr) => {
            let mut values = Vec::new();
            for item in arr {
                values.push(json_to_value(item));
            }
            Value::Array(values)
        },
        JsonValue::Object(obj) => {
            let mut map = HashMap::new();
            for (key, val) in obj {
                map.insert(key.clone(), json_to_value(val));
            }
            Value::Object(map)
        },
    }
}

/// Create a JSON.parse function
pub fn create_json_parse_function() -> Function {
    Function::new_native(
        Some("parse".to_string()),
        vec!["text".to_string(), "reviver".to_string()],
        |_this, args, env| {
            if args.is_empty() {
                return Err("JSON.parse requires at least one argument".to_string());
            }
            
            let text = match &args[0] {
                Value::String(s) => s.clone(),
                _ => return Err("JSON.parse argument must be a string".to_string()),
            };
            
            let reviver = if args.len() > 1 {
                match &args[1] {
                    Value::Function(f) => Some(f.clone()),
                    _ => None,
                }
            } else {
                None
            };
            
            // Parse the JSON
            let parsed = match serde_json::from_str::<JsonValue>(&text) {
                Ok(json) => json_to_value(&json),
                Err(err) => return Err(format!("JSON.parse error: {}", err)),
            };
            
            // Apply reviver if provided
            if let Some(reviver_fn) = reviver {
                apply_reviver(parsed, "", reviver_fn, env)
            } else {
                Ok(parsed)
            }
        },
    )
}

/// Apply a reviver function to a parsed JSON value
fn apply_reviver(value: Value, key: &str, reviver: Function, env: &Environment) -> Result<Value, String> {
    match value {
        Value::Identifier(_) => todo!("Identifier serialization not implemented"),
        Value::Object(obj) => {
            let mut result = HashMap::new();
            
            for (k, v) in obj {
                // Apply reviver to each property recursively
                let processed = apply_reviver(v, &k, reviver.clone(), env)?;
                
                // Call reviver on the property
                let reviver_result = reviver.call(
                    Value::Object(result.clone()),
                    &[Value::String(k.clone()), processed],
                    env,
                )?;
                
                // If reviver returns undefined, skip this property
                if !matches!(reviver_result, Value::Undefined) {
                    result.insert(k, reviver_result);
                }
            }
            
            // Call reviver on the object itself
            reviver.call(
                Value::Object(HashMap::new()),
                &[Value::String(key.to_string()), Value::Object(result)],
                env,
            )
        },
        Value::Array(arr) => {
            let mut result = Vec::new();
            
            for (i, v) in arr.iter().enumerate() {
                // Apply reviver to each element recursively
                let processed = apply_reviver(v.clone(), &i.to_string(), reviver.clone(), env)?;
                
                // Call reviver on the element
                let reviver_result = reviver.call(
                    Value::Array(result.clone()),
                    &[Value::String(i.to_string()), processed],
                    env,
                )?;
                
                // If reviver returns undefined, use null
                if matches!(reviver_result, Value::Undefined) {
                    result.push(Value::Null);
                } else {
                    result.push(reviver_result);
                }
            }
            
            // Call reviver on the array itself
            reviver.call(
                Value::Object(HashMap::new()),
                &[Value::String(key.to_string()), Value::Array(result)],
                env,
            )
        },
        _ => {
            // Call reviver on primitive values
            reviver.call(
                Value::Object(HashMap::new()),
                &[Value::String(key.to_string()), value],
                env,
            )
        },
    }
}

/// Create a JSON.stringify function
pub fn create_json_stringify_function() -> Function {
    Function::new_native(
        Some("stringify".to_string()),
        vec!["value".to_string(), "replacer".to_string(), "space".to_string()],
        |_this, args, env| {
            if args.is_empty() {
                return Ok(Value::String("undefined".to_string()));
            }
            
            let value = &args[0];
            
            let replacer = if args.len() > 1 {
                match &args[1] {
                    Value::Function(f) => Some(ReplacerType::Function(f.clone())),
                    Value::Array(arr) => {
                        let mut keys = Vec::new();
                        for item in arr {
                            if let Value::String(s) = item {
                                keys.push(s.clone());
                            } else if let Value::Number(n) = value {
                                if n.fract() == 0.0 {
                                    keys.push(n.to_string());
                                }
                            }
                        }
                        Some(ReplacerType::Array(keys))
                    },
                    _ => None,
                }
            } else {
                None
            };
            
            let space = if args.len() > 2 {
                match &args[2] {
                    Value::Number(n) => {
                        if n.fract() == 0.0 && *n >= 0.0 && *n <= 10.0 {
                            " ".repeat(*n as usize)
                        } else {
                            "".to_string()
                        }
                    },
                    Value::String(s) => {
                        if s.len() > 10 {
                            s[0..10].to_string()
                        } else {
                            s.clone()
                        }
                    },
                    _ => "".to_string(),
                }
            } else {
                "".to_string()
            };
            
            // Convert to JSON
            let json_value = match value_to_json(value) {
                Ok(json) => json,
                Err(err) => return Err(format!("JSON.stringify error: {}", err)),
            };
            
            // Apply replacer if provided
            let processed_json = if let Some(replacer_type) = replacer {
                apply_replacer(json_value, "", replacer_type, env)?
            } else {
                json_value
            };
            
            // Stringify with formatting
            let result = if space.is_empty() {
                serde_json::to_string(&processed_json)
            } else {
                serde_json::to_string_pretty(&processed_json)
            };
            
            match result {
                Ok(s) => Ok(Value::String(s)),
                Err(err) => Err(format!("JSON.stringify error: {}", err)),
            }
        },
    )
}

/// Replacer type for JSON.stringify
#[derive(Clone)]
enum ReplacerType {
    Function(Function),
    Array(Vec<String>),
}

/// Apply a replacer to a JSON value
fn apply_replacer(
    value: JsonValue,
    key: &str,
    replacer: ReplacerType,
    env: &Environment,
) -> Result<JsonValue, String> {
    match replacer {
        ReplacerType::Function(replacer_fn) => {
            // Convert JSON value to SmashLang value
            let smash_value = json_to_value(&value);
            
            // Call replacer function
            let result = replacer_fn.call(
                Value::Object(HashMap::new()),
                &[Value::String(key.to_string()), smash_value],
                env,
            )?;
            
            // Convert result back to JSON value
            value_to_json(&result)
        },
        ReplacerType::Array(ref keys) => {
            match value {
        Value::Identifier(_) => todo!("Identifier serialization not implemented"),
                JsonValue::Object(obj) => {
                    let mut result = serde_json::Map::new();
                    
                    for (k, v) in obj {
                        if keys.contains(&k) {
                            // Apply replacer to each property recursively
                            let processed = apply_replacer(v, &k, replacer.clone(), env)?;
                            result.insert(k, processed);
                        }
                    }
                    
                    Ok(JsonValue::Object(result))
                },
                JsonValue::Array(arr) => {
                    let mut result = Vec::new();
                    
                    for (i, v) in arr.iter().enumerate() {
                        // Apply replacer to each element recursively
                        let processed = apply_replacer(v.clone(), &i.to_string(), replacer.clone(), env)?;
                        result.push(processed);
                    }
                    
                    Ok(JsonValue::Array(result))
                },
                _ => Ok(value),
            }
        },
    }
}

/// Create a JSON object with parse and stringify methods
pub fn create_json_object() -> Value {
    let mut json_obj = HashMap::new();
    
    json_obj.insert("parse".to_string(), Value::Function(create_json_parse_function()));
    json_obj.insert("stringify".to_string(), Value::Function(create_json_stringify_function()));
    
    Value::Object(json_obj)
}