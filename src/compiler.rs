use std::process::Command;
use std::fs;
// Removed unused import
use colored::*;
use crate::lexer;
use crate::parser::Parser;
use crate::codegen::{generate_llvm_ir, FileType};

pub fn compile(output: &str, obj_file: &str, target: Option<&str>) -> std::io::Result<()> {
    let output_file = if cfg!(target_os = "windows") || target == Some("windows-x64") {
        format!("{}.exe", output)
    } else {
        output.to_string()
    };

    println!("{} Reading source file...", "Compiler:".blue());
    
    // The obj_file parameter is actually the input file path
    let input_file = obj_file;
    
    // Read the input file
    let source = match fs::read_to_string(&input_file) {
        Ok(content) => content,
        Err(e) => {
            println!("{} Failed to read source file: {}", "Error:".red(), e);
            return Err(e);
        }
    };
    
    println!("{} Lexing and parsing...", "Compiler:".blue());
    
    // Lex and parse the source code
    let tokens = lexer::tokenize(&source);
    let mut parser = Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(e) => {
            println!("{} Parse error: {}", "Error:".red(), e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, e.to_string()));
        }
    };
    
    println!("{} Generating code...", "Compiler:".blue());
    
    // Generate C code from the AST
    let (module, target_machine) = generate_llvm_ir(&ast, target);
    
    // Create a temporary C file
    let c_file = format!("{}.c", output);
    if let Err(e) = target_machine.write_to_file(&module, FileType::Object, &c_file) {
        println!("{} Failed to generate C code: {}", "Error:".red(), e);
        return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
    }
    
    println!("{} Linking executable...", "Compiler:".blue());
    
    // Compile the generated C code
    let mut clang = Command::new("clang");
    clang.arg(&c_file).arg("-o").arg(&output_file);

    // Add target-specific flags
    if let Some(target_triple) = target {
        clang.arg(format!("--target={}", target_triple));
        
        // Add Linux-specific flags and libraries
        if target_triple.contains("linux") {
            println!("{} Using Linux-specific compilation settings", "Compiler:".blue());
            // Add standard Linux libraries
            clang.arg("-lm").arg("-ldl").arg("-lpthread");
            
            // Check if we're targeting a specific architecture
            if target_triple.contains("x86_64") {
                println!("{} Targeting x86_64 Linux", "Compiler:".blue());
            } else if target_triple.contains("aarch64") {
                println!("{} Targeting ARM64 Linux", "Compiler:".blue());
            } else if target_triple.contains("arm") {
                println!("{} Targeting ARM Linux", "Compiler:".blue());
            }
        }
    } else if cfg!(target_os = "linux") {
        // If no target is specified but we're on Linux, add Linux libraries
        println!("{} Using Linux-specific compilation settings", "Compiler:".blue());
        clang.arg("-lm").arg("-ldl").arg("-lpthread");
    }
    
    // Execute the clang command
    let status = clang.status()?;
    
    if status.success() {
        println!("{} Successfully compiled to {}", "Compiler:".green(), output_file);
        Ok(())
    } else {
        println!("{} Failed to link executable", "Error:".red());
        Err(std::io::Error::new(std::io::ErrorKind::Other, "Linking failed"))
    }
}
