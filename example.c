
#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>
#include <pthread.h>
#include <time.h>
#include "runtime.h" // Include our runtime support

// Function prototypes
SmashValue* sleep_func_21(SmashValue* this_value, int argc, SmashValue** args);
SmashValue* test_func_23(SmashValue* this_value, int argc, SmashValue** args);
SmashValue* createPromise_func_38(SmashValue* this_value, int argc, SmashValue** args);
SmashValue* handlePromise_func_39(SmashValue* this_value, int argc, SmashValue** args);
SmashValue* handleError_func_41(SmashValue* this_value, int argc, SmashValue** args);
SmashValue* createDelayPromise_func_48(SmashValue* this_value, int argc, SmashValue** args);
SmashValue* processAsync_func_49(SmashValue* this_value, int argc, SmashValue** args);


// Function implementations
SmashValue* sleep_func_21(SmashValue* this_value, int argc, SmashValue** args) {
    // Create a new promise to return
    SmashValue* promise = smash_promise_create();
    // Parameter handling
    SmashValue* ms = (argc > 0) ? args[0] : smash_value_create_null();
    // Function body
        SmashValue* smash_str_22 = smash_value_create_string("Sleeping..."); // Expr String
        print(1, smash_str_22); // Print function call
            // TODO: Not implemented for statement node: Return(NewExpr { constructor: "Promise", args: [ArrowFunction { params: ["resolve", "reject"], body: [FunctionCall { name: "setTimeout", args: [Identifier("resolve"), Identifier("ms")] }], expression: true, is_async: false }] })
        // Default return for async function
    smash_promise_resolve(promise, smash_value_create_null());
    return promise;
}

SmashValue* test_func_23(SmashValue* this_value, int argc, SmashValue** args) {
    // Create a new promise to return
    SmashValue* promise = smash_promise_create();
    // Parameter handling
    // Function body
        SmashValue* smash_str_24 = smash_value_create_string("Starting test"); // Expr String
        print(1, smash_str_24); // Print function call
            // Error generating expression for const result: C code generation not implemented for expression node: AwaitExpr { expr: FunctionCall { name: "sleep", args: [Number(1000)] } }
        SmashValue* result = smash_value_create_null(); // Error fallback
            SmashValue* smash_str_25 = smash_value_create_string("Finished sleeping"); // Expr String
        print(1, smash_str_25); // Print function call
            // TODO: Not implemented for statement node: Return(Identifier("result"))
        // Default return for async function
    smash_promise_resolve(promise, smash_value_create_null());
    return promise;
}

SmashValue* createPromise_func_38(SmashValue* this_value, int argc, SmashValue** args) {
    // Parameter handling
    // Function body
        // TODO: Not implemented for statement node: Return(NewExpr { constructor: "Promise", args: [ArrowFunction { params: ["resolve", "reject"], body: [FunctionCall { name: "setTimeout", args: [ArrowFunction { params: [], body: [FunctionCall { name: "resolve", args: [String("Promise resolved!")] }], expression: false, is_async: false }, Number(1000)] }], expression: true, is_async: false }] })
        // Default return
    return smash_value_create_null();
}

SmashValue* handlePromise_func_39(SmashValue* this_value, int argc, SmashValue** args) {
    // Parameter handling
    SmashValue* result = (argc > 0) ? args[0] : smash_value_create_null();
    // Function body
        SmashValue* smash_str_40 = smash_value_create_string("Promise result:"); // Expr String
        print(2, smash_str_40, result); // Print function call
            // TODO: Not implemented for statement node: Return(BinaryOp { left: String("Processed: "), op: "+", right: Identifier("result") })
        // Default return
    return smash_value_create_null();
}

SmashValue* handleError_func_41(SmashValue* this_value, int argc, SmashValue** args) {
    // Parameter handling
    SmashValue* error = (argc > 0) ? args[0] : smash_value_create_null();
    // Function body
        SmashValue* smash_str_42 = smash_value_create_string("Promise error:"); // Expr String
        print(2, smash_str_42, error); // Print function call
        // Default return
    return smash_value_create_null();
}

SmashValue* createDelayPromise_func_48(SmashValue* this_value, int argc, SmashValue** args) {
    // Parameter handling
    SmashValue* ms = (argc > 0) ? args[0] : smash_value_create_null();
    SmashValue* value = (argc > 1) ? args[1] : smash_value_create_null();
    // Function body
        // TODO: Not implemented for statement node: Return(NewExpr { constructor: "Promise", args: [ArrowFunction { params: ["resolve", "reject"], body: [FunctionCall { name: "setTimeout", args: [ArrowFunction { params: [], body: [FunctionCall { name: "resolve", args: [Identifier("value")] }], expression: false, is_async: false }, Identifier("ms")] }], expression: true, is_async: false }] })
        // Default return
    return smash_value_create_null();
}

SmashValue* processAsync_func_49(SmashValue* this_value, int argc, SmashValue** args) {
    // Create a new promise to return
    SmashValue* promise = smash_promise_create();
    // Parameter handling
    // Function body
        // TODO: Not implemented for statement node: Try { body: [FunctionCall { name: "print", args: [String("Starting async operation...")] }, ConstDecl { name: "result1", value: AwaitExpr { expr: FunctionCall { name: "createDelayPromise", args: [Number(500), String("First result")] } } }, FunctionCall { name: "print", args: [String("Got first result:"), Identifier("result1")] }, ConstDecl { name: "result2", value: AwaitExpr { expr: FunctionCall { name: "createDelayPromise", args: [Number(500), BinaryOp { left: String("Second result: "), op: "+", right: Identifier("result1") }] } } }, FunctionCall { name: "print", args: [String("Got second result:"), Identifier("result2")] }, Return(BinaryOp { left: String("Completed: "), op: "+", right: Identifier("result2") })], catch_param: Some("error"), catch_body: [FunctionCall { name: "print", args: [String("Async error:"), Identifier("error")] }, Return(Null)], finally_body: None }
        // Default return for async function
    smash_promise_resolve(promise, smash_value_create_null());
    return promise;
}



int main(int argc, char** argv) {
    // Initialization
    SmashValue* smash_arr_0 = smash_value_create_array(3);
    SmashValue* smash_num_1 = smash_value_create_number(1); // Expr Number
    smash_array_push(smash_arr_0, smash_num_1);
    SmashValue* smash_num_2 = smash_value_create_number(2); // Expr Number
    smash_array_push(smash_arr_0, smash_num_2);
    SmashValue* smash_num_3 = smash_value_create_number(3); // Expr Number
    smash_array_push(smash_arr_0, smash_num_3);
    SmashValue* x = smash_arr_0;
    SmashValue* smash_arr_4 = smash_value_create_array(4);
    SmashValue* smash_bool_5 = smash_value_create_boolean(true); // Expr Boolean
    smash_array_push(smash_arr_4, smash_bool_5);
    SmashValue* smash_bool_6 = smash_value_create_boolean(false); // Expr Boolean
    smash_array_push(smash_arr_4, smash_bool_6);
    SmashValue* smash_bool_7 = smash_value_create_boolean(false); // Expr Boolean
    smash_array_push(smash_arr_4, smash_bool_7);
    SmashValue* smash_bool_8 = smash_value_create_boolean(true); // Expr Boolean
    smash_array_push(smash_arr_4, smash_bool_8);
    SmashValue* y = smash_arr_4;
    SmashValue* smash_obj_9 = smash_value_create_object(); // Create object
    SmashValue* smash_num_10 = smash_value_create_number(33); // Expr Number
    smash_object_set(smash_obj_9, "age", smash_num_10); // Set property
    SmashValue* smash_str_11 = smash_value_create_string("chovy"); // Expr String
    smash_object_set(smash_obj_9, "name", smash_str_11); // Set property
    SmashValue* user = smash_obj_9;
    SmashValue* smash_str_12 = smash_value_create_string("age:"); // Expr String
    SmashValue* prop_13 = smash_object_get(user, "age"); // Property access
    print(2, smash_str_12, prop_13); // Print function call
    // Start ForOf loop for variable 'i'
    int len_14 = smash_array_length(x);
    for (int i_14 = 0; i_14 < len_14; i_14++) {
        SmashValue* i = smash_array_get(x, i_14); // Assign current element
        print(1, i); // Print function call
    }
    // End ForOf loop for variable 'i'
    smash_value_free(x); // Free the iterable after the loop
    // Start ForIn loop for variable 'val'
    SmashValue* keys_15 = smash_object_get_keys(user);
    int len_15 = smash_array_length(keys_15);
    for (int i_15 = 0; i_15 < len_15; i_15++) {
        SmashValue* key_15 = smash_array_get(keys_15, i_15);
        char* val_str = smash_value_to_string(key_15);
        SmashValue* val = smash_value_create_string(val_str);
        free(val_str); // Free the temporary string
        {
            char* key_17 = smash_value_to_string(val); // Convert property to string
            SmashValue* prop_16 = smash_object_get(user, key_17); // Computed property access
            free(key_17); // Free temporary string
            print(2, val, prop_16); // Print function call
        }
        smash_value_free(val);
    }
    smash_value_free(keys_15); // Free the keys array
    // End ForIn loop for variable 'val'
    // Start ForOf loop for variable 'x'
    int len_18 = smash_array_length(y);
    for (int i_18 = 0; i_18 < len_18; i_18++) {
        SmashValue* x = smash_array_get(y, i_18); // Assign current element
        if (smash_value_is_truthy(x)) {
            {
                SmashValue* smash_str_19 = smash_value_create_string("true"); // Expr String
                print(1, smash_str_19); // Print function call
                continue;
            }
        } else {
            {
                SmashValue* smash_str_20 = smash_value_create_string("false"); // Expr String
                print(1, smash_str_20); // Print function call
            }
        }
        // End if statement
    }
    // End ForOf loop for variable 'x'
    smash_value_free(y); // Free the iterable after the loop
    // Create function object for sleep
    SmashValue* sleep = smash_value_create_function(sleep_func_21);
    // Create function object for test
    SmashValue* test = smash_value_create_function(test_func_23);
    SmashValue* smash_str_26 = smash_value_create_string("Before await"); // Expr String
    print(1, smash_str_26); // Print function call
    // Error generating expression for const result: C code generation not implemented for expression node: AwaitExpr { expr: FunctionCall { name: "test", args: [] } }
    SmashValue* result = smash_value_create_null(); // Error fallback
    SmashValue* smash_str_27 = smash_value_create_string("After await"); // Expr String
    print(1, smash_str_27); // Print function call
    SmashValue* smash_str_28 = smash_value_create_string("Result:"); // Expr String
    print(1, smash_str_28); // Print function call
    print(1, result); // Print function call
    SmashValue* smash_str_29 = smash_value_create_string("Hello, SmashLang!"); // Expr String
    SmashValue* message = smash_str_29;
    SmashValue* smash_num_30 = smash_value_create_number(2025); // Expr Number
    SmashValue* year = smash_num_30;
    SmashValue* smash_num_31 = smash_value_create_number(3.14); // Expr Float
    SmashValue* pi = smash_num_31;
    SmashValue* smash_bool_32 = smash_value_create_boolean(true); // Expr Boolean
    SmashValue* active = smash_bool_32;
    SmashValue* smash_bool_33 = smash_value_create_boolean(false); // Expr Boolean
    SmashValue* active2 = smash_bool_33;
    if (smash_value_is_truthy(active)) {
        {
            print(1, pi); // Print function call
        }
    }
    // End if statement
    SmashValue* unary_34 = smash_value_logical_not(active2); // Logical NOT
    if (smash_value_is_truthy(unary_34)) {
        {
            SmashValue* smash_str_35 = smash_value_create_string("not active"); // Expr String
            print(2, smash_str_35, pi); // Print function call
        }
    }
    // End if statement
    SmashValue* smash_str_36 = smash_value_create_string("smash.*"); // Expr String
    SmashValue* pattern = smash_str_36;
    print(1, message); // Print function call
    SmashValue* smash_str_37 = smash_value_create_string("Promise test:"); // Expr String
    print(1, smash_str_37); // Print function call
    // Create function object for createPromise
    SmashValue* createPromise = smash_value_create_function(createPromise_func_38);
    // Create function object for handlePromise
    SmashValue* handlePromise = smash_value_create_function(handlePromise_func_39);
    // Create function object for handleError
    SmashValue* handleError = smash_value_create_function(handleError_func_41);
    // Error generating expression for const promiseResult: C code generation not implemented for expression node: FunctionCall { name: "createPromise", args: [] }
    SmashValue* promiseResult = smash_value_create_null(); // Error fallback
    // TODO: Not implemented for statement node: MethodCall { object: Identifier("promiseResult"), method: "then", args: [Identifier("handlePromise")] }
    // TODO: Not implemented for statement node: MethodCall { object: Identifier("promiseResult"), method: "onCatch", args: [Identifier("handleError")] }
    SmashValue* smash_str_43 = smash_value_create_string("Try/Catch/Finally example:"); // Expr String
    print(1, smash_str_43); // Print function call
    // TODO: Not implemented for statement node: Try { body: [FunctionCall { name: "print", args: [String("Inside try block")] }, Throw(NewExpr { constructor: "Error", args: [String("Something went wrong")] }), FunctionCall { name: "print", args: [String("This will not be executed")] }], catch_param: Some("error"), catch_body: [FunctionCall { name: "print", args: [String("Inside catch block")] }, FunctionCall { name: "print", args: [BinaryOp { left: String("Error message: "), op: "+", right: Identifier("error") }] }], finally_body: Some([FunctionCall { name: "print", args: [String("Inside finally block - this always executes")] }]) }
    SmashValue* smash_str_44 = smash_value_create_string("Fetch example with Promise chaining:"); // Expr String
    print(1, smash_str_44); // Print function call
    // Error generating expression for const fetchResult: C code generation not implemented for expression node: FunctionCall { name: "fetch", args: [String("https://jsonplaceholder.typicode.com/todos/1")] }
    SmashValue* fetchResult = smash_value_create_null(); // Error fallback
    // Error generating expression for const jsonResult: C code generation not implemented for expression node: ArrowFunction { params: ["response"], body: [FunctionCall { name: "print", args: [BinaryOp { left: String("Response status: "), op: "+", right: PropertyAccess { object: Identifier("response"), property: "status" } }] }, Return(MethodCall { object: Identifier("response"), method: "json", args: [] })], expression: false, is_async: false }
    SmashValue* jsonResult = smash_value_create_null(); // Error fallback
    // Error generating expression for const processedResult: C code generation not implemented for expression node: ArrowFunction { params: ["data"], body: [FunctionCall { name: "print", args: [BinaryOp { left: String("Todo title: "), op: "+", right: PropertyAccess { object: Identifier("data"), property: "title" } }] }, Return(Identifier("data"))], expression: false, is_async: false }
    SmashValue* processedResult = smash_value_create_null(); // Error fallback
    // TODO: Not implemented for statement node: MethodCall { object: Identifier("processedResult"), method: "onCatch", args: [ArrowFunction { params: ["error"], body: [FunctionCall { name: "print", args: [BinaryOp { left: String("Fetch error: "), op: "+", right: Identifier("error") }] }], expression: false, is_async: false }] }
    SmashValue* smash_str_47 = smash_value_create_string("Async/await test:"); // Expr String
    print(1, smash_str_47); // Print function call
    // Create function object for createDelayPromise
    SmashValue* createDelayPromise = smash_value_create_function(createDelayPromise_func_48);
    // Create function object for processAsync
    SmashValue* processAsync = smash_value_create_function(processAsync_func_49);
    SmashValue* smash_str_50 = smash_value_create_string("Before calling async function"); // Expr String
    print(1, smash_str_50); // Print function call
    // Error generating expression for const asyncResult: C code generation not implemented for expression node: AwaitExpr { expr: FunctionCall { name: "processAsync", args: [] } }
    SmashValue* asyncResult = smash_value_create_null(); // Error fallback
    SmashValue* smash_str_51 = smash_value_create_string("After async function"); // Expr String
    print(1, smash_str_51); // Print function call
    SmashValue* smash_str_52 = smash_value_create_string("Final result:"); // Expr String
    print(2, smash_str_52, asyncResult); // Print function call

    // Cleanup if needed
    return 0;
}
