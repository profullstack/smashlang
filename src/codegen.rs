},
        AstNode::PreIncrement(expr) => {
            // Get the variable to increment
            let var_ptr = match &**expr {
                AstNode::Identifier(name) => {
                    match variables.get(name) {
                        Some(ptr) => *ptr,
                        None => return Err(CodegenError::UndefinedVariable(name.clone())),
                    }
                },
                _ => return Err(CodegenError::InvalidOperation("Pre-increment target must be an identifier".to_string())),
            };
            
            // Load the current value
            let var_val = builder.build_load(var_ptr, "var_val");
            
            // Increment the value
            let result = match var_val {
                BasicValueEnum::IntValue(int_val) => {
                    let one = i64_type.const_int(1, false);
                    let incremented = builder.build_int_add(int_val, one, "inctmp");
                    builder.build_store(var_ptr, incremented);
                    incremented.into()
                },
                BasicValueEnum::FloatValue(float_val) => {
                    let one = f64_type.const_float(1.0);
                    let incremented = builder.build_float_add(float_val, one, "inctmp");
                    builder.build_store(var_ptr, incremented);
                    incremented.into()
                },
                _ => return Err(CodegenError::TypeMismatch("Cannot increment this type".to_string())),
            };
            
            // Return the incremented value
            Ok(Some(result))
        },
        AstNode::PostIncrement(expr) => {
            // Get the variable to increment
            let var_ptr = match &**expr {
                AstNode::Identifier(name) => {
                    match variables.get(name) {
                        Some(ptr) => *ptr,
                        None => return Err(CodegenError::UndefinedVariable(name.clone())),
                    }
                },
                _ => return Err(CodegenError::InvalidOperation("Post-increment target must be an identifier".to_string())),
            };
            
            // Load the current value
            let var_val = builder.build_load(var_ptr, "var_val");
            
            // Store the original value to return
            let original = var_val;
            
            // Increment the value
            match var_val {
                BasicValueEnum::IntValue(int_val) => {
                    let one = i64_type.const_int(1, false);
                    let incremented = builder.build_int_add(int_val, one, "inctmp");
                    builder.build_store(var_ptr, incremented);
                },
                BasicValueEnum::FloatValue(float_val) => {
                    let one = f64_type.const_float(1.0);
                    let incremented = builder.build_float_add(float_val, one, "inctmp");
                    builder.build_store(var_ptr, incremented);
                },
                _ => return Err(CodegenError::TypeMismatch("Cannot increment this type".to_string())),
            };
            
            // Return the original value
            Ok(Some(original))
        },
        AstNode::PreDecrement(expr) => {
            // Get the variable to decrement
            let var_ptr = match &**expr {
                AstNode::Identifier(name) => {
                    match variables.get(name) {
                        Some(ptr) => *ptr,
                        None => return Err(CodegenError::UndefinedVariable(name.clone())),
                    }
                },
                _ => return Err(CodegenError::InvalidOperation("Pre-decrement target must be an identifier".to_string())),
            };
            
            // Load the current value
            let var_val = builder.build_load(var_ptr, "var_val");
            
            // Decrement the value
            let result = match var_val {
                BasicValueEnum::IntValue(int_val) => {
                    let one = i64_type.const_int(1, false);
                    let decremented = builder.build_int_sub(int_val, one, "dectmp");
                    builder.build_store(var_ptr, decremented);
                    decremented.into()
                },
                BasicValueEnum::FloatValue(float_val) => {
                    let one = f64_type.const_float(1.0);
                    let decremented = builder.build_float_sub(float_val, one, "dectmp");
                    builder.build_store(var_ptr, decremented);
                    decremented.into()
                },
                _ => return Err(CodegenError::TypeMismatch("Cannot decrement this type".to_string())),
            };
            
            // Return the decremented value
            Ok(Some(result))
        },
        AstNode::PostDecrement(expr) => {
            // Get the variable to decrement
            let var_ptr = match &**expr {
                AstNode::Identifier(name) => {
                    match variables.get(name) {
                        Some(ptr) => *ptr,
                        None => return Err(CodegenError::UndefinedVariable(name.clone())),
                    }
                },
                _ => return Err(CodegenError::InvalidOperation("Post-decrement target must be an identifier".to_string())),
            };
            
            // Load the current value
            let var_val = builder.build_load(var_ptr, "var_val");
            
            // Store the original value to return
            let original = var_val;
            
            // Decrement the value
            match var_val {
                BasicValueEnum::IntValue(int_val) => {
                    let one = i64_type.const_int(1, false);
                    let decremented = builder.build_int_sub(int_val, one, "dectmp");
                    builder.build_store(var_ptr, decremented);
                },
                BasicValueEnum::FloatValue(float_val) => {
                    let one = f64_type.const_float(1.0);
                    let decremented = builder.build_float_sub(float_val, one, "dectmp");
                    builder.build_store(var_ptr, decremented);
                },
                _ => return Err(CodegenError::TypeMismatch("Cannot decrement this type".to_string())),
            };
            
            // Return the original value
            Ok(Some(original))
        },
        AstNode::Throw(expr) => {
