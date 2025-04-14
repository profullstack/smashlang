use colored::*;
use std::env;
use std::fs;
use std::path::Path;
use std::process::{self, Command};
use std::io;

use smashlang::lexer::tokenize;
use smashlang::parser::Parser;
// Import removed to fix warning
use smashlang::codegen::CodeGenerator;

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
    println!("{} C file will be saved at {}", "Info:".blue(), c_file);
    
    // Generate C code from the AST
    println!("{} Generating code...", "Compiler:".blue());
    let mut generator = CodeGenerator::new();
    let c_code = match generator.generate(&ast) {
        Ok(code) => code,
        Err(e) => {
            println!("{} Failed to generate C code: {}", "Error:".red(), e);
            return Err(io::Error::new(io::ErrorKind::Other, e));
        }
    };
    
    // Print the AST for debugging
    println!("AST: {:?}", ast);
    
    // Save a copy of the C code for inspection
    let debug_c_file = format!("/tmp/smash_debug_{}.c", std::process::id());
    match fs::write(&debug_c_file, &c_code) {
        Ok(_) => {
            println!("Saved C code to {}", debug_c_file);
        },
        Err(e) => {
            eprintln!("Failed to save C code: {}", e);
        }
    }
    
    // Write the generated C code to the output file
    if let Err(e) = fs::write(&c_file, &c_code) {
        println!("{} Failed to write C code to file: {}", "Error:".red(), e);
        return Err(e);
    }
    
    // Read and print the generated C file for debugging
    match fs::read_to_string(&c_file) {
        Ok(content) => {
            println!("Generated C code:\n{}", content);
        },
        Err(e) => {
            eprintln!("Failed to read generated C file: {}", e);
        }
    }
    
    // Use the generated C code file
    println!("{} executable", "Compiling".green());
    
    // Compile and link the generated C code file
    println!("{} executable", "Linking".green());
    
    // Get the absolute path to the src directory
    let current_dir = std::env::current_dir().unwrap_or_default();
    let project_root = current_dir.ancestors().find(|p| p.join("src").join("runtime.h").exists())
        .unwrap_or(&current_dir);
    let src_path = project_root.join("src");
    
    println!("{} Using include path: {}", "Info:".blue(), src_path.display());
    
    // Get the path to runtime.c
    let runtime_c_path = src_path.join("runtime.c");
    
    let status = Command::new("clang")
        .arg(format!("-I{}", src_path.display()))  // Add absolute include path to src directory
        .arg(&c_file)
        .arg(runtime_c_path.to_str().unwrap())  // Add runtime.c to compilation
        .arg(src_path.join("simple_regex.c").to_str().unwrap())  // Add our simple regex implementation
        .arg("-o")
        .arg(output_file)
        .status()?;
    
    if !status.success() {
        eprintln!("{}: Failed to compile and link direct test code", "Error".red());
        process::exit(1);
    }
    
    // Skip the original compilation process and just report success
    match Ok::<(), std::io::Error>(()) {
        Ok(_) => {
            println!("{} Successfully compiled to {}", "Success:".green(), output_file);
            
            // Keep the C file for inspection
            println!("{} C file saved at {}", "Info:".blue(), c_file);
        },
        Err(e) => {
            eprintln!("{}: Linking failed: {}", "Error".red(), e);
            process::exit(1);
        }
    }
    
    Ok(())
}


