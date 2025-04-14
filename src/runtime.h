#ifndef SMASH_RUNTIME_H
#define SMASH_RUNTIME_H

#include <stdbool.h>
#include "simple_regex.h"

// --- Value Representation ---

// Forward declarations
typedef struct SmashValue SmashValue;
typedef struct SmashArray SmashArray;
typedef struct SmashPromise SmashPromise;
typedef void (*PromiseCallback)(SmashValue*, void*);

// Promise status enum
typedef enum {
    PROMISE_PENDING,
    PROMISE_FULFILLED,
    PROMISE_REJECTED
} PromiseStatus;

// Promise structure
struct SmashPromise {
    PromiseStatus status;
    SmashValue* result;  // Result or reason
    PromiseCallback on_fulfill;
    PromiseCallback on_reject;
    void* callback_data;
};

// Property structure for objects
typedef struct {
    char* key;
    struct SmashValue* value;
} SmashProperty;

// Object structure
typedef struct {
    SmashProperty* properties;
    int size;
} SmashObject;

typedef enum {
    SMASH_TYPE_NULL,
    SMASH_TYPE_UNDEFINED,
    SMASH_TYPE_BOOLEAN,
    SMASH_TYPE_NUMBER,
    SMASH_TYPE_STRING,
    SMASH_TYPE_ARRAY,
    SMASH_TYPE_OBJECT,
    SMASH_TYPE_PROMISE,
    SMASH_TYPE_FUNCTION
} SmashValueType;

// Forward declarations
typedef struct SmashValue SmashValue;
typedef struct SmashArray SmashArray;

// Dynamic Array Structure
struct SmashArray {
    SmashValue** elements; // Array of pointers to SmashValue
    int size;
    int capacity;
};

// Function type for async callbacks
typedef SmashValue* (*SmashFunction)(SmashValue* this_val, int count, SmashValue** args);

// Unified Value Structure
struct SmashValue {
    SmashValueType type;
    union {
        bool boolean;
        double number;
        char* string;      // Assume heap-allocated
        SmashArray* array;
        SmashObject* object;
        SmashPromise* promise;
        SmashFunction function;
    } data;
};

// --- Value Creation / Management ---
SmashValue* smash_value_create_null();
SmashValue* smash_value_clone(SmashValue* value); // Create a deep copy of a SmashValue
SmashValue* smash_value_create_object();
SmashValue* smash_object_get(SmashValue* obj, const char* property); // Get property from object
void smash_object_set(SmashValue* obj, const char* property, SmashValue* value); // Set property on object
SmashValue* smash_object_get_keys(SmashValue* obj); // Get array of property names
SmashValue* smash_value_create_boolean(bool val);
SmashValue* smash_value_create_number(double num);
SmashValue* smash_value_create_string(const char* str); // Creates a copy
SmashValue* smash_value_create_array(int initial_capacity);
// SmashValue* smash_value_create_object(); // Future use
void smash_value_free(SmashValue* value); // Important: Frees value and potentially nested data
SmashValue* smash_value_logical_not(SmashValue* value); // Logical NOT operation
bool smash_value_is_truthy(SmashValue* value); // Check if a value is truthy

// --- General Helpers ---
char* smash_value_to_string(SmashValue* value); // Converts any value to a new string
void print(int count, ...); // Declaration for print with variable arguments

// --- String helper functions (Keep existing declarations for now, but they might need updates later) ---
char* smash_string_to_upper(const char* str);
char* smash_string_to_lower(const char* str);
char* smash_string_trim(const char* str);
char* smash_string_trim_start(const char* str);
char* smash_string_trim_end(const char* str);
char* smash_string_char_at(const char* str, const char* index_str);
char* smash_string_concat(const char* str1, const char* str2);
char* smash_string_substring(const char* str, int start, int length);
char* smash_string_match(const char* str, const char* pattern);
char* smash_string_includes(const char* str, const char* search_str);
char* smash_string_index_of(const char* str, const char* search_str);
char* smash_string_slice(const char* str, const char* start_str, const char* end_str);

// --- Promise Operations ---
SmashValue* smash_promise_create();
void smash_promise_resolve(SmashValue* promise, SmashValue* value);
void smash_promise_reject(SmashValue* promise, SmashValue* reason);
SmashValue* smash_promise_then(SmashValue* promise, SmashValue* on_fulfilled, SmashValue* on_rejected);
SmashValue* smash_promise_catch(SmashValue* promise, SmashValue* on_rejected);

// --- Fetch API ---
SmashValue* smash_fetch(const char* url, SmashValue* options);
SmashValue* smash_response_json(SmashValue* response);
SmashValue* smash_response_text(SmashValue* response);

// Timer API
SmashValue* smash_set_timeout(SmashValue* callback, unsigned long delay_ms, int argc, SmashValue** args);
SmashValue* smash_sleep(unsigned long ms);

// Function API
SmashValue* smash_value_create_function(SmashFunction func);

char* smash_string_split(const char* str, const char* delimiter);
char* smash_string_repeat(const char* str, const char* count_str);
char* smash_get_length(const char* str); 

// --- Number helper functions (Keep existing declarations) ---
char* smash_number_to_string(const char* num_str);
char* smash_number_to_fixed(const char* num_str, const char* decimals_str);
char* smash_number_to_precision(const char* num_str, const char* precision_str);
char* smash_number_to_exponential(const char* num_str, const char* decimals_str);

// --- Array helper functions --- 
void smash_array_push(SmashValue* array_value, SmashValue* element_value); // New
int smash_array_length(SmashValue* array_value); // New
SmashValue* smash_array_get(SmashValue* array_value, int index); // New

// Existing char* based stubs (mark for deprecation/update)
char* smash_array_map(const char* array_str, const char* callback); 
char* smash_array_filter(const char* array_str, const char* callback);
char* smash_array_pop(const char* array_str);
char* smash_array_for_each(const char* array_str, const char* callback);
char* smash_array_find(const char* array_str, const char* callback);
char* smash_array_join(const char* array_str, const char* separator);
char* smash_array_reverse(const char* array_str);
char* smash_array_slice(const char* array_str, const char* start_str, const char* end_str);

// --- Object helper functions (Keep existing declarations) ---
char* smash_object_has_own_property(const char* object_str, const char* property);
char* smash_object_keys(const char* object_str);
char* smash_object_values(const char* object_str);
char* smash_object_entries(const char* object_str);
char* smash_object_to_string(const char* object_str);

// --- Generic helper functions for common methods (Keep existing declarations) ---
char* smash_to_string(const char* value);
char* smash_value_of(const char* value);
char* smash_slice(const char* value, const char* start_str, const char* end_str);

// --- Regex helper functions (Keep existing declarations) ---
typedef SimpleRegex SmashRegex;

// Regex function declarations
void smash_regex_free(SmashRegex* regex);
SmashRegex* smash_regex_create(const char* pattern, const char* flags);
char* smash_regex_match(SmashRegex* regex, const char* str);
char* smash_regex_replace(SmashRegex* regex, const char* str, const char* replacement);
int load_regex_library(void);
char* smash_string_match(const char* str, const char* pattern);
char* smash_string_replace(const char* str, const char* pattern, const char* replacement);
void smash_free_string(char* str);

#endif // SMASH_RUNTIME_H
