use colored::*;
use std::env;
use std::fs;
use std::path::Path;
use std::process::{self, Command};
use std::io;

use smashlang::lexer::tokenize;
use smashlang::parser::Parser;
use smashlang::compiler::compile;
use smashlang::codegen::{generate_llvm_ir, FileType};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 || args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) {
        println!("{}", "SmashLang Compiler".bright_cyan().bold());
        println!("{}", "Examples:".yellow());
        println!("  smashc hello.smash                   Compile hello.smash to default output");
        println!("  smashc hello.smash -o hello          Specify output filename");
        println!("  smashc hello.smash --target linux    Compile for Linux x86_64");
        println!("  smashc hello.smash --target linux-arm64  Compile for Linux ARM64 (e.g., Raspberry Pi 4)");
        println!("  smashc hello.smash --target windows  Cross-compile for Windows");
        println!("  smashc hello.smash --wasm            Compile to WebAssembly");
        return Ok(());
    }
    
    // Parse command line arguments
    let input_file = &args[1];
    let mut output_file = "a.out";
    let mut target = None;
    
    // Process command line options
    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "-o" | "--output" => {
                if i + 1 < args.len() {
                    output_file = &args[i + 1];
                    i += 2;
                } else {
                    eprintln!("{}: Missing output filename after -o", "Error".red());
                    process::exit(1);
                }
            },
            "--target" => {
                if i + 1 < args.len() {
                    let target_name = &args[i + 1];
                    // Map user-friendly target names to LLVM target triples
                    target = Some(match target_name.as_str() {
                        "linux" => "x86_64-unknown-linux-gnu",
                        "linux-arm64" => "aarch64-unknown-linux-gnu",
                        "macos" => "x86_64-apple-darwin",
                        "macos-arm64" => "aarch64-apple-darwin",
                        "windows" => "x86_64-pc-windows-gnu",
                        "wasm" => "wasm32-unknown-unknown",
                        _ => target_name, // Use as-is if not recognized
                    });
                    i += 2;
                } else {
                    eprintln!("{}: Missing target name after --target", "Error".red());
                    process::exit(1);
                }
            },
            _ => {
                eprintln!("{}: Unknown option: {}", "Warning".yellow(), args[i]);
                i += 1;
            }
        }
    }
    
    // Check if input file exists
    let input_path = Path::new(input_file);
    if !input_path.exists() {
        eprintln!("{}: Input file '{}' not found", "Error".red(), input_file);
        process::exit(1);
    }
    
    // Read the input file
    println!("{} {}", "Reading".green(), input_file);
    let source_code = match fs::read_to_string(input_file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("{}: Failed to read input file: {}", "Error".red(), e);
            process::exit(1);
        }
    };
    
    // Tokenize the source code
    println!("{} source code", "Tokenizing".green());
    let tokens = tokenize(&source_code);
    
    // Parse tokens into an AST
    println!("{} tokens into AST", "Parsing".green());
    let mut parser = Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("{}: {}", "Parse error".red(), e);
            process::exit(1);
        }
    };
    
    // Generate intermediate code using our codegen module
    println!("{} intermediate code", "Generating".green());
    let c_file = format!("{}.c", output_file);
    
    // Generate code using our codegen module
    let (module, target_machine) = generate_llvm_ir(&ast, target.as_deref());
    if let Err(e) = target_machine.write_to_file(&module, FileType::Object, &c_file) {
        eprintln!("{}: {}", "Code generation error".red(), e);
        process::exit(1);
    };
    
    // Compile the C file using clang
    println!("{} executable", "Compiling".green());
    let obj_file = format!("{}.o", output_file);
    
    // Compile C to object file
    let status = Command::new("clang")
        .arg("-c")
        .arg(&c_file)
        .arg("-o")
        .arg(&obj_file)
        .status()?;
    
    if !status.success() {
        eprintln!("{}: Failed to compile intermediate code", "Error".red());
        process::exit(1);
    }
    
    // Link the object file to create the final executable
    println!("{} executable", "Linking".green());
    match compile(output_file, &obj_file, target.as_deref()) {
        Ok(_) => {
            println!("{} Successfully compiled to {}", "Success:".green(), output_file);
            
            // Clean up temporary files
            let _ = fs::remove_file(&c_file);
            let _ = fs::remove_file(&obj_file);
        },
        Err(e) => {
            eprintln!("{}: Linking failed: {}", "Error".red(), e);
            process::exit(1);
        }
    }
    
    Ok(())
}


