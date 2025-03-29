use colored::*;
use std::env;
use std::path::Path;
use std::fs;
// Removed unused import: use std::io;
use chrono::Local;

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
        Some("create") => create_package(&args[2..]),
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
    println!("  create <name> [options]      Create a new package");
    println!("  install <package>           Install a package");
    println!("  uninstall <package>         Remove a package");
    println!("  list                        List installed packages");
    println!("  search <query>              Search for packages");
    println!("  help, --help, -h            Show this help message");
    println!("  version, --version, -v      Show version information");
    println!("");
    println!("{}", "Examples:".yellow());
    println!("  smashpkg create my-package        Create a new package named 'my-package'");
    println!("  smashpkg create utils/array       Create a new package in the 'utils' category");
    println!("  smashpkg install networking/hono  Install the Hono package");
    println!("  smashpkg list                    List installed packages");
    println!("  smashpkg search http             Search for HTTP-related packages");
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

fn create_package(args: &[String]) {
    if args.is_empty() {
        println!("{} No package name provided", "Error:".red());
        println!("Usage: {} <name> [options]", "smashpkg create".cyan());
        return;
    }
    
    let package_name = &args[0];
    let _packages_dir = get_packages_dir();
    let local_packages_dir = format!("{}/smashlang_packages", env::current_dir().unwrap_or_default().to_string_lossy());
    let package_dir = Path::new(&local_packages_dir).join(package_name);
    
    // Check if package already exists
    if package_dir.exists() {
        println!("{} Package '{}' already exists", "Error:".red(), package_name);
        return;
    }
    
    // Create package directory structure
    println!("{} Creating new package: {}", "SmashLang:".blue(), package_name.bright_cyan());
    
    // Create main package directory
    match fs::create_dir_all(&package_dir) {
        Ok(_) => println!("  {} Created package directory", "✓".green()),
        Err(e) => {
            println!("{} Failed to create package directory: {}", "Error:".red(), e);
            return;
        }
    }
    
    // Create src directory
    let src_dir = package_dir.join("src");
    match fs::create_dir_all(&src_dir) {
        Ok(_) => println!("  {} Created src directory", "✓".green()),
        Err(e) => {
            println!("{} Failed to create src directory: {}", "Error:".red(), e);
            return;
        }
    }
    
    // Create examples directory
    let examples_dir = package_dir.join("examples");
    match fs::create_dir_all(&examples_dir) {
        Ok(_) => println!("  {} Created examples directory", "✓".green()),
        Err(e) => {
            println!("{} Failed to create examples directory: {}", "Error:".red(), e);
            return;
        }
    }
    
    // Create tests directory
    let tests_dir = package_dir.join("tests");
    match fs::create_dir_all(&tests_dir) {
        Ok(_) => println!("  {} Created tests directory", "✓".green()),
        Err(e) => {
            println!("{} Failed to create tests directory: {}", "Error:".red(), e);
            return;
        }
    }
    
    // Create package.json file
    let package_json_path = package_dir.join("package.json");
    let package_json_content = format!(r#"{{
  "name": "{}",
  "version": "0.1.0",
  "description": "A SmashLang package",
  "main": "src/index.smash",
  "author": "",
  "license": "MIT",
  "dependencies": {{}}
}}
"#, package_name);
    
    match fs::write(&package_json_path, package_json_content) {
        Ok(_) => println!("  {} Created package.json", "✓".green()),
        Err(e) => {
            println!("{} Failed to create package.json: {}", "Error:".red(), e);
            return;
        }
    }
    
    // Create README.md file
    let readme_path = package_dir.join("README.md");
    let today = Local::now().format("%Y-%m-%d").to_string();
    let readme_content = format!(r#"# {}

A SmashLang package.

## Installation

```bash
smashpkg install {}
```

## Usage

```javascript
import {{ example }} from '{}';

// Your code here
```

## API

### example()

Description of the example function.

## License

MIT

## Created

{}"#, package_name, package_name, package_name, today);
    
    match fs::write(&readme_path, readme_content) {
        Ok(_) => println!("  {} Created README.md", "✓".green()),
        Err(e) => {
            println!("{} Failed to create README.md: {}", "Error:".red(), e);
            return;
        }
    }
    
    // Create index.smash file
    let index_path = src_dir.join("index.smash");
    let index_content = r#"/**
 * Example function
 * @returns {string} A greeting message
 */
export function example() {
  return 'Hello from SmashLang package!';
}
"#;
    
    match fs::write(&index_path, index_content) {
        Ok(_) => println!("  {} Created src/index.smash", "✓".green()),
        Err(e) => {
            println!("{} Failed to create src/index.smash: {}", "Error:".red(), e);
            return;
        }
    }
    
    // Create example.smash file
    let example_path = examples_dir.join("example.smash");
    let example_content = format!(r#"import {{ example }} from '{}';

console.log(example());
"#, package_name);
    
    match fs::write(&example_path, example_content) {
        Ok(_) => println!("  {} Created examples/example.smash", "✓".green()),
        Err(e) => {
            println!("{} Failed to create examples/example.smash: {}", "Error:".red(), e);
            return;
        }
    }
    
    // Create test file
    let test_path = tests_dir.join("index.test.smash");
    let test_content = format!(r#"import {{ example }} from '{}';
import {{ test, describe, expect, beforeEach, afterEach }} from 'std/testing';

test('example function returns correct greeting', () => {{
  expect(example()).toBe('Hello from SmashLang package!');
}});

describe('Package: {}', () => {{
  beforeEach(() => {{
    // Setup code for each test
  }});

  afterEach(() => {{
    // Cleanup code for each test
  }});

  test('can be imported correctly', () => {{
    expect(typeof example).toBe('function');
  }});

  test('returns a string', () => {{
    const result = example();
    expect(typeof result).toBe('string');
  }});
}});
"#, package_name, package_name);
    
    match fs::write(&test_path, test_content) {
        Ok(_) => println!("  {} Created tests/index.test.smash", "✓".green()),
        Err(e) => {
            println!("{} Failed to create tests/index.test.smash: {}", "Error:".red(), e);
            return;
        }
    }
    
    // Create .gitignore file
    let gitignore_path = package_dir.join(".gitignore");
    let gitignore_content = r#"# SmashLang package
node_modules/
dist/
.smash_cache/
*.log
"#;
    
    match fs::write(&gitignore_path, gitignore_content) {
        Ok(_) => println!("  {} Created .gitignore", "✓".green()),
        Err(e) => {
            println!("{} Failed to create .gitignore: {}", "Error:".red(), e);
            return;
        }
    }
    
    println!("\n{} Package '{}' created successfully in ./smashlang_packages/{}", "Success:".green(), package_name.bright_cyan(), package_name);
    println!("\nTo use this package in your SmashLang project:\n\n  import {{ example }} from '{}';\n", package_name);
}
