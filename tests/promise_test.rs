use smashlang::{execute, Value};

#[test]
fn test_promise_constructor() {
    let code = r#"
        let resolvedValue = null;
        
        let promise = new Promise((resolve, reject) => {
            resolve(42);
        });
        
        promise.then(value => {
            resolvedValue = value;
        });
        
        resolvedValue;
    "#;
    
    let result = execute(code).unwrap();
    assert_eq!(result.to_string(), "42");
}

#[test]
fn test_promise_then_chain() {
    let code = r#"
        let result = null;
        
        Promise.resolve(1)
            .then(value => value * 2)
            .then(value => value + 10)
            .then(value => {
                result = value;
                return value;
            });
        
        result;
    "#;
    
    let result = execute(code).unwrap();
    assert_eq!(result.to_string(), "12");
}

#[test]
fn test_promise_catch() {
    let code = r#"
        let result = null;
        
        Promise.reject("error")
            .catch(reason => {
                result = "Caught: " + reason;
                return result;
            });
        
        result;
    "#;
    
    let result = execute(code).unwrap();
    assert_eq!(result.to_string(), "Caught: error");
}

#[test]
fn test_promise_finally() {
    let code = r#"
        let cleanupDone = false;
        
        Promise.resolve(42)
            .finally(() => {
                cleanupDone = true;
            });
        
        cleanupDone;
    "#;
    
    let result = execute(code).unwrap();
    assert_eq!(result.to_string(), "true");
}

#[test]
fn test_promise_all() {
    let code = r#"
        let result = null;
        
        Promise.all([
            Promise.resolve(1),
            Promise.resolve(2),
            Promise.resolve(3)
        ]).then(values => {
            result = values;
        });
        
        result;
    "#;
    
    let result = execute(code).unwrap();
    assert_eq!(result.to_string(), "[1, 2, 3]");
}

#[test]
fn test_promise_race() {
    let code = r#"
        let result = null;
        
        Promise.race([
            new Promise(resolve => setTimeout(() => resolve("slow"), 100)),
            Promise.resolve("fast")
        ]).then(value => {
            result = value;
        });
        
        result;
    "#;
    
    let result = execute(code).unwrap();
    assert_eq!(result.to_string(), "fast");
}

#[test]
fn test_async_await() {
    let code = r#"
        let result = null;
        
        async function fetchData() {
            const value = await Promise.resolve(42);
            return value * 2;
        }
        
        fetchData().then(value => {
            result = value;
        });
        
        result;
    "#;
    
    let result = execute(code).unwrap();
    assert_eq!(result.to_string(), "84");
}

#[test]
fn test_promise_error_handling() {
    let code = r#"
        let result = null;
        
        async function processData() {
            try {
                await Promise.reject("failed");
                return "success";
            } catch (error) {
                return "Error: " + error;
            }
        }
        
        processData().then(value => {
            result = value;
        });
        
        result;
    "#;
    
    let result = execute(code).unwrap();
    assert_eq!(result.to_string(), "Error: failed");
}