use crate::parser::AstNode;

// Structure to manage code generation state
pub struct CodeGenerator {
    generated_code: String,
    temp_var_counter: u32,
    // Add other state needed, e.g., symbol table, current function context
}

impl CodeGenerator {
    pub fn new() -> Self {
        CodeGenerator {
            generated_code: String::new(),
            temp_var_counter: 0,
        }
    }

    // Main function to generate C code from AST
    pub fn generate(&mut self, ast: &[AstNode]) -> Result<String, String> {
        let mut main_code = String::new();
        self.temp_var_counter = 0; // Reset counter for each generation

        for node in ast {
            main_code.push_str(&self.generate_c_code_for_node(node, 1)?); // Indent level 1 for main body, propagate errors
        }

        // Construct the final C code with includes, main function, etc.
        let final_code = format!(r#"
#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>
#include "runtime.h" // Include our runtime support

int main(int argc, char** argv) {{
    // TODO: Initialization, e.g., GC_init() if using Boehm GC
{}
    // TODO: Cleanup if needed
    return 0;
}}
"#,
            main_code
        );

        self.generated_code = final_code;
        Ok(self.generated_code.clone())
    }

    // Helper method to generate C code for an expression
    // Returns a tuple: (code_string, result_variable_name)
    fn generate_c_code_for_expr(&mut self, expr: &AstNode, indent_level: usize) -> Result<(String, String), String> {
        let indent = "    ".repeat(indent_level);
        let mut code = String::new();

        match expr {
            AstNode::Number(n) => {
                let var_name = format!("smash_num_{}", self.temp_var_counter);
                self.temp_var_counter += 1;
                code.push_str(&format!("{}SmashValue* {} = smash_value_create_number({}); // Expr Number\n", indent, var_name, n));
                Ok((code, var_name))
            }
            AstNode::String(s) => {
                let var_name = format!("smash_str_{}", self.temp_var_counter);
                self.temp_var_counter += 1;
                let escaped_s = s.replace("\"", "\\\""); // Escape quotes for C string literal
                code.push_str(&format!("{}SmashValue* {} = smash_value_create_string(\"{}\"); // Expr String\n", indent, var_name, escaped_s));
                Ok((code, var_name))
            }
            AstNode::Boolean(b) => {
                let var_name = format!("smash_bool_{}", self.temp_var_counter);
                self.temp_var_counter += 1;
                let c_bool = if *b { "true" } else { "false" };
                code.push_str(&format!("{}SmashValue* {} = smash_value_create_boolean({}); // Expr Boolean\n", indent, var_name, c_bool));
                Ok((code, var_name))
            }
             AstNode::Null => {
                let var_name = format!("smash_null_{}", self.temp_var_counter);
                self.temp_var_counter += 1;
                code.push_str(&format!("{}SmashValue* {} = smash_value_create_null(); // Expr Null\n", indent, var_name));
                Ok((code, var_name))
            }
            AstNode::Identifier(name) => {
                // Assume identifier already exists as a SmashValue* in the C scope
                Ok((String::new(), name.clone())) // No code needed, just return the name
            }
            AstNode::ArrayLiteral(elements) => {
                let array_var = format!("smash_arr_{}", self.temp_var_counter);
                self.temp_var_counter += 1;
                code.push_str(&format!("{}SmashValue* {} = smash_value_create_array({});\n", indent, array_var, elements.len()));

                for element_expr in elements {
                    // Generate code for the element expression
                    let (elem_code, elem_var) = self.generate_c_code_for_expr(element_expr, indent_level)?;
                    code.push_str(&elem_code); // Add the code to create the element's SmashValue*
                    // Push the resulting element variable into the array
                    code.push_str(&format!("{}smash_array_push({}, {});\n", indent, array_var, elem_var));
                    // TODO: Consider freeing temporary element SmashValues if they are not identifiers
                }
                Ok((code, array_var))
            }
            AstNode::ObjectLiteral(properties) => {
                // Create a new object
                let object_var = format!("smash_obj_{}", self.temp_var_counter);
                self.temp_var_counter += 1;
                code.push_str(&format!("{}SmashValue* {} = smash_value_create_object(); // Create object\n", indent, object_var));
                
                // Set each property on the object
                for (key, value) in properties {
                    // Generate code for the property value
                    match self.generate_c_code_for_expr(value, indent_level) {
                        Ok((value_code, value_var)) => {
                            // Add the value code first
                            code.push_str(&value_code);
                            
                            // Set the property on the object
                            code.push_str(&format!("{}smash_object_set({}, \"{}\", {}); // Set property\n", 
                                              indent, object_var, key, value_var));
                        }
                        Err(e) => {
                            return Err(format!("Error generating code for object property '{}': {}", key, e));
                        }
                    }
                }
                
                Ok((code, object_var))
            }
            // Handle property access (e.g., obj.property)
            AstNode::PropertyAccess { object, property } => {
                // Generate code for the object expression
                let (obj_code, obj_var) = self.generate_c_code_for_expr(object, indent_level)?;
                
                // Create a new variable for the property value
                let prop_var = format!("prop_{}", self.temp_var_counter);
                self.temp_var_counter += 1;
                
                // Add the object code first
                code.push_str(&obj_code);
                
                // Generate code to access the property
                code.push_str(&format!("{}SmashValue* {} = smash_object_get({}, \"{}\"); // Property access\n", 
                                      indent, prop_var, obj_var, property));
                
                Ok((code, prop_var))
            },
            // Handle float literals
            AstNode::Float(f) => {
                let var_name = format!("smash_num_{}", self.temp_var_counter);
                self.temp_var_counter += 1;
                code.push_str(&format!("{}SmashValue* {} = smash_value_create_number({}); // Expr Float\n", indent, var_name, f));
                Ok((code, var_name))
            },
            // Add other expression types as needed (FunctionCall returning value, BinaryOp, etc.)
            _ => Err(format!("C code generation not implemented for expression node: {:?}", expr)),
        }
    }

    // Helper method to generate C code for a specific AST node statement
    fn generate_c_code_for_node(&mut self, node: &AstNode, indent_level: usize) -> Result<String, String> {
        let indent = "    ".repeat(indent_level);
        let mut code = String::new();

        match node {
            AstNode::LetDecl { name, value } => {
                // Generate code for the value expression
                match self.generate_c_code_for_expr(value, indent_level) {
                    Ok((expr_code, expr_var)) => {
                        code.push_str(&expr_code); // Code to create the value
                        // Declare the variable and assign the result from the expression
                        code.push_str(&format!("{}SmashValue* {} = {};\n", indent, name, expr_var));
                    }
                    Err(e) => {
                        code.push_str(&format!("{}// Error generating expression for let {}: {}\n", indent, name, e));
                        code.push_str(&format!("{}SmashValue* {} = smash_value_create_null(); // Error fallback\n", indent, name));
                    }
                }
            }
            AstNode::FunctionCall { name, args } => {
                if name == "print" {
                    // Special handling for print function with any number of arguments
                    let mut arg_codes = String::new();
                    let mut arg_vars = Vec::new();
                    
                    // Generate code for each argument
                    for arg in args {
                        match self.generate_c_code_for_expr(arg, indent_level) {
                            Ok((arg_code, arg_var)) => {
                                arg_codes.push_str(&arg_code);
                                arg_vars.push(arg_var);
                            }
                            Err(e) => {
                                code.push_str(&format!("{}// Error generating argument for print: {}\n", indent, e));
                                return Err(format!("Error generating argument for print: {}", e));
                            }
                        }
                    }
                    
                    // Add the argument code to the main code
                    code.push_str(&arg_codes);
                    
                    // Generate the print function call with the number of arguments and the argument variables
                    code.push_str(&format!("{}print({}, {});", indent, args.len(), arg_vars.join(", ")));
                    code.push_str(" // Print function call\n");
                    
                    return Ok(code);
                } else {
                    // Handle other function calls
                     code.push_str(&format!("{}// TODO: Call function '{}' (Not implemented correctly for SmashValue yet)\n", indent, name));
                    // Generate args, call C function... complex if functions return SmashValue* etc.
                }
            }
            AstNode::ForOf { var_name, iterable, body } => {
                 match self.generate_c_code_for_expr(iterable, indent_level) {
                    Ok((iterable_code, iterable_var)) => {
                        let index_var = format!("i_{}", self.temp_var_counter);
                        let length_var = format!("len_{}", self.temp_var_counter);
                        self.temp_var_counter += 1;

                        code.push_str(&format!("{}// Start ForOf loop for variable '{}'\n", indent, var_name));
                        code.push_str(&iterable_code); // Code to get the iterable SmashValue*

                        code.push_str(&format!("{indent}int {length_var} = smash_array_length({});\n", iterable_var));
                        code.push_str(&format!("{indent}for (int {index_var} = 0; {index_var} < {length_var}; {index_var}++) {{\n", index_var = index_var, length_var = length_var));

                        // Declare the loop variable INSIDE the loop scope
                        code.push_str(&format!("{indent}    SmashValue* {var_name} = smash_array_get({iterable_var}, {index_var}); // Assign current element\n", iterable_var = iterable_var, index_var = index_var, var_name = var_name));

                        // Generate body code within the loop
                        // Check if body is a Block or a single statement
                        match **body {
                            AstNode::Block(ref statements) => {
                                // Handle block of statements
                                for stmt in statements {
                                    let stmt_code = self.generate_c_code_for_node(stmt, indent_level + 1)?;
                                    code.push_str(&stmt_code);
                                }
                            },
                            _ => {
                                // Handle single statement
                                let stmt_code = self.generate_c_code_for_node(body, indent_level + 1)?;
                                code.push_str(&stmt_code);
                            }
                        }

                        code.push_str(&format!("{indent}}}\n")); // Close loop body
                        code.push_str(&format!("{}// End ForOf loop for variable '{}'\n", indent, var_name));
                        code.push_str(&format!("{indent}smash_value_free({iterable_var}); // Free the iterable after the loop\n"));
                        return Ok(code)
                    }
                    Err(e) => {
                        // Handle error from generate_c_code_for_expr
                        return Err(format!("Error generating code for ForOf iterable: {}", e))
                    }
                }
            }
             AstNode::Block(statements) => {
                 code.push_str(&format!("{}{{\n", indent)); // Opening brace for block
                 for stmt in statements {
                     code.push_str(&self.generate_c_code_for_node(stmt, indent_level + 1)?);
                 }
                 code.push_str(&format!("{}}}\n", indent)); // Closing brace for block
                 return Ok(code)
             }

            // We handle expressions when they appear as values in LetDecl or args in FunctionCall
            // If an expression appears standalone as a statement, we might evaluate it but discard the result
            // or just print a comment. For now, let's generate code to evaluate it.
             AstNode::Number(_) | AstNode::String(_) | AstNode::Boolean(_) |
             AstNode::Null | AstNode::Identifier(_) | AstNode::ArrayLiteral(_) |
             AstNode::ObjectLiteral(_) => {
                match self.generate_c_code_for_expr(node, indent_level) {
                    Ok((expr_code, _expr_var)) => {
                        code.push_str(&expr_code);
                        code.push_str(&format!("{}// Evaluated standalone expression, result unused/discarded\n", indent));
                        // TODO: Free _expr_var if temporary?
                        return Ok(code)
                    }
                    Err(e) => {
                        return Err(format!("Error evaluating standalone expression: {}", e))
                    }
                }
             }
            // ... other statement types like IfStatement, ReturnStatement, etc. ...
            _ => {
                code.push_str(&format!("{}// C code generation not implemented for statement node: {:?}\n", indent, node));
                return Ok(code)
            }
        }
        // This line should never be reached as all match arms return now
        return Ok(code)
    }
}

// -- Remove or comment out the old Module/TargetMachine structs and functions --
/*
pub enum FileType { Object, Assembly }
pub struct Module<'a> { ast: &'a [AstNode] }
impl<'a> Module<'a> {
    pub fn new(ast: &'a [AstNode]) -> Self { Module { ast } }
    pub fn to_c_code(&self) -> String { /* ... old implementation ... */ }
    fn generate_c_code_for_node(&self, node: &AstNode, indent_level: usize) -> String { /* ... old implementation ... */ }
}
pub struct TargetMachine { target_triple: String }
impl TargetMachine {
    pub fn new(target_triple: &str) -> Self { TargetMachine { target_triple: target_triple.to_string() } }
    pub fn compile_to_file(&self, module: &Module, output_path: &str, file_type: FileType, include_path: &str) -> Result<(), String> { /* ... old implementation ... */ }
}
pub fn generate_llvm_ir<'a>(ast: &'a [AstNode], target: Option<&str>) -> (Module<'a>, TargetMachine) { /* ... old implementation ... */ }
*/
