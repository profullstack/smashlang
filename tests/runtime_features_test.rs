use smashlang::{execute, Value};
use std::thread;
use std::time::Duration;

#[test]
fn test_setTimeout() {
    let code = r#"
        let result = null;
        
        setTimeout(() => {
            result = "Timeout executed";
        }, 100);
        
        // Wait for the timeout to execute
        setTimeout(() => {
            // This will run after the first timeout
        }, 200);
        
        result;
    "#;
    
    let result = execute(code).unwrap();
    
    // Initially, result should be null
    assert_eq!(result.to_string(), "null");
    
    // Wait for the timeout to execute
    thread::sleep(Duration::from_millis(300));
    
    // Re-execute to check the result
    let code = r#"result;"#;
    let result = execute(code).unwrap();
    
    // Now result should be updated
    assert_eq!(result.to_string(), "Timeout executed");
}

#[test]
fn test_setInterval() {
    let code = r#"
        let counter = 0;
        
        const intervalId = setInterval(() => {
            counter += 1;
        }, 100);
        
        // Store the interval ID
        intervalId;
    "#;
    
    let result = execute(code).unwrap();
    
    // Result should be the interval ID (a number)
    assert!(matches!(result, Value::Number(_)));
    
    // Wait for the interval to execute a few times
    thread::sleep(Duration::from_millis(350));
    
    // Check the counter
    let code = r#"counter;"#;
    let result = execute(code).unwrap();
    
    // Counter should be at least 3
    if let Value::Number(n) = result {
        assert!(n >= 3.0);
    } else {
        panic!("Expected number");
    }
    
    // Clear the interval
    let code = r#"clearInterval(intervalId);"#;
    execute(code).unwrap();
    
    // Wait to ensure the interval is cleared
    thread::sleep(Duration::from_millis(200));
    
    // Store the current counter value
    let code = r#"const currentCounter = counter; currentCounter;"#;
    let result = execute(code).unwrap();
    let current_counter = if let Value::Number(n) = result { n } else { panic!("Expected number") };
    
    // Wait again to see if the counter still increases
    thread::sleep(Duration::from_millis(200));
    
    // Check if the counter has increased
    let code = r#"counter;"#;
    let result = execute(code).unwrap();
    
    // Counter should not have increased
    if let Value::Number(n) = result {
        assert_eq!(n, current_counter);
    } else {
        panic!("Expected number");
    }
}

#[test]
fn test_clearTimeout() {
    let code = r#"
        let result = "Initial";
        
        const timeoutId = setTimeout(() => {
            result = "Timeout executed";
        }, 200);
        
        // Clear the timeout
        clearTimeout(timeoutId);
        
        // Wait to ensure the timeout would have executed
        setTimeout(() => {
            // This will run after the first timeout would have executed
        }, 300);
        
        timeoutId;
    "#;
    
    let result = execute(code).unwrap();
    
    // Result should be the timeout ID (a number)
    assert!(matches!(result, Value::Number(_)));
    
    // Wait for the timeout to execute (if it wasn't cleared)
    thread::sleep(Duration::from_millis(400));
    
    // Check if the result was changed
    let code = r#"result;"#;
    let result = execute(code).unwrap();
    
    // Result should still be "Initial"
    assert_eq!(result.to_string(), "Initial");
}

#[test]
fn test_json_parse() {
    let code = r#"
        // Parse a JSON string
        const jsonString = '{"name":"John","age":30,"isAdmin":false,"hobbies":["reading","gaming"]}';
        const parsed = JSON.parse(jsonString);
        
        // Check the parsed values
        const results = [
            parsed.name,
            parsed.age,
            parsed.isAdmin,
            parsed.hobbies[0],
            parsed.hobbies[1],
            parsed.hobbies.length
        ];
        
        results;
    "#;
    
    let result = execute(code).unwrap();
    
    if let Value::Array(values) = result {
        assert_eq!(values.len(), 6);
        assert_eq!(values[0].to_string(), "John");
        assert_eq!(values[1].to_string(), "30");
        assert_eq!(values[2].to_string(), "false");
        assert_eq!(values[3].to_string(), "reading");
        assert_eq!(values[4].to_string(), "gaming");
        assert_eq!(values[5].to_string(), "2");
    } else {
        panic!("Expected array result");
    }
}

#[test]
fn test_json_parse_with_reviver() {
    let code = r#"
        // Parse a JSON string with a reviver function
        const jsonString = '{"name":"John","age":30,"birthDate":"2000-01-01T00:00:00.000Z"}';
        const parsed = JSON.parse(jsonString, (key, value) => {
            if (key === "birthDate") {
                return new Date(value);
            }
            return value;
        });
        
        // Check the parsed values
        const results = [
            parsed.name,
            parsed.age,
            typeof parsed.birthDate,
            parsed.birthDate instanceof Date
        ];
        
        results;
    "#;
    
    let result = execute(code).unwrap();
    
    if let Value::Array(values) = result {
        assert_eq!(values.len(), 4);
        assert_eq!(values[0].to_string(), "John");
        assert_eq!(values[1].to_string(), "30");
        assert_eq!(values[2].to_string(), "object");
        assert_eq!(values[3].to_string(), "true");
    } else {
        panic!("Expected array result");
    }
}

#[test]
fn test_json_stringify() {
    let code = r#"
        // Create an object to stringify
        const obj = {
            name: "John",
            age: 30,
            isAdmin: false,
            hobbies: ["reading", "gaming"],
            address: {
                city: "New York",
                zip: "10001"
            }
        };
        
        // Stringify the object
        const jsonString = JSON.stringify(obj);
        
        // Parse it back to verify
        const parsed = JSON.parse(jsonString);
        
        // Check the parsed values
        const results = [
            parsed.name,
            parsed.age,
            parsed.isAdmin,
            parsed.hobbies[0],
            parsed.hobbies[1],
            parsed.address.city,
            parsed.address.zip
        ];
        
        results;
    "#;
    
    let result = execute(code).unwrap();
    
    if let Value::Array(values) = result {
        assert_eq!(values.len(), 7);
        assert_eq!(values[0].to_string(), "John");
        assert_eq!(values[1].to_string(), "30");
        assert_eq!(values[2].to_string(), "false");
        assert_eq!(values[3].to_string(), "reading");
        assert_eq!(values[4].to_string(), "gaming");
        assert_eq!(values[5].to_string(), "New York");
        assert_eq!(values[6].to_string(), "10001");
    } else {
        panic!("Expected array result");
    }
}

#[test]
fn test_json_stringify_with_replacer() {
    let code = r#"
        // Create an object to stringify
        const obj = {
            name: "John",
            age: 30,
            password: "secret",
            address: {
                city: "New York",
                zip: "10001"
            }
        };
        
        // Stringify with a replacer function to hide the password
        const jsonString1 = JSON.stringify(obj, (key, value) => {
            if (key === "password") {
                return undefined;
            }
            return value;
        });
        
        // Stringify with a replacer array to only include certain properties
        const jsonString2 = JSON.stringify(obj, ["name", "age"]);
        
        // Parse them back to verify
        const parsed1 = JSON.parse(jsonString1);
        const parsed2 = JSON.parse(jsonString2);
        
        // Check the parsed values
        const results = [
            parsed1.name,
            parsed1.age,
            parsed1.password === undefined,
            parsed1.address.city,
            parsed2.name,
            parsed2.age,
            parsed2.address === undefined
        ];
        
        results;
    "#;
    
    let result = execute(code).unwrap();
    
    if let Value::Array(values) = result {
        assert_eq!(values.len(), 7);
        assert_eq!(values[0].to_string(), "John");
        assert_eq!(values[1].to_string(), "30");
        assert_eq!(values[2].to_string(), "true");
        assert_eq!(values[3].to_string(), "New York");
        assert_eq!(values[4].to_string(), "John");
        assert_eq!(values[5].to_string(), "30");
        assert_eq!(values[6].to_string(), "true");
    } else {
        panic!("Expected array result");
    }
}

#[test]
fn test_local_storage() {
    let code = r#"
        // Set items in localStorage
        localStorage.setItem("username", "john_doe");
        localStorage.setItem("theme", "dark");
        
        // Get items from localStorage
        const username = localStorage.getItem("username");
        const theme = localStorage.getItem("theme");
        const nonexistent = localStorage.getItem("nonexistent");
        
        // Check localStorage length
        const length = localStorage.length;
        
        // Get key by index
        const firstKey = localStorage.key(0);
        
        // Remove an item
        localStorage.removeItem("theme");
        const themeAfterRemove = localStorage.getItem("theme");
        const lengthAfterRemove = localStorage.length;
        
        // Clear localStorage
        localStorage.clear();
        const lengthAfterClear = localStorage.length;
        
        [username, theme, nonexistent, length, firstKey, themeAfterRemove, lengthAfterRemove, lengthAfterClear];
    "#;
    
    let result = execute(code).unwrap();
    
    if let Value::Array(values) = result {
        assert_eq!(values.len(), 8);
        assert_eq!(values[0].to_string(), "john_doe");
        assert_eq!(values[1].to_string(), "dark");
        assert_eq!(values[2].to_string(), "null");
        assert_eq!(values[3].to_string(), "2");
        assert!(values[4].to_string() == "username" || values[4].to_string() == "theme");
        assert_eq!(values[5].to_string(), "null");
        assert_eq!(values[6].to_string(), "1");
        assert_eq!(values[7].to_string(), "0");
    } else {
        panic!("Expected array result");
    }
}

#[test]
fn test_location() {
    let code = r#"
        // Get initial location properties
        const initialHref = location.href;
        const initialProtocol = location.protocol;
        const initialHost = location.host;
        const initialPathname = location.pathname;
        
        // Change location
        location.href = "https://example.com/path?query=value#hash";
        
        // Get updated location properties
        const updatedHref = location.href;
        const updatedProtocol = location.protocol;
        const updatedHost = location.host;
        const updatedPathname = location.pathname;
        const updatedSearch = location.search;
        const updatedHash = location.hash;
        
        [initialHref, initialProtocol, initialHost, initialPathname, 
         updatedHref, updatedProtocol, updatedHost, updatedPathname, updatedSearch, updatedHash];
    "#;
    
    let result = execute(code).unwrap();
    
    if let Value::Array(values) = result {
        assert_eq!(values.len(), 10);
        assert_eq!(values[0].to_string(), "http://localhost");
        assert_eq!(values[1].to_string(), "http:");
        assert_eq!(values[2].to_string(), "localhost");
        assert_eq!(values[3].to_string(), "/");
        assert_eq!(values[4].to_string(), "https://example.com/path?query=value#hash");
        assert_eq!(values[5].to_string(), "https:");
        assert_eq!(values[6].to_string(), "example.com");
        assert_eq!(values[7].to_string(), "/path");
        assert_eq!(values[8].to_string(), "?query=value");
        assert_eq!(values[9].to_string(), "#hash");
    } else {
        panic!("Expected array result");
    }
}

#[test]
fn test_navigator() {
    let code = r#"
        // Get navigator properties
        const userAgent = navigator.userAgent;
        const platform = navigator.platform;
        const language = navigator.language;
        const online = navigator.onLine;
        
        [userAgent, platform, language, online];
    "#;
    
    let result = execute(code).unwrap();
    
    if let Value::Array(values) = result {
        assert_eq!(values.len(), 4);
        assert_eq!(values[0].to_string(), "SmashLang/1.0");
        assert_eq!(values[1].to_string(), "SmashLang");
        assert_eq!(values[2].to_string(), "en-US");
        assert_eq!(values[3].to_string(), "true");
    } else {
        panic!("Expected array result");
    }
}

#[test]
fn test_document() {
    let code = r#"
        // Get document properties
        const initialTitle = document.title;
        
        // Set document properties
        document.title = "New Title";
        const updatedTitle = document.title;
        
        // Test cookies
        document.cookie = "name=John; path=/";
        document.cookie = "theme=dark; path=/";
        const cookies = document.cookie;
        
        [initialTitle, updatedTitle, cookies];
    "#;
    
    let result = execute(code).unwrap();
    
    if let Value::Array(values) = result {
        assert_eq!(values.len(), 3);
        assert_eq!(values[0].to_string(), "SmashLang Document");
        assert_eq!(values[1].to_string(), "New Title");
        assert!(values[2].to_string().contains("name=John") && values[2].to_string().contains("theme=dark"));
    } else {
        panic!("Expected array result");
    }
}