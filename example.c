
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
        SmashValue* smash_str_21 = smash_value_create_string("end"); // Expr String
        print(1, smash_str_21); // Print function call
    }
    // End ForOf loop for variable 'x'
    smash_value_free(y); // Free the iterable after the loop
    SmashValue* smash_str_22 = smash_value_create_string("Hello, SmashLang!"); // Expr String
    SmashValue* message = smash_str_22;
    SmashValue* smash_num_23 = smash_value_create_number(2025); // Expr Number
    SmashValue* year = smash_num_23;
    SmashValue* smash_num_24 = smash_value_create_number(3.14); // Expr Float
    SmashValue* pi = smash_num_24;
    SmashValue* smash_bool_25 = smash_value_create_boolean(true); // Expr Boolean
    SmashValue* active = smash_bool_25;
    SmashValue* smash_bool_26 = smash_value_create_boolean(false); // Expr Boolean
    SmashValue* active2 = smash_bool_26;
    if (smash_value_is_truthy(active)) {
        {
            print(1, pi); // Print function call
        }
    }
    // End if statement
    SmashValue* unary_27 = smash_value_logical_not(active2); // Logical NOT
    if (smash_value_is_truthy(unary_27)) {
        {
            SmashValue* smash_str_28 = smash_value_create_string("not active"); // Expr String
            print(2, smash_str_28, pi); // Print function call
        }
    }
    // End if statement
    SmashValue* smash_str_29 = smash_value_create_string("smash.*"); // Expr String
    SmashValue* pattern = smash_str_29;
    print(1, message); // Print function call

    // TODO: Cleanup if needed
    return 0;
}
