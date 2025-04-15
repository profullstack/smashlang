use smashlang::{execute, Value};

#[test]
fn test_map_basic() {
    let code = r#"
        // Create a new Map
        const map = new Map();
        
        // Set values
        map.set('key1', 'value1');
        map.set('key2', 'value2');
        map.set(42, 'number key');
        
        // Get values
        const value1 = map.get('key1');
        const value2 = map.get('key2');
        const value3 = map.get(42);
        const value4 = map.get('nonexistent');
        
        // Check if map has keys
        const has1 = map.has('key1');
        const has2 = map.has('nonexistent');
        
        // Get size
        const size = map.size;
        
        [value1, value2, value3, value4, has1, has2, size];
    "#;
    
    let result = execute(code).unwrap();
    
    if let Value::Array(values) = result {
        assert_eq!(values.len(), 7);
        assert_eq!(values[0].to_string(), "value1");
        assert_eq!(values[1].to_string(), "value2");
        assert_eq!(values[2].to_string(), "number key");
        assert_eq!(values[3].to_string(), "undefined");
        assert_eq!(values[4].to_string(), "true");
        assert_eq!(values[5].to_string(), "false");
        assert_eq!(values[6].to_string(), "3");
    } else {
        panic!("Expected array result");
    }
}

#[test]
fn test_map_with_objects() {
    let code = r#"
        // Create objects to use as keys
        const obj1 = { id: 1 };
        const obj2 = { id: 2 };
        
        // Create a new Map
        const map = new Map();
        
        // Set values with object keys
        map.set(obj1, 'object 1');
        map.set(obj2, 'object 2');
        
        // Get values
        const value1 = map.get(obj1);
        const value2 = map.get(obj2);
        const value3 = map.get({ id: 1 }); // Different object with same content
        
        // Check if map has keys
        const has1 = map.has(obj1);
        const has2 = map.has({ id: 1 }); // Different object with same content
        
        [value1, value2, value3, has1, has2];
    "#;
    
    let result = execute(code).unwrap();
    
    if let Value::Array(values) = result {
        assert_eq!(values.len(), 5);
        assert_eq!(values[0].to_string(), "object 1");
        assert_eq!(values[1].to_string(), "object 2");
        assert_eq!(values[2].to_string(), "undefined"); // Different object with same content
        assert_eq!(values[3].to_string(), "true");
        assert_eq!(values[4].to_string(), "false"); // Different object with same content
    } else {
        panic!("Expected array result");
    }
}

#[test]
fn test_map_delete_and_clear() {
    let code = r#"
        // Create a new Map
        const map = new Map();
        
        // Set values
        map.set('key1', 'value1');
        map.set('key2', 'value2');
        map.set('key3', 'value3');
        
        // Delete a key
        const deleted1 = map.delete('key2');
        const deleted2 = map.delete('nonexistent');
        
        // Check size after delete
        const sizeAfterDelete = map.size;
        
        // Clear the map
        map.clear();
        
        // Check size after clear
        const sizeAfterClear = map.size;
        
        [deleted1, deleted2, sizeAfterDelete, sizeAfterClear];
    "#;
    
    let result = execute(code).unwrap();
    
    if let Value::Array(values) = result {
        assert_eq!(values.len(), 4);
        assert_eq!(values[0].to_string(), "true");
        assert_eq!(values[1].to_string(), "false");
        assert_eq!(values[2].to_string(), "2");
        assert_eq!(values[3].to_string(), "0");
    } else {
        panic!("Expected array result");
    }
}

#[test]
fn test_map_iteration() {
    let code = r#"
        // Create a new Map
        const map = new Map();
        
        // Set values
        map.set('key1', 'value1');
        map.set('key2', 'value2');
        map.set('key3', 'value3');
        
        // Get keys
        const keys = Array.from(map.keys());
        
        // Get values
        const values = Array.from(map.values());
        
        // Get entries
        const entries = Array.from(map.entries());
        
        // Use forEach
        const forEachResults = [];
        map.forEach((value, key) => {
            forEachResults.push(`${key}:${value}`);
        });
        
        [keys, values, entries, forEachResults];
    "#;
    
    let result = execute(code).unwrap();
    
    if let Value::Array(values) = result {
        assert_eq!(values.len(), 4);
        
        // Check keys
        if let Value::Array(keys) = &values[0] {
            assert_eq!(keys.len(), 3);
            assert!(keys.iter().any(|k| k.to_string() == "key1"));
            assert!(keys.iter().any(|k| k.to_string() == "key2"));
            assert!(keys.iter().any(|k| k.to_string() == "key3"));
        } else {
            panic!("Expected array for keys");
        }
        
        // Check values
        if let Value::Array(vals) = &values[1] {
            assert_eq!(vals.len(), 3);
            assert!(vals.iter().any(|v| v.to_string() == "value1"));
            assert!(vals.iter().any(|v| v.to_string() == "value2"));
            assert!(vals.iter().any(|v| v.to_string() == "value3"));
        } else {
            panic!("Expected array for values");
        }
        
        // Check entries
        if let Value::Array(entries) = &values[2] {
            assert_eq!(entries.len(), 3);
            
            for entry in entries {
                if let Value::Array(entry_arr) = entry {
                    assert_eq!(entry_arr.len(), 2);
                    let key = &entry_arr[0];
                    let value = &entry_arr[1];
                    
                    if key.to_string() == "key1" {
                        assert_eq!(value.to_string(), "value1");
                    } else if key.to_string() == "key2" {
                        assert_eq!(value.to_string(), "value2");
                    } else if key.to_string() == "key3" {
                        assert_eq!(value.to_string(), "value3");
                    } else {
                        panic!("Unexpected key in entries");
                    }
                } else {
                    panic!("Expected array for entry");
                }
            }
        } else {
            panic!("Expected array for entries");
        }
        
        // Check forEach results
        if let Value::Array(for_each) = &values[3] {
            assert_eq!(for_each.len(), 3);
            assert!(for_each.iter().any(|r| r.to_string() == "key1:value1"));
            assert!(for_each.iter().any(|r| r.to_string() == "key2:value2"));
            assert!(for_each.iter().any(|r| r.to_string() == "key3:value3"));
        } else {
            panic!("Expected array for forEach results");
        }
    } else {
        panic!("Expected array result");
    }
}

#[test]
fn test_map_constructor() {
    let code = r#"
        // Create a Map from an array of key-value pairs
        const map1 = new Map([
            ['key1', 'value1'],
            ['key2', 'value2'],
            [42, 'number key']
        ]);
        
        // Create a Map from another Map
        const map2 = new Map(map1);
        
        // Create a Map from an object (keys will be strings)
        const obj = { key1: 'value1', key2: 'value2' };
        const map3 = new Map(Object.entries(obj));
        
        [map1.size, map2.size, map3.size];
    "#;
    
    let result = execute(code).unwrap();
    
    if let Value::Array(values) = result {
        assert_eq!(values.len(), 3);
        assert_eq!(values[0].to_string(), "3");
        assert_eq!(values[1].to_string(), "3");
        assert_eq!(values[2].to_string(), "2");
    } else {
        panic!("Expected array result");
    }
}

#[test]
fn test_set_basic() {
    let code = r#"
        // Create a new Set
        const set = new Set();
        
        // Add values
        set.add('value1');
        set.add('value2');
        set.add('value1'); // Duplicate, should be ignored
        set.add(42);
        
        // Check if set has values
        const has1 = set.has('value1');
        const has2 = set.has('nonexistent');
        
        // Get size
        const size = set.size;
        
        [has1, has2, size];
    "#;
    
    let result = execute(code).unwrap();
    
    if let Value::Array(values) = result {
        assert_eq!(values.len(), 3);
        assert_eq!(values[0].to_string(), "true");
        assert_eq!(values[1].to_string(), "false");
        assert_eq!(values[2].to_string(), "3"); // value1, value2, 42 (duplicate value1 ignored)
    } else {
        panic!("Expected array result");
    }
}

#[test]
fn test_set_with_objects() {
    let code = r#"
        // Create objects to add to the set
        const obj1 = { id: 1 };
        const obj2 = { id: 2 };
        
        // Create a new Set
        const set = new Set();
        
        // Add objects
        set.add(obj1);
        set.add(obj2);
        set.add(obj1); // Duplicate, should be ignored
        
        // Check if set has objects
        const has1 = set.has(obj1);
        const has2 = set.has({ id: 1 }); // Different object with same content
        
        // Get size
        const size = set.size;
        
        [has1, has2, size];
    "#;
    
    let result = execute(code).unwrap();
    
    if let Value::Array(values) = result {
        assert_eq!(values.len(), 3);
        assert_eq!(values[0].to_string(), "true");
        assert_eq!(values[1].to_string(), "false"); // Different object with same content
        assert_eq!(values[2].to_string(), "2"); // obj1, obj2 (duplicate obj1 ignored)
    } else {
        panic!("Expected array result");
    }
}

#[test]
fn test_set_delete_and_clear() {
    let code = r#"
        // Create a new Set
        const set = new Set();
        
        // Add values
        set.add('value1');
        set.add('value2');
        set.add('value3');
        
        // Delete a value
        const deleted1 = set.delete('value2');
        const deleted2 = set.delete('nonexistent');
        
        // Check size after delete
        const sizeAfterDelete = set.size;
        
        // Clear the set
        set.clear();
        
        // Check size after clear
        const sizeAfterClear = set.size;
        
        [deleted1, deleted2, sizeAfterDelete, sizeAfterClear];
    "#;
    
    let result = execute(code).unwrap();
    
    if let Value::Array(values) = result {
        assert_eq!(values.len(), 4);
        assert_eq!(values[0].to_string(), "true");
        assert_eq!(values[1].to_string(), "false");
        assert_eq!(values[2].to_string(), "2");
        assert_eq!(values[3].to_string(), "0");
    } else {
        panic!("Expected array result");
    }
}

#[test]
fn test_set_iteration() {
    let code = r#"
        // Create a new Set
        const set = new Set();
        
        // Add values
        set.add('value1');
        set.add('value2');
        set.add('value3');
        
        // Get values
        const values = Array.from(set.values());
        
        // Get entries (key and value are the same in a Set)
        const entries = Array.from(set.entries());
        
        // Use forEach
        const forEachResults = [];
        set.forEach(value => {
            forEachResults.push(value);
        });
        
        [values, entries, forEachResults];
    "#;
    
    let result = execute(code).unwrap();
    
    if let Value::Array(values) = result {
        assert_eq!(values.len(), 3);
        
        // Check values
        if let Value::Array(vals) = &values[0] {
            assert_eq!(vals.len(), 3);
            assert!(vals.iter().any(|v| v.to_string() == "value1"));
            assert!(vals.iter().any(|v| v.to_string() == "value2"));
            assert!(vals.iter().any(|v| v.to_string() == "value3"));
        } else {
            panic!("Expected array for values");
        }
        
        // Check entries
        if let Value::Array(entries) = &values[1] {
            assert_eq!(entries.len(), 3);
            
            for entry in entries {
                if let Value::Array(entry_arr) = entry {
                    assert_eq!(entry_arr.len(), 2);
                    let key = &entry_arr[0];
                    let value = &entry_arr[1];
                    
                    // In a Set, key and value are the same
                    assert_eq!(key.to_string(), value.to_string());
                    assert!(["value1", "value2", "value3"].contains(&key.to_string().as_str()));
                } else {
                    panic!("Expected array for entry");
                }
            }
        } else {
            panic!("Expected array for entries");
        }
        
        // Check forEach results
        if let Value::Array(for_each) = &values[2] {
            assert_eq!(for_each.len(), 3);
            assert!(for_each.iter().any(|v| v.to_string() == "value1"));
            assert!(for_each.iter().any(|v| v.to_string() == "value2"));
            assert!(for_each.iter().any(|v| v.to_string() == "value3"));
        } else {
            panic!("Expected array for forEach results");
        }
    } else {
        panic!("Expected array result");
    }
}

#[test]
fn test_set_constructor() {
    let code = r#"
        // Create a Set from an array
        const set1 = new Set(['value1', 'value2', 'value1', 42]);
        
        // Create a Set from another Set
        const set2 = new Set(set1);
        
        // Create a Set from a string (each character becomes an element)
        const set3 = new Set('hello');
        
        [set1.size, set2.size, set3.size];
    "#;
    
    let result = execute(code).unwrap();
    
    if let Value::Array(values) = result {
        assert_eq!(values.len(), 3);
        assert_eq!(values[0].to_string(), "3"); // value1, value2, 42 (duplicate value1 ignored)
        assert_eq!(values[1].to_string(), "3");
        assert_eq!(values[2].to_string(), "4"); // h, e, l, o (duplicate l ignored)
    } else {
        panic!("Expected array result");
    }
}

#[test]
fn test_weak_map() {
    let code = r#"
        // Create a new WeakMap
        const weakMap = new WeakMap();
        
        // Create objects to use as keys
        const obj1 = { id: 1 };
        const obj2 = { id: 2 };
        
        // Set values
        weakMap.set(obj1, 'object 1');
        weakMap.set(obj2, 'object 2');
        
        // Try to set a non-object key (should throw an error)
        let errorMessage = '';
        try {
            weakMap.set('string key', 'value');
        } catch (error) {
            errorMessage = error.message;
        }
        
        // Get values
        const value1 = weakMap.get(obj1);
        const value2 = weakMap.get(obj2);
        
        // Check if weakMap has keys
        const has1 = weakMap.has(obj1);
        const has2 = weakMap.has({ id: 1 }); // Different object with same content
        
        // Delete a key
        const deleted = weakMap.delete(obj1);
        const hasAfterDelete = weakMap.has(obj1);
        
        [value1, value2, has1, has2, deleted, hasAfterDelete, errorMessage.includes('must be objects')];
    "#;
    
    let result = execute(code).unwrap();
    
    if let Value::Array(values) = result {
        assert_eq!(values.len(), 7);
        assert_eq!(values[0].to_string(), "object 1");
        assert_eq!(values[1].to_string(), "object 2");
        assert_eq!(values[2].to_string(), "true");
        assert_eq!(values[3].to_string(), "false"); // Different object with same content
        assert_eq!(values[4].to_string(), "true");
        assert_eq!(values[5].to_string(), "false");
        assert_eq!(values[6].to_string(), "true"); // Error message contains "must be objects"
    } else {
        panic!("Expected array result");
    }
}

#[test]
fn test_weak_set() {
    let code = r#"
        // Create a new WeakSet
        const weakSet = new WeakSet();
        
        // Create objects to add to the set
        const obj1 = { id: 1 };
        const obj2 = { id: 2 };
        
        // Add objects
        weakSet.add(obj1);
        weakSet.add(obj2);
        
        // Try to add a non-object value (should throw an error)
        let errorMessage = '';
        try {
            weakSet.add('string value');
        } catch (error) {
            errorMessage = error.message;
        }
        
        // Check if weakSet has objects
        const has1 = weakSet.has(obj1);
        const has2 = weakSet.has({ id: 1 }); // Different object with same content
        
        // Delete an object
        const deleted = weakSet.delete(obj1);
        const hasAfterDelete = weakSet.has(obj1);
        
        [has1, has2, deleted, hasAfterDelete, errorMessage.includes('must be objects')];
    "#;
    
    let result = execute(code).unwrap();
    
    if let Value::Array(values) = result {
        assert_eq!(values.len(), 5);
        assert_eq!(values[0].to_string(), "true");
        assert_eq!(values[1].to_string(), "false"); // Different object with same content
        assert_eq!(values[2].to_string(), "true");
        assert_eq!(values[3].to_string(), "false");
        assert_eq!(values[4].to_string(), "true"); // Error message contains "must be objects"
    } else {
        panic!("Expected array result");
    }
}