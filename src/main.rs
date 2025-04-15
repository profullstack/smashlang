use std::env;
use std::fs;
use std::path::Path;
use std::process;
use colored::*;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;

mod lexer;
mod parser;
mod interpreter;
mod compiler;

use lexer::Lexer;
use parser::SmashParser as Parser;
use parser::AstNode;
use interpreter::Interpreter;
use compiler::Compiler;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        // No arguments provided, start REPL
        start_repl();
    } else {
        // First argument is the command
        match args[1].as_str() {
            "run" => {
                if args.len() < 3 {
                    eprintln!("Error: No file specified");
                    print_usage();
                    process::exit(1);
                }
                
                let file_path = &args[2];
                run_file(file_path);
            },
            "compile" => {
                if args.len() < 3 {
                    eprintln!("Error: No file specified");
                    print_usage();
                    process::exit(1);
                }
                
                let file_path = &args[2];
                compile_file(file_path);
            },
            "help" => {
                print_usage();
            },
            "version" => {
                print_version();
            },
            _ => {
                eprintln!("Error: Unknown command '{}'", args[1]);
                print_usage();
                process::exit(1);
            }
        }
    }
}

fn print_usage() {
    println!("SmashLang - A JavaScript-like language in Rust");
    println!();
    println!("Usage:");
    println!("  smash                   Start the REPL");
    println!("  smash run <file>        Run a SmashLang file");
    println!("  smash compile <file>    Compile a SmashLang file");
    println!("  smash help              Show this help message");
    println!("  smash version           Show version information");
}

fn print_version() {
    let version = env!("CARGO_PKG_VERSION");
    let git_hash = option_env!("GIT_HASH").unwrap_or("unknown");
    
    println!("SmashLang v{} ({})", version, git_hash);
    println!("A JavaScript-like language in Rust");
}

fn start_repl() {
    println!("{}", "SmashLang REPL".bright_green().bold());
    println!("Type .help for commands, .exit to quit");
    
    let mut rl = DefaultEditor::new().unwrap();
    let _interpreter = Interpreter::new();
    
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line).unwrap();
                
                if line.trim().is_empty() {
                    continue;
                }
                
                // Handle REPL commands
                if line.starts_with(".") {
                    match line.trim() {
                        ".exit" | ".quit" => break,
                        ".help" => {
                            println!("REPL Commands:");
                            println!("  .exit, .quit    Exit the REPL");
                            println!("  .help           Show this help message");
                            println!("  .version        Show version information");
                            continue;
                        },
                        ".version" => {
                            print_version();
                            continue;
                        },
                        _ => {
                            println!("Unknown command: {}", line);
                            continue;
                        }
                    }
                }
                
                // Execute the code
                match execute_code(&line) {
                    Ok(result) => println!("{}", result.to_string().bright_cyan()),
                    Err(err) => println!("{}: {}", "Error".bright_red(), err),
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("Ctrl-C");
                break;
            },
            Err(ReadlineError::Eof) => {
                println!("Ctrl-D");
                break;
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}

fn run_file(file_path: &str) {
    let path = Path::new(file_path);
    
    if !path.exists() {
        eprintln!("Error: File '{}' not found", file_path);
        process::exit(1);
    }
    
    let source = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading file '{}': {}", file_path, err);
            process::exit(1);
        }
    };
    
    match execute_code(&source) {
        Ok(_) => {},
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    }
}

fn compile_file(file_path: &str) {
    let path = Path::new(file_path);
    
    if !path.exists() {
        eprintln!("Error: File '{}' not found", file_path);
        process::exit(1);
    }
    
    let source = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading file '{}': {}", file_path, err);
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
            eprintln!("Parse error: {}", err);
            process::exit(1);
        }
    };
    
    // Convert to AST
    let ast = match pairs.next().and_then(AstNode::from_pair) {
        Some(ast) => ast,
        None => {
            eprintln!("Failed to convert parse tree to AST");
            process::exit(1);
        }
    };
    
    // Compile the AST
    let mut compiler = Compiler::new();
    let compiled_fn = match compiler.compile(&ast) {
        Ok(compiled_fn) => compiled_fn,
        Err(err) => {
            eprintln!("Compilation error: {}", err);
            process::exit(1);
        }
    };
    
    // Generate output file name
    let output_path = path.with_extension("out");
    let output_file = output_path.to_str().unwrap();
    
    println!("Compiled to: {}", output_file);
    
    // Execute the compiled function
    let result = unsafe { compiled_fn.execute() };
    println!("Execution result: {}", result);
}

fn execute_code(source: &str) -> Result<interpreter::Value, String> {
    // Parse the source code
    let mut lexer = Lexer::new(source);
    let _tokens = lexer.tokenize();
    
    // Parse the source code
    let mut pairs = match Parser::parse(source) {
        Ok(pairs) => pairs,
        Err(err) => {
            return Err(format!("Parse error: {}", err));
        }
    };
    
    // Convert to AST
    let ast = match pairs.next().and_then(AstNode::from_pair) {
        Some(ast) => ast,
        None => {
            return Err("Failed to convert parse tree to AST".to_string());
        }
    };
    
    // Interpret the AST
    let interpreter = Interpreter::new();
    match interpreter.evaluate(&ast) {
        Ok(value) => Ok(value),
        Err(err) => Err(format!("Runtime error: {}", err)),
    }
}
