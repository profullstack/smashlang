use std::collections::{HashMap, HashSet};
use std::cell::RefCell;
use std::rc::Rc;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use crate::interpreter::{Value, Function, Environment};

/// Represents a Map in the SmashLang runtime
#[derive(Debug, Clone)]
pub struct Map {
    /// Internal storage for the map
    entries: RefCell<HashMap<MapKey, Value>>,
}

impl Map {
    /// Create a new empty Map
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            entries: RefCell::new(HashMap::new()),
        })
    }
    
    /// Create a Map from an iterable
    pub fn from_iterable(iterable: &Value) -> Result<Rc<Self>, String> {
        let map = Self::new();
        
        match iterable {
            Value::Array(items) => {
                for item in items {
                    if let Value::Array(entry) = item {
                        if entry.len() >= 2 {
                            let key = &entry[0];
                            let value = &entry[1];
                            map.set(key.clone(), value.clone())?;
                        } else {
                            return Err("Map entries must have at least 2 elements".to_string());
                        }
                    } else {
                        return Err("Map entries must be arrays".to_string());
                    }
                }
            },
            Value::Object(obj) => {
                for (key, value) in obj {
                    map.set(Value::String(key.clone()), value.clone())?;
                }
            },
            Value::Map(other_map) => {
                for (key, value) in other_map.entries() {
                    map.set(key, value)?;
                }
            },
            _ => {
                return Err(format!("Cannot create Map from {}", iterable.type_name()));
            }
        }
        
        Ok(map)
    }
    
    /// Set a key-value pair in the map
    pub fn set(&self, key: Value, value: Value) -> Result<Value, String> {
        let map_key = MapKey::from_value(key.clone())?;
        self.entries.borrow_mut().insert(map_key, value);
        Ok(Value::Map(self.clone()))
    }
    
    /// Get a value from the map
    pub fn get(&self, key: &Value) -> Value {
        match MapKey::from_value(key.clone()) {
            Ok(map_key) => {
                match self.entries.borrow().get(&map_key) {
                    Some(value) => value.clone(),
                    None => Value::Undefined,
                }
            },
            Err(_) => Value::Undefined,
        }
    }
    
    /// Check if the map has a key
    pub fn has(&self, key: &Value) -> bool {
        match MapKey::from_value(key.clone()) {
            Ok(map_key) => self.entries.borrow().contains_key(&map_key),
            Err(_) => false,
        }
    }
    
    /// Delete a key from the map
    pub fn delete(&self, key: &Value) -> bool {
        match MapKey::from_value(key.clone()) {
            Ok(map_key) => self.entries.borrow_mut().remove(&map_key).is_some(),
            Err(_) => false,
        }
    }
    
    /// Clear all entries from the map
    pub fn clear(&self) {
        self.entries.borrow_mut().clear();
    }
    
    /// Get the number of entries in the map
    pub fn size(&self) -> usize {
        self.entries.borrow().len()
    }
    
    /// Get all entries in the map
    pub fn entries(&self) -> Vec<(Value, Value)> {
        self.entries.borrow().iter()
            .map(|(k, v)| (k.to_value(), v.clone()))
            .collect()
    }
    
    /// Get all keys in the map
    pub fn keys(&self) -> Vec<Value> {
        self.entries.borrow().keys()
            .map(|k| k.to_value())
            .collect()
    }
    
    /// Get all values in the map
    pub fn values(&self) -> Vec<Value> {
        self.entries.borrow().values()
            .cloned()
            .collect()
    }
    
    /// Execute a callback for each entry in the map
    pub fn for_each<F>(&self, callback: F) -> Result<(), String>
    where
        F: Fn(Value, Value, Value) -> Result<(), String>,
    {
        for (key, value) in self.entries() {
            callback(value, key.clone(), Value::Map(self.clone()))?;
        }
        Ok(())
    }
}

/// Key type for Map
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum MapKey {
    Null,
    Boolean(bool),
    Number(i64),
    String(String),
    Object(usize), // Use object identity (memory address)
}

impl MapKey {
    /// Create a MapKey from a Value
    fn from_value(value: Value) -> Result<Self, String> {
        match value {
            Value::Null => Ok(MapKey::Null),
            Value::Boolean(b) => Ok(MapKey::Boolean(b)),
            Value::Number(n) => {
                if n.is_nan() {
                    Ok(MapKey::Number(0))
                } else if n.is_infinite() {
                    if n.is_sign_positive() {
                        Ok(MapKey::Number(i64::MAX))
                    } else {
                        Ok(MapKey::Number(i64::MIN))
                    }
                } else if n.fract() == 0.0 && n >= i64::MIN as f64 && n <= i64::MAX as f64 {
                    Ok(MapKey::Number(n as i64))
                } else {
                    // Hash the float value
                    let mut hasher = DefaultHasher::new();
                    n.to_bits().hash(&mut hasher);
                    Ok(MapKey::Number(hasher.finish() as i64))
                }
            },
            Value::String(s) => Ok(MapKey::String(s)),
            Value::Object(_) | Value::Array(_) | Value::Function(_) | Value::Promise(_) | 
            Value::Class(_) | Value::ClassInstance(_) | Value::Map(_) | Value::Set(_) | 
            Value::WeakMap(_) | Value::WeakSet(_) => {
                // Use object identity (memory address)
                let ptr = &value as *const Value as usize;
                Ok(MapKey::Object(ptr))
            },
            Value::Undefined => Err("Cannot use undefined as a Map key".to_string()),
        }
    }
    
    /// Convert a MapKey back to a Value
    fn to_value(&self) -> Value {
        match self {
            MapKey::Null => Value::Null,
            MapKey::Boolean(b) => Value::Boolean(*b),
            MapKey::Number(n) => Value::Number(*n as f64),
            MapKey::String(s) => Value::String(s.clone()),
            MapKey::Object(_) => Value::Object(HashMap::new()), // Cannot recover original object
        }
    }
}

/// Represents a Set in the SmashLang runtime
#[derive(Debug, Clone)]
pub struct Set {
    /// Internal storage for the set
    values: RefCell<HashSet<MapKey>>,
}

impl Set {
    /// Create a new empty Set
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            values: RefCell::new(HashSet::new()),
        })
    }
    
    /// Create a Set from an iterable
    pub fn from_iterable(iterable: &Value) -> Result<Rc<Self>, String> {
        let set = Self::new();
        
        match iterable {
            Value::Array(items) => {
                for item in items {
                    set.add(item.clone())?;
                }
            },
            Value::String(s) => {
                for c in s.chars() {
                    set.add(Value::String(c.to_string()))?;
                }
            },
            Value::Set(other_set) => {
                for value in other_set.values() {
                    set.add(value)?;
                }
            },
            Value::Map(map) => {
                for (key, _) in map.entries() {
                    set.add(key)?;
                }
            },
            Value::Object(obj) => {
                for key in obj.keys() {
                    set.add(Value::String(key.clone()))?;
                }
            },
            _ => {
                return Err(format!("Cannot create Set from {}", iterable.type_name()));
            }
        }
        
        Ok(set)
    }
    
    /// Add a value to the set
    pub fn add(&self, value: Value) -> Result<Value, String> {
        let map_key = MapKey::from_value(value.clone())?;
        self.values.borrow_mut().insert(map_key);
        Ok(Value::Set(self.clone()))
    }
    
    /// Check if the set has a value
    pub fn has(&self, value: &Value) -> bool {
        match MapKey::from_value(value.clone()) {
            Ok(map_key) => self.values.borrow().contains(&map_key),
            Err(_) => false,
        }
    }
    
    /// Delete a value from the set
    pub fn delete(&self, value: &Value) -> bool {
        match MapKey::from_value(value.clone()) {
            Ok(map_key) => self.values.borrow_mut().remove(&map_key),
            Err(_) => false,
        }
    }
    
    /// Clear all values from the set
    pub fn clear(&self) {
        self.values.borrow_mut().clear();
    }
    
    /// Get the number of values in the set
    pub fn size(&self) -> usize {
        self.values.borrow().len()
    }
    
    /// Get all values in the set
    pub fn values(&self) -> Vec<Value> {
        self.values.borrow().iter()
            .map(|k| k.to_value())
            .collect()
    }
    
    /// Execute a callback for each value in the set
    pub fn for_each<F>(&self, callback: F) -> Result<(), String>
    where
        F: Fn(Value, Value, Value) -> Result<(), String>,
    {
        for value in self.values() {
            callback(value.clone(), value.clone(), Value::Set(self.clone()))?;
        }
        Ok(())
    }
}

/// Represents a WeakMap in the SmashLang runtime
#[derive(Debug, Clone)]
pub struct WeakMap {
    /// Internal storage for the weak map
    entries: RefCell<HashMap<usize, Value>>,
}

impl WeakMap {
    /// Create a new empty WeakMap
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            entries: RefCell::new(HashMap::new()),
        })
    }
    
    /// Create a WeakMap from an iterable
    pub fn from_iterable(iterable: &Value) -> Result<Rc<Self>, String> {
        let weak_map = Self::new();
        
        match iterable {
            Value::Array(items) => {
                for item in items {
                    if let Value::Array(entry) = item {
                        if entry.len() >= 2 {
                            let key = &entry[0];
                            let value = &entry[1];
                            weak_map.set(key.clone(), value.clone())?;
                        } else {
                            return Err("WeakMap entries must have at least 2 elements".to_string());
                        }
                    } else {
                        return Err("WeakMap entries must be arrays".to_string());
                    }
                }
            },
            Value::WeakMap(_other_map) => {
                // Cannot iterate over WeakMap entries
                return Err("Cannot create WeakMap from another WeakMap".to_string());
            },
            _ => {
                return Err(format!("Cannot create WeakMap from {}", iterable.type_name()));
            }
        }
        
        Ok(weak_map)
    }
    
    /// Set a key-value pair in the weak map
    pub fn set(&self, key: Value, value: Value) -> Result<Value, String> {
        match key {
            Value::Object(_) | Value::Array(_) | Value::Function(_) | Value::Promise(_) | 
            Value::Class(_) | Value::ClassInstance(_) | Value::Map(_) | Value::Set(_) | 
            Value::WeakMap(_) | Value::WeakSet(_) => {
                let ptr = &key as *const Value as usize;
                self.entries.borrow_mut().insert(ptr, value);
                Ok(Value::WeakMap(self.clone()))
            },
            _ => Err("WeakMap keys must be objects".to_string()),
        }
    }
    
    /// Get a value from the weak map
    pub fn get(&self, key: &Value) -> Value {
        match key {
            Value::Object(_) | Value::Array(_) | Value::Function(_) | Value::Promise(_) | 
            Value::Class(_) | Value::ClassInstance(_) | Value::Map(_) | Value::Set(_) | 
            Value::WeakMap(_) | Value::WeakSet(_) => {
                let ptr = key as *const Value as usize;
                match self.entries.borrow().get(&ptr) {
                    Some(value) => value.clone(),
                    None => Value::Undefined,
                }
            },
            _ => Value::Undefined,
        }
    }
    
    /// Check if the weak map has a key
    pub fn has(&self, key: &Value) -> bool {
        match key {
            Value::Object(_) | Value::Array(_) | Value::Function(_) | Value::Promise(_) | 
            Value::Class(_) | Value::ClassInstance(_) | Value::Map(_) | Value::Set(_) | 
            Value::WeakMap(_) | Value::WeakSet(_) => {
                let ptr = key as *const Value as usize;
                self.entries.borrow().contains_key(&ptr)
            },
            _ => false,
        }
    }
    
    /// Delete a key from the weak map
    pub fn delete(&self, key: &Value) -> bool {
        match key {
            Value::Object(_) | Value::Array(_) | Value::Function(_) | Value::Promise(_) | 
            Value::Class(_) | Value::ClassInstance(_) | Value::Map(_) | Value::Set(_) | 
            Value::WeakMap(_) | Value::WeakSet(_) => {
                let ptr = key as *const Value as usize;
                self.entries.borrow_mut().remove(&ptr).is_some()
            },
            _ => false,
        }
    }
}

/// Represents a WeakSet in the SmashLang runtime
#[derive(Debug, Clone)]
pub struct WeakSet {
    /// Internal storage for the weak set
    values: RefCell<HashSet<usize>>,
}

impl WeakSet {
    /// Create a new empty WeakSet
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            values: RefCell::new(HashSet::new()),
        })
    }
    
    /// Create a WeakSet from an iterable
    pub fn from_iterable(iterable: &Value) -> Result<Rc<Self>, String> {
        let weak_set = Self::new();
        
        match iterable {
            Value::Array(items) => {
                for item in items {
                    weak_set.add(item.clone())?;
                }
            },
            Value::WeakSet(_other_set) => {
                // Cannot iterate over WeakSet values
                return Err("Cannot create WeakSet from another WeakSet".to_string());
            },
            _ => {
                return Err(format!("Cannot create WeakSet from {}", iterable.type_name()));
            }
        }
        
        Ok(weak_set)
    }
    
    /// Add a value to the weak set
    pub fn add(&self, value: Value) -> Result<Value, String> {
        match value {
            Value::Object(_) | Value::Array(_) | Value::Function(_) | Value::Promise(_) | 
            Value::Class(_) | Value::ClassInstance(_) | Value::Map(_) | Value::Set(_) | 
            Value::WeakMap(_) | Value::WeakSet(_) => {
                let ptr = &value as *const Value as usize;
                self.values.borrow_mut().insert(ptr);
                Ok(Value::WeakSet(self.clone()))
            },
            _ => Err("WeakSet values must be objects".to_string()),
        }
    }
    
    /// Check if the weak set has a value
    pub fn has(&self, value: &Value) -> bool {
        match value {
            Value::Object(_) | Value::Array(_) | Value::Function(_) | Value::Promise(_) | 
            Value::Class(_) | Value::ClassInstance(_) | Value::Map(_) | Value::Set(_) | 
            Value::WeakMap(_) | Value::WeakSet(_) => {
                let ptr = value as *const Value as usize;
                self.values.borrow().contains(&ptr)
            },
            _ => false,
        }
    }
    
    /// Delete a value from the weak set
    pub fn delete(&self, value: &Value) -> bool {
        match value {
            Value::Object(_) | Value::Array(_) | Value::Function(_) | Value::Promise(_) | 
            Value::Class(_) | Value::ClassInstance(_) | Value::Map(_) | Value::Set(_) | 
            Value::WeakMap(_) | Value::WeakSet(_) => {
                let ptr = value as *const Value as usize;
                self.values.borrow_mut().remove(&ptr)
            },
            _ => false,
        }
    }
}