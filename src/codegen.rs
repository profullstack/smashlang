use crate::parser::AstNode;

// Simple structure to represent a target machine for C code generation
pub struct TargetMachine {
    pub target_triple: String, // Made public to avoid unused field warning
}

impl TargetMachine {
    pub fn new(target_triple: &str) -> Self {
        TargetMachine {
            target_triple: target_triple.to_string(),
        }
    }
    
    // Write C code to a file
    pub fn write_to_file(&self, module: &Module, _file_type: FileType, output_path: &str) -> Result<(), String> {
        // We could use self.target_triple here for target-specific code generation
        // For now, we're using a simplified approach
        use std::fs;
        use std::io::Write;
        
        let c_code = module.to_c_code();
        
        match fs::File::create(output_path) {
            Ok(mut file) => {
                match file.write_all(c_code.as_bytes()) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(format!("Failed to write to file: {}", e)),
                }
            },
            Err(e) => Err(format!("Failed to create file: {}", e)),
        }
    }
}

// Simple enum to represent file types
pub enum FileType {
    Object,
    Assembly,
}

// Simple module structure to hold generated code
pub struct Module<'a> {
    ast: &'a [AstNode],
}

impl<'a> Module<'a> {
    pub fn new(ast: &'a [AstNode]) -> Self {
        Module { ast }
    }
    
    // Convert AST to C code
    pub fn to_c_code(&self) -> String {
        let mut code = String::new();
        let mut main_code = String::new();
        
        // Add standard includes
        code.push_str("#include <stdio.h>\n");
        code.push_str("#include <stdlib.h>\n");
        code.push_str("#include <string.h>\n");
        code.push_str("#include <stdbool.h>\n\n");
        
        // Add helper function for string concatenation
        code.push_str("// Helper function for string concatenation\n");
        code.push_str("char* smash_string_concat(const char* a, const char* b) {\n");
        code.push_str("    size_t len_a = strlen(a);\n");
        code.push_str("    size_t len_b = strlen(b);\n");
        code.push_str("    char* result = (char*)malloc(len_a + len_b + 1);\n");
        code.push_str("    if (result) {\n");
        code.push_str("        strcpy(result, a);\n");
        code.push_str("        strcat(result, b);\n");
        code.push_str("    }\n");
        code.push_str("    return result;\n");
        code.push_str("}\n\n");
        
        // Process each AST node
        for node in self.ast {
            match node {
                AstNode::Function { .. } => {
                    // Function definitions go outside main
                    code.push_str(&self.generate_c_code_for_node(node, 0));
                },
                AstNode::FunctionCall { name, args } => {
                    if name == "main" && args.is_empty() {
                        // Skip the main function call as we'll add it automatically
                    } else {
                        // Other function calls go inside main
                        main_code.push_str(&self.generate_c_code_for_node(node, 1));
                    }
                },
                _ => {
                    // Everything else goes inside main
                    main_code.push_str(&self.generate_c_code_for_node(node, 1));
                }
            }
        }
        
        // Add C main function with the collected code
        code.push_str("int main(int argc, char** argv) {\n");
        code.push_str(&main_code);
        code.push_str("    return 0;\n");
        code.push_str("};\n");
        
        code
    }
    
    // Helper method to generate C code for a specific AST node
    fn generate_c_code_for_node(&self, node: &AstNode, indent_level: usize) -> String {
        let indent = "    ".repeat(indent_level);
        let mut code = String::new();
        
        match node {
            AstNode::Block(statements) => {
                code.push_str("\n");
                for stmt in statements {
                    code.push_str(&self.generate_c_code_for_node(stmt, indent_level));
                }
            },
            AstNode::LetDecl { name, value } => {
                code.push_str(&format!("{indent}char* {name} = "));
                code.push_str(&self.generate_c_code_for_expr(value, indent_level));
                code.push_str(";\n");
            },
            AstNode::ConstDecl { name, value } => {
                code.push_str(&format!("{indent}const char* {name} = "));
                code.push_str(&self.generate_c_code_for_expr(value, indent_level));
                code.push_str(";\n");
            },
            AstNode::FunctionCall { name, args } => {
                if name == "print" {
                    // Special handling for print function
                    if args.len() > 0 {
                        match &args[0] {
                            AstNode::String(s) => {
                                // For string literals, print them directly
                                let escaped = s.replace("\"", "\\\"");
                                code.push_str(&format!("{indent}printf(\"%s\\n\", \"{}\");\n", escaped));
                            },
                            _ => {
                                // For other expressions, evaluate them
                                code.push_str(&format!("{indent}printf(\"%s\\n\", "));
                                code.push_str(&self.generate_c_code_for_expr(&args[0], indent_level));
                                code.push_str(");\n");
                            }
                        }
                    }
                } else {
                    // Regular function call
                    code.push_str(&format!("{indent}{name}("));
                    for (i, arg) in args.iter().enumerate() {
                        if i > 0 {
                            code.push_str(", ");
                        }
                        code.push_str(&self.generate_c_code_for_expr(arg, indent_level));
                    }
                    code.push_str(");\n");
                }
            },
            AstNode::If { condition, then_branch, else_branch } => {
                code.push_str(&format!("{indent}if ("));
                code.push_str(&self.generate_c_code_for_expr(condition, indent_level));
                code.push_str(") {\n");
                
                // then_branch is a Box<AstNode>, not an Option
                code.push_str(&self.generate_c_code_for_node(then_branch, indent_level + 1));
                
                code.push_str(&format!("{}}}\n", indent));
                
                if let Some(else_node) = else_branch {
                    code.push_str(&format!("{}else {{\n", indent));
                    code.push_str(&self.generate_c_code_for_node(else_node, indent_level + 1));
                    code.push_str(&format!("{}}}\n", indent));
                }
            },
            AstNode::Return(expr) => {
                code.push_str(&format!("{indent}return "));
                code.push_str(&self.generate_c_code_for_expr(expr, indent_level));
                code.push_str(";\n");
            },
            AstNode::Function { name, params, body } => {
                // Generate function declaration
                code.push_str(&format!("{indent}char* {name}("));
                
                // Add parameters
                for (i, param) in params.iter().enumerate() {
                    if i > 0 {
                        code.push_str(", ");
                    }
                    code.push_str(&format!("char* {param}"));
                }
                code.push_str(") {\n");
                
                // Add function body
                for stmt in body {
                    code.push_str(&self.generate_c_code_for_node(stmt, indent_level + 1));
                }
                
                // Add default return if none is present
                code.push_str(&format!("{indent}    return \"\";\n"));
                
                // Close function
                code.push_str(&format!("{}}}\n\n", indent));
            },
            _ => {
                code.push_str(&format!("{indent}// Unimplemented AST node type\n"));
            }
        }
        
        code
    }
    
    // Helper method to generate C code for expressions
    fn generate_c_code_for_expr(&self, expr: &AstNode, indent_level: usize) -> String {
        match expr {
            AstNode::Number(n) => {
                // Convert number to string
                format!("(char*)\"{}\"" , n)
            },
            AstNode::String(s) => {
                // Escape quotes in the string
                let escaped = s.replace("\"", "\\\"");
                format!("\"{}\"" , escaped)
            },
            AstNode::Boolean(b) => {
                if *b {
                    "\"true\"".to_string()
                } else {
                    "\"false\"".to_string()
                }
            },
            AstNode::Identifier(name) => {
                name.clone()
            },
            AstNode::BinaryOp { left, op, right } => {
                let left_code = self.generate_c_code_for_expr(left, indent_level);
                let right_code = self.generate_c_code_for_expr(right, indent_level);
                
                match op.as_str() {
                    "+" => format!("smash_string_concat({}, {})", left_code, right_code),
                    "<" => format!("(strcmp({}, {}) < 0 ? \"true\" : \"false\")", left_code, right_code),
                    ">" => format!("(strcmp({}, {}) > 0 ? \"true\" : \"false\")", left_code, right_code),
                    "<=" => format!("(strcmp({}, {}) <= 0 ? \"true\" : \"false\")", left_code, right_code),
                    ">=" => format!("(strcmp({}, {}) >= 0 ? \"true\" : \"false\")", left_code, right_code),
                    "==" => format!("(strcmp({}, {}) == 0 ? \"true\" : \"false\")", left_code, right_code),
                    "!=" => format!("(strcmp({}, {}) != 0 ? \"true\" : \"false\")", left_code, right_code),
                    "&&" => format!("(strcmp({}, \"true\") == 0 && strcmp({}, \"true\") == 0 ? \"true\" : \"false\")", left_code, right_code),
                    "||" => format!("(strcmp({}, \"true\") == 0 || strcmp({}, \"true\") == 0 ? \"true\" : \"false\")", left_code, right_code),
                    _ => format!("/* Unsupported operator: {} */ \"{} {} {}\"" , op, left_code, op, right_code)
                }
            },
            AstNode::UnaryOp { op, expr } => {
                let expr_code = self.generate_c_code_for_expr(expr, indent_level);
                
                match op.as_str() {
                    "!" => format!("(strcmp({}, \"true\") == 0 ? \"false\" : \"true\")", expr_code),
                    _ => format!("/* Unsupported unary operator: {} */ \"{}{}\"", op, op, expr_code)
                }
            },
            AstNode::TernaryOp { condition, true_expr, false_expr } => {
                let cond_code = self.generate_c_code_for_expr(condition, indent_level);
                let then_code = self.generate_c_code_for_expr(true_expr, indent_level);
                let else_code = self.generate_c_code_for_expr(false_expr, indent_level);
                
                format!("(strcmp({}, \"true\") == 0 ? {} : {})", cond_code, then_code, else_code)
            },
            AstNode::FunctionCall { name, args } => {
                let mut call = format!("{name}(");
                
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        call.push_str(", ");
                    }
                    call.push_str(&self.generate_c_code_for_expr(arg, indent_level));
                }
                
                call.push_str(")");
                call
            },
            _ => {
                format!("\"Unsupported expression type\"" )
            }
        }
    }
}

// Generate LLVM IR (simplified to C code for now)
pub fn generate_llvm_ir<'a>(
    ast: &'a [AstNode],
    target: Option<&str>,
) -> (Module<'a>, TargetMachine) {
    // Create a module with the AST
    let module = Module::new(ast);
    
    // Create a target machine with the specified target triple
    let target_triple = match target {
        Some(t) => t,
        None => "x86_64-unknown-linux-gnu", // Default target
    };
    
    let target_machine = TargetMachine::new(target_triple);
    
    (module, target_machine)
}
