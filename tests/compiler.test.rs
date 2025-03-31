use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::tempdir;

#[test]
fn test_compiler_basic_functionality() {
    // Skip this test if we're not on a system where we can run the compiler
    if !Path::new("/usr/bin/clang").exists() && !Path::new("/usr/local/bin/clang").exists() {
        println!("Skipping test_compiler_basic_functionality: clang not found");
        return;
    }

    // Create a temporary directory for our test files
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let test_file_path = temp_dir.path().join("test.smash");
    let output_path = temp_dir.path().join("test_output");
    
    // Write a simple SmashLang program to the test file
    fs::write(&test_file_path, "print;").expect("Failed to write test file");
    
    // Run the compiler on the test file
    let status = Command::new("cargo")
        .args(["run", "--bin", "smashc", 
               test_file_path.to_str().unwrap(), 
               "-o", output_path.to_str().unwrap()])
        .status()
        .expect("Failed to execute compiler");
    
    // Check that the compiler ran successfully
    assert!(status.success(), "Compiler did not exit successfully");
    
    // Check that the output file was created
    assert!(output_path.exists(), "Output file was not created");
    
    // Run the compiled program
    let run_status = Command::new(output_path.to_str().unwrap())
        .status()
        .expect("Failed to run compiled program");
    
    // Check that the program ran successfully
    assert!(run_status.success(), "Compiled program did not run successfully");
}

#[test]
fn test_compiler_variable_declaration() {
    // Skip this test if we're not on a system where we can run the compiler
    if !Path::new("/usr/bin/clang").exists() && !Path::new("/usr/local/bin/clang").exists() {
        println!("Skipping test_compiler_variable_declaration: clang not found");
        return;
    }

    // Create a temporary directory for our test files
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let test_file_path = temp_dir.path().join("test_var.smash");
    let output_path = temp_dir.path().join("test_var_output");
    
    // Write a SmashLang program with variable declaration
    fs::write(&test_file_path, "let x = 10;\nprint;").expect("Failed to write test file");
    
    // Run the compiler on the test file
    let status = Command::new("cargo")
        .args(["run", "--bin", "smashc", 
               test_file_path.to_str().unwrap(), 
               "-o", output_path.to_str().unwrap()])
        .status()
        .expect("Failed to execute compiler");
    
    // Check that the compiler ran successfully
    assert!(status.success(), "Compiler did not exit successfully with variable declaration");
    
    // Check that the output file was created
    assert!(output_path.exists(), "Output file was not created for variable declaration test");
    
    // Run the compiled program
    let run_status = Command::new(output_path.to_str().unwrap())
        .status()
        .expect("Failed to run compiled program with variable declaration");
    
    // Check that the program ran successfully
    assert!(run_status.success(), "Compiled program with variable declaration did not run successfully");
}

#[test]
fn test_compiler_examples() {
    // Skip this test if we're not on a system where we can run the compiler
    if !Path::new("/usr/bin/clang").exists() && !Path::new("/usr/local/bin/clang").exists() {
        println!("Skipping test_compiler_examples: clang not found");
        return;
    }

    // Test that all examples in the docs/getting-started directory compile successfully
    let examples_dir = Path::new("docs/getting-started");
    
    if !examples_dir.exists() {
        println!("Skipping test_compiler_examples: Examples directory not found");
        return;
    }
    
    // Create a temporary directory for outputs
    let temp_dir = tempdir().expect("Failed to create temp directory");
    
    // Find all .smash files in the examples directory
    let entries = fs::read_dir(examples_dir).expect("Failed to read examples directory");
    
    for entry in entries {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();
        
        if path.extension().and_then(|ext| ext.to_str()) == Some("smash") {
            // Skip README.md, run_all_examples.sh, etc.
            let file_stem = path.file_stem().unwrap().to_str().unwrap();
            let output_path = temp_dir.path().join(file_stem);
            
            println!("Testing example: {}", path.display());
            
            // Run the compiler on the example file
            let status = Command::new("cargo")
                .args(["run", "--bin", "smashc", 
                       path.to_str().unwrap(), 
                       "-o", output_path.to_str().unwrap()])
                .status()
                .expect(&format!("Failed to execute compiler on {}", path.display()));
            
            // Check that the compiler ran successfully
            assert!(status.success(), "Compiler failed on example: {}", path.display());
            
            // Check that the output file was created
            assert!(output_path.exists(), "Output file was not created for example: {}", path.display());
            
            // Run the compiled program
            let run_status = Command::new(output_path.to_str().unwrap())
                .status()
                .expect(&format!("Failed to run compiled example: {}", path.display()));
            
            // Check that the program ran successfully
            assert!(run_status.success(), "Compiled example did not run successfully: {}", path.display());
        }
    }
}
