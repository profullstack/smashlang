use inkwell::context::Context;
use inkwell::targets::{InitializationConfig, Target, TargetMachine, TargetTriple, CodeModel, RelocMode};
use inkwell::OptimizationLevel;
use inkwell::builder::Builder;
use inkwell::values::{FunctionValue, BasicValueEnum, IntValue, FloatValue, PointerValue};
use inkwell::types::BasicTypeEnum;

use crate::parser::AstNode;

use std::collections::HashMap;
use std::fmt;
use std::error::Error;

// Define a CodegenError type for code generation errors
#[derive(Debug)]
pub enum CodegenError {
    UnsupportedFeature(String),
    UndefinedVariable(String),
    UndefinedFunction(String),
    TypeMismatch(String),
    InvalidOperation(String),
    LLVMError(String),
}

impl fmt::Display for CodegenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CodegenError::UnsupportedFeature(feature) => write!(f, "Unsupported feature: {}", feature),
            CodegenError::UndefinedVariable(name) => write!(f, "Undefined variable: {}", name),
            CodegenError::UndefinedFunction(name) => write!(f, "Undefined function: {}", name),
            CodegenError::TypeMismatch(msg) => write!(f, "Type mismatch: {}", msg),
            CodegenError::InvalidOperation(msg) => write!(f, "Invalid operation: {}", msg),
            CodegenError::LLVMError(msg) => write!(f, "LLVM error: {}", msg),
        }
    }
}

impl Error for CodegenError {}

pub fn generate_llvm_ir<'ctx>(
    ast: &[AstNode],
    target: Option<&str>,
) -> Result<(inkwell::module::Module<'ctx>, TargetMachine), CodegenError> {
    Target::initialize_all(&InitializationConfig::default());
    let context = Context::create();
    let module = context.create_module("smashlang");
    let builder = context.create_builder();
    
    // Define basic types
    let i64_type = context.i64_type();
    let f64_type = context.f64_type();
    let i8_type = context.i8_type();
    let i1_type = context.bool_type();
    let void_type = context.void_type();
    let string_type = i8_type.ptr_type(inkwell::AddressSpace::Generic);

    let target_triple = TargetTriple::create(match target {
        Some(t) => t,
        None => TargetMachine::get_default_triple().as_str().to_str().unwrap(),
    });

    let target = Target::from_triple(&target_triple)
        .map_err(|e| CodegenError::LLVMError(format!("Failed to get target from triple: {}", e)))?;
    
    let target_machine = target
        .create_target_machine(
            &target_triple,
            "generic",
            "",
            OptimizationLevel::Default,
            RelocMode::Default,
            CodeModel::Default,
        )
        .map_err(|e| CodegenError::LLVMError(format!("Failed to create target machine: {}", e)))?;

    let mut function_map = HashMap::new();
    let mut variables = HashMap::new();

    // First pass: register all functions
    for node in ast {
        match node {
            AstNode::Function { name, params, .. } => {
                let fn_type = i64_type.fn_type(&vec![i64_type.into(); params.len()], false);
                let function = module.add_function(name, fn_type, None);
                function_map.insert(name.clone(), function);
            }
            _ => {}
        }
    }

    // Second pass: generate code for each node
    for node in ast {
        match node {
            AstNode::Function { name, params, body } => {
                codegen_function(&context, &module, &builder, &i64_type, &f64_type, &function_map, name, params, body)?;
            }
            AstNode::LetDecl { name, value } => {
                if let Some(val) = gen_expr(&context, &builder, &i64_type, &f64_type, &function_map, &variables, value)? {
                    println!("Generated let {} = {:?}", name, val.print_to_string());
                }
            }
            _ => {}
        }
    }

    Ok((module, target_machine))
}

fn codegen_function<'ctx>(
    context: &'ctx Context,
    module: &inkwell::module::Module<'ctx>,
    builder: &Builder<'ctx>,
    i64_type: &inkwell::types::IntType<'ctx>,
    f64_type: &inkwell::types::FloatType<'ctx>,
    function_map: &HashMap<String, FunctionValue<'ctx>>,
    name: &str,
    params: &[String],
    body: &[AstNode],
) -> Result<(), CodegenError> {
    let function = match function_map.get(name) {
        Some(f) => *f,
        None => return Err(CodegenError::UndefinedFunction(name.to_string())),
    };
    
    let entry = context.append_basic_block(function, "entry");
    builder.position_at_end(entry);

    // Create a new scope for function parameters
    let mut local_variables = HashMap::new();

    for (i, param) in function.get_param_iter().enumerate() {
        param.set_name(&params[i]);
        
        // Allocate stack space for the parameter
        let alloca = builder.build_alloca(i64_type.as_basic_type_enum(), &params[i]);
        builder.build_store(alloca, param);
        
        // Add to our variables map
        local_variables.insert(params[i].clone(), alloca);
    }

    // Create a stack of loop blocks for handling break and continue
    let mut loop_stack: Vec<(inkwell::basic_block::BasicBlock, inkwell::basic_block::BasicBlock)> = Vec::new();
    
    for stmt in body {
        match stmt {
            AstNode::Return(expr) => {
                if let Some(ret_val) = gen_expr(context, builder, i64_type, f64_type, function_map, &local_variables, expr)? {
                    match ret_val {
                        BasicValueEnum::IntValue(val) => {
                            builder.build_return(Some(&val));
                        },
                        BasicValueEnum::FloatValue(val) => {
                            // Convert float to int for now (simplified)
                            let int_val = builder.build_float_to_signed_int(val, *i64_type, "float_to_int");
                            builder.build_return(Some(&int_val));
                        },
                        _ => {
                            // For other types, return a default value for now
                            let default_val = i64_type.const_int(0, false);
                            builder.build_return(Some(&default_val));
                        }
                    }
                }
            }
            AstNode::Break => {
                // If we're in a loop, jump to the after_loop block
                if let Some((_, after_loop)) = loop_stack.last() {
                    builder.build_unconditional_branch(*after_loop);
                } else {
                    return Err(CodegenError::InvalidOperation("Break statement outside of loop".to_string()));
                }
            }
            AstNode::Continue => {
                // If we're in a loop, jump to the loop_header block
                if let Some((loop_header, _)) = loop_stack.last() {
                    builder.build_unconditional_branch(*loop_header);
                } else {
                    return Err(CodegenError::InvalidOperation("Continue statement outside of loop".to_string()));
                }
            }
            AstNode::Try { body, catch_param, catch_body, finally_body } => {
                // Create basic blocks for try, catch, finally, and continue
                let try_block = context.append_basic_block(function, "try");
                let catch_block = context.append_basic_block(function, "catch");
                let finally_block = context.append_basic_block(function, "finally");
                let continue_block = context.append_basic_block(function, "continue");
                
                // Set up exception handling mechanism
                // In a real implementation, this would involve setting up landing pads
                // and exception handling tables, but for simplicity, we'll just branch
                
                // Jump to try block
                builder.build_unconditional_branch(try_block);
                builder.position_at_end(try_block);
                
                // Generate code for try block
                for try_stmt in body {
                    match try_stmt {
                        AstNode::Return(expr) => {
                            if let Some(ret_val) = gen_expr(context, builder, i64_type, f64_type, function_map, &local_variables, expr)? {
                                // If there's a finally block, we need to execute it before returning
                                if finally_body.is_some() {
                                    builder.build_unconditional_branch(finally_block);
                                    builder.position_at_end(finally_block);
                                    // Generate finally code here...
                                    match ret_val {
                                        BasicValueEnum::IntValue(val) => {
                                            builder.build_return(Some(&val));
                                        },
                                        _ => {
                                            // For other types, return a default value for now
                                            let default_val = i64_type.const_int(0, false);
                                            builder.build_return(Some(&default_val));
                                        }
                                    }
                                } else {
                                    match ret_val {
                                        BasicValueEnum::IntValue(val) => {
                                            builder.build_return(Some(&val));
                                        },
                                        _ => {
                                            // For other types, return a default value for now
                                            let default_val = i64_type.const_int(0, false);
                                            builder.build_return(Some(&default_val));
                                        }
                                    }
                                }
                            }
                        }
                        AstNode::Throw(expr) => {
                            // In a real implementation, this would throw an exception
                            // For now, we'll just jump to the catch block
                            builder.build_unconditional_branch(catch_block);
                        }
                        _ => {
                            // Generate code for other statements in try block
                            // This is a simplified implementation
                        }
                    }
                }
                
                // At the end of try block, jump to finally if it exists, otherwise to continue
                if finally_body.is_some() {
                    builder.build_unconditional_branch(finally_block);
                } else {
                    builder.build_unconditional_branch(continue_block);
                }
                
                // Generate catch block
                builder.position_at_end(catch_block);
                
                // In a real implementation, we would extract the exception info here
                // and make it available via the catch_param
                
                // Generate code for catch block
                for catch_stmt in catch_body {
                    match catch_stmt {
                        AstNode::Return(expr) => {
                            if let Some(ret_val) = gen_expr(context, builder, i64_type, f64_type, function_map, &local_variables, expr)? {
                                // If there's a finally block, we need to execute it before returning
                                if finally_body.is_some() {
                                    builder.build_unconditional_branch(finally_block);
                                    builder.position_at_end(finally_block);
                                    // Generate finally code here...
                                    match ret_val {
                                        BasicValueEnum::IntValue(val) => {
                                            builder.build_return(Some(&val));
                                        },
                                        _ => {
                                            // For other types, return a default value for now
                                            let default_val = i64_type.const_int(0, false);
                                            builder.build_return(Some(&default_val));
                                        }
                                    }
                                } else {
                                    match ret_val {
                                        BasicValueEnum::IntValue(val) => {
                                            builder.build_return(Some(&val));
                                        },
                                        _ => {
                                            // For other types, return a default value for now
                                            let default_val = i64_type.const_int(0, false);
                                            builder.build_return(Some(&default_val));
                                        }
                                    }
                                }
                            }
                        }
                        _ => {
                            // Generate code for other statements in catch block
                            // This is a simplified implementation
                        }
                    }
                }
                
                // At the end of catch block, jump to finally if it exists, otherwise to continue
                if finally_body.is_some() {
                    builder.build_unconditional_branch(finally_block);
                } else {
                    builder.build_unconditional_branch(continue_block);
                }
                
                // Generate finally block if it exists
                if let Some(finally_stmts) = finally_body {
                    builder.position_at_end(finally_block);
                    
                    for finally_stmt in finally_stmts {
                        match finally_stmt {
                            AstNode::Return(expr) => {
                                if let Some(ret_val) = gen_expr(context, builder, i64_type, f64_type, function_map, &local_variables, expr)? {
                                    match ret_val {
                                        BasicValueEnum::IntValue(val) => {
                                            builder.build_return(Some(&val));
                                        },
                                        _ => {
                                            // For other types, return a default value for now
                                            let default_val = i64_type.const_int(0, false);
                                            builder.build_return(Some(&default_val));
                                        }
                                    }
                                }
                            }
                            _ => {
                                // Generate code for other statements in finally block
                                // This is a simplified implementation
                            }
                        }
                    }
                    
                    // At the end of finally block, jump to continue
                    builder.build_unconditional_branch(continue_block);
                }
                
                // Continue with the rest of the function
                builder.position_at_end(continue_block);
            }
            AstNode::Throw(expr) => {
                // In a real implementation, this would throw an exception
                // For now, we'll just generate a runtime error
                if let Some(_) = gen_expr(context, builder, i64_type, f64_type, function_map, &local_variables, expr)? {
                    // In a real implementation, we would create an exception object and throw it
                    // For now, we'll just return a special error value
                    let error_val = i64_type.const_int(0xDEADBEEF, false); // Special error value
                    builder.build_return(Some(&error_val));
                }
            }
            _ => {}
        }
    }

    // If we reach the end of the function without a return, add a default return
    if !builder.get_insert_block().unwrap().get_terminator().is_some() {
        let default_val = i64_type.const_int(0, false);
        builder.build_return(Some(&default_val));
    }

    Ok(())
}

fn gen_expr<'ctx>(
    context: &'ctx Context,
    builder: &Builder<'ctx>,
    i64_type: &inkwell::types::IntType<'ctx>,
    f64_type: &inkwell::types::FloatType<'ctx>,
    function_map: &HashMap<String, FunctionValue<'ctx>>,
    variables: &HashMap<String, PointerValue<'ctx>>,
    expr: &AstNode,
) -> Result<Option<BasicValueEnum<'ctx>>, CodegenError> {
    match expr {
        AstNode::Number(n) => Ok(Some(i64_type.const_int(*n as u64, true).into())),
        AstNode::Float(f) => Ok(Some(f64_type.const_float(*f).into())),
        AstNode::String(s) => {
            // Create a global string constant
            let string_val = builder.build_global_string_ptr(s, "string_literal");
            Ok(Some(string_val.as_pointer_value().into()))
        },
        AstNode::Boolean(b) => {
            let bool_val = if *b { 1 } else { 0 };
            Ok(Some(i64_type.const_int(bool_val, false).into()))
        },
        AstNode::Null => Ok(Some(i64_type.const_int(0, false).into())),
        AstNode::Identifier(name) => {
            // Load the value of the variable
            if let Some(ptr) = variables.get(name) {
                let loaded = builder.build_load(*ptr, name);
                Ok(Some(loaded))
            } else {
                Err(CodegenError::UndefinedVariable(name.clone()))
            }
        },
        AstNode::BinaryOp { left, op, right } => {
            let left_val = gen_expr(context, builder, i64_type, f64_type, function_map, variables, left)?
                .ok_or_else(|| CodegenError::InvalidOperation("Left operand of binary operation is invalid".to_string()))?;
            
            let right_val = gen_expr(context, builder, i64_type, f64_type, function_map, variables, right)?
                .ok_or_else(|| CodegenError::InvalidOperation("Right operand of binary operation is invalid".to_string()))?;
            
            // Handle different types of operands
            match (left_val, right_val) {
                (BasicValueEnum::IntValue(left_int), BasicValueEnum::IntValue(right_int)) => {
                    // Integer operations
                    match op.as_str() {
                        "+" => Ok(Some(builder.build_int_add(left_int, right_int, "addtmp").into())),
                        "-" => Ok(Some(builder.build_int_sub(left_int, right_int, "subtmp").into())),
                        "*" => Ok(Some(builder.build_int_mul(left_int, right_int, "multmp").into())),
                        "/" => {
                            // Check for division by zero
                            let is_zero = builder.build_int_compare(
                                inkwell::IntPredicate::EQ,
                                right_int,
                                i64_type.const_int(0, false),
                                "is_zero"
                            );
                            
                            // Create basic blocks for division and division by zero
                            let current_block = builder.get_insert_block().unwrap();
                            let function = current_block.get_parent().unwrap();
                            let div_block = context.append_basic_block(function, "div");
                            let div_by_zero_block = context.append_basic_block(function, "div_by_zero");
                            let continue_block = context.append_basic_block(function, "continue");
                            
                            // Branch based on whether the divisor is zero
                            builder.build_conditional_branch(is_zero, div_by_zero_block, div_block);
                            
                            // Division by zero block
                            builder.position_at_end(div_by_zero_block);
                            let error_val = i64_type.const_int(0xDEADBEEF, false); // Special error value
                            builder.build_store(variables.get("__error").unwrap_or(&variables.get("__result").unwrap_or(&variables.get("result").unwrap_or(&variables.get("_").unwrap_or(&variables.get("tmp").unwrap_or(&variables.get("__tmp").unwrap_or(&variables.get("__div_result").unwrap())))))), error_val);
                            builder.build_unconditional_branch(continue_block);
                            
                            // Normal division block
                            builder.position_at_end(div_block);
                            let div_result = builder.build_int_signed_div(left_int, right_int, "divtmp");
                            builder.build_unconditional_branch(continue_block);
                            
                            // Continue block
                            builder.position_at_end(continue_block);
                            let phi = builder.build_phi(i64_type, "div_result");
                            phi.add_incoming(&[
                                (&error_val, div_by_zero_block),
                                (&div_result, div_block)
                            ]);
                            
                            Ok(Some(phi.as_basic_value()))
                        },
                        "%" => Ok(Some(builder.build_int_signed_rem(left_int, right_int, "modtmp").into())),
                        "==" => {
                            let cmp = builder.build_int_compare(inkwell::IntPredicate::EQ, left_int, right_int, "eqtmp");
                            Ok(Some(builder.build_int_z_extend(cmp, *i64_type, "booltmp").into()))
                        },
                        "!=" => {
                            let cmp = builder.build_int_compare(inkwell::IntPredicate::NE, left_int, right_int, "netmp");
                            Ok(Some(builder.build_int_z_extend(cmp, *i64_type, "booltmp").into()))
                        },
                        "<" => {
                            let cmp = builder.build_int_compare(inkwell::IntPredicate::SLT, left_int, right_int, "lttmp");
                            Ok(Some(builder.build_int_z_extend(cmp, *i64_type, "booltmp").into()))
                        },
                        "<=" => {
                            let cmp = builder.build_int_compare(inkwell::IntPredicate::SLE, left_int, right_int, "letmp");
                            Ok(Some(builder.build_int_z_extend(cmp, *i64_type, "booltmp").into()))
                        },
                        ">" => {
                            let cmp = builder.build_int_compare(inkwell::IntPredicate::SGT, left_int, right_int, "gttmp");
                            Ok(Some(builder.build_int_z_extend(cmp, *i64_type, "booltmp").into()))
                        },
                        ">=" => {
                            let cmp = builder.build_int_compare(inkwell::IntPredicate::SGE, left_int, right_int, "getmp");
                            Ok(Some(builder.build_int_z_extend(cmp, *i64_type, "booltmp").into()))
                        },
                        _ => Err(CodegenError::UnsupportedFeature(format!("Binary operator: {}", op))),
                    }
                },
                (BasicValueEnum::FloatValue(left_float), BasicValueEnum::FloatValue(right_float)) => {
                    // Float operations
                    match op.as_str() {
                        "+" => Ok(Some(builder.build_float_add(left_float, right_float, "addtmp").into())),
                        "-" => Ok(Some(builder.build_float_sub(left_float, right_float, "subtmp").into())),
                        "*" => Ok(Some(builder.build_float_mul(left_float, right_float, "multmp").into())),
                        "/" => Ok(Some(builder.build_float_div(left_float, right_float, "divtmp").into())),
                        "==" => {
                            let cmp = builder.build_float_compare(inkwell::FloatPredicate::OEQ, left_float, right_float, "eqtmp");
                            Ok(Some(builder.build_int_z_extend(cmp, *i64_type, "booltmp").into()))
                        },
                        "!=" => {
                            let cmp = builder.build_float_compare(inkwell::FloatPredicate::ONE, left_float, right_float, "netmp");
                            Ok(Some(builder.build_int_z_extend(cmp, *i64_type, "booltmp").into()))
                        },
                        "<" => {
                            let cmp = builder.build_float_compare(inkwell::FloatPredicate::OLT, left_float, right_float, "lttmp");
                            Ok(Some(builder.build_int_z_extend(cmp, *i64_type, "booltmp").into()))
                        },
                        "<=" => {
                            let cmp = builder.build_float_compare(inkwell::FloatPredicate::OLE, left_float, right_float, "letmp");
                            Ok(Some(builder.build_int_z_extend(cmp, *i64_type, "booltmp").into()))
                        },
                        ">" => {
                            let cmp = builder.build_float_compare(inkwell::FloatPredicate::OGT, left_float, right_float, "gttmp");
                            Ok(Some(builder.build_int_z_extend(cmp, *i64_type, "booltmp").into()))
                        },
                        ">=" => {
                            let cmp = builder.build_float_compare(inkwell::FloatPredicate::OGE, left_float, right_float, "getmp");
                            Ok(Some(builder.build_int_z_extend(cmp, *i64_type, "booltmp").into()))
                        },
                        _ => Err(CodegenError::UnsupportedFeature(format!("Binary operator: {}", op))),
                    }
                },
                _ => Err(CodegenError::TypeMismatch(format!("Incompatible types for binary operator: {}", op))),
            }
        },
        AstNode::FunctionCall { name, args } => {
            let function = match function_map.get(name) {
                Some(f) => *f,
                None => return Err(CodegenError::UndefinedFunction(name.clone())),
            };
            
            let mut compiled_args = vec![];
            
            for arg in args {
                let arg_val = gen_expr(context, builder, i64_type, f64_type, function_map, variables, arg)?
                    .ok_or_else(|| CodegenError::InvalidOperation("Invalid function argument".to_string()))?;
                compiled_args.push(arg_val);
            }
            
            let call_site = builder.build_call(function, &compiled_args, "calltmp");
            Ok(call_site.try_as_basic_value().left())
        },
        AstNode::ArrayLiteral(elements) => {
            // For simplicity, we'll just return the first element for now
            // In a real implementation, we would allocate an array and store all elements
            if let Some(element) = elements.first() {
                gen_expr(context, builder, i64_type, f64_type, function_map, variables, element)
            } else {
                // Empty array, return null pointer
                Ok(Some(i64_type.const_int(0, false).into()))
            }
        },
        AstNode::Throw(expr) => {
            // In a real implementation, this would create an exception object
            // For now, we'll just generate a special error value
            let error_val = i64_type.const_int(0xDEADBEEF, false); // Special error value
            Ok(Some(error_val.into()))
        },
        AstNode::NewExpr { constructor, args } => {
            // Special handling for Error constructor
            if constructor == "Error" {
                // Create an error object - for now just use a special value
                // In a real implementation, this would create a proper error object
                let error_val = i64_type.const_int(0xDEADBEEF, false); // Special error value
                Ok(Some(error_val.into()))
            } else {
                // For other constructors, we would call the appropriate constructor function
                // For now, just return a placeholder value
                let placeholder = i64_type.const_int(0, false);
                Ok(Some(placeholder.into()))
            }
        },
        _ => Err(CodegenError::UnsupportedFeature(format!("AST node: {:?}", expr))),
    }
}
