
#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>
#include "runtime.h" // Include our runtime support

int main(int argc, char** argv) {
    // TODO: Initialization, e.g., GC_init() if using Boehm GC
    SmashValue* smash_arr_0 = smash_value_create_array(3);
    SmashValue* smash_num_1 = smash_value_create_number(1); // Expr Number
    smash_array_push(smash_arr_0, smash_num_1);
    SmashValue* smash_num_2 = smash_value_create_number(2); // Expr Number
    smash_array_push(smash_arr_0, smash_num_2);
    SmashValue* smash_num_3 = smash_value_create_number(3); // Expr Number
    smash_array_push(smash_arr_0, smash_num_3);
    SmashValue* x = smash_arr_0;
    SmashValue* smash_obj_4 = smash_value_create_object(); // Create object
    SmashValue* smash_str_5 = smash_value_create_string("chovy"); // Expr String
    smash_object_set(smash_obj_4, "name", smash_str_5); // Set property
    SmashValue* smash_num_6 = smash_value_create_number(33); // Expr Number
    smash_object_set(smash_obj_4, "age", smash_num_6); // Set property
    SmashValue* user = smash_obj_4;
    SmashValue* smash_str_7 = smash_value_create_string("age:"); // Expr String
    SmashValue* prop_8 = smash_object_get(user, "age"); // Property access
    print(2, smash_str_7, prop_8); // Print function call
    // Start ForOf loop for variable 'i'
    int len_9 = smash_array_length(x);
    for (int i_9 = 0; i_9 < len_9; i_9++) {
        SmashValue* i = smash_array_get(x, i_9); // Assign current element
        print(1, i); // Print function call
    }
    // End ForOf loop for variable 'i'
    smash_value_free(x); // Free the iterable after the loop
    SmashValue* smash_str_10 = smash_value_create_string("Hello, SmashLang!"); // Expr String
    SmashValue* message = smash_str_10;
    SmashValue* smash_num_11 = smash_value_create_number(2025); // Expr Number
    SmashValue* year = smash_num_11;
    SmashValue* smash_num_12 = smash_value_create_number(3.14); // Expr Float
    SmashValue* pi = smash_num_12;
    SmashValue* smash_bool_13 = smash_value_create_boolean(true); // Expr Boolean
    SmashValue* active = smash_bool_13;
    SmashValue* smash_bool_14 = smash_value_create_boolean(false); // Expr Boolean
    SmashValue* active2 = smash_bool_14;
    // C code generation not implemented for statement node: If { condition: Identifier("active"), then_branch: Block([FunctionCall { name: "print", args: [Identifier("pi")] }]), else_branch: None }
    // C code generation not implemented for statement node: If { condition: UnaryOp { op: "!", expr: Identifier("active2") }, then_branch: Block([FunctionCall { name: "print", args: [String("not active"), Identifier("pi")] }]), else_branch: None }
    SmashValue* smash_str_15 = smash_value_create_string("smash.*"); // Expr String
    SmashValue* pattern = smash_str_15;
    print(1, message); // Print function call

    // TODO: Cleanup if needed
    return 0;
}
