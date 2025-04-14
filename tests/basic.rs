use smashlang::{execute, compile, Value};

#[test]
fn test_basic_arithmetic() {
    let source = "
        let x = 10;
        let y = 20;
        x + y;
    ";
    
    // Test interpreter
    let result = execute(source).unwrap();
    match result {
        Value::Number(n) => assert_eq!(n, 30.0),
        _ => panic!("Expected number, got: {:?}", result),
    }
    
    // Test compiler (if supported on this platform)
    #[cfg(not(target_arch = "wasm32"))]
    {
        let compiled_fn = compile(source).unwrap();
        let result = unsafe { compiled_fn.execute() };
        assert_eq!(result, 30);
    }
}

#[test]
fn test_variables_and_scopes() {
    let source = "
        let x = 5;
        {
            let y = 10;
            x + y;
        }
    ";
    
    let result = execute(source).unwrap();
    match result {
        Value::Number(n) => assert_eq!(n, 15.0),
        _ => panic!("Expected number, got: {:?}", result),
    }
}

#[test]
fn test_conditionals() {
    let source = "
        let x = 10;
        let result = 0;
        
        if (x > 5) {
            result = 1;
        } else {
            result = 2;
        }
        
        result;
    ";
    
    let result = execute(source).unwrap();
    match result {
        Value::Number(n) => assert_eq!(n, 1.0),
        _ => panic!("Expected number, got: {:?}", result),
    }
}

#[test]
fn test_loops() {
    let source = "
        let sum = 0;
        for (let i = 1; i <= 5; i = i + 1) {
            sum = sum + i;
        }
        sum;
    ";
    
    let result = execute(source).unwrap();
    match result {
        Value::Number(n) => assert_eq!(n, 15.0),
        _ => panic!("Expected number, got: {:?}", result),
    }
}

#[test]
fn test_functions() {
    let source = "
        fn add(a, b) {
            return a + b;
        }
        
        add(5, 7);
    ";
    
    let result = execute(source).unwrap();
    match result {
        Value::Number(n) => assert_eq!(n, 12.0),
        _ => panic!("Expected number, got: {:?}", result),
    }
}

#[test]
fn test_arrays() {
    let source = "
        let arr = [1, 2, 3, 4, 5];
        let sum = 0;
        
        for (let i = 0; i < 5; i = i + 1) {
            sum = sum + arr[i];
        }
        
        sum;
    ";
    
    let result = execute(source).unwrap();
    match result {
        Value::Number(n) => assert_eq!(n, 15.0),
        _ => panic!("Expected number, got: {:?}", result),
    }
}

#[test]
fn test_objects() {
    let source = "
        let person = {
            name: 'John',
            age: 30,
            greet: fn() {
                return 'Hello, ' + this.name;
            }
        };
        
        person.age;
    ";
    
    let result = execute(source).unwrap();
    match result {
        Value::Number(n) => assert_eq!(n, 30.0),
        _ => panic!("Expected number, got: {:?}", result),
    }
}

#[test]
fn test_error_handling() {
    let source = "
        let result = 0;
        
        try {
            throw 'Error!';
            result = 1;
        } catch (e) {
            result = 2;
        } finally {
            result = result + 1;
        }
        
        result;
    ";
    
    let result = execute(source).unwrap();
    match result {
        Value::Number(n) => assert_eq!(n, 3.0),
        _ => panic!("Expected number, got: {:?}", result),
    }
}

#[test]
fn test_regex() {
    let source = "
        let pattern = /^hello/;
        let str = 'hello world';
        pattern.test(str);
    ";
    
    let result = execute(source).unwrap();
    match result {
        Value::Bool(b) => assert!(b),
        _ => panic!("Expected boolean, got: {:?}", result),
    }
}