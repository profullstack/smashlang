use colored::*;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        show_usage();
        process::exit(1);
    }
    
    let command = &args[1];
    
    match command.as_str() {
        "--help" | "-h" => show_help(),
        "--version" | "-v" => show_version(),
        _ => {
            // Check if it's a file
            if command.ends_with(".smash") && Path::new(command).exists() {
                compile_file(command, &args[2..]);
            } else {
                println!("{} Unknown command or file not found: {}", "Error:".red(), command);
                show_usage();
                process::exit(1);
            }
        }
    }
}

fn show_usage() {
    println!("Usage: {} [options] <file.smash>", "smashc".cyan());
    println!("Run {} for more information", "smashc --help".green());
}

fn show_help() {
    println!("{} - The SmashLang Compiler", "SmashLang Compiler v0.1.0".blue());
    println!("");
    println!("{}", "Usage:".yellow());
    println!("  smashc [options] <file.smash>");
    println!("");
    println!("{}", "Options:".yellow());
    println!("  -h, --help                 Show this help message");
    println!("  -v, --version              Show version information");
    println!("  -o, --output <file>        Specify output file name");
    println!("  --target <platform>        Target platform (linux, macos, windows)");
    println!("  --wasm                     Compile to WebAssembly");
    println!("  --debug                    Include debug information");
    println!("  --release                  Optimize for release");
    println!("  --lib                      Compile as a library");
    println!("");
    println!("{}", "Examples:".yellow());
    println!("  smashc hello.smash                   Compile hello.smash to default output");
    println!("  smashc hello.smash -o hello          Specify output filename");
    println!("  smashc hello.smash --target windows  Cross-compile for Windows");
    println!("  smashc hello.smash --wasm            Compile to WebAssembly");
    println!("");
    println!("{}", "Documentation:".yellow());
    println!("  Visit {} for full documentation", "https://smashlang.com/docs/compiler".cyan());
}

fn show_version() {
    println!("{}", "SmashLang Compiler v0.1.0".blue());
}

fn compile_file(filename: &str, args: &[String]) {
    println!("{} Compiling {}", "SmashLang:".blue(), filename);
    
    // Parse compilation options
    let mut output_file = None;
    let mut target = "native";
    let mut wasm_mode = false;
    let mut debug_mode = false;
    let mut release_mode = false;
    let mut lib_mode = false;
    
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-o" | "--output" => {
                if i + 1 < args.len() {
                    output_file = Some(args[i + 1].clone());
                    i += 1;
                }
            },
            "--target" => {
                if i + 1 < args.len() {
                    target = &args[i + 1];
                    i += 1;
                }
            },
            "--wasm" => wasm_mode = true,
            "--debug" => debug_mode = true,
            "--release" => release_mode = true,
            "--lib" => lib_mode = true,
            _ => {}
        }
        i += 1;
    }
    
    // Determine output file name if not specified
    let output = match output_file {
        Some(name) => name,
        None => {
            let path = Path::new(filename);
            let stem = path.file_stem().unwrap_or_default().to_string_lossy();
            if wasm_mode {
                format!("{}.wasm", stem)
            } else if lib_mode {
                format!("lib{}.{}", stem, if target == "windows" { "dll" } else if target == "macos" { "dylib" } else { "so" })
            } else {
                format!("{}{}", stem, if target == "windows" { ".exe" } else { "" })
            }
        }
    };
    
    // Read source file
    match fs::read_to_string(filename) {
        Ok(source) => {
            println!("Source file size: {} bytes", source.len());
            
            // This is a placeholder for actual compilation
            println!("This is a placeholder for the actual compiler implementation.");
            println!("In the future, this will compile SmashLang code to native binaries.");
            
            // Show compilation options
            println!("\n{}", "Compilation options:".bright_cyan());
            println!("  Output file: {}", output);
            println!("  Target platform: {}", target);
            if wasm_mode { println!("  WebAssembly mode: enabled"); }
            if debug_mode { println!("  Debug mode: enabled"); }
            if release_mode { println!("  Release mode: enabled"); }
            if lib_mode { println!("  Library mode: enabled"); }
            
            println!("\n{} Compilation successful (placeholder)", "Success:".green());
        },
        Err(e) => {
            println!("{} Failed to read source file: {}", "Error:".red(), e);
            process::exit(1);
        }
    }
}
