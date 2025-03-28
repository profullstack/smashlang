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

    for stmt in body {
        match stmt {
            AstNode::Return(expr) => {
                if let Some(ret_val) = gen_expr(context, builder, i64_type, function_map, expr) {
                    builder.build_return(Some(&ret_val));
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
        _ => None,
    }
}
