use colored::*;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;
use std::time::{Duration, Instant};

use std::process::Command;

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
    println!("  --timeout=<ms>     Test timeout in milliseconds (default: 5000)");
    println!("  --fail-fast        Stop on first test failure");
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

/// Output format for test results
enum OutputFormat {
    Pretty,
    Json,
    Tap,
}

/// Options for running tests
struct TestOptions {
    verbose: bool,
    tag: Option<String>,
    format: OutputFormat,
    timeout: u64,
    fail_fast: bool,
}

fn parse_options(options: &[String]) -> TestOptions {
    let verbose = options.contains(&"--verbose".to_string());
    
    let tag = options.iter()
        .find(|opt| opt.starts_with("--tag="))
        .map(|opt| opt.split('=').nth(1).unwrap_or("").to_string());
    
    let format = options.iter()
        .find(|opt| opt.starts_with("--format="))
        .map(|opt| opt.split('=').nth(1).unwrap_or("pretty"))
        .map(|fmt| match fmt {
            "json" => OutputFormat::Json,
            "tap" => OutputFormat::Tap,
            _ => OutputFormat::Pretty,
        })
        .unwrap_or(OutputFormat::Pretty);
    
    let timeout = options.iter()
        .find(|opt| opt.starts_with("--timeout="))
        .map(|opt| opt.split('=').nth(1).unwrap_or("5000"))
        .and_then(|t| t.parse::<u64>().ok())
        .unwrap_or(5000);
    
    let fail_fast = options.contains(&"--fail-fast".to_string());
    
    TestOptions {
        verbose,
        tag,
        format,
        timeout,
        fail_fast,
    }
}

fn run_tests(path: &str, options: &[String]) {
    let test_options = parse_options(options);
    
    println!("{} Running tests in: {}", "SmashTest:".blue(), path);
    
    if let Some(tag) = &test_options.tag {
        println!("  Filtering by tag: {}", tag.cyan());
    }
    
    // Find test files
    let test_files = find_test_files(path);
    
    if test_files.is_empty() {
        println!("{} No test files found", "Warning:".yellow());
        return;
    }
    
    println!("  Found {} test files", test_files.len());
    
    if test_options.verbose {
        for file in &test_files {
            println!("  - {}", file.display());
        }
    }
    
    // Initialize test counters
    let mut total_tests = 0;
    let mut passed_tests = 0;
    let mut failed_tests = 0;
    let mut skipped_tests = 0;
    let mut failed_files = Vec::new();
    
    // Track execution time
    let start_time = Instant::now();
    
    // Run each test file
    for file in &test_files {
        let file_result = run_test_file(file, &test_options);
        
        // Update counters
        total_tests += file_result.passed + file_result.failed + file_result.skipped;
        passed_tests += file_result.passed;
        failed_tests += file_result.failed;
        skipped_tests += file_result.skipped;
        
        // Track failed files
        if file_result.failed > 0 {
            failed_files.push(file.clone());
            
            // Stop on first failure if fail-fast is enabled
            if test_options.fail_fast {
                println!("{} Stopping due to --fail-fast option", "Note:".yellow());
                break;
            }
        }
    }
    
    let duration = start_time.elapsed();
    
    // Output results based on format
    match test_options.format {
        OutputFormat::Pretty => output_pretty_results(
            total_tests, passed_tests, failed_tests, skipped_tests, 
            duration, &failed_files, test_options.verbose
        ),
        OutputFormat::Json => output_json_results(
            total_tests, passed_tests, failed_tests, skipped_tests, 
            duration, &failed_files
        ),
        OutputFormat::Tap => output_tap_results(
            total_tests, passed_tests, failed_tests, skipped_tests
        ),
    }
    
    // Exit with error code if any tests failed
    if failed_tests > 0 {
        process::exit(1);
    }
}

/// Result of running a test file
struct TestFileResult {
    passed: usize,
    failed: usize,
    skipped: usize,
    test_results: Vec<(String, TestStatus)>,
}

impl TestFileResult {
    fn new() -> Self {
        TestFileResult {
            passed: 0,
            failed: 0,
            skipped: 0,
            test_results: Vec::new(),
        }
    }
}

/// Status of a test
enum TestStatus {
    Pass,
    Fail(String),
    Skip(String),
}

fn run_test_file(file_path: &Path, options: &TestOptions) -> TestFileResult {
    println!("{} Running tests in {}", "File:".blue(), file_path.display());
    
    // Verify the file exists and is readable
    if !file_path.exists() {
        println!("{} File does not exist: {}", "Error:".red(), file_path.display());
        return TestFileResult::new();
    }
    
    // Build the command to run the test file
    let mut cmd = Command::new("smash");
    cmd.arg(file_path);
    
    // Add testing flags
    cmd.arg("--test");
    
    // Add tag filter if specified
    if let Some(tag) = &options.tag {
        cmd.arg(format!("--test-tag={}", tag));
    }
    
    // Add timeout if specified
    cmd.arg(format!("--test-timeout={}", options.timeout));
    
    // Run the command and capture output
    let output = match cmd.output() {
        Ok(output) => output,
        Err(e) => {
            println!("{} Failed to execute test: {}", "Error:".red(), e);
            let mut result = TestFileResult::new();
            result.failed = 1; // Count execution failure as a test failure
            return result;
        }
    };
    
    // Parse the test results from the output
    let mut result = TestFileResult::new();
    
    if output.status.success() {
        // Parse the stdout for test results
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Simple parsing of test results from stdout
        // Format expected: [PASS/FAIL/SKIP] Test name - Optional message
        for line in stdout.lines() {
            if line.starts_with("[PASS]") {
                let test_name = line[6..].trim().to_string();
                result.test_results.push((test_name, TestStatus::Pass));
                result.passed += 1;
            } else if line.starts_with("[FAIL]") {
                let parts: Vec<&str> = line[6..].splitn(2, " - ").collect();
                let test_name = parts[0].trim().to_string();
                let message = parts.get(1).map_or("", |m| *m).to_string();
                result.test_results.push((test_name, TestStatus::Fail(message)));
                result.failed += 1;
            } else if line.starts_with("[SKIP]") {
                let parts: Vec<&str> = line[6..].splitn(2, " - ").collect();
                let test_name = parts[0].trim().to_string();
                let reason = parts.get(1).map_or("", |m| *m).to_string();
                result.test_results.push((test_name, TestStatus::Skip(reason)));
                result.skipped += 1;
            }
        }
    } else {
        // If the command failed, count it as a test failure
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("{} Test execution failed: {}", "Error:".red(), stderr);
        result.failed = 1;
    }
    
    // Print detailed results if verbose
    if options.verbose {
        for (test_name, status) in &result.test_results {
            match status {
                TestStatus::Pass => {
                    println!("  {} {}", "✓".green(), test_name);
                },
                TestStatus::Fail(message) => {
                    if message.is_empty() {
                        println!("  {} {}", "✗".red(), test_name);
                    } else {
                        println!("  {} {} - {}", "✗".red(), test_name, message);
                    }
                },
                TestStatus::Skip(reason) => {
                    if reason.is_empty() {
                        println!("  {} {}", "⚠".yellow(), test_name);
                    } else {
                        println!("  {} {} - {}", "⚠".yellow(), test_name, reason);
                    }
                },
            }
        }
        println!("");
    }
    
    result
}

fn output_pretty_results(
    total: usize, passed: usize, failed: usize, skipped: usize,
    duration: Duration, failed_files: &[PathBuf], verbose: bool
) {
    println!("");
    println!("{} Test Results:", "SmashTest:".blue());
    println!("  Total:   {}", total);
    println!("  Passed:  {}", passed.to_string().green());
    println!("  Failed:  {}", failed.to_string().red());
    println!("  Skipped: {}", skipped.to_string().yellow());
    println!("  Time:    {:.2}s", duration.as_secs_f64());
    
    if !failed_files.is_empty() && verbose {
        println!("");
        println!("{} Failed test files:", "Failures:".red());
        for file in failed_files {
            println!("  - {}", file.display());
        }
    }
}

fn output_json_results(
    total: usize, passed: usize, failed: usize, skipped: usize,
    duration: Duration, failed_files: &[PathBuf]
) {
    let failed_paths: Vec<String> = failed_files.iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect();
    
    let json = format!(r#"{{\n  "total": {},\n  "passed": {},\n  "failed": {},\n  "skipped": {},\n  "duration": {},\n  "failed_files": {:?}\n}}"#,
        total, passed, failed, skipped, duration.as_secs_f64(), failed_paths
    );
    
    println!("{}", json);
}

fn output_tap_results(total: usize, passed: usize, failed: usize, skipped: usize) {
    println!("TAP version 13");
    println!("1..{}", total);
    println!("# tests {}", total);
    println!("# pass {}", passed);
    println!("# fail {}", failed);
    println!("# skip {}", skipped);
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
