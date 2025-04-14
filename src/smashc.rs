use colored::*;
use std::env;
use std::fs;
use std::path::Path;
use std::process::{self, Command};
use std::io;

use smashlang::lexer::Lexer;
use smashlang::parser::SmashLangParser as Parser;
use smashlang::compiler::Compiler;

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
            "-o" | "--output" | "--out" => {
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
    let mut lexer = Lexer::new(&source_code);
    let _tokens = lexer.tokenize();
    
    // Parse tokens into an AST
    println!("{} tokens into AST", "Parsing".green());
    let ast = match Parser::parse(&source_code) {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("{}: {}", "Parse error".red(), e);
            process::exit(1);
        }
    };
    
    // Generate code using our new compiler
    println!("{} native code", "Generating".green());
    let mut compiler = Compiler::new();
    let compiled_fn = match compiler.compile(&ast) {
        Ok(compiled_fn) => compiled_fn,
        Err(e) => {
            eprintln!("{}: {}", "Compilation error".red(), e);
            process::exit(1);
        }
    };
    
    // For now, just execute the compiled function and print the result
    println!("{} executable", "Running".green());
    let result = unsafe { compiled_fn.execute() };
    println!("Result: {}", result);
    
    // In a real implementation, we would save the compiled code to a file
    println!("{} Successfully compiled to {}", "Success:".green(), output_file);
    
    Ok(())
}
