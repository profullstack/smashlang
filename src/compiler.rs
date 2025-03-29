// Only import what we need based on feature flags
#[cfg(feature = "compiler")]
use std::fs;
#[cfg(feature = "compiler")]
use std::path::Path;
#[cfg(feature = "compiler")]
use std::process::Command;

#[cfg(feature = "compiler")]
use crate::lexer::tokenize;
#[cfg(feature = "compiler")]
use crate::parser::Parser;
#[cfg(feature = "compiler")]
use crate::codegen::generate_llvm_ir;

pub fn compile_file(path: &str, output: &str, target: Option<&str>, emit: &str) {
    #[cfg(not(feature = "compiler"))]
    {
        println!("Compilation requires the 'compiler' feature to be enabled.");
        println!("Please build with: cargo build --features=compiler");
        return;
    }
    
    #[cfg(feature = "compiler")]
    {
        let mut full_code = String::new();

        // Load std.smash first
        if Path::new("std.smash").exists() {
            let std_code = fs::read_to_string("std.smash").expect("Failed to read std.smash");
            full_code.push_str(&std_code);
            full_code.push('\n');
        }

        // Then user program
        let code = fs::read_to_string(path).expect("Failed to read input file");
        full_code.push_str(&code);

        // Tokenize and parse
        let tokens = tokenize(&full_code);
        let mut parser = Parser::new(tokens);
        let ast = match parser.parse() {
            Ok(ast) => ast,
            Err(e) => {
                println!("Parse error: {}", e);
                return;
            }
        };

        let (llvm_module, target_machine) = generate_llvm_ir(&ast, target);

        match emit {
            "ir" => {
                llvm_module.print_to_file(format!("{output}.ll")).unwrap();
            }
            "obj" => {
                target_machine
                    .write_to_file(&llvm_module, inkwell::targets::FileType::Object, Path::new(&format!("{output}.o")))
                    .unwrap();
            }
            "exe" => {
                let obj_file = format!("{output}.o");
                target_machine
                    .write_to_file(&llvm_module, inkwell::targets::FileType::Object, Path::new(&obj_file))
                    .unwrap();

                let output_file = if cfg!(target_os = "windows") || target == Some("windows-x64") {
                    format!("{output}.exe")
                } else {
                    output.to_string()
                };

                let mut clang = Command::new("clang");
                clang.arg(&obj_file).arg("-o").arg(&output_file);

                if let Some(target_triple) = target {
                    clang.arg(format!("--target={}", target_triple));
                }

                let status = clang.status().expect("Failed to invoke clang");
                if !status.success() {
                    panic!("Clang failed to link the executable");
                }
            }
            _ => panic!("Invalid emit type: {}", emit),
        }
    }
}
