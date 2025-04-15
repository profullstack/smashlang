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
        Value::Map(_) => Ok(JsonValue::Null),
        Value::Set(_) => Ok(JsonValue::Null),
        Value::WeakMap(_) => Ok(JsonValue::Null),
        Value::WeakSet(_) => Ok(JsonValue::Null),
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
        JsonValue::String(s) => Value::String(s.clone()),
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

/// Create a JSON object with stringify and parse methods
pub fn create_json_object() -> Value {
    let mut json_obj = HashMap::new();
    
    // JSON.stringify
    json_obj.insert("stringify".to_string(), Value::Function(Function::new_native(
        Some("stringify".to_string()),
        vec!["value".to_string(), "replacer".to_string(), "space".to_string()],
        |_this, args, env| {
            if args.is_empty() {
                return Ok(Value::String("undefined".to_string()));
            }
            
            let value = &args[0];
            let replacer = args.get(1);
            let space = args.get(2);
            
            // Convert value to JSON
            let mut json_value = value_to_json(value)?;
            
            // Apply replacer if provided
            if let Some(replacer) = replacer {
                json_value = apply_replacer(json_value, "", replacer.clone(), env)?;
            }
            
            // Apply space if provided
            let formatted = match space {
                Some(Value::Number(n)) if *n > 0.0 => {
                    let spaces = n.min(10.0) as usize;
                    serde_json::to_string_pretty(&json_value)
                        .unwrap_or_else(|_| "null".to_string())
                        .replace("  ", &" ".repeat(spaces))
                },
                Some(Value::String(s)) if !s.is_empty() => {
                    let indent = s.chars().take(10).collect::<String>();
                    serde_json::to_string_pretty(&json_value)
                        .unwrap_or_else(|_| "null".to_string())
                        .replace("  ", &indent)
                },
                _ => serde_json::to_string(&json_value).unwrap_or_else(|_| "null".to_string()),
            };
            
            Ok(Value::String(formatted))
        },
    )));
    
    // JSON.parse
    json_obj.insert("parse".to_string(), Value::Function(Function::new_native(
        Some("parse".to_string()),
        vec!["text".to_string(), "reviver".to_string()],
        |_this, args, env| {
            if args.is_empty() {
                return Err("JSON.parse requires at least 1 argument".to_string());
            }
            
            let text = match &args[0] {
                Value::String(s) => s,
                _ => return Err("JSON.parse requires a string argument".to_string()),
            };
            
            let reviver = args.get(1);
            
            // Parse JSON
            let json_value = match serde_json::from_str::<JsonValue>(text) {
                Ok(v) => v,
                Err(e) => return Err(format!("Invalid JSON: {}", e)),
            };
            
            // Apply reviver if provided
            let value = if let Some(reviver) = reviver {
                if let Value::Function(reviver_fn) = reviver {
                    apply_reviver(json_value, "", reviver_fn.clone(), env)?
                } else {
                    json_to_value(&json_value)
                }
            } else {
                json_to_value(&json_value)
            };
            
            Ok(value)
        },
    )));
    
    Value::Object(json_obj)
}

/// Enum to represent the type of replacer
enum ReplacerType {
    Function(Function),
    Array(Vec<String>),
}

/// Apply a replacer to a JSON value
fn apply_replacer(value: JsonValue, key: &str, replacer: Value, env: &Environment) -> Result<JsonValue, String> {
    let replacer_type = match &replacer {
        Value::Function(f) => ReplacerType::Function(f.clone()),
        Value::Array(ref arr) => {
            let mut keys = Vec::new();
            for item in arr {
                if let Value::String(s) = item {
                    keys.push(s.clone());
                }
            }
            ReplacerType::Array(keys)
        },
        _ => return Ok(value),
    };
    
    match replacer_type {
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
                _ => Ok(value),
            }
        },
    }
}

/// Apply a reviver to a JSON value
fn apply_reviver(value: JsonValue, key: &str, reviver: Function, env: &Environment) -> Result<Value, String> {
    match value {
        JsonValue::Object(obj) => {
            let mut result = HashMap::new();
            
            for (k, v) in obj {
                // Apply reviver to each property recursively
                let processed = apply_reviver(v, &k, reviver.clone(), env)?;
                
                // Call reviver function
                let reviver_result = reviver.call(
                    Value::Object(result.clone()),
                    &[Value::String(k.clone()), processed],
                    env,
                )?;
                
                // If reviver returns undefined, remove the property
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
        JsonValue::Array(arr) => {
            let mut result = Vec::new();
            
            for (i, v) in arr.iter().enumerate() {
                // Apply reviver to each element recursively
                let processed = apply_reviver(v.clone(), &i.to_string(), reviver.clone(), env)?;
                
                // Call reviver function
                let reviver_result = reviver.call(
                    Value::Array(result.clone()),
                    &[Value::String(i.to_string()), processed],
                    env,
                )?;
                
                // If reviver returns undefined, set element to null
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
            // Convert JSON value to SmashLang value
            let smash_value = json_to_value(&value);
            
            // Call reviver function
            reviver.call(
                Value::Object(HashMap::new()),
                &[Value::String(key.to_string()), smash_value],
                env,
            )
        },
    }
}