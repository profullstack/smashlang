use colored::*;
use std::env;
use std::path::Path;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args.get(1).map(|s| s.as_str());

    match command {
        Some("--help") | Some("-h") => show_help(),
        Some("--version") | Some("-v") => show_version(),
        Some("install") => install_package(args.get(2)),
        Some("uninstall") => uninstall_package(args.get(2)),
        Some("list") => list_packages(),
        Some("search") => search_packages(args.get(2)),
        Some(cmd) => {
            println!("{} Unknown command: {}", "Error:".red(), cmd);
            println!("Run {} for usage information", "smashpkg --help".cyan());
        },
        None => {
            println!("{} SmashLang Package Manager v0.1.0", "SmashLang:".blue());
            println!("Run {} for usage information", "smashpkg --help".cyan());
        }
    }
}

fn show_help() {
    println!("{} - The SmashLang Package Manager", "SmashLang Package Manager v0.1.0".blue());
    println!("");
    println!("{}", "Usage:".yellow());
    println!("  smashpkg <command> [options]");
    println!("");
    println!("{}", "Commands:".yellow());
    println!("  install <package>           Install a package");
    println!("  uninstall <package>         Remove a package");
    println!("  list                        List installed packages");
    println!("  search <query>              Search for packages");
    println!("  help, --help, -h            Show this help message");
    println!("  version, --version, -v      Show version information");
    println!("");
    println!("{}", "Examples:".yellow());
    println!("  smashpkg install networking/hono    Install the Hono package");
    println!("  smashpkg list                      List installed packages");
    println!("  smashpkg search http               Search for HTTP-related packages");
    println!("");
    println!("{}", "Documentation:".yellow());
    println!("  Visit {} for full documentation", "https://smashlang.com/docs/packages".cyan());
}

fn show_version() {
    println!("{}", "SmashLang Package Manager v0.1.0".blue());
}

fn get_packages_dir() -> String {
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    
    if cfg!(target_os = "windows") {
        format!("{}/AppData/Local/SmashLang/packages", home)
    } else if cfg!(target_os = "macos") {
        format!("{}/Library/Application Support/SmashLang/packages", home)
    } else {
        format!("{}/local/share/smashlang/packages", home)
    }
}

fn install_package(package_name: Option<&String>) {
    match package_name {
        Some(name) => {
            println!("{} Installing package: {}", "SmashLang:".blue(), name);
            println!("This functionality is not yet fully implemented.");
            
            // Create the packages directory if it doesn't exist
            let packages_dir = get_packages_dir();
            if !Path::new(&packages_dir).exists() {
                if let Err(e) = fs::create_dir_all(&packages_dir) {
                    println!("{} Failed to create packages directory: {}", "Error:".red(), e);
                    return;
                }
            }
            
            // Create a placeholder file for the package
            let package_path = format!("{}/{}", packages_dir, name.replace("/", "_"));
            if let Err(e) = fs::write(&package_path, format!("Package: {}\nVersion: 0.1.0\nInstalled: {}", name, chrono::Local::now())) {
                println!("{} Failed to install package: {}", "Error:".red(), e);
            } else {
                println!("{} Package {} installed successfully", "Success:".green(), name);
            }
        },
        None => {
            println!("{} No package specified", "Error:".red());
            println!("Usage: smashpkg install <package>");
        }
    }
}

fn uninstall_package(package_name: Option<&String>) {
    match package_name {
        Some(name) => {
            println!("{} Uninstalling package: {}", "SmashLang:".blue(), name);
            println!("This functionality is not yet fully implemented.");
            
            // Check if the package exists
            let packages_dir = get_packages_dir();
            let package_path = format!("{}/{}", packages_dir, name.replace("/", "_"));
            
            if Path::new(&package_path).exists() {
                if let Err(e) = fs::remove_file(&package_path) {
                    println!("{} Failed to uninstall package: {}", "Error:".red(), e);
                } else {
                    println!("{} Package {} uninstalled successfully", "Success:".green(), name);
                }
            } else {
                println!("{} Package {} is not installed", "Error:".red(), name);
            }
        },
        None => {
            println!("{} No package specified", "Error:".red());
            println!("Usage: smashpkg uninstall <package>");
        }
    }
}

fn list_packages() {
    println!("{} Installed packages:", "SmashLang:".blue());
    
    let packages_dir = get_packages_dir();
    if !Path::new(&packages_dir).exists() {
        println!("No packages installed");
        return;
    }
    
    match fs::read_dir(&packages_dir) {
        Ok(entries) => {
            let mut found = false;
            for entry in entries {
                if let Ok(entry) = entry {
                    found = true;
                    let name = entry.file_name().to_string_lossy().to_string();
                    println!("  - {}", name.replace("_", "/"));
                }
            }
            
            if !found {
                println!("No packages installed");
            }
        },
        Err(e) => {
            println!("{} Failed to read packages directory: {}", "Error:".red(), e);
        }
    }
}

fn search_packages(query: Option<&String>) {
    match query {
        Some(q) => {
            println!("{} Searching for packages matching: {}", "SmashLang:".blue(), q);
            println!("This functionality is not yet fully implemented.");
            
            // Mock search results
            println!("{} Search results:", "SmashLang:".blue());
            println!("  - networking/http: Basic HTTP client and server");
            println!("  - networking/hono: Lightweight web framework");
            println!("  - networking/websocket: WebSocket implementation");
        },
        None => {
            println!("{} No search query specified", "Error:".red());
            println!("Usage: smashpkg search <query>");
        }
    }
}
