use smashlang::{execute, Value};

#[test]
fn test_optional_chaining() {
    let code = r#"
        const user = {
            name: "John",
            address: {
                city: "New York",
                zip: "10001"
            }
        };
        
        // Optional property access
        const city = user?.address?.city;
        
        // Optional property access with undefined
        const country = user?.address?.country;
        
        // Optional property access with null
        const nullObj = null;
        const nullProp = nullObj?.property;
        
        [city, country, nullProp];
    "#;
    
    let result = execute(code).unwrap();
    
    if let Value::Array(values) = result {
        assert_eq!(values.len(), 3);
        assert_eq!(values[0].to_string(), "New York");
        assert_eq!(values[1].to_string(), "undefined");
        assert_eq!(values[2].to_string(), "undefined");
    } else {
        panic!("Expected array result");
    }
}

#[test]
fn test_optional_computed_property_access() {
    let code = r#"
        const user = {
            name: "John",
            "full-name": "John Doe",
            address: {
                city: "New York",
                zip: "10001"
            }
        };
        
        // Optional computed property access
        const fullName = user?.["full-name"];
        
        // Optional computed property access with undefined
        const country = user?.address?.["country"];
        
        // Optional computed property access with null
        const nullObj = null;
        const nullProp = nullObj?.["property"];
        
        [fullName, country, nullProp];
    "#;
    
    let result = execute(code).unwrap();
    
    if let Value::Array(values) = result {
        assert_eq!(values.len(), 3);
        assert_eq!(values[0].to_string(), "John Doe");
        assert_eq!(values[1].to_string(), "undefined");
        assert_eq!(values[2].to_string(), "undefined");
    } else {
        panic!("Expected array result");
    }
}

#[test]
fn test_optional_method_call() {
    let code = r#"
        const user = {
            name: "John",
            greet() {
                return `Hello, ${this.name}!`;
            },
            address: {
                getCity() {
                    return "New York";
                }
            }
        };
        
        // Optional method call
        const greeting = user?.greet?.();
        
        // Optional method call with undefined method
        const farewell = user?.farewell?.();
        
        // Optional method call with null
        const nullObj = null;
        const nullMethod = nullObj?.method?.();
        
        // Optional chained method call
        const city = user?.address?.getCity?.();
        
        [greeting, farewell, nullMethod, city];
    "#;
    
    let result = execute(code).unwrap();
    
    if let Value::Array(values) = result {
        assert_eq!(values.len(), 4);
        assert_eq!(values[0].to_string(), "Hello, John!");
        assert_eq!(values[1].to_string(), "undefined");
        assert_eq!(values[2].to_string(), "undefined");
        assert_eq!(values[3].to_string(), "New York");
    } else {
        panic!("Expected array result");
    }
}

#[test]
fn test_nullish_coalescing() {
    let code = r#"
        const a = null;
        const b = undefined;
        const c = 0;
        const d = "";
        const e = false;
        const f = "value";
        
        // Nullish coalescing with null
        const a_result = a ?? "default";
        
        // Nullish coalescing with undefined
        const b_result = b ?? "default";
        
        // Nullish coalescing with falsy values (not null or undefined)
        const c_result = c ?? "default";
        const d_result = d ?? "default";
        const e_result = e ?? "default";
        
        // Nullish coalescing with truthy value
        const f_result = f ?? "default";
        
        // Chained nullish coalescing
        const g_result = a ?? b ?? c ?? "default";
        
        [a_result, b_result, c_result, d_result, e_result, f_result, g_result];
    "#;
    
    let result = execute(code).unwrap();
    
    if let Value::Array(values) = result {
        assert_eq!(values.len(), 7);
        assert_eq!(values[0].to_string(), "default");
        assert_eq!(values[1].to_string(), "default");
        assert_eq!(values[2].to_string(), "0");
        assert_eq!(values[3].to_string(), "");
        assert_eq!(values[4].to_string(), "false");
        assert_eq!(values[5].to_string(), "value");
        assert_eq!(values[6].to_string(), "0");
    } else {
        panic!("Expected array result");
    }
}

#[test]
fn test_nullish_assignment() {
    let code = r#"
        let a = null;
        let b = undefined;
        let c = 0;
        let d = "value";
        
        // Nullish assignment with null
        a ??= "default";
        
        // Nullish assignment with undefined
        b ??= "default";
        
        // Nullish assignment with falsy value (not null or undefined)
        c ??= "default";
        
        // Nullish assignment with truthy value
        d ??= "default";
        
        [a, b, c, d];
    "#;
    
    let result = execute(code).unwrap();
    
    if let Value::Array(values) = result {
        assert_eq!(values.len(), 4);
        assert_eq!(values[0].to_string(), "default");
        assert_eq!(values[1].to_string(), "default");
        assert_eq!(values[2].to_string(), "0");
        assert_eq!(values[3].to_string(), "value");
    } else {
        panic!("Expected array result");
    }
}

#[test]
fn test_logical_assignment() {
    let code = r#"
        let a = null;
        let b = undefined;
        let c = 0;
        let d = "";
        let e = false;
        let f = "value";
        let g = true;
        
        // Logical AND assignment
        a &&= "value";
        b &&= "value";
        c &&= "value";
        d &&= "value";
        e &&= "value";
        f &&= "new value";
        g &&= "new value";
        
        const and_results = [a, b, c, d, e, f, g];
        
        // Reset values
        a = null;
        b = undefined;
        c = 0;
        d = "";
        e = false;
        f = "value";
        g = true;
        
        // Logical OR assignment
        a ||= "value";
        b ||= "value";
        c ||= "value";
        d ||= "value";
        e ||= "value";
        f ||= "new value";
        g ||= "new value";
        
        const or_results = [a, b, c, d, e, f, g];
        
        [and_results, or_results];
    "#;
    
    let result = execute(code).unwrap();
    
    if let Value::Array(values) = result {
        assert_eq!(values.len(), 2);
        
        if let Value::Array(and_results) = &values[0] {
            assert_eq!(and_results.len(), 7);
            assert_eq!(and_results[0].to_string(), "null");
            assert_eq!(and_results[1].to_string(), "undefined");
            assert_eq!(and_results[2].to_string(), "0");
            assert_eq!(and_results[3].to_string(), "");
            assert_eq!(and_results[4].to_string(), "false");
            assert_eq!(and_results[5].to_string(), "new value");
            assert_eq!(and_results[6].to_string(), "new value");
        } else {
            panic!("Expected array for and_results");
        }
        
        if let Value::Array(or_results) = &values[1] {
            assert_eq!(or_results.len(), 7);
            assert_eq!(or_results[0].to_string(), "value");
            assert_eq!(or_results[1].to_string(), "value");
            assert_eq!(or_results[2].to_string(), "value");
            assert_eq!(or_results[3].to_string(), "value");
            assert_eq!(or_results[4].to_string(), "value");
            assert_eq!(or_results[5].to_string(), "value");
            assert_eq!(or_results[6].to_string(), "true");
        } else {
            panic!("Expected array for or_results");
        }
    } else {
        panic!("Expected array result");
    }
}

#[test]
fn test_array_destructuring() {
    let code = r#"
        const array = [1, 2, 3, 4, 5];
        
        // Basic destructuring
        const [a, b, c] = array;
        
        // Destructuring with rest
        const [first, second, ...rest] = array;
        
        // Destructuring with default values
        const [x, y, z = 30, w = 40] = [10, 20];
        
        // Destructuring with skipped values
        const [, , third] = array;
        
        // Nested destructuring
        const nested = [1, [2, 3], 4];
        const [n1, [n2, n3], n4] = nested;
        
        [a, b, c, first, second, rest, x, y, z, w, third, n1, n2, n3, n4];
    "#;
    
    let result = execute(code).unwrap();
    
    if let Value::Array(values) = result {
        assert_eq!(values.len(), 15);
        assert_eq!(values[0].to_string(), "1");
        assert_eq!(values[1].to_string(), "2");
        assert_eq!(values[2].to_string(), "3");
        assert_eq!(values[3].to_string(), "1");
        assert_eq!(values[4].to_string(), "2");
        assert_eq!(values[5].to_string(), "[3, 4, 5]");
        assert_eq!(values[6].to_string(), "10");
        assert_eq!(values[7].to_string(), "20");
        assert_eq!(values[8].to_string(), "30");
        assert_eq!(values[9].to_string(), "40");
        assert_eq!(values[10].to_string(), "3");
        assert_eq!(values[11].to_string(), "1");
        assert_eq!(values[12].to_string(), "2");
        assert_eq!(values[13].to_string(), "3");
        assert_eq!(values[14].to_string(), "4");
    } else {
        panic!("Expected array result");
    }
}

#[test]
fn test_object_destructuring() {
    let code = r#"
        const person = {
            name: "John",
            age: 30,
            address: {
                city: "New York",
                zip: "10001"
            },
            hobbies: ["reading", "gaming"]
        };
        
        // Basic destructuring
        const { name, age } = person;
        
        // Destructuring with different variable names
        const { name: fullName, age: years } = person;
        
        // Destructuring with default values
        const { name: n, country = "USA" } = person;
        
        // Destructuring nested objects
        const { address: { city, zip } } = person;
        
        // Destructuring with rest
        const { name: personName, ...rest } = person;
        
        // Destructuring arrays in objects
        const { hobbies: [firstHobby, secondHobby] } = person;
        
        [name, age, fullName, years, n, country, city, zip, rest, firstHobby, secondHobby];
    "#;
    
    let result = execute(code).unwrap();
    
    if let Value::Array(values) = result {
        assert_eq!(values.len(), 11);
        assert_eq!(values[0].to_string(), "John");
        assert_eq!(values[1].to_string(), "30");
        assert_eq!(values[2].to_string(), "John");
        assert_eq!(values[3].to_string(), "30");
        assert_eq!(values[4].to_string(), "John");
        assert_eq!(values[5].to_string(), "USA");
        assert_eq!(values[6].to_string(), "New York");
        assert_eq!(values[7].to_string(), "10001");
        
        if let Value::Object(rest_obj) = &values[8] {
            assert!(rest_obj.contains_key("age"));
            assert!(rest_obj.contains_key("address"));
            assert!(rest_obj.contains_key("hobbies"));
            assert!(!rest_obj.contains_key("name"));
        } else {
            panic!("Expected object for rest");
        }
        
        assert_eq!(values[9].to_string(), "reading");
        assert_eq!(values[10].to_string(), "gaming");
    } else {
        panic!("Expected array result");
    }
}

#[test]
fn test_spread_operator() {
    let code = r#"
        // Array spread
        const array1 = [1, 2, 3];
        const array2 = [4, 5, 6];
        
        // Spread in array literals
        const combined = [...array1, ...array2];
        const withValues = [0, ...array1, 4];
        
        // Object spread
        const obj1 = { a: 1, b: 2 };
        const obj2 = { c: 3, d: 4 };
        
        // Spread in object literals
        const combinedObj = { ...obj1, ...obj2 };
        const withProps = { x: 0, ...obj1, y: 3 };
        
        // Spread with overrides
        const overrideObj = { ...obj1, b: 3 };
        
        // Function calls with spread
        function sum(...numbers) {
            return numbers.reduce((total, n) => total + n, 0);
        }
        
        const numbers = [1, 2, 3, 4, 5];
        const total = sum(...numbers);
        
        [combined, withValues, combinedObj, withProps, overrideObj, total];
    "#;
    
    let result = execute(code).unwrap();
    
    if let Value::Array(values) = result {
        assert_eq!(values.len(), 6);
        
        if let Value::Array(combined) = &values[0] {
            assert_eq!(combined.len(), 6);
            assert_eq!(combined[0].to_string(), "1");
            assert_eq!(combined[5].to_string(), "6");
        } else {
            panic!("Expected array for combined");
        }
        
        if let Value::Array(with_values) = &values[1] {
            assert_eq!(with_values.len(), 5);
            assert_eq!(with_values[0].to_string(), "0");
            assert_eq!(with_values[4].to_string(), "4");
        } else {
            panic!("Expected array for withValues");
        }
        
        if let Value::Object(combined_obj) = &values[2] {
            assert_eq!(combined_obj.len(), 4);
            assert!(combined_obj.contains_key("a"));
            assert!(combined_obj.contains_key("b"));
            assert!(combined_obj.contains_key("c"));
            assert!(combined_obj.contains_key("d"));
        } else {
            panic!("Expected object for combinedObj");
        }
        
        if let Value::Object(with_props) = &values[3] {
            assert_eq!(with_props.len(), 4);
            assert!(with_props.contains_key("x"));
            assert!(with_props.contains_key("a"));
            assert!(with_props.contains_key("b"));
            assert!(with_props.contains_key("y"));
        } else {
            panic!("Expected object for withProps");
        }
        
        if let Value::Object(override_obj) = &values[4] {
            assert_eq!(override_obj.len(), 2);
            assert!(override_obj.contains_key("a"));
            assert!(override_obj.contains_key("b"));
            assert_eq!(override_obj.get("b").unwrap().to_string(), "3");
        } else {
            panic!("Expected object for overrideObj");
        }
        
        assert_eq!(values[5].to_string(), "15");
    } else {
        panic!("Expected array result");
    }
}