use smashlang::codegen::{generate_llvm_ir, TargetMachine, FileType, Module};
use smashlang::lexer::tokenize;
use smashlang::parser::Parser;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_codegen_basic_program() {
    // Test code generation for a basic program
    let input = "print;"; 
    let tokens = tokenize(input);
    let mut parser = Parser::new(tokens);
    
    let ast = parser.parse().expect("Parser should succeed");
    
    // Generate code using the current implementation
    let (module, target_machine) = generate_llvm_ir(&ast, Some("x86_64-unknown-linux-gnu"));
    
    // Get the generated C code
    let generated_code = module.to_c_code();
    
    // Check that the generated code contains expected C code patterns
    assert!(generated_code.contains("#include"), "Generated code should include C headers");
    assert!(generated_code.contains("int main"), "Generated code should have a main function");
}

#[test]
fn test_codegen_variable_declaration() {
    // Test code generation for variable declarations
    let input = "let x = 10;"; 
    let tokens = tokenize(input);
    let mut parser = Parser::new(tokens);
    
    let ast = parser.parse().expect("Parser should succeed");
    
    // Generate code using the current implementation
    let (module, target_machine) = generate_llvm_ir(&ast, Some("x86_64-unknown-linux-gnu"));
    
    // Get the generated C code
    let generated_code = module.to_c_code();
    
    // Check that the generated code contains variable declaration
    assert!(generated_code.contains("int x = 10"), "Generated code should include variable declaration");
}

#[test]
fn test_codegen_to_file() {
    // Test writing generated code to a file
    let input = "print;"; 
    let tokens = tokenize(input);
    let mut parser = Parser::new(tokens);
    
    let ast = parser.parse().expect("Parser should succeed");
    
    // Generate code using the current implementation
    let (module, target_machine) = generate_llvm_ir(&ast, Some("x86_64-unknown-linux-gnu"));
    
    // Create a temporary directory and file
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let output_path = temp_dir.path().join("test_output.c");
    
    // Write the generated code to a file using the target machine
    target_machine.write_to_file(&module, FileType::Object, output_path.to_str().unwrap())
        .expect("Failed to write code to file");
    
    // Check that the file exists and contains the generated code
    assert!(output_path.exists(), "Output file should exist");
    
    let file_contents = fs::read_to_string(&output_path).expect("Failed to read output file");
    assert!(file_contents.contains("#include"), "File should contain C headers");
    assert!(file_contents.contains("int main"), "File should contain main function");
}
