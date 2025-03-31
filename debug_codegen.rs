use std::fs;
use crate::parser::AstNode;
use crate::lexer::tokenize;
use crate::parser::Parser;
use crate::codegen::{generate_llvm_ir, FileType};

pub fn debug_codegen(source_code: &str, output_file: &str) -> Result<(), String> {
    println!("Debugging code generation for:\n{}", source_code);
    
    // Tokenize the source code
    let tokens = tokenize(source_code);
    println!("Tokens: {:?}", tokens);
    
    // Parse tokens into an AST
    let mut parser = Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(e) => return Err(format!("Parse error: {}", e)),
    };
    
    println!("AST: {:?}", ast);
    
    // Generate code using our codegen module
    let (module, target_machine) = generate_llvm_ir(&ast, None);
    
    // Write the C code to a file
    let c_file = format!("{}.c", output_file);
    if let Err(e) = target_machine.write_to_file(&module, FileType::Object, &c_file) {
        return Err(format!("Code generation error: {}", e));
    }
    
    // Read and print the generated C file
    match fs::read_to_string(&c_file) {
        Ok(content) => {
            println!("Generated C code:\n{}", content);
        },
        Err(e) => {
            return Err(format!("Failed to read generated C file: {}", e));
        }
    }
    
    Ok(())
}
