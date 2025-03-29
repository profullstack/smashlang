// Only import what we need based on feature flags
#[cfg(feature = "compiler")]
use std::fs;
#[cfg(feature = "compiler")]
use std::path::Path;
#[cfg(feature = "compiler")]
use std::process::Command;
#[cfg(feature = "compiler")]
use std::io::{self, Write};
#[cfg(feature = "compiler")]
use colored::*;

#[cfg(feature = "compiler")]
use crate::lexer::tokenize;
#[cfg(feature = "compiler")]
use crate::parser::{Parser, ParseError};
#[cfg(feature = "compiler")]
use crate::codegen::{generate_llvm_ir, CodegenError};

use std::fmt;
use std::error::Error;

// Define a CompilerError enum to represent different types of errors
#[derive(Debug)]
pub enum CompilerError {
    IoError(io::Error),
    ParseError(String),
    CodegenError(String),
    LinkError(String),
}

impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CompilerError::IoError(err) => write!(f, "I/O error: {}", err),
            CompilerError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            CompilerError::CodegenError(msg) => write!(f, "Code generation error: {}", msg),
            CompilerError::LinkError(msg) => write!(f, "Linking error: {}", msg),
        }
    }
}

impl Error for CompilerError {}

impl From<io::Error> for CompilerError {
    fn from(error: io::Error) -> Self {
        CompilerError::IoError(error)
    }
}

impl From<CodegenError> for CompilerError {
    fn from(error: CodegenError) -> Self {
        CompilerError::CodegenError(error.to_string())
    }
}

pub fn compile_file(path: &str, output: &str, target: Option<&str>, emit: &str) -> Result<(), CompilerError> {
    #[cfg(not(feature = "compiler"))]
    {
        println!("Compilation requires the 'compiler' feature to be enabled.");
        println!("Please build with: cargo build --features=compiler");
        return Ok(());
    }
    
    #[cfg(feature = "compiler")]
    {
        let mut full_code = String::new();

        // Load std.smash first
        if Path::new("std.smash").exists() {
            let std_code = fs::read_to_string("std.smash")
                .map_err(|e| {
                    eprintln!("{} Failed to read std.smash: {}", "Error:".red(), e);
                    CompilerError::IoError(e)
                })?;
            full_code.push_str(&std_code);
            full_code.push('\n');
        }

        // Then user program
        let code = fs::read_to_string(path)
            .map_err(|e| {
                eprintln!("{} Failed to read input file {}: {}", "Error:".red(), path, e);
                CompilerError::IoError(e)
            })?;
        full_code.push_str(&code);

        // Tokenize and parse
        println!("{} Tokenizing source code...", "Compiler:".blue());
        let tokens = tokenize(&full_code);
        
        println!("{} Parsing tokens...", "Compiler:".blue());
        let mut parser = Parser::new(tokens);
        let ast = parser.parse()
            .map_err(|e| {
                eprintln!("{} Parse error: {}", "Error:".red(), e);
                CompilerError::ParseError(e.to_string())
            })?;

        println!("{} Generating LLVM IR...", "Compiler:".blue());
        let (llvm_module, target_machine) = generate_llvm_ir(&ast, target)
            .map_err(|e| {
                eprintln!("{} Code generation error: {}", "Error:".red(), e);
                CompilerError::CodegenError(e.to_string())
            })?;

        println!("{} Emitting output format: {}", "Compiler:".blue(), emit);
        match emit {
            "ir" => {
                let output_path = format!("{output}.ll");
                llvm_module.print_to_file(&output_path)
                    .map_err(|e| {
                        eprintln!("{} Failed to write IR to file: {}", "Error:".red(), e);
                        CompilerError::IoError(io::Error::new(io::ErrorKind::Other, e))
                    })?;
                println!("{} LLVM IR written to {}", "Success:".green(), output_path);
            }
            "obj" => {
                let output_path = format!("{output}.o");
                target_machine
                    .write_to_file(&llvm_module, inkwell::targets::FileType::Object, Path::new(&output_path))
                    .map_err(|e| {
                        eprintln!("{} Failed to write object file: {}", "Error:".red(), e);
                        CompilerError::IoError(io::Error::new(io::ErrorKind::Other, e))
                    })?;
                println!("{} Object file written to {}", "Success:".green(), output_path);
            }
            "exe" => {
                let obj_file = format!("{output}.o");
                target_machine
                    .write_to_file(&llvm_module, inkwell::targets::FileType::Object, Path::new(&obj_file))
                    .map_err(|e| {
                        eprintln!("{} Failed to write object file: {}", "Error:".red(), e);
                        CompilerError::IoError(io::Error::new(io::ErrorKind::Other, e))
                    })?;

                let output_file = if cfg!(target_os = "windows") || target == Some("windows-x64") {
                    format!("{output}.exe")
                } else {
                    output.to_string()
                };

                println!("{} Linking executable...", "Compiler:".blue());
                let mut clang = Command::new("clang");
                clang.arg(&obj_file).arg("-o").arg(&output_file);

                if let Some(target_triple) = target {
                    clang.arg(format!("--target={}", target_triple));
                }

                let status = clang.status()
                    .map_err(|e| {
                        eprintln!("{} Failed to invoke clang: {}", "Error:".red(), e);
                        CompilerError::LinkError(format!("Failed to invoke clang: {}", e))
                    })?;
                
                if !status.success() {
                    let error_msg = format!("Clang failed to link the executable (exit code: {:?})", status.code());
                    eprintln!("{} {}", "Error:".red(), error_msg);
                    return Err(CompilerError::LinkError(error_msg));
                }

                println!("{} Executable written to {}", "Success:".green(), output_file);
            }
            _ => {
                let error_msg = format!("Invalid emit type: {}", emit);
                eprintln!("{} {}", "Error:".red(), error_msg);
                return Err(CompilerError::CodegenError(error_msg));
            }
        }

        Ok(())
    }
}
