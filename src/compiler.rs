use std::fs;
use std::path::Path;
use std::process::Command;
use crate::lexer::tokenize;
use crate::parser::Parser;
use crate::codegen::generate_llvm_ir;

pub fn compile_file(path: &str, output: &str, target: Option<&str>, emit: &str) {
    let mut full_code = String::new();

    // Load std.smash first
    if Path::new("std.smash").exists() {
        let std_code = fs::read_to_string("std.smash").expect("Failed to read std.smash");
        full_code.push_str(&std_code);
        full_code.push('
');
    }

    // Then user program
    let code = fs::read_to_string(path).expect("Failed to read input file");
    full_code.push_str(&code);

    let tokens = tokenize(&full_code);
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

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
