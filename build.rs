use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/grammar.pest");
    
    // Store the git hash for version information
    let git_hash = get_git_hash();
    if let Some(hash) = git_hash {
        println!("cargo:rustc-env=GIT_HASH={}", hash);
    }
    
    // Set up platform-specific configurations
    let target = env::var("TARGET").unwrap();
    
    if target.contains("wasm32") {
        println!("cargo:rustc-cfg=feature=\"wasm\"");
    } else {
        println!("cargo:rustc-cfg=feature=\"native\"");
    }
    
    // Set up OS-specific configurations
    if target.contains("windows") {
        println!("cargo:rustc-cfg=os=\"windows\"");
    } else if target.contains("apple") {
        println!("cargo:rustc-cfg=os=\"macos\"");
        
        if target.contains("ios") {
            println!("cargo:rustc-cfg=os=\"ios\"");
        }
    } else if target.contains("android") {
        println!("cargo:rustc-cfg=os=\"android\"");
    } else if target.contains("linux") {
        println!("cargo:rustc-cfg=os=\"linux\"");
    }
}

fn get_git_hash() -> Option<String> {
    let output = Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
        .ok()?;
    
    if output.status.success() {
        let hash = String::from_utf8(output.stdout).ok()?;
        Some(hash.trim().to_string())
    } else {
        // If git command fails, try to read from a file
        if Path::new("git_hash.txt").exists() {
            fs::read_to_string("git_hash.txt").ok()
        } else {
            None
        }
    }
}
