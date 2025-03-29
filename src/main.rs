use std::env;
use std::process;
use std::fs;
use std::path::Path;
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
  smash --version          Display version information
  smash --docs [topic]     View documentation in the terminal");
    println!("{}", "Options:".yellow());
    println!("  --out <output>          Specify output filename
  --target <target>        Specify target platform
  --emit ir|obj|exe        Specify output format");
    println!("{}", "Examples:".yellow());
    println!("  smash example.smash
  smash example.smash --out myprogram --emit exe
  smash example.smash --target wasm32
  smash --docs             View documentation topics
  smash --docs functions   View documentation on functions");
}

fn display_docs(topic: Option<&String>) {
    // Base directory for documentation
    let docs_dir = Path::new(env!("")).join("../../docs");
    
    if !docs_dir.exists() {
        println!("{}: Documentation directory not found", "Error".red());
        println!("Expected documentation at: {}", docs_dir.display());
        process::exit(1);
    }
    
    match topic {
        None => {
            // Display available documentation topics
            println!("{}", "SmashLang Documentation".bright_cyan().bold());
            println!("Available topics:
");
            
            // Read README.md to display main topics
            let readme_path = docs_dir.join("README.md");
            if readme_path.exists() {
                let content = fs::read_to_string(readme_path)
                    .expect("Failed to read documentation index");
                
                // Extract and display section headers from README
                for line in content.lines() {
                    if line.starts_with("### ") {
                        println!("  {}", line.trim_start_matches("### ").bright_green());
                    } else if line.starts_with("- [") && line.contains("](./") {
                        // Extract topic name and path from markdown link
                        if let Some(name_end) = line.find("](./") {
                            let name = line[3..name_end].to_string();
                            let path_start = name_end + 4;
                            let path_end = line[path_start..].find(")").unwrap_or(line.len() - path_start) + path_start;
                            let path = line[path_start..path_end].to_string();
                            
                            println!("    - {} (--docs {})", name, path.replace(".md", "").replace("/", "."));
                        }
                    }
                }
                
                println!("
Usage: smash --docs <topic>
Example: smash --docs getting-started.installation");
            } else {
                println!("{}: Documentation index not found", "Warning".yellow());
                // Fall back to listing directories
                if let Ok(entries) = fs::read_dir(&docs_dir) {
                    for entry in entries.filter_map(Result::ok) {
                        if entry.path().is_dir() {
                            println!("  {}", entry.file_name().to_string_lossy().bright_green());
                        }
                    }
                }
            }
        },
        Some(topic) => {
            // Try to find and display the requested documentation
            let topic_path = topic.replace(".", "/");
            let mut doc_path = docs_dir.join(&topic_path);
            
            // If the path doesn't have an extension, assume it's a markdown file
            if !doc_path.extension().is_some() {
                doc_path.set_extension("md");
            }
            
            // If the exact path doesn't exist, try some alternatives
            if !doc_path.exists() {
                // Try with .md extension
                doc_path = docs_dir.join(format!("{}.md", topic_path));
                
                // Try in language directory
                if !doc_path.exists() {
                    doc_path = docs_dir.join("language").join(format!("{}.md", topic_path));
                }
                
                // Try in getting-started directory
                if !doc_path.exists() {
                    doc_path = docs_dir.join("getting-started").join(format!("{}.md", topic_path));
                }
            }
            
            if doc_path.exists() {
                // Read and display the documentation file
                match fs::read_to_string(&doc_path) {
                    Ok(content) => {
                        // Simple markdown to terminal text conversion
                        let mut in_code_block = false;
                        
                        for line in content.lines() {
                            if line.starts_with("```") {
                                in_code_block = !in_code_block;
                                if in_code_block {
                                    println!("{}", "─────────────────────────────────────────".bright_black());
                                } else {
                                    println!("{}", "─────────────────────────────────────────".bright_black());
                                }
                                continue;
                            }
                            
                            if in_code_block {
                                println!("{}", line.bright_yellow());
                            } else if line.starts_with("# ") {
                                println!("{}", line.trim_start_matches("# ").bright_cyan().bold());
                            } else if line.starts_with("## ") {
                                println!("
{}", line.trim_start_matches("## ").bright_green().bold());
                            } else if line.starts_with("### ") {
                                println!("
{}", line.trim_start_matches("### ").yellow().bold());
                            } else if line.starts_with("- ") || line.starts_with("* ") {
                                println!("  • {}", line.trim_start_matches("- ").trim_start_matches("* "));
                            } else {
                                println!("{}", line);
                            }
                        }
                    },
                    Err(e) => {
                        println!("{}: Failed to read documentation: {}", "Error".red(), e);
                        process::exit(1);
                    }
                }
            } else {
                println!("{}: Documentation topic '{}' not found", "Error".red(), topic);
                println!("Try 'smash --docs' to see available topics");
                process::exit(1);
            }
        }
    }
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
        "--docs" => {
            display_docs(if args.len() > 2 { Some(&args[2]) } else { None });
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
