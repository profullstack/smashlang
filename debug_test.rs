use std::fs;
use std::env;

mod debug_codegen;

fn main() {
    // Simple test program
    let source_code = r#"
    // Simple test program
    print("Hello from SmashLang!");
    print("This should be visible in the output.");
    
    const a = 10;
    const b = 20;
    
    if (a < b) {
        print("a is less than b");
    }
    "#;
    
    match debug_codegen::debug_codegen(source_code, "/tmp/debug_test") {
        Ok(_) => println!("Debug successful"),
        Err(e) => eprintln!("Debug failed: {}", e),
    }
}
