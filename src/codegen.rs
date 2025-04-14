use crate::parser::AstNode;
use std::process::{Command, Stdio};
use std::io::Write;
use rand; // Added for unique variable names

// Helper function to parse a regex pattern string into pattern and flags
fn parse_regex_pattern(regex: &str) -> (&str, &str) {
    // Regex pattern format is typically /pattern/flags
    // We need to extract the pattern and flags
    
    // First, check if the pattern has flags
    if let Some(last_slash_pos) = regex.rfind('/') {
        if last_slash_pos > 1 && last_slash_pos < regex.len() - 1 {
            // There are flags after the last slash
            let pattern = &regex[1..last_slash_pos];
            let flags = &regex[last_slash_pos+1..];
            return (pattern, flags);
        }
    }
    
    // If we get here, either there are no flags or the format is unusual
    // Try to extract just the pattern between slashes
    if regex.starts_with('/') && regex.ends_with('/') && regex.len() >= 2 {
        let pattern = &regex[1..regex.len()-1];
        return (pattern, "");
    }
    
    // If all else fails, return the whole string as the pattern with no flags
    (regex, "")
}

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
    
    // Compile C code to an object file
    pub fn compile_c_code(&self, c_code: &str, output_file: &str) -> Result<(), String> {
        let mut compiler = Command::new("gcc");
        compiler.arg("-I./src") // Ensure the src directory is included
                .arg("-o")
                .arg(output_file)
                .arg("-")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped());

        let mut child = compiler.spawn().map_err(|e| e.to_string())?;
        let stdin = child.stdin.as_mut().ok_or("Failed to open stdin")?;
        stdin.write_all(c_code.as_bytes()).map_err(|e| e.to_string())?;

        let output = child.wait_with_output().map_err(|e| e.to_string())?;
        if output.status.success() {
            Ok(())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).into_owned())
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
        // Add all necessary includes
        code.push_str("#include <stdio.h>\n");
        code.push_str("#include <stdlib.h>\n");
        code.push_str("#include <stdbool.h>\n");
        code.push_str("#include <string.h>\n");
        code.push_str("#include <ctype.h>\n");
        code.push_str("#include <dlfcn.h>\n");
        code.push_str("#include \"simple_regex.h\"\n");  // Include our simple regex implementation
        code.push_str("#include \"runtime.h\"\n");  // Include runtime.h which has all regex declarations

        // Forward declare regex functions
        code.push_str("// Forward declarations\n");
        code.push_str("void smash_regex_free(SmashRegex* regex);\n");
        code.push_str("SmashRegex* smash_regex_create(const char* pattern, const char* flags);\n");
        code.push_str("char* smash_regex_match(SmashRegex* regex, const char* str);\n");
        code.push_str("char* smash_regex_replace(SmashRegex* regex, const char* str, const char* replacement);\n\n");

        // We'll collect code for inside the main function here
        let mut node_code = String::new();

        // Helper functions are defined in runtime.c
        code.push_str("// Helper functions are defined in runtime.c\n\n");

        // Process each AST node
        let mut has_main_function = false;
        // We'll track if we have a main function
        
        for node in self.ast {
            match node {
                AstNode::Function { name, params, body } => {
                    if name == "main" {
                        // Remember that we found a main function
                        has_main_function = true;
                        
                        // Instead of generating a char* main(), we'll call it smash_main()
                        // and call it from our C main function
                        let mut modified_function = format!("char* smash_main(\n");
                        
                        // Add parameters
                        for (i, param) in params.iter().enumerate() {
                            if i > 0 {
                                modified_function.push_str(", ");
                            }
                            modified_function.push_str(&format!("char* {param}"));
                        }
                        modified_function.push_str(") {\n");
                        
                        // Add function body
                        for stmt in body {
                            modified_function.push_str(&self.generate_c_code_for_node(stmt, 1));
                        }
                        
                        // Add default return if none is present
                        modified_function.push_str("    return \"\";\n");
                        
                        // Close function
                        modified_function.push_str("}\n\n");
                        
                        code.push_str(&modified_function);
                    } else {
                        // Other function definitions go outside main
                        code.push_str(&self.generate_c_code_for_node(node, 0));
                    }
                },
                AstNode::FunctionCall { name, args } => {
                    if name == "main" && args.is_empty() {
                        // Skip the main function call as we'll add it automatically
                    } else {
                        // Other function calls go inside main
                        node_code.push_str(&self.generate_c_code_for_node(node, 1));
                    }
                },
                _ => {
                    // Everything else goes inside main
                    node_code.push_str(&self.generate_c_code_for_node(node, 1));
                }
            }
        }
        
        // If we have a main function, add a call to smash_main in our C main
        if has_main_function {
            node_code.push_str("    char* result = smash_main();\n");
            node_code.push_str("    if (result && strlen(result) > 0) {\n");
            node_code.push_str("        printf(\"%s\\n\", result);\n");
            node_code.push_str("    }\n");
        }
        
        // Add C main function with the collected code - ONLY PLACE where main is defined
        code.push_str("int main(int argc, char** argv) {\n");
        code.push_str("    // Initialize the regex library\n");
        code.push_str("    if (!load_regex_library()) {\n");
        code.push_str("        fprintf(stderr, \"Failed to load regex library. Regex operations will not work.\\n\");\n");
        code.push_str("    }\n\n");
        code.push_str(&node_code);
        code.push_str("    return 0;\n");
        code.push_str("}\n");
        
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
                code.push_str(&format!("{indent}SmashValue* {name} = "));
                code.push_str(&self.generate_c_code_for_expr(value, indent_level));
                code.push_str(";\n");
            },
            AstNode::ConstDecl { name, value } => {
                code.push_str(&format!("{indent}SmashValue* {name} = "));
                code.push_str(&self.generate_c_code_for_expr(value, indent_level));
                code.push_str(";\n");
            },
            AstNode::FunctionCall { name, args } => {
                if name == "print" {
                    // Special handling for print function
                    if args.len() > 0 {
                        // Print the first argument
                        match &args[0] {
                            AstNode::String(s) => {
                                // For string literals, print them directly
                                let escaped = s.replace("\"", "\\\"");
                                code.push_str(&format!("{indent}printf(\"%s\", \"{}\");
", escaped));
                            },
                            _ => {
                                // For other expressions, evaluate them
                                code.push_str(&format!("{indent}printf(\"%s\", "));
                                code.push_str(&self.generate_c_code_for_expr(&args[0], indent_level));
                                code.push_str(");
");
                            }
                        }
                        
                        // Handle additional arguments if present
                        for arg in args.iter().skip(1) {
                            code.push_str(&format!("{indent}printf(\" \");
"));  // Print a space separator
                            match arg {
                                AstNode::String(s) => {
                                    // For string literals, print them directly
                                    let escaped = s.replace("\"", "\\\"");
                                    code.push_str(&format!("{indent}printf(\"%s\", \"{}\");
", escaped));
                                },
                                _ => {
                                    // For other expressions, evaluate them
                                    code.push_str(&format!("{indent}printf(\"%s\", "));
                                    code.push_str(&self.generate_c_code_for_expr(arg, indent_level));
                                    code.push_str(");
");
                                }
                            }
                        }
                        
                        // Print newline at the end
                        code.push_str(&format!("{indent}printf(\"\\n\");
"));
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
            AstNode::ForOf { var_name, iterable, body } => {
                let iterable_code = self.generate_c_code_for_expr(iterable, indent_level); // Get C code for the iterable
                let loop_id = rand::random::<u32>(); // Unique ID for loop variables
                let index_var = format!("i_{}", loop_id);
                let length_var = format!("len_{}", loop_id);
                let iterable_var = format!("iter_{}", loop_id);

                code.push_str(&format!("{indent}// Start ForOf loop for variable '{var_name}'\n"));
                // Assume iterable_code evaluates to a SmashValue* for the array
                code.push_str(&format!("{indent}SmashValue* {iterable_var} = {};\n", iterable_code));
                code.push_str(&format!("{indent}int {length_var} = smash_array_length({iterable_var});\n"));
                code.push_str(&format!("{indent}for (int {index_var} = 0; {index_var} < {length_var}; {index_var}++) {{\n"));
                // Declare the loop variable within the loop scope - assuming smash_array_get returns SmashValue*
                code.push_str(&format!("{}    SmashValue* {} = smash_array_get({iterable_var}, {index_var}); // Assign current element\n", indent, var_name));

                // Generate code for the loop body
                code.push_str(&self.generate_c_code_for_node(body, indent_level + 1));

                code.push_str(&format!("{indent}}}\n")); // End for loop
                code.push_str(&format!("{indent}// End ForOf loop for variable '{var_name}'\n"));
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
                format!("(char*)\"{}\"", n)
            },
            AstNode::String(s) => {
                // Escape quotes in the string
                let escaped = s.replace("\"", "\\\"");
                format!("\"{}\"", escaped)
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
                    _ => format!("/* Unsupported operator: {} */ \"{} {} {}\"", op, left_code, op, right_code)
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
            AstNode::TemplateLiteral(parts) => {
                // For empty template literals, return an empty string
                if parts.is_empty() {
                    return "\"\"".to_string();
                }
                
                // Start with an empty string if we need to concatenate multiple parts
                let mut result = String::new();
                let mut is_first = true;
                
                for part in parts {
                    let part_code = self.generate_c_code_for_expr(part, indent_level);
                    
                    if is_first {
                        // First part doesn't need concatenation
                        result = part_code;
                        is_first = false;
                    } else {
                        // Concatenate with previous parts
                        result = format!("smash_string_concat({}, {})", result, part_code);
                    }
                }
                
                result
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
            AstNode::PropertyAccess { object, property } => {
                let obj_code = self.generate_c_code_for_expr(object, indent_level);
                
                // Handle common properties based on the object type
                // For now, we'll only implement a few common properties like length
                if property == "length" {
                    // For strings, length is the string length
                    format!("smash_get_length({})", obj_code)
                } else {
                    // For other properties, we'll just return a placeholder
                    format!("\"Property {} not implemented\"", property)
                }
            },
            AstNode::MethodCall { object, method, args } => {
                let obj_code = self.generate_c_code_for_expr(object, indent_level);
                
                // Generate code for the arguments
                let mut arg_codes = Vec::new();
                for arg in args {
                    arg_codes.push(self.generate_c_code_for_expr(arg, indent_level));
                }
                
                // Handle common methods based on the object type and method name
                match method.as_str() {
                    // Regex methods
                    "test" => {
                        if arg_codes.len() < 1 {
                            format!("\"Error: test requires a string to test\"")
                        } else {
                            format!("(smash_regex_test({}, {}) ? \"true\" : \"false\")", obj_code, arg_codes[0])
                        }
                    },
                    "match" => {
                        if arg_codes.len() < 1 {
                            format!("\"Error: match requires a string to match\"")
                        } else {
                            format!("smash_string_match({}, {})", obj_code, arg_codes[0])
                        }
                    },
                    "replace" => {
                        if arg_codes.len() < 2 {
                            format!("\"Error: replace requires a pattern and replacement\"")
                        } else {
                            format!("smash_string_replace({}, {}, {})", obj_code, arg_codes[0], arg_codes[1])
                        }
                    },
                    "map" => {
                        // For array.map, we need to implement a map function in C
                        // This is a simplified implementation that assumes the first argument is a function
                        if arg_codes.len() < 1 {
                            format!("\"Error: map requires a callback function\"")
                        } else {
                            format!("smash_array_map({}, {})", obj_code, arg_codes[0])
                        }
                    },
                    "filter" => {
                        // For array.filter, we need to implement a filter function in C
                        // This is a simplified implementation that assumes the first argument is a function
                        if arg_codes.len() < 1 {
                            format!("\"Error: filter requires a callback function\"")
                        } else {
                            format!("smash_array_filter({}, {})", obj_code, arg_codes[0])
                        }
                    },
                    "push" => {
                        // For array.push, add an element to the array
                        if arg_codes.len() < 1 {
                            format!("\"Error: push requires at least one argument\"")
                        } else {
                            let args_str = arg_codes.join(", ");
                            format!("smash_array_push({}, {})", obj_code, args_str)
                        }
                    },
                    "pop" => {
                        // For array.pop, remove the last element from the array
                        format!("smash_array_pop({})", obj_code)
                    },
                    // String methods
                    "toUpperCase" => {
                        format!("smash_string_to_upper({})", obj_code)
                    },
                    "toLowerCase" => {
                        format!("smash_string_to_lower({})", obj_code)
                    },
                    "trim" => {
                        format!("smash_string_trim({})", obj_code)
                    },
                    "trimStart" => {
                        format!("smash_string_trim_start({})", obj_code)
                    },
                    "trimEnd" => {
                        format!("smash_string_trim_end({})", obj_code)
                    },
                    "charAt" => {
                        if arg_codes.len() < 1 {
                            format!("\"Error: charAt requires an index\"")
                        } else {
                            format!("smash_string_char_at({}, {})", obj_code, arg_codes[0])
                        }
                    },
                    "concat" => {
                        if arg_codes.len() < 1 {
                            format!("\"Error: concat requires a string\"")
                        } else {
                            format!("smash_string_concat({}, {})", obj_code, arg_codes[0])
                        }
                    },
                    "includes" => {
                        if arg_codes.len() < 1 {
                            format!("\"Error: includes requires a search string\"")
                        } else {
                            format!("smash_string_includes({}, {})", obj_code, arg_codes[0])
                        }
                    },
                    "indexOf" => {
                        if arg_codes.len() < 1 {
                            format!("\"Error: indexOf requires a search string\"")
                        } else {
                            format!("smash_string_index_of({}, {})", obj_code, arg_codes[0])
                        }
                    },
                    "split" => {
                        if arg_codes.len() < 1 {
                            format!("\"Error: split requires a delimiter\"")
                        } else {
                            format!("smash_string_split({}, {})", obj_code, arg_codes[0])
                        }
                    },
                    "repeat" => {
                        if arg_codes.len() < 1 {
                            format!("\"Error: repeat requires a count\"")
                        } else {
                            format!("smash_string_repeat({}, {})", obj_code, arg_codes[0])
                        }
                    },
                    // Number methods
                    "toFixed" => {
                        if arg_codes.len() < 1 {
                            format!("\"Error: toFixed requires decimal places\"")
                        } else {
                            format!("smash_number_to_fixed({}, {})", obj_code, arg_codes[0])
                        }
                    },
                    "toPrecision" => {
                        if arg_codes.len() < 1 {
                            format!("\"Error: toPrecision requires precision\"")
                        } else {
                            format!("smash_number_to_precision({}, {})", obj_code, arg_codes[0])
                        }
                    },
                    "toExponential" => {
                        if arg_codes.len() < 1 {
                            format!("\"Error: toExponential requires decimal places\"")
                        } else {
                            format!("smash_number_to_exponential({}, {})", obj_code, arg_codes[0])
                        }
                    },
                    // Array methods
                    "forEach" => {
                        if arg_codes.len() < 1 {
                            format!("\"Error: forEach requires a callback function\"")
                        } else {
                            format!("smash_array_for_each({}, {})", obj_code, arg_codes[0])
                        }
                    },
                    "find" => {
                        if arg_codes.len() < 1 {
                            format!("\"Error: find requires a callback function\"")
                        } else {
                            format!("smash_array_find({}, {})", obj_code, arg_codes[0])
                        }
                    },
                    "join" => {
                        let separator = if arg_codes.len() > 0 { arg_codes[0].clone() } else { "\",\"" .to_string() };
                        format!("smash_array_join({}, {})", obj_code, separator)
                    },
                    "reverse" => {
                        format!("smash_array_reverse({})", obj_code)
                    },
                    // Object methods
                    "hasOwnProperty" => {
                        if arg_codes.len() < 1 {
                            format!("\"Error: hasOwnProperty requires a property name\"")
                        } else {
                            format!("smash_object_has_own_property({}, {})", obj_code, arg_codes[0])
                        }
                    },
                    "keys" => {
                        format!("smash_object_keys({})", obj_code)
                    },
                    "values" => {
                        format!("smash_object_values({})", obj_code)
                    },
                    "entries" => {
                        format!("smash_object_entries({})", obj_code)
                    },
                    // Common methods that might be used by different types
                    "toString" => {
                        // Handle toString based on object type
                        format!("smash_to_string({})", obj_code)
                    },
                    "valueOf" => {
                        // Handle valueOf based on object type
                        format!("smash_value_of({})", obj_code)
                    },
                    "slice" => {
                        // Handle slice based on object type (string or array)
                        if arg_codes.len() < 2 {
                            format!("\"Error: slice requires start and end indices\"")
                        } else {
                            format!("smash_slice({}, {}, {})", obj_code, arg_codes[0], arg_codes[1])
                        }
                    },
                    _ => {
                        // For other methods, we'll just return a placeholder
                        format!("\"Method {} not implemented\"", method)
                    }
                }
            },
            AstNode::Regex(pattern) => {
                // Parse the regex pattern and flags
                let (pattern_str, flags) = parse_regex_pattern(pattern);
                
                // Create a regex object using the runtime implementation
                format!("smash_regex_create(\"{}\", \"{}\")", pattern_str, flags)
            },
            AstNode::ArrayLiteral(elements) => {
                let array_var = format!("smash_arr_{}", rand::random::<u32>());
                let mut setup_code = String::new();

                // Declare the SmashValue* for the array
                setup_code.push_str(&format!(
                    "SmashValue* {} = smash_value_create_array({});\n",
                    array_var,
                    elements.len() // Initial capacity hint
                ));

                // Generate code for each element and push it
                for element_node in elements {
                    let element_code = self.generate_c_code_for_expr(element_node, indent_level);
                    setup_code.push_str(&format!(
                        "smash_array_push({}, {});\n",
                        array_var, 
                        element_code
                    ));
                }
                
                format!("{}{}", setup_code, array_var)
            },
            AstNode::ObjectLiteral(_properties) => {
                // TODO: Implement Object Literal generation
                let object_var = format!("smash_obj_{}", rand::random::<u32>());
                let code = format!("SmashValue* {} = smash_value_create_null(); // Placeholder for object\n", object_var);
                code
            },
            _ => {
                // Handle other potential expression types or return an error/placeholder
                format!("/* Unhandled expression type */ \"Error\"")
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
