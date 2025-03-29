println!("{}", "Examples:".yellow());
    println!("  smashc hello.smash                   Compile hello.smash to default output");
    println!("  smashc hello.smash -o hello          Specify output filename");
    println!("  smashc hello.smash --target linux    Compile for Linux x86_64");
    println!("  smashc hello.smash --target linux-arm64  Compile for Linux ARM64 (e.g., Raspberry Pi 4)");
    println!("  smashc hello.smash --target windows  Cross-compile for Windows");
    println!("  smashc hello.smash --wasm            Compile to WebAssembly");
