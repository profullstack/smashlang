use std::env;
use std::process;
use colored::*;

use smashlang::compiler::compile_file;
use smashlang::repl::Repl;

const VERSION: &str = "0.1.0";

fn display_help() {
    println!("{}", "SmashLang Compiler".bright_cyan().bold());
    println!("Version: {}", VERSION);
    println!("{}", "Usage:".yellow());
    println!("  smash                   Start the REPL
  smash repl               Start the REPL
  smash <file.smash>       Compile a SmashLang file
  smash --help             Display this help message
  smash --version          Display version information");
    println!("{}", "Options:".yellow());
    println!("  --out <output>          Specify output filename
  --target <target>        Specify target platform
  --emit ir|obj|exe        Specify output format");
    println!("{}", "Examples:".yellow());
    println!("  smash example.smash
  smash example.smash --out myprogram --emit exe
  smash example.smash --target wasm32");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // If no arguments provided, start REPL mode
    if args.len() < 2 {
        println!("{}", "Starting SmashLang REPL...".bright_cyan());
        let mut repl = Repl::new();
        repl.run();
        return;
    }
    
    // Process command line arguments
    match args[1].as_str() {
        "repl" => {
            println!("{}", "Starting SmashLang REPL...".bright_cyan());
            let mut repl = Repl::new();
            repl.run();
            return;
        },
        "--help" | "-h" => {
            display_help();
            return;
        },
        "--version" | "-v" => {
            println!("SmashLang version {}", VERSION);
            return;
        },

        _ => {
            // Assume it's a file to compile
            let input = &args[1];
            let mut output = "out";
            let mut target = None;
            let mut emit = "exe";

            let mut i = 2;
            while i < args.len() {
                if i >= args.len() {
                    println!("{}: Missing value for argument {}", "Error".red(), args[i-1]);
                    process::exit(1);
                }
                
                match args[i].as_str() {
                    "--out" => {
                        i += 1;
                        if i >= args.len() {
                            println!("{}: Missing value for --out", "Error".red());
                            process::exit(1);
                        }
                        output = &args[i];
                    }
                    "--target" => {
                        i += 1;
                        if i >= args.len() {
                            println!("{}: Missing value for --target", "Error".red());
                            process::exit(1);
                        }
                        target = Some(args[i].as_str());
                    }
                    "--emit" => {
                        i += 1;
                        if i >= args.len() {
                            println!("{}: Missing value for --emit", "Error".red());
                            process::exit(1);
                        }
                        let emit_value = args[i].as_str();
                        if !["ir", "obj", "exe"].contains(&emit_value) {
                            println!("{}: Invalid emit format '{}'. Must be one of: ir, obj, exe", "Error".red(), emit_value);
                            process::exit(1);
                        }
                        emit = emit_value;
                    }
                    _ => {
                        println!("{}: Unknown option: {}", "Warning".yellow(), args[i]);
                    }
                }
                i += 1;
            }

            println!("{} {} to {}.{}", "Compiling".bright_green(), input, output, emit);
            compile_file(input, output, target, emit);
            println!("{}", "Compilation complete!".bright_green());
        }
    }
}
