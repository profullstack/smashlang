use inkwell::context::Context;
use inkwell::targets::{InitializationConfig, Target, TargetMachine, TargetTriple, TargetOptions, CodeModel, RelocMode};
use inkwell::OptimizationLevel;
use inkwell::builder::Builder;
use inkwell::values::{FunctionValue, BasicValueEnum, IntValue};

use crate::parser::AstNode;

use std::collections::HashMap;

pub fn generate_llvm_ir<'ctx>(
    ast: &[AstNode],
    target: Option<&str>,
) -> (inkwell::module::Module<'ctx>, TargetMachine) {
    Target::initialize_all(&InitializationConfig::default());
    let context = Context::create();
    let module = context.create_module("smashlang");
    let builder = context.create_builder();
    let i64_type = context.i64_type();

    let target_triple = TargetTriple::create(match target {
        Some(t) => t,
        None => TargetMachine::get_default_triple().as_str().to_str().unwrap(),
    });

    let target = Target::from_triple(&target_triple).unwrap();
    let target_machine = target
        .create_target_machine(
            &target_triple,
            "generic",
            "",
            OptimizationLevel::Default,
            RelocMode::Default,
            CodeModel::Default,
        )
        .unwrap();

    let mut function_map = HashMap::new();

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

    for node in ast {
        match node {
            AstNode::Function { name, params, body } => {
                codegen_function(&context, &module, &builder, &i64_type, &function_map, name, params, body);
            }
            AstNode::LetDecl { name, value } => {
                if let Some(val) = gen_expr(&context, &builder, &i64_type, &function_map, value) {
                    println!("Generated let {} = {:?}", name, val.print_to_string());
                }
            }
            _ => {}
        }
    }

    (module, target_machine)
}

fn codegen_function<'ctx>(
    context: &'ctx Context,
    module: &inkwell::module::Module<'ctx>,
    builder: &Builder<'ctx>,
    i64_type: &inkwell::types::IntType<'ctx>,
    function_map: &HashMap<String, FunctionValue<'ctx>>,
    name: &str,
    params: &[String],
    body: &[AstNode],
) {
    let function = *function_map.get(name).unwrap();
    let entry = context.append_basic_block(function, "entry");
    builder.position_at_end(entry);

    for (i, param) in function.get_param_iter().enumerate() {
        param.set_name(&params[i]);
    }

    // Create a stack of loop blocks for handling break and continue
    let mut loop_stack: Vec<(inkwell::basic_block::BasicBlock, inkwell::basic_block::BasicBlock)> = Vec::new();
    
    for stmt in body {
        match stmt {
            AstNode::Return(expr) => {
                if let Some(ret_val) = gen_expr(context, builder, i64_type, function_map, expr) {
                    builder.build_return(Some(&ret_val));
                }
            }
            AstNode::Break => {
                // If we're in a loop, jump to the after_loop block
                if let Some((_, after_loop)) = loop_stack.last() {
                    builder.build_unconditional_branch(*after_loop);
                } else {
                    // Error: break outside of loop
                    // For now, just continue execution
                }
            }
            AstNode::Continue => {
                // If we're in a loop, jump to the loop_header block
                if let Some((loop_header, _)) = loop_stack.last() {
                    builder.build_unconditional_branch(*loop_header);
                } else {
                    // Error: continue outside of loop
                    // For now, just continue execution
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
                            if let Some(ret_val) = gen_expr(context, builder, i64_type, function_map, expr) {
                                // If there's a finally block, we need to execute it before returning
                                if finally_body.is_some() {
                                    builder.build_unconditional_branch(finally_block);
                                    builder.position_at_end(finally_block);
                                    // Generate finally code here...
                                    builder.build_return(Some(&ret_val));
                                } else {
                                    builder.build_return(Some(&ret_val));
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
                            if let Some(ret_val) = gen_expr(context, builder, i64_type, function_map, expr) {
                                // If there's a finally block, we need to execute it before returning
                                if finally_body.is_some() {
                                    builder.build_unconditional_branch(finally_block);
                                    builder.position_at_end(finally_block);
                                    // Generate finally code here...
                                    builder.build_return(Some(&ret_val));
                                } else {
                                    builder.build_return(Some(&ret_val));
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
                                if let Some(ret_val) = gen_expr(context, builder, i64_type, function_map, expr) {
                                    builder.build_return(Some(&ret_val));
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
                if let Some(_) = gen_expr(context, builder, i64_type, function_map, expr) {
                    // In a real implementation, we would create an exception object and throw it
                    // For now, we'll just return a special error value
                    let error_val = i64_type.const_int(0xDEADBEEF, false); // Special error value
                    builder.build_return(Some(&error_val));
                }
            }
            _ => {}
        }
    }
}

fn gen_expr<'ctx>(
    context: &'ctx Context,
    builder: &Builder<'ctx>,
    i64_type: &inkwell::types::IntType<'ctx>,
    function_map: &HashMap<String, FunctionValue<'ctx>>,
    expr: &AstNode,
) -> Option<IntValue<'ctx>> {
    match expr {
        AstNode::Number(n) => Some(i64_type.const_int(*n as u64, true)),
        AstNode::BinaryOp { left, op, right } => {
            let left_val = gen_expr(context, builder, i64_type, function_map, left)?;
            let right_val = gen_expr(context, builder, i64_type, function_map, right)?;

            match op.as_str() {
                "+" => Some(builder.build_int_add(left_val, right_val, "addtmp")),
                "-" => Some(builder.build_int_sub(left_val, right_val, "subtmp")),
                _ => None,
            }
        }
        AstNode::FunctionCall { name, args } => {
            let function = *function_map.get(name)?;
            let mut compiled_args = vec![];

            for arg in args {
                let arg_val = gen_expr(context, builder, i64_type, function_map, arg)?;
                compiled_args.push(arg_val.into());
            }

            let call_site = builder.build_call(function, &compiled_args, "calltmp");
            call_site.try_as_basic_value().left().unwrap().into_int_value().into()
        }
        AstNode::Throw(expr) => {
            // In a real implementation, this would create an exception object
            // For now, we'll just generate a special error value
            let error_val = i64_type.const_int(0xDEADBEEF, false); // Special error value
            Some(error_val)
        },
        AstNode::NewExpr { constructor, args } => {
            // Special handling for Error constructor
            if constructor == "Error" {
                // Create an error object - for now just use a special value
                // In a real implementation, this would create a proper error object
                let error_val = i64_type.const_int(0xDEADBEEF, false); // Special error value
                Some(error_val)
            } else {
                // For other constructors, we would call the appropriate constructor function
                // For now, just return a placeholder value
                let placeholder = i64_type.const_int(0, false);
                Some(placeholder)
            }
        }
        _ => None,
    }
}
