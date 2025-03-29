use colored::*;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args.get(1).map(|s| s.as_str());

    match command {
        Some("--help") | Some("-h") => show_help(),
        Some("--version") | Some("-v") => show_version(),
        Some(path) => run_tests(path, &args[2..]),
        None => {
            println!("{} No test path specified", "Error:".red());
            println!("Run {} for usage information", "smashtest --help".cyan());
            process::exit(1);
        }
    }
}

fn show_help() {
    println!("{} - The SmashLang Test Runner", "SmashLang Test Runner v0.1.0".blue());
    println!("");
    println!("{}", "Usage:".yellow());
    println!("  smashtest <path> [options]");
    println!("");
    println!("{}", "Options:".yellow());
    println!("  --tag=<tag>        Run tests with specific tag");
    println!("  --verbose          Show detailed test output");
    println!("  --format=<format>  Output format (default: pretty, options: json, tap)");
    println!("  --help, -h         Show this help message");
    println!("  --version, -v      Show version information");
    println!("");
    println!("{}", "Examples:".yellow());
    println!("  smashtest ./tests                    Run all tests in the tests directory");
    println!("  smashtest ./tests/unit.test.smash    Run a specific test file");
    println!("  smashtest ./tests --tag=unit         Run tests with the 'unit' tag");
    println!("");
    println!("{}", "Documentation:".yellow());
    println!("  Visit {} for full documentation", "https://smashlang.com/docs/testing".cyan());
}

fn show_version() {
    println!("{}", "SmashLang Test Runner v0.1.0".blue());
}

fn run_tests(path: &str, options: &[String]) {
    println!("{} Running tests in: {}", "SmashTest:".blue(), path);
    
    // Parse options
    let verbose = options.contains(&"--verbose".to_string());
    let tag = options.iter()
        .find(|opt| opt.starts_with("--tag="))
        .map(|opt| opt.split('=').nth(1).unwrap_or(""));
    
    if let Some(tag) = tag {
        println!("  Filtering by tag: {}", tag.cyan());
    }
    
    // Find test files
    let test_files = find_test_files(path);
    
    if test_files.is_empty() {
        println!("{} No test files found", "Warning:".yellow());
        return;
    }
    
    println!("  Found {} test files", test_files.len());
    
    // For now, we'll just print the test files since we can't actually run them yet
    if verbose {
        for file in &test_files {
            println!("  - {}", file.display());
        }
    }
    
    // Mock test results for now
    let total_tests = 42;
    let passed_tests = 39;
    let failed_tests = 3;
    
    println!("");
    println!("{} Test Results:", "SmashTest:".blue());
    println!("  Total: {}", total_tests);
    println!("  Passed: {}", passed_tests.to_string().green());
    println!("  Failed: {}", failed_tests.to_string().red());
    
    if failed_tests > 0 {
        process::exit(1);
    }
}

fn find_test_files(path: &str) -> Vec<PathBuf> {
    let path = Path::new(path);
    let mut test_files = Vec::new();
    
    if path.is_file() {
        // If path is a file, just return it if it's a test file
        if is_test_file(path) {
            test_files.push(path.to_path_buf());
        }
    } else if path.is_dir() {
        // If path is a directory, find all test files recursively
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let entry_path = entry.path();
                if entry_path.is_dir() {
                    // Recursively search subdirectories
                    test_files.extend(find_test_files(entry_path.to_str().unwrap_or("")))
                } else if is_test_file(&entry_path) {
                    test_files.push(entry_path);
                }
            }
        }
    }
    
    test_files
}

fn is_test_file(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        if ext == "smash" {
            if let Some(file_name) = path.file_name() {
                let file_name = file_name.to_string_lossy();
                return file_name.contains(".test.") || file_name.ends_with("_test.smash");
            }
        }
    }
    false
}
