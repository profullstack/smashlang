use std::process::Command;

fn main() {
    // Try to get the git hash
    let output = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output();
    
    if let Ok(output) = output {
        if output.status.success() {
            let git_hash = String::from_utf8_lossy(&output.stdout).trim().to_string();
            println!("cargo:rustc-env=GIT_HASH={}", git_hash);
        }
    }
    
    // Make the build rerun if the git HEAD changes
    println!("cargo:rerun-if-changed=.git/HEAD");
    
    // Compile the runtime.c file
    println!("cargo:rerun-if-changed=src/runtime.c");
    
    #[cfg(feature = "compiler")]
    {
        // Build the runtime library
        cc::Build::new()
            .file("src/runtime.c")
            .compile("smash_runtime");
        
        println!("cargo:rustc-link-lib=smash_runtime");
    }
}
