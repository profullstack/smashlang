use colored::*;
use std::env;
use std::fs;
use std::path::Path;
use std::process;
use std::io;

use smashlang::lexer::Lexer;
use smashlang::parser::{SmashParser as Parser, AstNode};
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
                        "windows" => "x86_64-pc-windows-msvc",
                        _ => {
                            eprintln!("{}: Unknown target '{}'", "Error".red(), target_name);
                            process::exit(1);
                        }
                    });
                    i += 2;
                } else {
                    eprintln!("{}: Missing target name after --target", "Error".red());
                    process::exit(1);
                }
            },
            "--wasm" => {
                target = Some("wasm32-unknown-unknown");
                i += 1;
            },
            _ => {
                eprintln!("{}: Unknown option '{}'", "Error".red(), args[i]);
                process::exit(1);
            }
        }
    }
    
    // Check if input file exists
    let path = Path::new(input_file);
    if !path.exists() {
        eprintln!("{}: File '{}' not found", "Error".red(), input_file);
        process::exit(1);
    }
    
    // Read input file
    let source = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("{}: Failed to read '{}': {}", "Error".red(), input_file, err);
            process::exit(1);
        }
    };
    
    // Parse the source code
    let mut lexer = Lexer::new(&source);
    let _tokens = lexer.tokenize();
    
    // Parse the source code
    let mut pairs = match Parser::parse(&source) {
        Ok(pairs) => pairs,
        Err(err) => {
            eprintln!("{}: {}", "Parse error".red(), err);
            process::exit(1);
        }
    };
    
    // Convert to AST
    let ast = match pairs.next().and_then(AstNode::from_pair) {
        Some(ast) => ast,
        None => {
            eprintln!("{}: Failed to convert parse tree to AST", "Error".red());
            process::exit(1);
        }
    };
    
    // Compile the AST
    let mut compiler = Compiler::new();
    
    // Set target if specified
    if let Some(target_triple) = target {
        println!("{}: Targeting {}", "Info".blue(), target_triple);
        // Note: Compiler::set_target is not implemented yet
        // compiler.set_target(target_triple);
    }
    
    let compiled_fn = match compiler.compile(&ast) {
        Ok(compiled_fn) => compiled_fn,
        Err(err) => {
            eprintln!("{}: {}", "Compilation error".red(), err);
            process::exit(1);
        }
    };
    
    // Write output file
    println!("{}: Compiled to {}", "Success".green(), output_file);
    
    // Execute the compiled function
    let result = unsafe { compiled_fn.execute() };
    println!("Execution result: {}", result);
    
    Ok(())
}
