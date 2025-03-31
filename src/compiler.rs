use std::process::Command;
use colored::*;

pub fn compile(output: &str, obj_file: &str, target: Option<&str>) -> std::io::Result<()> {
    let output_file = if cfg!(target_os = "windows") || target == Some("windows-x64") {
        format!("{}.exe", output)
    } else {
        output.to_string()
    };

    println!("{} Linking executable...", "Compiler:".blue());
    let mut clang = Command::new("clang");
    clang.arg(&obj_file).arg("-o").arg(&output_file);

    // Add target-specific flags
    if let Some(target_triple) = target {
        clang.arg(format!("--target={}", target_triple));
        
        // Add Linux-specific flags and libraries
        if target_triple.contains("linux") {
            println!("{} Using Linux-specific compilation settings", "Compiler:".blue());
            // Add standard Linux libraries
            clang.arg("-lm").arg("-ldl").arg("-lpthread");
            
            // Check if we're targeting a specific architecture
            if target_triple.contains("x86_64") {
                println!("{} Targeting x86_64 Linux", "Compiler:".blue());
            } else if target_triple.contains("aarch64") {
                println!("{} Targeting ARM64 Linux", "Compiler:".blue());
            } else if target_triple.contains("arm") {
                println!("{} Targeting ARM Linux", "Compiler:".blue());
            }
        }
    } else if cfg!(target_os = "linux") {
        // If no target is specified but we're on Linux, add Linux libraries
        println!("{} Using Linux-specific compilation settings", "Compiler:".blue());
        clang.arg("-lm").arg("-ldl").arg("-lpthread");
    }
    
    // Execute the clang command
    let status = clang.status()?;
    
    if status.success() {
        println!("{} Successfully compiled to {}", "Compiler:".green(), output_file);
        Ok(())
    } else {
        println!("{} Failed to link executable", "Error:".red());
        Err(std::io::Error::new(std::io::ErrorKind::Other, "Linking failed"))
    }
}
