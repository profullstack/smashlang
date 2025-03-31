use colored::*;
use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 || args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) {
        println!("{}", "Examples:".yellow());
        println!("  smashc hello.smash                   Compile hello.smash to default output");
        println!("  smashc hello.smash -o hello          Specify output filename");
        println!("  smashc hello.smash --target linux    Compile for Linux x86_64");
        println!("  smashc hello.smash --target linux-arm64  Compile for Linux ARM64 (e.g., Raspberry Pi 4)");
        println!("  smashc hello.smash --target windows  Cross-compile for Windows");
        println!("  smashc hello.smash --wasm            Compile to WebAssembly");
        return;
    }
    
    // TODO: Implement the actual compiler logic here
    println!("Compiler not yet implemented");
}
