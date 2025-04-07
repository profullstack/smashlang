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
        let mut build = cc::Build::new();
        build.file("src/runtime.c");
        
        // macOS-specific configuration
        #[cfg(target_os = "macos")]
        {
            // Link against system frameworks
            println!("cargo:rustc-link-lib=framework=CoreFoundation");
            println!("cargo:rustc-link-lib=framework=Security");
            
            // Try multiple methods to find PCRE
            let pcre_found = if let Ok(lib) = pkg_config::probe_library("libpcre") {
                // Use pkg-config paths
                for path in lib.include_paths {
                    build.include(path);
                }
                for path in lib.link_paths {
                    println!("cargo:rustc-link-search={}", path.display());
                }
                true
            } else {
                // Try standard system paths first
                if std::path::Path::new("/usr/local/include/pcre.h").exists() {
                    build.include("/usr/local/include");
                    println!("cargo:rustc-link-search=/usr/local/lib");
                    true
                } else if let Ok(output) = std::process::Command::new("brew").args(["--prefix", "pcre"]).output() {
                    // Try Homebrew path as last resort
                    if output.status.success() {
                        let prefix = String::from_utf8_lossy(&output.stdout).trim().to_string();
                        build.include(format!("{}/include", prefix));
                        println!("cargo:rustc-link-search={}/lib", prefix);
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            };
            
            if !pcre_found {
                panic!("Could not find PCRE library. Please ensure it is installed.");
            }
            
            println!("cargo:rustc-link-lib=pcre");
            
            // Add macOS-specific compiler flags
            build.flag("-framework")
                .flag("CoreFoundation")
                .flag("-framework")
                .flag("Security");
        }
        
        build.compile("smash_runtime");
        println!("cargo:rustc-link-lib=smash_runtime");
    }
}
