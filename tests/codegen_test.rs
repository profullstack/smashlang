use crate::lexer::tokenize;
use crate::parser::Parser;
use crate::codegen::generate_llvm_ir;

#[cfg(test)]
mod tests {
    use super::*;
    use inkwell::context::Context;
    use inkwell::OptimizationLevel;
    use std::error::Error;

    // Helper function to compile a code snippet and return the LLVM IR
    fn compile_to_ir(code: &str) -> String {
        let tokens = tokenize(code);
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        let (module, _) = generate_llvm_ir(&ast, None);
        
        module.print_to_string().to_string()
    }

    #[test]
    fn test_codegen_if_statement() {
        let code = "fn test() { if (x > 0) { return 1; } else { return 0; } }";
        let ir = compile_to_ir(code);
        
        // Check that the IR contains the expected basic blocks
        assert!(ir.contains("then:"));
        assert!(ir.contains("else:"));
        assert!(ir.contains("ifcont:"));
        
        // Check for conditional branch instruction
        assert!(ir.contains("br i1"));
    }

    #[test]
    fn test_codegen_while_loop() {
        let code = "fn test() { while (i < 10) { i = i + 1; } }";
        let ir = compile_to_ir(code);
        
        // Check that the IR contains the expected basic blocks
        assert!(ir.contains("loop_header:"));
        assert!(ir.contains("loop_body:"));
        assert!(ir.contains("after_loop:"));
        
        // Check for conditional branch instruction
        assert!(ir.contains("br i1"));
    }

    #[test]
    fn test_codegen_for_loop() {
        let code = "fn test() { for (let i = 0; i < 10; i++) { print(i); } }";
        let ir = compile_to_ir(code);
        
        // Check that the IR contains the expected basic blocks
        assert!(ir.contains("for_header:"));
        assert!(ir.contains("for_body:"));
        assert!(ir.contains("for_update:"));
        assert!(ir.contains("after_for:"));
        
        // Check for conditional branch instruction
        assert!(ir.contains("br i1"));
    }

    #[test]
    fn test_codegen_for_in_loop() {
        let code = "fn test() { for (let key in object) { print(key); } }";
        let ir = compile_to_ir(code);
        
        // Check that the IR contains the expected basic blocks
        assert!(ir.contains("forin_setup:"));
        assert!(ir.contains("forin_header:"));
        assert!(ir.contains("forin_body:"));
        assert!(ir.contains("after_forin:"));
        
        // Check for conditional branch instruction
        assert!(ir.contains("br i1"));
        
        // Check for array allocation (keys array)
        assert!(ir.contains("alloca [1 x i64]"));
    }

    #[test]
    fn test_codegen_for_of_loop() {
        let code = "fn test() { for (let item of array) { print(item); } }";
        let ir = compile_to_ir(code);
        
        // Check that the IR contains the expected basic blocks
        assert!(ir.contains("forof_setup:"));
        assert!(ir.contains("forof_header:"));
        assert!(ir.contains("forof_body:"));
        assert!(ir.contains("after_forof:"));
        
        // Check for conditional branch instruction
        assert!(ir.contains("br i1"));
        
        // Check for array allocation
        assert!(ir.contains("alloca [1 x i64]"));
    }

    #[test]
    fn test_codegen_do_while_loop() {
        let code = "fn test() { do { i = i + 1; } while (i < 10); }";
        let ir = compile_to_ir(code);
        
        // Check that the IR contains the expected basic blocks
        assert!(ir.contains("do_body:"));
        assert!(ir.contains("do_cond:"));
        assert!(ir.contains("after_do:"));
        
        // Check for conditional branch instruction
        assert!(ir.contains("br i1"));
    }

    #[test]
    fn test_codegen_nested_loops() {
        let code = "fn test() { for (let i = 0; i < 10; i++) { for (let j = 0; j < 10; j++) { print(i * j); } } }";
        let ir = compile_to_ir(code);
        
        // Check that the IR contains multiple for loop blocks
        let for_header_count = ir.matches("for_header").count();
        let for_body_count = ir.matches("for_body").count();
        let for_update_count = ir.matches("for_update").count();
        
        // We should have at least 2 of each (one for each loop)
        assert!(for_header_count >= 2);
        assert!(for_body_count >= 2);
        assert!(for_update_count >= 2);
    }

    #[test]
    fn test_codegen_break_continue() {
        let code = "fn test() { while (true) { if (x > 10) { break; } else { continue; } } }";
        let ir = compile_to_ir(code);
        
        // Check for unconditional branches (from break and continue)
        let br_count = ir.matches("br label").count();
        
        // We should have at least 2 unconditional branches (one for break, one for continue)
        assert!(br_count >= 2);
    }

    #[test]
    fn test_codegen_switch() {
        let code = "fn test() { switch (x) { case 1: return 1; case 2: return 2; default: return 0; } }";
        let ir = compile_to_ir(code);
        
        // Check for switch instruction or equivalent if-else chain
        assert!(ir.contains("switch") || ir.contains("icmp eq"));
        
        // Check for case blocks
        assert!(ir.contains("case") || ir.contains("switch_cmp"));
        
        // Check for return instructions
        let ret_count = ir.matches("ret i64").count();
        assert!(ret_count >= 3); // One for each case and default
    }
}
# Merged from codegen_tests.rs
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
    
    // Instead of directly accessing the private to_c_code method,
    // we'll write the code to a temporary file and check its contents
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let output_path = temp_dir.path().join("output.c");
    let output_path_str = output_path.to_str().unwrap();
    
    // Write the module to a file
    target_machine.write_to_file(&module, FileType::Object, output_path_str)
        .expect("Failed to write to file");
    
    // Read the file contents
    let generated_code = fs::read_to_string(output_path_str)
        .expect("Failed to read generated code");
    
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
    
    // Instead of directly accessing the private to_c_code method,
    // we'll write the code to a temporary file and check its contents
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let output_path = temp_dir.path().join("output.c");
    let output_path_str = output_path.to_str().unwrap();
    
    // Write the module to a file
    target_machine.write_to_file(&module, FileType::Object, output_path_str)
        .expect("Failed to write to file");
    
    // Read the file contents
    let generated_code = fs::read_to_string(output_path_str)
        .expect("Failed to read generated code");
    
    // Check that the generated code contains a comment about variable declaration
    // The current implementation only adds a comment, not actual variable code
    assert!(generated_code.contains("Variable declaration") && generated_code.contains("x"), "Generated code should include a comment about variable declaration");
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

// Tests merged from codegen_tests.rs
