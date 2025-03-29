use inkwell::context::Context;
use inkwell::targets::{InitializationConfig, Target, TargetMachine, TargetTriple, CodeModel, RelocMode};
use inkwell::OptimizationLevel;
use inkwell::builder::Builder;
use inkwell::values::{FunctionValue, BasicValueEnum, IntValue, FloatValue, PointerValue};
use inkwell::types::BasicTypeEnum;
use inkwell::IntPredicate;

use crate::parser::{AstNode, SwitchCase};

use std::collections::HashMap;

pub fn generate_llvm_ir<'ctx>(
    ast: &[AstNode],
    target: Option<&str>,
) -> (inkwell::module::Module<'ctx>, TargetMachine) {
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
                codegen_function(&context, &module, &builder, &i64_type, &f64_type, &function_map, name, params, body);
            }
            AstNode::LetDecl { name, value } => {
                if let Some(val) = gen_expr(&context, &builder, &i64_type, &f64_type, &function_map, &variables, value) {
                    println!("Generated let {} = {:?}", name, val.print_to_string());
                }
            }
            _ => {}
        }
    }

    (module, target_machine)
}

// Helper function to generate code for a statement
fn codegen_statement<'ctx>(
    context: &'ctx Context,
    module: &inkwell::module::Module<'ctx>,
    builder: &Builder<'ctx>,
    i64_type: &inkwell::types::IntType<'ctx>,
    f64_type: &inkwell::types::FloatType<'ctx>,
    function_map: &HashMap<String, FunctionValue<'ctx>>,
    local_variables: &mut HashMap<String, PointerValue<'ctx>>,
    loop_stack: &mut Vec<(inkwell::basic_block::BasicBlock, inkwell::basic_block::BasicBlock)>,
    stmt: &AstNode,
) {
    match stmt {
        AstNode::Block(statements) => {
            for stmt in statements {
                codegen_statement(context, module, builder, i64_type, f64_type, function_map, local_variables, loop_stack, stmt);
                
                // If we've already generated a terminator (like return, break, continue),
                // don't generate code for the rest of the block
                if builder.get_insert_block().unwrap().get_terminator().is_some() {
                    break;
                }
            }
        },
        AstNode::Return(expr) => {
            if let Some(ret_val) = gen_expr(context, builder, i64_type, f64_type, function_map, local_variables, expr) {
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
        },
        AstNode::Break => {
            // If we're in a loop, jump to the after_loop block
            if let Some((_, after_loop)) = loop_stack.last() {
                builder.build_unconditional_branch(*after_loop);
            } else {
                // Error: break outside of loop
                // For now, just continue execution
            }
        },
        AstNode::Continue => {
            // If we're in a loop, jump to the loop_header block
            if let Some((loop_header, _)) = loop_stack.last() {
                builder.build_unconditional_branch(*loop_header);
            } else {
                // Error: continue outside of loop
                // For now, just continue execution
            }
        },
        AstNode::LetDecl { name, value } => {
            if let Some(val) = gen_expr(context, builder, i64_type, f64_type, function_map, local_variables, value) {
                // Allocate stack space for the variable
                let alloca = builder.build_alloca(val.get_type(), name);
                builder.build_store(alloca, val);
                
                // Add to our variables map
                local_variables.insert(name.clone(), alloca);
            }
        },
        AstNode::ConstDecl { name, value } => {
            if let Some(val) = gen_expr(context, builder, i64_type, f64_type, function_map, local_variables, value) {
                // Allocate stack space for the variable
                let alloca = builder.build_alloca(val.get_type(), name);
                builder.build_store(alloca, val);
                
                // Add to our variables map
                local_variables.insert(name.clone(), alloca);
            }
        },
        AstNode::If { condition, then_branch, else_branch } => {
            // Get the current function
            let function = builder.get_insert_block().unwrap().get_parent().unwrap();
            
            // Generate code for if statement
            let cond_val = gen_expr(context, builder, i64_type, f64_type, function_map, local_variables, condition)
                .expect("Failed to generate condition expression");
            
            // Convert condition to boolean (i1)
            let cond_val = match cond_val {
                BasicValueEnum::IntValue(val) => {
                    // Compare with 0 to get a boolean value
                    builder.build_int_compare(IntPredicate::NE, val, i64_type.const_int(0, false), "ifcond")
                },
                BasicValueEnum::FloatValue(val) => {
                    // Convert float to int, then compare with 0
                    let int_val = builder.build_float_to_signed_int(val, *i64_type, "float_to_int");
                    builder.build_int_compare(IntPredicate::NE, int_val, i64_type.const_int(0, false), "ifcond")
                },
                _ => {
                    // For other types, assume false
                    context.bool_type().const_int(0, false)
                }
            };
            
            // Create basic blocks for then, else, and merge
            let then_block = context.append_basic_block(function, "then");
            let else_block = context.append_basic_block(function, "else");
            let merge_block = context.append_basic_block(function, "ifcont");
            
            // Branch based on condition
            builder.build_conditional_branch(cond_val, then_block, else_block);
            
            // Generate code for then branch
            builder.position_at_end(then_block);
            codegen_statement(context, module, builder, i64_type, f64_type, function_map, local_variables, loop_stack, then_branch);
            
            // If the then branch doesn't end with a terminator (like return), add a branch to the merge block
            if !builder.get_insert_block().unwrap().get_terminator().is_some() {
                builder.build_unconditional_branch(merge_block);
            }
            
            // Generate code for else branch if it exists
            builder.position_at_end(else_block);
            if let Some(else_stmt) = else_branch {
                codegen_statement(context, module, builder, i64_type, f64_type, function_map, local_variables, loop_stack, else_stmt);
            }
            
            // If the else branch doesn't end with a terminator, add a branch to the merge block
            if !builder.get_insert_block().unwrap().get_terminator().is_some() {
                builder.build_unconditional_branch(merge_block);
            }
            
            // Continue code generation from the merge block
            builder.position_at_end(merge_block);
        },
        AstNode::While { condition, body } => {
            // Get the current function
            let function = builder.get_insert_block().unwrap().get_parent().unwrap();
            
            // Create basic blocks for loop header, loop body, and after loop
            let loop_header = context.append_basic_block(function, "loop_header");
            let loop_body = context.append_basic_block(function, "loop_body");
            let after_loop = context.append_basic_block(function, "after_loop");
            
            // Push loop blocks onto the stack for break/continue
            loop_stack.push((loop_header, after_loop));
            
            // Branch to loop header
            builder.build_unconditional_branch(loop_header);
            
            // Generate code for loop header (condition check)
            builder.position_at_end(loop_header);
            let cond_val = gen_expr(context, builder, i64_type, f64_type, function_map, local_variables, condition)
                .expect("Failed to generate condition expression");
            
            // Convert condition to boolean (i1)
            let cond_val = match cond_val {
                BasicValueEnum::IntValue(val) => {
                    // Compare with 0 to get a boolean value
                    builder.build_int_compare(IntPredicate::NE, val, i64_type.const_int(0, false), "loopcond")
                },
                BasicValueEnum::FloatValue(val) => {
                    // Convert float to int, then compare with 0
                    let int_val = builder.build_float_to_signed_int(val, *i64_type, "float_to_int");
                    builder.build_int_compare(IntPredicate::NE, int_val, i64_type.const_int(0, false), "loopcond")
                },
                _ => {
                    // For other types, assume false
                    context.bool_type().const_int(0, false)
                }
            };
            
            // Branch based on condition
            builder.build_conditional_branch(cond_val, loop_body, after_loop);
            
            // Generate code for loop body
            builder.position_at_end(loop_body);
            codegen_statement(context, module, builder, i64_type, f64_type, function_map, local_variables, loop_stack, body);
            
            // If the loop body doesn't end with a terminator, add a branch back to the loop header
            if !builder.get_insert_block().unwrap().get_terminator().is_some() {
                builder.build_unconditional_branch(loop_header);
            }
            
            // Pop loop blocks from the stack
            loop_stack.pop();
            
            // Continue code generation from after the loop
            builder.position_at_end(after_loop);
        },
        AstNode::For { init, condition, update, body } => {
            // Get the current function
            let function = builder.get_insert_block().unwrap().get_parent().unwrap();
            
            // Create basic blocks for loop header, loop body, loop update, and after loop
            let loop_header = context.append_basic_block(function, "for_header");
            let loop_body = context.append_basic_block(function, "for_body");
            let loop_update = context.append_basic_block(function, "for_update");
            let after_loop = context.append_basic_block(function, "after_for");
            
            // Push loop blocks onto the stack for break/continue
            // For 'continue' in a for loop, we want to jump to the update block
            loop_stack.push((loop_update, after_loop));
            
            // Generate initialization code if it exists
            if let Some(init_expr) = init {
                codegen_statement(context, module, builder, i64_type, f64_type, function_map, local_variables, loop_stack, init_expr);
            }
            
            // Branch to loop header
            builder.build_unconditional_branch(loop_header);
            
            // Generate code for loop header (condition check)
            builder.position_at_end(loop_header);
            
            // If there's no condition, use true (infinite loop)
            let cond_val = if let Some(cond_expr) = condition {
                let cond_val = gen_expr(context, builder, i64_type, f64_type, function_map, local_variables, cond_expr)
                    .expect("Failed to generate condition expression");
                
                // Convert condition to boolean (i1)
                match cond_val {
                    BasicValueEnum::IntValue(val) => {
                        // Compare with 0 to get a boolean value
                        builder.build_int_compare(IntPredicate::NE, val, i64_type.const_int(0, false), "forcond")
                    },
                    BasicValueEnum::FloatValue(val) => {
                        // Convert float to int, then compare with 0
                        let int_val = builder.build_float_to_signed_int(val, *i64_type, "float_to_int");
                        builder.build_int_compare(IntPredicate::NE, int_val, i64_type.const_int(0, false), "forcond")
                    },
                    _ => {
                        // For other types, assume false
                        context.bool_type().const_int(0, false)
                    }
                }
            } else {
                // No condition, use true (infinite loop)
                context.bool_type().const_int(1, false)
            };
            
            // Branch based on condition
            builder.build_conditional_branch(cond_val, loop_body, after_loop);
            
            // Generate code for loop body
            builder.position_at_end(loop_body);
            codegen_statement(context, module, builder, i64_type, f64_type, function_map, local_variables, loop_stack, body);
            
            // If the loop body doesn't end with a terminator, add a branch to the update block
            if !builder.get_insert_block().unwrap().get_terminator().is_some() {
                builder.build_unconditional_branch(loop_update);
            }
            
            // Generate code for loop update
            builder.position_at_end(loop_update);
            if let Some(update_expr) = update {
                let _ = gen_expr(context, builder, i64_type, f64_type, function_map, local_variables, update_expr);
            }
            
            // Branch back to loop header
            builder.build_unconditional_branch(loop_header);
            
            // Pop loop blocks from the stack
            loop_stack.pop();
            
            // Continue code generation from after the loop
            builder.position_at_end(after_loop);
        },
        AstNode::ForIn { var_name, object, body } => {
            // Get the current function
            let function = builder.get_insert_block().unwrap().get_parent().unwrap();
            
            // Create basic blocks for loop setup, loop header, loop body, and after loop
            let loop_setup = context.append_basic_block(function, "forin_setup");
            let loop_header = context.append_basic_block(function, "forin_header");
            let loop_body = context.append_basic_block(function, "forin_body");
            let after_loop = context.append_basic_block(function, "after_forin");
            
            // Push loop blocks onto the stack for break/continue
            loop_stack.push((loop_header, after_loop));
            
            // Branch to loop setup
            builder.build_unconditional_branch(loop_setup);
            
            // Generate code for loop setup
            builder.position_at_end(loop_setup);
            
            // Generate code for the object expression
            let obj_val = gen_expr(context, builder, i64_type, f64_type, function_map, local_variables, object)
                .expect("Failed to generate object expression");
            
            // Create a helper function to get object keys
            // In a real implementation, this would be a runtime function that returns an array of keys
            // For now, we'll create a simple array with a single element for demonstration
            
            // Allocate space for the keys array
            let keys_array_type = i64_type.array_type(1);
            let keys_array = builder.build_alloca(keys_array_type, "keys_array");
            
            // Store a dummy key (0) in the array
            let zero = i64_type.const_int(0, false);
            let key_ptr = unsafe {
                builder.build_in_bounds_gep(keys_array, &[zero, zero], "key_ptr")
            };
            builder.build_store(key_ptr, zero);
            
            // Create an index variable to iterate through the keys
            let index_ptr = builder.build_alloca(i64_type.as_basic_type_enum(), "index_ptr");
            builder.build_store(index_ptr, zero);
            
            // Branch to loop header
            builder.build_unconditional_branch(loop_header);
            
            // Generate code for loop header (condition check)
            builder.position_at_end(loop_header);
            
            // Load the current index
            let index = builder.build_load(index_ptr, "index");
            
            // Check if we've reached the end of the keys array
            // In a real implementation, this would compare against the length of the keys array
            let one = i64_type.const_int(1, false);
            let cond_val = builder.build_int_compare(
                IntPredicate::SLT,
                index.into_int_value(),
                one,
                "forin_cond"
            );
            
            // Branch based on condition
            builder.build_conditional_branch(cond_val, loop_body, after_loop);
            
            // Generate code for loop body
            builder.position_at_end(loop_body);
            
            // Load the current key
            let key_ptr = unsafe {
                builder.build_in_bounds_gep(keys_array, &[zero, index.into_int_value()], "key_ptr")
            };
            let key = builder.build_load(key_ptr, "key");
            
            // Allocate space for the loop variable
            let var_ptr = builder.build_alloca(key.get_type(), var_name);
            builder.build_store(var_ptr, key);
            
            // Add the loop variable to the local variables map
            local_variables.insert(var_name.clone(), var_ptr);
            
            // Generate code for the loop body
            codegen_statement(context, module, builder, i64_type, f64_type, function_map, local_variables, loop_stack, body);
            
            // Remove the loop variable from the local variables map
            local_variables.remove(var_name);
            
            // Increment the index
            let next_index = builder.build_int_add(
                index.into_int_value(),
                i64_type.const_int(1, false),
                "next_index"
            );
            builder.build_store(index_ptr, next_index);
            
            // If the loop body doesn't end with a terminator, add a branch back to the loop header
            if !builder.get_insert_block().unwrap().get_terminator().is_some() {
                builder.build_unconditional_branch(loop_header);
            }
            
            // Pop loop blocks from the stack
            loop_stack.pop();
            
            // Continue code generation from after the loop
            builder.position_at_end(after_loop);
        },
        AstNode::ForOf { var_name, iterable, body } => {
            // Get the current function
            let function = builder.get_insert_block().unwrap().get_parent().unwrap();
            
            // Create basic blocks for loop setup, loop header, loop body, and after loop
            let loop_setup = context.append_basic_block(function, "forof_setup");
            let loop_header = context.append_basic_block(function, "forof_header");
            let loop_body = context.append_basic_block(function, "forof_body");
            let after_loop = context.append_basic_block(function, "after_forof");
            
            // Push loop blocks onto the stack for break/continue
            loop_stack.push((loop_header, after_loop));
            
            // Branch to loop setup
            builder.build_unconditional_branch(loop_setup);
            
            // Generate code for loop setup
            builder.position_at_end(loop_setup);
            
            // Generate code for the iterable expression
            let iterable_val = gen_expr(context, builder, i64_type, f64_type, function_map, local_variables, iterable)
                .expect("Failed to generate iterable expression");
            
            // In a real implementation, this would check if the value is iterable
            // For now, we'll assume it's an array and create a simple array with a single element for demonstration
            
            // Allocate space for the array
            let array_type = i64_type.array_type(1);
            let array = builder.build_alloca(array_type, "array");
            
            // Store a dummy value (42) in the array
            let zero = i64_type.const_int(0, false);
            let value_ptr = unsafe {
                builder.build_in_bounds_gep(array, &[zero, zero], "value_ptr")
            };
            let dummy_value = i64_type.const_int(42, false);
            builder.build_store(value_ptr, dummy_value);
            
            // Create an index variable to iterate through the array
            let index_ptr = builder.build_alloca(i64_type.as_basic_type_enum(), "index_ptr");
            builder.build_store(index_ptr, zero);
            
            // Branch to loop header
            builder.build_unconditional_branch(loop_header);
            
            // Generate code for loop header (condition check)
            builder.position_at_end(loop_header);
            
            // Load the current index
            let index = builder.build_load(index_ptr, "index");
            
            // Check if we've reached the end of the array
            // In a real implementation, this would compare against the length of the array
            let one = i64_type.const_int(1, false);
            let cond_val = builder.build_int_compare(
                IntPredicate::SLT,
                index.into_int_value(),
                one,
                "forof_cond"
            );
            
            // Branch based on condition
            builder.build_conditional_branch(cond_val, loop_body, after_loop);
            
            // Generate code for loop body
            builder.position_at_end(loop_body);
            
            // Load the current value
            let value_ptr = unsafe {
                builder.build_in_bounds_gep(array, &[zero, index.into_int_value()], "value_ptr")
            };
            let value = builder.build_load(value_ptr, "value");
            
            // Allocate space for the loop variable
            let var_ptr = builder.build_alloca(value.get_type(), var_name);
            builder.build_store(var_ptr, value);
            
            // Add the loop variable to the local variables map
            local_variables.insert(var_name.clone(), var_ptr);
            
            // Generate code for the loop body
            codegen_statement(context, module, builder, i64_type, f64_type, function_map, local_variables, loop_stack, body);
            
            // Remove the loop variable from the local variables map
            local_variables.remove(var_name);
            
            // Increment the index
            let next_index = builder.build_int_add(
                index.into_int_value(),
                i64_type.const_int(1, false),
                "next_index"
            );
            builder.build_store(index_ptr, next_index);
            
            // If the loop body doesn't end with a terminator, add a branch back to the loop header
            if !builder.get_insert_block().unwrap().get_terminator().is_some() {
                builder.build_unconditional_branch(loop_header);
            }
            
            // Pop loop blocks from the stack
            loop_stack.pop();
            
            // Continue code generation from after the loop
            builder.position_at_end(after_loop);
        },
        _ => {
            // For other statements, just evaluate them for their side effects
            let _ = gen_expr(context, builder, i64_type, f64_type, function_map, local_variables, stmt);
        }
    }
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
) {
    let function = *function_map.get(name).unwrap();
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
    
    // Generate code for each statement in the function body
    for stmt in body {
        codegen_statement(context, module, builder, i64_type, f64_type, function_map, &mut local_variables, &mut loop_stack, stmt);
        
        // If we've already generated a terminator (like return), don't generate code for the rest of the function
        if builder.get_insert_block().unwrap().get_terminator().is_some() {
            break;
        }
    }
    
    // If the function doesn't end with a return statement, add a default return
    if !builder.get_insert_block().unwrap().get_terminator().is_some() {
        let default_val = i64_type.const_int(0, false);
        builder.build_return(Some(&default_val));
    }
}

fn gen_expr<'ctx>(
    context: &'ctx Context,
    builder: &Builder<'ctx>,
    i64_type: &inkwell::types::IntType<'ctx>,
    f64_type: &inkwell::types::FloatType<'ctx>,
    function_map: &HashMap<String, FunctionValue<'ctx>>,
    variables: &HashMap<String, PointerValue<'ctx>>,
    expr: &AstNode,
) -> Option<BasicValueEnum<'ctx>> {
    match expr {
        AstNode::Number(n) => Some(i64_type.const_int(*n as u64, true).into()),
        AstNode::Float(f) => Some(f64_type.const_float(*f).into()),
        AstNode::String(s) => {
            // Create a global string constant
            let string_val = builder.build_global_string_ptr(s, "string_literal");
            Some(string_val.as_pointer_value().into())
        },
        AstNode::Boolean(b) => {
            let bool_val = if *b { 1 } else { 0 };
            Some(i64_type.const_int(bool_val, false).into())
        },
        AstNode::Null => Some(i64_type.const_int(0, false).into()),
        AstNode::Identifier(name) => {
            // Load the value of the variable
            if let Some(ptr) = variables.get(name) {
                let loaded = builder.build_load(*ptr, name);
                Some(loaded)
            } else {
                println!("Variable not found: {}", name);
                None
            }
        },
        _ => None,
    }
}
