use smashlang::{execute, Value};
use smashlang::runtime::module::{Module, ModuleRegistry};
use std::fs;
use std::path::Path;

fn setup_test_modules() {
    // Create test directory if it doesn't exist
    let test_dir = Path::new("test_modules");
    if !test_dir.exists() {
        fs::create_dir(test_dir).unwrap();
    }
    
    // Create math.smash module
    let math_module = r#"
        // math.smash
        export const PI = 3.14159;
        export const E = 2.71828;
        
        export function square(x) {
            return x * x;
        }
        
        export function cube(x) {
            return x * x * x;
        }
        
        export default {
            name: "math",
            version: "1.0.0"
        };
    "#;
    fs::write(test_dir.join("math.smash"), math_module).unwrap();
    
    // Create utils.smash module
    let utils_module = r#"
        // utils.smash
        export function capitalize(str) {
            return str.charAt(0).toUpperCase() + str.slice(1);
        }
        
        export function sum(...args) {
            return args.reduce((total, num) => total + num, 0);
        }
    "#;
    fs::write(test_dir.join("utils.smash"), utils_module).unwrap();
    
    // Create index.smash module
    let index_module = r#"
        // index.smash
        import math, { PI, square } from './math.smash';
        import { capitalize, sum } from './utils.smash';
        
        export const VERSION = "1.0.0";
        
        export function calculateArea(radius) {
            return PI * square(radius);
        }
        
        export function formatName(name) {
            return capitalize(name);
        }
        
        export function add(...numbers) {
            return sum(...numbers);
        }
        
        export { math };
    "#;
    fs::write(test_dir.join("index.smash"), index_module).unwrap();
    
    // Create re-export.smash module
    let reexport_module = r#"
        // re-export.smash
        export * from './math.smash';
        export { capitalize as formatString } from './utils.smash';
    "#;
    fs::write(test_dir.join("re-export.smash"), reexport_module).unwrap();
}

fn cleanup_test_modules() {
    // Remove test directory
    let test_dir = Path::new("test_modules");
    if test_dir.exists() {
        fs::remove_dir_all(test_dir).unwrap();
    }
}

#[test]
fn test_basic_import_export() {
    setup_test_modules();
    
    let code = r#"
        import { PI, square } from './test_modules/math.smash';
        
        PI * square(2);
    "#;
    
    let result = execute(code).unwrap();
    assert_eq!(result.to_string(), "12.56636");
    
    cleanup_test_modules();
}

#[test]
fn test_default_import() {
    setup_test_modules();
    
    let code = r#"
        import math from './test_modules/math.smash';
        
        math.name;
    "#;
    
    let result = execute(code).unwrap();
    assert_eq!(result.to_string(), "math");
    
    cleanup_test_modules();
}

#[test]
fn test_namespace_import() {
    setup_test_modules();
    
    let code = r#"
        import * as mathUtils from './test_modules/math.smash';
        
        mathUtils.PI * mathUtils.square(2);
    "#;
    
    let result = execute(code).unwrap();
    assert_eq!(result.to_string(), "12.56636");
    
    cleanup_test_modules();
}

#[test]
fn test_mixed_imports() {
    setup_test_modules();
    
    let code = r#"
        import math, { PI, square as sq } from './test_modules/math.smash';
        
        const result = {
            name: math.name,
            area: PI * sq(2)
        };
        
        result.area;
    "#;
    
    let result = execute(code).unwrap();
    assert_eq!(result.to_string(), "12.56636");
    
    cleanup_test_modules();
}

#[test]
fn test_re_exports() {
    setup_test_modules();
    
    let code = r#"
        import { PI, formatString } from './test_modules/re-export.smash';
        
        formatString("hello") + " " + PI;
    "#;
    
    let result = execute(code).unwrap();
    assert_eq!(result.to_string(), "Hello 3.14159");
    
    cleanup_test_modules();
}

#[test]
fn test_export_all() {
    setup_test_modules();
    
    let code = r#"
        import { PI, square, cube } from './test_modules/re-export.smash';
        
        cube(PI) + square(2);
    "#;
    
    let result = execute(code).unwrap();
    assert_eq!(result.to_string(), "31.006276019");
    
    cleanup_test_modules();
}

#[test]
fn test_module_registry() {
    setup_test_modules();
    
    // Create module registry
    let registry = ModuleRegistry::new();
    
    // Load a module
    let module_path = Path::new("test_modules/math.smash").canonicalize().unwrap();
    let module = registry.load_module(module_path.to_str().unwrap()).unwrap();
    
    // Link the module
    module.link(&registry).unwrap();
    
    // Evaluate the module
    let exports = module.evaluate(&registry).unwrap();
    
    // Check exports
    if let Value::Object(exports_map) = exports {
        assert!(exports_map.contains_key("PI"));
        assert!(exports_map.contains_key("square"));
        assert!(exports_map.contains_key("default"));
    } else {
        panic!("Expected exports to be an object");
    }
    
    cleanup_test_modules();
}

#[test]
fn test_circular_dependencies() {
    // Create test directory if it doesn't exist
    let test_dir = Path::new("test_modules");
    if !test_dir.exists() {
        fs::create_dir(test_dir).unwrap();
    }
    
    // Create circular dependency modules
    let module_a = r#"
        // a.smash
        import { b_value } from './b.smash';
        
        export const a_value = "Module A";
        export const combined = a_value + " imports " + b_value;
    "#;
    fs::write(test_dir.join("a.smash"), module_a).unwrap();
    
    let module_b = r#"
        // b.smash
        import { a_value } from './a.smash';
        
        export const b_value = "Module B";
        export const combined = b_value + " imports " + a_value;
    "#;
    fs::write(test_dir.join("b.smash"), module_b).unwrap();
    
    let code = r#"
        import { combined } from './test_modules/a.smash';
        combined;
    "#;
    
    let result = execute(code).unwrap();
    assert_eq!(result.to_string(), "Module A imports Module B");
    
    cleanup_test_modules();
}