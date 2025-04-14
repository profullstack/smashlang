
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
    // Start ForIn loop for variable 'val'
    SmashValue* keys_10 = smash_object_get_keys(user);
    int len_10 = smash_array_length(keys_10);
    for (int i_10 = 0; i_10 < len_10; i_10++) {
        SmashValue* key_10 = smash_array_get(keys_10, i_10);
        char* val_str = smash_value_to_string(key_10);
        SmashValue* val = smash_value_create_string(val_str);
        free(val_str); // Free the temporary string
        {
            char* key_12 = smash_value_to_string(val); // Convert property to string
            SmashValue* prop_11 = smash_object_get(user, key_12); // Computed property access
            free(key_12); // Free temporary string
            print(2, val, prop_11); // Print function call
        }
        smash_value_free(val);
    }
    smash_value_free(keys_10); // Free the keys array
    // End ForIn loop for variable 'val'
    SmashValue* smash_str_13 = smash_value_create_string("Hello, SmashLang!"); // Expr String
    SmashValue* message = smash_str_13;
    SmashValue* smash_num_14 = smash_value_create_number(2025); // Expr Number
    SmashValue* year = smash_num_14;
    SmashValue* smash_num_15 = smash_value_create_number(3.14); // Expr Float
    SmashValue* pi = smash_num_15;
    SmashValue* smash_bool_16 = smash_value_create_boolean(true); // Expr Boolean
    SmashValue* active = smash_bool_16;
    SmashValue* smash_bool_17 = smash_value_create_boolean(false); // Expr Boolean
    SmashValue* active2 = smash_bool_17;
    // Start if statement
    if (smash_value_is_truthy(active)) {
        {
            print(1, pi); // Print function call
        }
    }
    // End if statement
    SmashValue* unary_18 = smash_value_logical_not(active2); // Logical NOT
    // Start if statement
    if (smash_value_is_truthy(unary_18)) {
        {
            SmashValue* smash_str_19 = smash_value_create_string("not active"); // Expr String
            print(2, smash_str_19, pi); // Print function call
        }
    }
    // End if statement
    SmashValue* smash_str_20 = smash_value_create_string("smash.*"); // Expr String
    SmashValue* pattern = smash_str_20;
    print(1, message); // Print function call

    // TODO: Cleanup if needed
    return 0;
}
