mod lexer;
mod parser;
mod codegen;
mod compiler;

use std::env;
use compiler::compile_file;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: smashc <file.smash> [--out <output>] [--target <target>] [--emit ir|obj|exe]");
        return;
    }

    let input = &args[1];
    let mut output = "out";
    let mut target = None;
    let mut emit = "exe";

    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                i += 1;
                output = &args[i];
            }
            "--target" => {
                i += 1;
                target = Some(args[i].as_str());
            }
            "--emit" => {
                i += 1;
                emit = &args[i];
            }
            _ => {}
        }
        i += 1;
    }

    compile_file(input, output, target, emit);
}
