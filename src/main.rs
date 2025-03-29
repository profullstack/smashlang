use std::env;
use std::process;
use std::fs;
use std::path::Path;
use colored::*;

use smashlang::compiler::compile_file;
use smashlang::repl::Repl;

const VERSION: &str = "0.1.0";

// Get git hash at compile time if available
const GIT_HASH: Option<&str> = option_env!("GIT_HASH");

fn display_help() {
    println!("{}", "SmashLang Compiler".bright_cyan().bold());
    println!("Version: {}", VERSION);
    println!("{}", "Usage:".yellow());
    println!("  smash                   Start the REPL
  smash repl               Start the REPL
  smash <file.smash>       Compile a SmashLang file
  smash --help             Display this help message
  smash --version          Display version information
  smash docs [topic]       View documentation in the terminal
  smash docs browser       Generate HTML docs and open in browser");
    println!("{}", "Options:".yellow());
    println!("  --out <output>          Specify output filename
  --target <target>        Specify target platform
  --emit ir|obj|exe        Specify output format");
    println!("{}", "Examples:".yellow());
    println!("  smash example.smash
  smash example.smash --out myprogram --emit exe
  smash example.smash --target wasm32
  smash docs               View documentation topics
  smash docs functions     View documentation on functions
  smash docs browser       Generate HTML docs and open in browser");
}

use std::process::Command;

fn display_docs(topic: Option<&String>, browser_mode: bool) {
    // Try to find documentation in multiple locations
    let possible_doc_locations = [
        Path::new("docs").to_path_buf(),
        Path::new("../docs").to_path_buf(),
        // Try to find docs relative to the executable location
        std::env::current_exe().ok().and_then(|p| p.parent().map(|d| d.join("docs"))).unwrap_or_else(|| Path::new("/usr/local/share/smashlang/docs").to_path_buf()),
        // Common installation locations
        Path::new("/usr/local/share/smashlang/docs").to_path_buf(),
        Path::new("/usr/share/smashlang/docs").to_path_buf(),
        Path::new("~/.local/share/smashlang/docs").to_path_buf(),
    ];
    
    // Find the first valid docs directory
    let docs_dir = possible_doc_locations.iter()
        .find(|&path| path.exists())
        .cloned()
        .unwrap_or_else(|| Path::new("docs").to_path_buf());
    
    if !docs_dir.exists() {
        println!("{}: Documentation directory not found", "Error".red());
        println!("Expected documentation at: {}", docs_dir.display());
        println!("Looked in: {}", possible_doc_locations.iter().map(|p| p.display().to_string()).collect::<Vec<_>>().join(", "));
        process::exit(1);
    }
    
    // Create output directory for HTML docs if in browser mode
    let html_output_dir = if browser_mode {
        let output_dir = Path::new("docs_html");
        if !output_dir.exists() {
            match fs::create_dir_all(output_dir) {
                Ok(_) => {},
                Err(e) => {
                    println!("{}: Failed to create HTML output directory: {}", "Error".red(), e);
                    process::exit(1);
                }
            }
        }
        Some(output_dir)
    } else {
        None
    };
    
    match topic {
        None => {
            // Display available documentation topics
            println!("{}", "SmashLang Documentation".bright_cyan().bold());
            println!("Available topics:
");
            
            // Read README.md to display main topics
            let readme_path = docs_dir.join("README.md");
            if readme_path.exists() {
                let content = fs::read_to_string(&readme_path)
                    .expect("Failed to read documentation index");
                
                // If in browser mode, convert README to HTML and open it
                if browser_mode {
                    generate_html_doc(&readme_path, html_output_dir.as_ref().unwrap(), true);
                    return;
                }
                
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
                            
                            println!("    - {} (docs {})", name, path.replace(".md", "").replace("/", "."));
                        }
                    }
                }
                
                println!("
Usage: smash docs <topic>
Example: smash docs getting-started.installation");
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
            // Check if it's a special subcommand
            if topic == "browser" {
                display_docs(None, true);
                return;
            }
            
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
                // If in browser mode, convert to HTML and open it
                if browser_mode {
                    generate_html_doc(&doc_path, html_output_dir.as_ref().unwrap(), true);
                    return;
                }
                
                // Read and display the documentation file in terminal mode
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
                println!("Try 'smash docs' to see available topics");
                process::exit(1);
            }
        }
    }
}

// Function to generate HTML documentation using Pandoc
fn generate_html_doc(markdown_path: &Path, output_dir: &Path, open_browser: bool) {
    let file_stem = markdown_path.file_stem().unwrap_or_default().to_string_lossy();
    let html_path = output_dir.join(format!("{}.html", file_stem));
    
    println!("{} Converting {} to HTML...", "Docs:".blue(), markdown_path.display());
    
    // Call pandoc to convert markdown to HTML
    let status = Command::new("pandoc")
        .arg(markdown_path)
        .arg("-o")
        .arg(&html_path)
        .arg("--standalone")
        .arg("--metadata")
        .arg("title=SmashLang Documentation")
        .arg("--css=https://cdn.jsdelivr.net/npm/water.css@2/out/water.css")
        .status();
    
    match status {
        Ok(exit_status) => {
            if exit_status.success() {
                println!("{} HTML documentation generated: {}", "Success:".green(), html_path.display());
                
                // Open in browser if requested
                if open_browser {
                    println!("{} Opening documentation in browser...", "Docs:".blue());
                    
                    #[cfg(target_os = "linux")]
                    {
                        let _ = Command::new("xdg-open")
                            .arg(&html_path)
                            .spawn();
                    }
                    
                    #[cfg(target_os = "macos")]
                    {
                        let _ = Command::new("open")
                            .arg(&html_path)
                            .spawn();
                    }
                    
                    #[cfg(target_os = "windows")]
                    {
                        let _ = Command::new("cmd")
                            .args(["/c", "start", html_path.to_str().unwrap_or("")])
                            .spawn();
                    }
                }
            } else {
                println!("{} Failed to generate HTML documentation", "Error:".red());
                println!("Make sure pandoc is installed: https://pandoc.org/installing.html");
            }
        },
        Err(e) => {
            println!("{} Failed to run pandoc: {}", "Error:".red(), e);
            println!("Make sure pandoc is installed: https://pandoc.org/installing.html");
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
        "docs" => {
            // Check if we have a browser subcommand
            if args.len() > 2 && args[2] == "browser" {
                display_docs(None, true);
            } else {
                display_docs(if args.len() > 2 { Some(&args[2]) } else { None }, false);
            }
            return;
        },
        "--help" | "-h" => {
            display_help();
            return;
        },
        "--version" | "-v" => {
            // Display version with git hash if available
            match GIT_HASH {
                Some(hash) if !hash.is_empty() => println!("SmashLang version {} (git: {})", VERSION, hash),
                _ => println!("SmashLang version {}", VERSION),
            }
            return;
        },
        "--docs" => {
            // For backward compatibility
            display_docs(if args.len() > 2 { Some(&args[2]) } else { None }, false);
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
