use crate::parser::AstNode;

// Structure to manage code generation state
pub struct CodeGenerator {
    generated_code: String,
    temp_var_counter: u32,
    function_prototypes: Vec<String>,      // Store function prototypes for declaration
    function_implementations: Vec<String>, // Store function implementations for definition
    // Add other state needed, e.g., symbol table, current function context
}

impl CodeGenerator {
    pub fn new() -> Self {
        CodeGenerator {
            generated_code: String::new(),
            temp_var_counter: 0,
            function_prototypes: Vec::new(),
            function_implementations: Vec::new(),
        }
    }
    
    // Helper method to generate unique temporary variable IDs
    fn next_temp_id(&mut self) -> u32 {
        let id = self.temp_var_counter;
        self.temp_var_counter += 1;
        id
    }

    // Main function to generate C code from AST
    pub fn generate(&mut self, ast: &[AstNode]) -> Result<String, String> {
        // Reset state for each generation
        self.temp_var_counter = 0;
        self.function_prototypes.clear();
        self.function_implementations.clear();
        
        let mut main_code = String::new();

        for node in ast {
            match self.generate_c_code_for_node(node, 1) {
                Ok((code, _)) => main_code.push_str(&code),
                Err(e) => return Err(e),
            }
        }

        // Combine function prototypes
        let function_prototypes = if !self.function_prototypes.is_empty() {
            format!("// Function prototypes
{}
", self.function_prototypes.join("
"))
        } else {
            String::new()
        };
        
        // Combine function implementations
        let function_implementations = if !self.function_implementations.is_empty() {
            format!("// Function implementations
{}
", self.function_implementations.join("
"))
        } else {
            String::new()
        };

        // Construct the final C code with includes, function declarations, implementations, and main function
        let final_code = format!(r#"
#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>
#include <pthread.h>
#include <time.h>
#include "runtime.h" // Include our runtime support

{}

{}

int main(int argc, char** argv) {{
    // Initialization
{}
    // Cleanup if needed
    return 0;
}}
"#,
            function_prototypes,
            function_implementations,
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
            // Handle computed property access (e.g., obj[expr])
            AstNode::MethodCall { object, method, args } => {
                // Generate code for the object
                let (obj_code, obj_var) = self.generate_c_code_for_expr(object, indent_level)?;
                
                // Add the object code first
                code.push_str(&obj_code);
                
                // Generate code for the method access
                let method_var = format!("method_{}", self.temp_var_counter);
                self.temp_var_counter += 1;
                
                code.push_str(&format!("{}SmashValue* {} = smash_object_get({}, \"{}\"); // Method lookup\n", 
                                      indent, method_var, obj_var, method));
                
                // Check if the property is a function
                code.push_str(&format!("{}if ({} == NULL || {}->type != SMASH_TYPE_FUNCTION) {{\n", indent, method_var, method_var));
                code.push_str(&format!("{}    fprintf(stderr, \"Error: method '{}' not found or not a function\\n\");\n", indent, method));
                code.push_str(&format!("{}    return NULL;\n", indent));
                code.push_str(&format!("{}}}\n", indent));
                
                // Generate code for the arguments
                let mut arg_codes = String::new();
                let mut arg_vars = Vec::new();
                
                for arg in args {
                    let (arg_code, arg_var) = self.generate_c_code_for_expr(arg, indent_level)?;
                    arg_codes.push_str(&arg_code);
                    arg_vars.push(arg_var);
                }
                
                // Add the argument code to the main code
                code.push_str(&arg_codes);
                
                // Create an array of arguments
                let args_array_var = format!("args_{}", self.temp_var_counter);
                self.temp_var_counter += 1;
                
                code.push_str(&format!("{}// Create array of arguments for method call\n", indent));
                code.push_str(&format!("{}SmashValue* {}[{}];\n", indent, args_array_var, args.len()));
                
                // Assign arguments to the array
                for (i, arg_var) in arg_vars.iter().enumerate() {
                    code.push_str(&format!("{}{}[{}] = {};\n", indent, args_array_var, i, arg_var));
                }
                
                // Call the method
                let result_var = format!("method_result_{}", self.temp_var_counter);
                self.temp_var_counter += 1;
                
                code.push_str(&format!("{}// Call the method\n", indent));
                code.push_str(&format!("{}SmashValue* {} = {}->data.function({}, {}, {});\n", 
                                      indent, result_var, method_var, obj_var, args.len(), 
                                      if args.len() > 0 { args_array_var.as_str() } else { "NULL" }));
                
                Ok((code, result_var))
            },
            AstNode::ComputedPropertyAccess { object, property } => {
                // Generate code for the object expression
                let (obj_code, obj_var) = self.generate_c_code_for_expr(object, indent_level)?;
                
                // Generate code for the property expression
                let (prop_expr_code, prop_expr_var) = self.generate_c_code_for_expr(property, indent_level)?;
                
                // Create a new variable for the property value
                let prop_var = format!("prop_{}", self.temp_var_counter);
                self.temp_var_counter += 1;
                
                // Add the object code first
                code.push_str(&obj_code);
                
                // Add the property expression code
                code.push_str(&prop_expr_code);
                
                // Convert the property expression to a string (for use as a key)
                let key_var = format!("key_{}", self.temp_var_counter);
                self.temp_var_counter += 1;
                code.push_str(&format!("{}char* {} = smash_value_to_string({}); // Convert property to string\n", 
                                       indent, key_var, prop_expr_var));
                
                // Generate code to access the property using the string key
                code.push_str(&format!("{}SmashValue* {} = smash_object_get({}, {}); // Computed property access\n", 
                                       indent, prop_var, obj_var, key_var));
                
                // Free the temporary string
                code.push_str(&format!("{}free({}); // Free temporary string\n", indent, key_var));
                
                Ok((code, prop_var))
            },
            // Handle float literals
            AstNode::Float(f) => {
                let var_name = format!("smash_num_{}", self.temp_var_counter);
                self.temp_var_counter += 1;
                code.push_str(&format!("{}SmashValue* {} = smash_value_create_number({}); // Expr Float\n", indent, var_name, f));
                Ok((code, var_name))
            },
            
            // Handle unary operations
            AstNode::UnaryOp { op, expr } => {
                // Generate code for the operand
                let (expr_code, expr_var) = self.generate_c_code_for_expr(expr, indent_level)?;
                
                // Add the operand code first
                code.push_str(&expr_code);
                
                let result_var = format!("unary_{}", self.temp_var_counter);
                self.temp_var_counter += 1;
                
                match op.as_str() {
                    "!" => {
                        // Logical NOT operation
                        code.push_str(&format!("{}SmashValue* {} = smash_value_logical_not({}); // Logical NOT\n", 
                                          indent, result_var, expr_var));
                    },
                    _ => {
                        return Err(format!("Unsupported unary operator: {}", op));
                    }
                }
                
                Ok((code, result_var))
            },
            // Handle new expressions (e.g., new Promise())
            AstNode::NewExpr { constructor, args } => {
                let mut code = String::new();
                let result_var = format!("new_expr_{}", self.temp_var_counter);
                self.temp_var_counter += 1;
                
                // Handle Promise constructor specifically
                if constructor == "Promise" {
                    // Create a new Promise
                    code.push_str(&format!("{}SmashValue* {} = smash_promise_create(); // Create new Promise\n", 
                                       indent, result_var));
                    
                    // If there are arguments, assume the first one is the executor function
                    if !args.is_empty() {
                        // We expect the first argument to be an arrow function with resolve and reject parameters
                        if let AstNode::ArrowFunction { params, body, expression: _, is_async: _ } = &args[0] {
                            if params.len() >= 1 {
                                // Extract resolve parameter name (first parameter)
                                let resolve_param = &params[0];
                                
                                // Get the second parameter (reject) if it exists
                                let reject_param = if params.len() >= 2 {
                                    Some(&params[1])
                                } else {
                                    None
                                };
                                
                                // Generate code for the body of the arrow function
                                let mut body_code = String::new();
                                for stmt in body {
                                    // Check if the statement is a function call to setTimeout
                                    if let AstNode::FunctionCall { name, args: call_args } = stmt {
                                        if name == "setTimeout" && call_args.len() >= 2 {
                                            // This is a setTimeout call, extract the callback and delay
                                            if let AstNode::Identifier(callback_name) = &call_args[0] {
                                                if callback_name == resolve_param {
                                                    // This is calling setTimeout with the resolve function
                                                    if let AstNode::Identifier(delay_var) = &call_args[1] {
                                                        // Generate code to call setTimeout with the resolve function
                                                        code.push_str(&format!("{}// Create a function that resolves the promise\n", indent));
                                                        code.push_str(&format!("{}SmashFunction resolve_fn = ^SmashValue*(SmashValue* this_val, int argc, SmashValue** args) {{\n", indent));
                                                        code.push_str(&format!("{}    // Resolve the promise\n", indent));
                                                        code.push_str(&format!("{}    smash_promise_resolve({}, {});\n", indent, result_var, delay_var));
                                                        code.push_str(&format!("{}    return smash_value_create_null();\n", indent));
                                                        code.push_str(&format!("{}}}; // End of resolve function\n", indent));
                                                        
                                                        // Create a function value for the resolve callback
                                                        code.push_str(&format!("{}// Create function value for resolve callback\n", indent));
                                                        code.push_str(&format!("{}SmashValue* resolve_value = malloc(sizeof(SmashValue));\n", indent));
                                                        code.push_str(&format!("{}resolve_value->type = SMASH_TYPE_FUNCTION;\n", indent));
                                                        code.push_str(&format!("{}resolve_value->data.function = resolve_fn;\n", indent));
                                                        
                                                        // Call setTimeout with the resolve function
                                                        code.push_str(&format!("{}// Call setTimeout with the resolve function\n", indent));
                                                        code.push_str(&format!("{}smash_set_timeout(resolve_value, {}, 0, NULL);\n", indent, delay_var));
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    // For other constructors, generate a generic object
                    code.push_str(&format!("{}SmashValue* {} = smash_value_create_object(); // Generic object for constructor {}\n", 
                                       indent, result_var, constructor));
                }
                
                Ok((code, result_var))
            },
            AstNode::MethodCall { object, method, args } => {
                // Generate code for the object expression
                let (obj_code, obj_var) = self.generate_c_code_for_expr(object, indent_level)?;
                code.push_str(&obj_code);
                
                let result_var = format!("method_call_{}", self.temp_var_counter);
                self.temp_var_counter += 1;
                
                // Handle specific method calls
                match method.as_str() {
                    "then" => {
                        // Promise.then() implementation
                        if args.len() >= 1 {
                            // Generate code for the callback function
                            let callback_func_name = format!("then_callback_{}", self.temp_var_counter);
                            self.temp_var_counter += 1;
                            
                            // Generate function prototype
                            let prototype = format!("SmashValue* {}(SmashValue* this_val, int argc, SmashValue** args);", callback_func_name);
                            self.function_prototypes.push(prototype);
                            
                            // Generate function implementation
                            let mut func_impl = format!("SmashValue* {}(SmashValue* this_val, int argc, SmashValue** args) {{\n", callback_func_name);
                            func_impl.push_str("    // Extract the value from args[0]\n");
                            func_impl.push_str("    SmashValue* value = argc > 0 ? args[0] : smash_value_create_null();\n");
                            
                            // Generate the callback body based on the arrow function
                            if let AstNode::ArrowFunction { params, body, expression, is_async } = &args[0] {
                                if !params.is_empty() {
                                    let param_name = &params[0];
                                    func_impl.push_str(&format!("    // Assign parameter {} to the argument value\n", param_name));
                                    func_impl.push_str(&format!("    SmashValue* {} = value;\n", param_name));
                                    
                                    // Generate code for the function body
                                    for stmt in body {
                                        let (stmt_code, _) = self.generate_c_code_for_node(stmt, 1)?;
                                        func_impl.push_str(&stmt_code);
                                    }
                                }
                            }
                            
                            func_impl.push_str("    return smash_value_create_null(); // Default return value\n");
                            func_impl.push_str("}\n");
                            
                            self.function_implementations.push(func_impl);
                            
                            // Create a function value for the callback
                            code.push_str(&format!("{}SmashValue* {}_value = smash_value_create_function({});\n", 
                                                indent, callback_func_name, callback_func_name));
                            
                            // Call then on the promise
                            code.push_str(&format!("{}SmashValue* {} = smash_promise_then({}, {}_value, NULL);\n", 
                                                indent, result_var, obj_var, callback_func_name));
                        } else {
                            // No callback provided, just chain the promise
                            code.push_str(&format!("{}SmashValue* {} = smash_promise_then({}, NULL, NULL);\n", 
                                                indent, result_var, obj_var));
                        }
                    },
                    "catch" | "onCatch" => {
                        // Promise.catch() implementation
                        if args.len() >= 1 {
                            // Generate code for the callback function
                            let callback_func_name = format!("catch_callback_{}", self.temp_var_counter);
                            self.temp_var_counter += 1;
                            
                            // Generate function prototype
                            let prototype = format!("SmashValue* {}(SmashValue* this_val, int argc, SmashValue** args);", callback_func_name);
                            self.function_prototypes.push(prototype);
                            
                            // Generate function implementation
                            let mut func_impl = format!("SmashValue* {}(SmashValue* this_val, int argc, SmashValue** args) {{\n", callback_func_name);
                            func_impl.push_str("    // Extract the error from args[0]\n");
                            func_impl.push_str("    SmashValue* error = argc > 0 ? args[0] : smash_value_create_null();\n");
                            
                            // Generate the callback body based on the arrow function
                            if let AstNode::ArrowFunction { params, body, expression, is_async } = &args[0] {
                                if !params.is_empty() {
                                    let param_name = &params[0];
                                    func_impl.push_str(&format!("    // Assign parameter {} to the error value\n", param_name));
                                    func_impl.push_str(&format!("    SmashValue* {} = error;\n", param_name));
                                    
                                    // Generate code for the function body
                                    for stmt in body {
                                        let (stmt_code, _) = self.generate_c_code_for_node(stmt, 1)?;
                                        func_impl.push_str(&stmt_code);
                                    }
                                }
                            }
                            
                            func_impl.push_str("    return smash_value_create_null(); // Default return value\n");
                            func_impl.push_str("}\n");
                            
                            self.function_implementations.push(func_impl);
                            
                            // Create a function value for the callback
                            code.push_str(&format!("{}SmashValue* {}_value = smash_value_create_function({});\n", 
                                                indent, callback_func_name, callback_func_name));
                            
                            // Call catch/onCatch on the promise
                            if method == "catch" {
                                code.push_str(&format!("{}SmashValue* {} = smash_promise_catch({}, {}_value);\n", 
                                                    indent, result_var, obj_var, callback_func_name));
                            } else {
                                code.push_str(&format!("{}SmashValue* {} = smash_promise_on_catch({}, {}_value);\n", 
                                                    indent, result_var, obj_var, callback_func_name));
                            }
                        } else {
                            // No callback provided, just chain the promise
                            if method == "catch" {
                                code.push_str(&format!("{}SmashValue* {} = smash_promise_catch({}, NULL);\n", 
                                                    indent, result_var, obj_var));
                            } else {
                                code.push_str(&format!("{}SmashValue* {} = smash_promise_on_catch({}, NULL);\n", 
                                                    indent, result_var, obj_var));
                            }
                        }
                    },
                    _ => {
                        // Generic method call
                    self.temp_var_counter += 1;
                    code.push_str(&format!("{}SmashValue* {} = smash_async_function_wrapper({});\n", indent, async_var, result_var));
                    return Ok((code, async_var));
                }
                
                Ok((code, result_var))
            },
            // Add other expression types as needed (FunctionCall returning value, BinaryOp, etc.)
            _ => Err(format!("C code generation not implemented for expression node: {:?}", expr)),
        }
    }

    // Helper method to generate C code for a specific AST node statement
    // Returns a tuple: (code_string, result_variable_name)
    fn generate_c_code_for_node(&mut self, node: &AstNode, indent_level: usize) -> Result<(String, String), String> {
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
            },
            AstNode::ConstDecl { name, value } => {
                // Generate code for the value expression (same as LetDecl since C doesn't have const for variables)
                match self.generate_c_code_for_expr(value, indent_level) {
                    Ok((expr_code, expr_var)) => {
                        code.push_str(&expr_code); // Code to create the value
                        // Declare the variable and assign the result from the expression
                        code.push_str(&format!("{}SmashValue* {} = {};\n", indent, name, expr_var));
                    }
                    Err(e) => {
                        code.push_str(&format!("{}// Error generating expression for const {}: {}\n", indent, name, e));
                        code.push_str(&format!("{}SmashValue* {} = smash_value_create_null(); // Error fallback\n", indent, name));
                    }
                }
            }
            AstNode::Function { name, params, body, is_async } => {
                // Generate a unique function ID
                let func_id = self.next_temp_id();
                let func_name = format!("{}_func_{}", name, func_id);
                
                // Store function prototype for later declaration
                self.function_prototypes.push(format!("SmashValue* {}(SmashValue* this_value, int argc, SmashValue** args);", func_name));
                
                // Store function implementation for later definition
                let mut func_impl = String::new();
                
                // Function header with appropriate return type
                if *is_async {
                    func_impl.push_str(&format!("SmashValue* {}(SmashValue* this_value, int argc, SmashValue** args) {{\n", func_name));
                    func_impl.push_str("    // Create a new promise to return\n");
                    func_impl.push_str("    SmashValue* promise = smash_promise_create();\n");
                } else {
                    func_impl.push_str(&format!("SmashValue* {}(SmashValue* this_value, int argc, SmashValue** args) {{\n", func_name));
                }
                
                // Parameter handling
                func_impl.push_str("    // Parameter handling\n");
                for (i, param) in params.iter().enumerate() {
                    func_impl.push_str(&format!("    SmashValue* {} = (argc > {}) ? args[{}] : smash_value_create_null();\n", param, i, i));
                }
                
                // Function body
                func_impl.push_str("    // Function body\n");
                for node in body {
                    match self.generate_c_code_for_node(node, 1) {
                        Ok((node_code, _)) => {
                            func_impl.push_str(&format!("    {}", node_code.replace("\n", "\n    ")));
                        },
                        Err(e) => return Err(e),
                    }
                }
                
                // Default return if no explicit return statement
                if *is_async {
                    func_impl.push_str("    // Default return for async function\n");
                    func_impl.push_str("    smash_promise_resolve(promise, smash_value_create_null());\n");
                    func_impl.push_str("    return promise;\n");
                } else {
                    func_impl.push_str("    // Default return\n");
                    func_impl.push_str("    return smash_value_create_null();\n");
                }
                
                // End function
                func_impl.push_str("}\n");
                
                self.function_implementations.push(func_impl);
                
                // Create function object and assign to variable in the current code
                code.push_str(&format!("{}// Create function object for {}\n", indent, name));
                code.push_str(&format!("{}SmashValue* {} = smash_value_create_function({});\n", indent, name, func_name));
                
                return Ok((code, String::new()));
            },
            AstNode::ArrowFunction { params, body, expression, is_async } => {
                // Generate a unique function ID
                let func_id = self.next_temp_id();
                let var_name = format!("arrow_func_{}", func_id);
                
                // Arrow function declaration
                code.push_str(&format!("{}// Arrow function declaration\n", indent));
                
                // Create function parameter list
                let param_list = params.join(", ");
                
                // Function header with appropriate return type
                if *is_async {
                    code.push_str(&format!("{}SmashValue* {}_impl(SmashValue* this_value, int argc, SmashValue** args) {{\n", indent, var_name));
                    code.push_str(&format!("{}    // Create a new promise to return\n", indent));
                    code.push_str(&format!("{}    SmashValue* promise = smash_promise_create();\n", indent));
                } else {
                    code.push_str(&format!("{}SmashValue* {}_impl(SmashValue* this_value, int argc, SmashValue** args) {{\n", indent, var_name));
                }
                
                // Parameter handling
                code.push_str(&format!("{}    // Parameter handling\n", indent));
                for (i, param) in params.iter().enumerate() {
                    code.push_str(&format!("{}    SmashValue* {} = (argc > {}) ? args[{}] : smash_value_create_null();\n", indent, param, i, i));
                }
                
                // Function body
                code.push_str(&format!("{}    // Function body\n", indent));
                
                if *expression {
                    // Expression body (implicit return)
                    if body.len() == 1 {
                        match self.generate_c_code_for_expr(&body[0], indent_level + 1) {
                            Ok((expr_code, expr_var)) => {
                                code.push_str(&expr_code);
                                
                                if *is_async {
                                    code.push_str(&format!("{}    smash_promise_resolve(promise, {});\n", indent, expr_var));
                                    code.push_str(&format!("{}    return promise;\n", indent));
                                } else {
                                    code.push_str(&format!("{}    return {};\n", indent, expr_var));
                                }
                            }
                            Err(e) => {
                                return Err(format!("Error generating code for arrow function expression body: {}", e));
                            }
                        }
                    }
                } else {
                    // Block body (explicit return needed)
                    for node in body {
                        let node_code = self.generate_c_code_for_node(node, indent_level + 1)?;
                        code.push_str(&node_code.0);
                    }
                    
                    // Default return if no explicit return statement
                    if *is_async {
                        code.push_str(&format!("{}    // Default return for async function\n", indent));
                        code.push_str(&format!("{}    smash_promise_resolve(promise, smash_value_create_null());\n", indent));
                        code.push_str(&format!("{}    return promise;\n", indent));
                    } else {
                        code.push_str(&format!("{}    // Default return\n", indent));
                        code.push_str(&format!("{}    return smash_value_create_null();\n", indent));
                    }
                }
                
                // End function
                code.push_str(&format!("{}}}\n\n", indent));
                
                // Create function object
                code.push_str(&format!("{}// Create arrow function object\n", indent));
                code.push_str(&format!("{}SmashValue* {} = smash_value_create_function({}_impl);\n", indent, var_name, var_name));
                
                return Ok((code, String::new()));
            },
            AstNode::AwaitExpr { expr } => {
                // Generate code for the expression being awaited
                match self.generate_c_code_for_expr(expr, indent_level) {
                    Ok((expr_code, expr_var)) => {
                        let result_var = format!("await_result_{}", self.next_temp_id());
                        
                        code.push_str(&expr_code);
                        code.push_str(&format!("{}// Await expression\n", indent));
                        code.push_str(&format!("{}SmashValue* {} = NULL;\n", indent, result_var));
                        
                        // Create a callback function to handle the promise resolution
                        code.push_str(&format!("{}// Create callback for await\n", indent));
                        code.push_str(&format!("{}SmashValue* await_callback = smash_value_create_function(function(SmashValue* this_value, int argc, SmashValue** args) {{\n", indent));
                        code.push_str(&format!("{}    if (argc > 0) {{\n", indent));
                        code.push_str(&format!("{}        {} = args[0];\n", indent, result_var));
                        code.push_str(&format!("{}    }}\n", indent));
                        code.push_str(&format!("{}    return smash_value_create_null();\n", indent));
                        code.push_str(&format!("{}}}));\n", indent));
                        
                        // Add the callback to the promise
                        code.push_str(&format!("{}smash_promise_then({}, await_callback, NULL);\n", indent, expr_var));
                        
                        // Wait for the promise to resolve (in a real implementation, this would be non-blocking)
                        code.push_str(&format!("{}// Wait for promise to resolve (simplified implementation)\n", indent));
                        code.push_str(&format!("{}while ({} == NULL) {{\n", indent, result_var));
                        code.push_str(&format!("{}    // In a real implementation, this would yield control to the event loop\n", indent));
                        code.push_str(&format!("{}    // For now, we'll just simulate a small delay\n", indent));
                        code.push_str(&format!("{}    usleep(1000); // 1ms delay\n", indent));
                        code.push_str(&format!("{}}}\n", indent));
                        
                        // Create a temporary variable to hold the result
                        let temp_var = format!("temp_{}", self.next_temp_id());
                        code.push_str(&format!("{}SmashValue* {} = {};\n", indent, temp_var, result_var));
                        
                        return Ok((code, String::new()));
                    }
                    Err(e) => {
                        return Err(format!("Error generating code for await expression: {}", e));
                    }
                }
            },
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
                    
                    return Ok((code, String::new()));
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
                                    code.push_str(&stmt_code.0);
                                }
                            },
                            _ => {
                                // Handle single statement
                                let stmt_code = self.generate_c_code_for_node(body, indent_level + 1)?;
                                code.push_str(&stmt_code.0);
                            }
                        }

                        code.push_str(&format!("{indent}}}\n")); // Close loop body
                        code.push_str(&format!("{}// End ForOf loop for variable '{}'\n", indent, var_name));
                        code.push_str(&format!("{indent}smash_value_free({iterable_var}); // Free the iterable after the loop\n"));
                        return Ok((code, String::new()))
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
                     let (stmt_code, _) = self.generate_c_code_for_node(stmt, indent_level + 1)?;
                code.push_str(&stmt_code);
                 }
                 code.push_str(&format!("{}}}\n", indent)); // Closing brace for block
                 return Ok((code, String::new()))
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
                        return Ok((code, String::new()))
                    }
                    Err(e) => {
                        return Err(format!("Error evaluating standalone expression: {}", e))
                    }
                }
             }
            // Handle If statements
            AstNode::If { condition, then_branch, else_branch } => {
                // Generate code for the condition
                match self.generate_c_code_for_expr(condition, indent_level) {
                    Ok((cond_code, cond_var)) => {
                        // Add the condition code first
                        code.push_str(&cond_code);
                        
                        // Generate the if statement
                        code.push_str(&format!("{}if (smash_value_is_truthy({})) {{\n", indent, cond_var));
                        
                        // Generate code for the then branch
                        match self.generate_c_code_for_node(then_branch, indent_level + 1) {
                            Ok(then_code) => {
                                code.push_str(&then_code.0);
                                
                                // Handle the else branch if it exists
                                if let Some(else_branch) = else_branch {
                                    code.push_str(&format!("{}}} else {{\n", indent));
                                    match self.generate_c_code_for_node(else_branch, indent_level + 1) {
                                        Ok(else_code) => {
                                            code.push_str(&else_code.0);
                                        }
                                        Err(e) => {
                                            return Err(format!("Error generating code for else branch: {}", e));
                                        }
                                    }
                                }
                                
                                // Close the if statement
                                code.push_str(&format!("{}}}\n", indent));
                                code.push_str(&format!("{}// End if statement\n", indent));
                            }
                            Err(e) => {
                                return Err(format!("Error generating code for then branch: {}", e));
                            }
                        }
                    }
                    Err(e) => {
                        return Err(format!("Error generating code for if condition: {}", e));
                    }
                }
                
                return Ok((code, String::new()));
            },
            // Handle continue statement
            AstNode::Continue => {
                code.push_str(&format!("{}{};\n", indent, "continue"));
                return Ok((code, String::new()));
            },
            // Handle ForIn loops (iterate over object properties)
            AstNode::ForIn { var_name, object, body } => {
                // Generate code for the object expression
                match self.generate_c_code_for_expr(object, indent_level) {
                    Ok((obj_code, obj_var)) => {
                        // Add the object code first
                        code.push_str(&obj_code);
                        
                        // Generate a unique ID for this loop
                        let loop_id = self.next_temp_id();
                        
                        // Start the ForIn loop
                        code.push_str(&format!("{}// Start ForIn loop for variable '{}'\n", indent, var_name));
                        
                        // Get the keys from the object
                        code.push_str(&format!("{}SmashValue* keys_{} = smash_object_get_keys({});\n", indent, loop_id, obj_var));
                        code.push_str(&format!("{}int len_{} = smash_array_length(keys_{});\n", indent, loop_id, loop_id));
                        
                        // Start the loop
                        code.push_str(&format!("{}for (int i_{} = 0; i_{} < len_{}; i_{}++) {{\n", indent, loop_id, loop_id, loop_id, loop_id));
                        
                        // Get the current key
                        let inner_indent = "    ".repeat(indent_level + 1);
                        code.push_str(&format!("{}SmashValue* key_{} = smash_array_get(keys_{}, i_{});\n", inner_indent, loop_id, loop_id, loop_id));
                        code.push_str(&format!("{}char* {}_str = smash_value_to_string(key_{});\n", inner_indent, var_name, loop_id));
                        code.push_str(&format!("{}SmashValue* {} = smash_value_create_string({}_str);\n", inner_indent, var_name, var_name));
                        code.push_str(&format!("{}free({}_str); // Free the temporary string\n", inner_indent, var_name));
                        
                        // Generate code for the loop body
                        match self.generate_c_code_for_node(body, indent_level + 1) {
                            Ok(body_code) => {
                                code.push_str(&body_code.0);
                                
                                // Free the key variable at the end of each iteration
                                code.push_str(&format!("{}smash_value_free({});\n", inner_indent, var_name));
                            }
                            Err(e) => {
                                return Err(format!("Error generating code for ForIn body: {}", e));
                            }
                        }
                        
                        // Close the loop
                        code.push_str(&format!("{}}}\n", indent));
                        
                        // Free the keys array
                        code.push_str(&format!("{}smash_value_free(keys_{}); // Free the keys array\n", indent, loop_id));
                        code.push_str(&format!("{}// End ForIn loop for variable '{}'\n", indent, var_name));
                    }
                    Err(e) => {
                        return Err(format!("Error generating code for ForIn object: {}", e));
                    }
                }
                
                return Ok((code, String::new()));
            },
            // ... other statement types like ReturnStatement, etc. ...
            _ => {
                code.push_str(&format!("{}// TODO: Not implemented for statement node: {:?}\n", indent, node));
                return Ok((code, String::new()));
            }
        }
        // This line should never be reached as all match arms return now
        return Ok((code, String::new()))
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
