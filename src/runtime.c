#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include "runtime.h"

// --- Value Creation Implementations ---

SmashValue* smash_value_create_null() {
    SmashValue* value = (SmashValue*)malloc(sizeof(SmashValue));
    value->type = SMASH_TYPE_NULL;
    return value;
}

SmashValue* smash_value_create_boolean(bool val) {
    SmashValue* value = (SmashValue*)malloc(sizeof(SmashValue));
    value->type = SMASH_TYPE_BOOLEAN;
    value->data.boolean = val;
    return value;
}

SmashValue* smash_value_create_number(double num) {
    SmashValue* value = (SmashValue*)malloc(sizeof(SmashValue));
    value->type = SMASH_TYPE_NUMBER;
    value->data.number = num;
    return value;
}

// Creates a heap-allocated copy of the input string
SmashValue* smash_value_create_string(const char* str) {
    SmashValue* value = (SmashValue*)malloc(sizeof(SmashValue));
    value->type = SMASH_TYPE_STRING;
    value->data.string = str ? strdup(str) : strdup(""); // Handle NULL input
    return value;
}

SmashValue* smash_value_create_array(int initial_capacity) {
    SmashValue* value = (SmashValue*)malloc(sizeof(SmashValue));
    value->type = SMASH_TYPE_ARRAY;
    
    SmashArray* array = (SmashArray*)malloc(sizeof(SmashArray));
    array->size = 0;
    array->capacity = initial_capacity > 0 ? initial_capacity : 4; // Default capacity
    array->elements = (SmashValue**)malloc(array->capacity * sizeof(SmashValue*));
    
    value->data.array = array;
    return value;
}

// --- Value Freeing Implementation ---

void smash_value_free(SmashValue* value) {
    if (!value) return;

    switch (value->type) {
        case SMASH_TYPE_STRING:
            free(value->data.string);
            break;
        case SMASH_TYPE_ARRAY: {
            SmashArray* array = value->data.array;
            // Free all elements within the array recursively
            for (int i = 0; i < array->size; i++) {
                smash_value_free(array->elements[i]);
            }
            free(array->elements);
            free(array);
            break;
        }
        case SMASH_TYPE_OBJECT: 
            // TODO: Implement object freeing
            break;
        case SMASH_TYPE_NULL:
        case SMASH_TYPE_UNDEFINED:
        case SMASH_TYPE_BOOLEAN:
        case SMASH_TYPE_NUMBER:
            // No nested data to free for these types
            break;
    }
    free(value); // Free the SmashValue struct itself
}

// --- Array Function Implementations ---

void smash_array_push(SmashValue* array_value, SmashValue* element_value) {
    if (!array_value || array_value->type != SMASH_TYPE_ARRAY) {
        // Handle error: Not an array
        fprintf(stderr, "Error: smash_array_push called on non-array value.\n");
        // Consider freeing element_value if ownership is transferred here?
        return;
    }

    SmashArray* array = array_value->data.array;

    // Resize if necessary
    if (array->size >= array->capacity) {
        array->capacity = array->capacity == 0 ? 4 : array->capacity * 2;
        array->elements = (SmashValue**)realloc(array->elements, array->capacity * sizeof(SmashValue*));
        if (!array->elements) {
            fprintf(stderr, "Error: Failed to reallocate memory for array push.\n");
            // Handle allocation failure (maybe exit?)
            exit(EXIT_FAILURE);
        }
    }

    array->elements[array->size++] = element_value; // Add the element
}

int smash_array_length(SmashValue* array_value) {
    if (!array_value || array_value->type != SMASH_TYPE_ARRAY) {
        // Handle error: Not an array
        fprintf(stderr, "Error: smash_array_length called on non-array value.\n");
        return 0; // Or some error code
    }
    return array_value->data.array->size;
}

SmashValue* smash_array_get(SmashValue* array_value, int index) {
    if (!array_value || array_value->type != SMASH_TYPE_ARRAY) {
        fprintf(stderr, "Error: smash_array_get called on non-array value.\n");
        return smash_value_create_null(); // Return null on error
    }

    SmashArray* array = array_value->data.array;
    if (index < 0 || index >= array->size) {
        fprintf(stderr, "Error: Array index %d out of bounds (size %d).\n", index, array->size);
        return smash_value_create_null(); // Return null for out-of-bounds
    }

    // Note: Returning direct pointer. Caller should not free this.
    // Consider returning a copy if ownership rules are different.
    return array->elements[index]; 
}

// --- General Helper Implementations ---

// Basic implementation to convert SmashValue to a string for printing/debugging
// Returns a new heap-allocated string.
char* smash_value_to_string(SmashValue* value) {
    if (!value) return strdup("undefined"); // Or "(null pointer)"

    char buffer[256]; // Static buffer for simplicity, consider dynamic for large outputs

    switch (value->type) {
        case SMASH_TYPE_NULL:
            return strdup("null");
        case SMASH_TYPE_UNDEFINED:
            return strdup("undefined");
        case SMASH_TYPE_BOOLEAN:
            return strdup(value->data.boolean ? "true" : "false");
        case SMASH_TYPE_NUMBER:
            snprintf(buffer, sizeof(buffer), "%g", value->data.number); // Use %g for clean float/int representation
            return strdup(buffer);
        case SMASH_TYPE_STRING:
            // Return a copy, assuming the caller might modify/free it.
            // Or, if used only for printing, could return value->data.string directly?
            return strdup(value->data.string);
        case SMASH_TYPE_ARRAY: {
            SmashArray* array = value->data.array;
            // Simple representation: [element1,element2,...]
            // This could get very long! Needs careful memory management if dynamic.
            char* result = strdup("[");
            for (int i = 0; i < array->size; i++) {
                char* elem_str = smash_value_to_string(array->elements[i]);
                char* old_result = result;
                // Allocate enough space: current result + element string + ", " + "]\0"
                result = (char*)malloc(strlen(old_result) + strlen(elem_str) + 4);
                strcpy(result, old_result);
                strcat(result, elem_str);
                if (i < array->size - 1) {
                    strcat(result, ",");
                }
                free(old_result);
                free(elem_str);
            }
            char* final_result = (char*)malloc(strlen(result) + 2);
            strcpy(final_result, result);
            strcat(final_result, "]");
            free(result);
            return final_result;
         }
        case SMASH_TYPE_OBJECT:
            return strdup("[object Object]"); // Placeholder
        default:
            return strdup("[unknown type]");
    }
}

// Implementation for the print function with variable arguments
#include <stdarg.h>
#include <string.h>
void print(int count, ...) {
    va_list args;
    va_start(args, count);
    
    for (int i = 0; i < count; i++) {
        SmashValue* value = va_arg(args, SmashValue*);
        char* str = smash_value_to_string(value);
        
        if (str) {
            printf("%s", str);
            free(str); // Free the string returned by smash_value_to_string
        } else {
            printf("(error converting value to string)");
        }
        
        // Print a space between arguments, but not after the last one
        if (i < count - 1) {
            printf(" ");
        }
    }
    
    // Print newline at the end
    printf("\n");
    
    va_end(args);
    // Note: print does not take ownership or free the 'value' arguments.
}

// Forward declarations for regex functions
void smash_regex_free(SmashRegex* regex);
SmashRegex* smash_regex_create(const char* pattern, const char* flags);
char* smash_regex_match(SmashRegex* regex, const char* str);
char* smash_regex_replace(SmashRegex* regex, const char* str, const char* replacement);
int smash_regex_test(SmashRegex* regex, const char* str);
char* smash_string_match(const char* str, const char* pattern);
char* smash_string_replace(const char* str, const char* pattern, const char* replacement);
void smash_free_string(char* str);
int load_regex_library(void);

// Free a regex pattern
void smash_regex_free(SmashRegex* regex) {
    simple_regex_free(regex);
}

// Create a new regex pattern
SmashRegex* smash_regex_create(const char* pattern, const char* flags) {
    return simple_regex_create(pattern, flags);
}

// String helper functions

// Convert a string to uppercase
char* smash_string_to_upper(const char* str) {
    if (str == NULL) return NULL;
    
    size_t len = strlen(str);
    char* result = (char*)malloc(len + 1);
    
    for (size_t i = 0; i < len; i++) {
        result[i] = toupper(str[i]);
    }
    result[len] = '\0';
    
    return result;
}

// Convert a string to lowercase
char* smash_string_to_lower(const char* str) {
    if (str == NULL) return NULL;
    
    size_t len = strlen(str);
    char* result = (char*)malloc(len + 1);
    
    for (size_t i = 0; i < len; i++) {
        result[i] = tolower(str[i]);
    }
    result[len] = '\0';
    
    return result;
}

// Trim whitespace from a string
char* smash_string_trim(const char* str) {
    if (str == NULL) return NULL;
    
    size_t len = strlen(str);
    if (len == 0) return strdup("");
    
    // Find the first non-whitespace character
    size_t start = 0;
    while (start < len && isspace(str[start])) {
        start++;
    }
    
    // Find the last non-whitespace character
    size_t end = len - 1;
    while (end > start && isspace(str[end])) {
        end--;
    }
    
    // Create a new string with the trimmed content
    size_t trimmed_len = end - start + 1;
    char* result = (char*)malloc(trimmed_len + 1);
    strncpy(result, str + start, trimmed_len);
    result[trimmed_len] = '\0';
    
    return result;
}

// Trim whitespace from the start of a string
char* smash_string_trim_start(const char* str) {
    if (str == NULL) return NULL;
    
    size_t len = strlen(str);
    if (len == 0) return strdup("");
    
    // Find the first non-whitespace character
    size_t start = 0;
    while (start < len && isspace(str[start])) {
        start++;
    }
    
    // Create a new string with the trimmed content
    return strdup(str + start);
}

// Trim whitespace from the end of a string
char* smash_string_trim_end(const char* str) {
    if (str == NULL) return NULL;
    
    size_t len = strlen(str);
    if (len == 0) return strdup("");
    
    // Find the last non-whitespace character
    size_t end = len - 1;
    while (end > 0 && isspace(str[end])) {
        end--;
    }
    
    // Create a new string with the trimmed content
    size_t trimmed_len = end + 1;
    char* result = (char*)malloc(trimmed_len + 1);
    strncpy(result, str, trimmed_len);
    result[trimmed_len] = '\0';
    
    return result;
}

// Get a character at a specific index
char* smash_string_char_at(const char* str, const char* index_str) {
    if (str == NULL || index_str == NULL) return strdup("");
    
    int index = atoi(index_str);
    size_t len = strlen(str);
    
    if (index < 0 || (size_t)index >= len) {
        return strdup("");
    }
    
    char* result = (char*)malloc(2);
    result[0] = str[index];
    result[1] = '\0';
    
    return result;
}

// Concatenate two strings
char* smash_string_concat(const char* str1, const char* str2) {
    if (str1 == NULL) str1 = "";
    if (str2 == NULL) str2 = "";
    
    size_t len1 = strlen(str1);
    size_t len2 = strlen(str2);
    
    char* result = (char*)malloc(len1 + len2 + 1);
    strcpy(result, str1);
    strcat(result, str2);
    
    return result;
}

// Check if a string includes another string
char* smash_string_includes(const char* str, const char* search_str) {
    if (str == NULL || search_str == NULL) {
        return strdup("false");
    }
    
    if (strstr(str, search_str) != NULL) {
        return strdup("true");
    } else {
        return strdup("false");
    }
}

// Find the index of a substring
char* smash_string_index_of(const char* str, const char* search_str) {
    if (str == NULL || search_str == NULL) {
        return strdup("-1");
    }
    
    const char* found = strstr(str, search_str);
    if (found != NULL) {
        char result[32];
        sprintf(result, "%ld", found - str);
        return strdup(result);
    } else {
        return strdup("-1");
    }
}

// Extract a portion of a string
char* smash_string_slice(const char* str, const char* start_str, const char* end_str) {
    if (str == NULL) return strdup("");
    
    int start = atoi(start_str);
    int end = atoi(end_str);
    size_t len = strlen(str);
    
    // Adjust indices if they're out of bounds
    if (start < 0) start = 0;
    if ((size_t)end > len) end = len;
    if (start >= end) return strdup("");
    
    // Extract the substring
    size_t slice_len = end - start;
    char* result = (char*)malloc(slice_len + 1);
    strncpy(result, str + start, slice_len);
    result[slice_len] = '\0';
    
    return result;
}

// Split a string by a delimiter
char* smash_string_split(const char* str, const char* delimiter) {
    if (str == NULL || delimiter == NULL) {
        return strdup("[]");
    }
    
    // This is a simplified implementation that returns a string representation of an array
    // In a real implementation, you would return an actual array
    char* result = strdup("[]");
    // TODO: Implement a proper split function
    
    return result;
}

// Repeat a string multiple times
char* smash_string_repeat(const char* str, const char* count_str) {
    if (str == NULL || count_str == NULL) {
        return strdup("");
    }
    
    int count = atoi(count_str);
    if (count <= 0) return strdup("");
    
    size_t len = strlen(str);
    char* result = (char*)malloc(len * count + 1);
    result[0] = '\0';
    
    for (int i = 0; i < count; i++) {
        strcat(result, str);
    }
    
    return result;
}

// Get the length of a string
char* smash_get_length(const char* str) {
    if (str == NULL) return strdup("0");
    
    char result[32];
    sprintf(result, "%zu", strlen(str));
    return strdup(result);
}

// Number helper functions

// Convert a number to a string
char* smash_number_to_string(const char* num_str) {
    return strdup(num_str);
}

// Format a number with fixed decimal places
char* smash_number_to_fixed(const char* num_str, const char* decimals_str) {
    if (num_str == NULL || decimals_str == NULL) {
        return strdup("0");
    }
    
    double num = atof(num_str);
    int decimals = atoi(decimals_str);
    
    if (decimals < 0) decimals = 0;
    if (decimals > 20) decimals = 20;
    
    char format[16];
    sprintf(format, "%%.%df", decimals);
    
    char result[64];
    sprintf(result, format, num);
    
    return strdup(result);
}

// Format a number with specified precision
char* smash_number_to_precision(const char* num_str, const char* precision_str) {
    if (num_str == NULL || precision_str == NULL) {
        return strdup("0");
    }
    
    double num = atof(num_str);
    int precision = atoi(precision_str);
    
    if (precision < 1) precision = 1;
    if (precision > 21) precision = 21;
    
    char format[16];
    sprintf(format, "%%.%dg", precision);
    
    char result[64];
    sprintf(result, format, num);
    
    return strdup(result);
}

// Format a number in exponential notation
char* smash_number_to_exponential(const char* num_str, const char* decimals_str) {
    if (num_str == NULL || decimals_str == NULL) {
        return strdup("0");
    }
    
    double num = atof(num_str);
    int decimals = atoi(decimals_str);
    
    if (decimals < 0) decimals = 0;
    if (decimals > 20) decimals = 20;
    
    char format[16];
    sprintf(format, "%%.%de", decimals);
    
    char result[64];
    sprintf(result, format, num);
    
    return strdup(result);
}

// Array helper functions

// Map function for arrays
char* smash_array_map(const char* array_str, const char* callback) {
    // This is a simplified implementation
    // In a real implementation, you would parse the array, apply the callback to each element,
    // and return a new array
    return strdup("[Mapped array]");
}

// Filter function for arrays
char* smash_array_filter(const char* array_str, const char* callback) {
    // This is a simplified implementation
    // In a real implementation, you would parse the array, filter elements using the callback,
    // and return a new array
    return strdup("[Filtered array]");
}

// Pop an element from an array
char* smash_array_pop(const char* array_str) {
    // This is a simplified implementation
    // In a real implementation, you would parse the array, remove the last element, and return it
    return strdup("Popped element");
}

// ForEach function for arrays
char* smash_array_for_each(const char* array_str, const char* callback) {
    // This is a simplified implementation
    // In a real implementation, you would parse the array and apply the callback to each element
    return strdup("undefined");
}

// Find function for arrays
char* smash_array_find(const char* array_str, const char* callback) {
    // This is a simplified implementation
    // In a real implementation, you would parse the array, find an element using the callback,
    // and return it
    return strdup("Found element");
}

// Join function for arrays
char* smash_array_join(const char* array_str, const char* separator) {
    // This is a simplified implementation
    // In a real implementation, you would parse the array, join the elements with the separator,
    // and return the resulting string
    return strdup("Joined array");
}

// Reverse function for arrays
char* smash_array_reverse(const char* array_str) {
    // This is a simplified implementation
    // In a real implementation, you would parse the array, reverse the elements, and return the new array
    return strdup("[Reversed array]");
}

// Slice function for arrays
char* smash_array_slice(const char* array_str, const char* start_str, const char* end_str) {
    // This is a simplified implementation
    // In a real implementation, you would parse the array, extract the specified slice,
    // and return the new array
    return strdup("[Sliced array]");
}

// Object helper functions

// Check if an object has a property
char* smash_object_has_own_property(const char* object_str, const char* property) {
    // This is a simplified implementation
    // In a real implementation, you would parse the object and check if it has the property
    return strdup("true");
}

// Get the keys of an object
char* smash_object_keys(const char* object_str) {
    // This is a simplified implementation
    // In a real implementation, you would parse the object and return an array of its keys
    return strdup("[Object keys]");
}

// Get the values of an object
char* smash_object_values(const char* object_str) {
    // This is a simplified implementation
    // In a real implementation, you would parse the object and return an array of its values
    return strdup("[Object values]");
}

// Get the entries of an object
char* smash_object_entries(const char* object_str) {
    // This is a simplified implementation
    // In a real implementation, you would parse the object and return an array of its entries
    return strdup("[Object entries]");
}

// Convert an object to a string
char* smash_object_to_string(const char* object_str) {
    // This is a simplified implementation
    // In a real implementation, you would parse the object and convert it to a string representation
    return strdup("[Object]");
}

// Generic helper functions for common methods

// Generic toString method that works for any type
char* smash_to_string(const char* value) {
    if (value == NULL) return strdup("undefined");
    
    // Try to determine the type of the value and call the appropriate toString method
    // This is a simplified implementation
    if (value[0] == '[' && value[1] == 'O' && value[2] == 'b') {
        // Looks like an object
        return smash_object_to_string(value);
    } else if (value[0] == '[' && value[1] == 'A' && value[2] == 'r') {
        // Looks like an array
        return strdup(value); // Just return the array representation for now
    } else if ((value[0] >= '0' && value[0] <= '9') || value[0] == '-' || value[0] == '+') {
        // Looks like a number
        return smash_number_to_string(value);
    } else {
        // Assume it's a string or other type, just return it
        return strdup(value);
    }
}

// Generic valueOf method that works for any type
char* smash_value_of(const char* value) {
    if (value == NULL) return strdup("undefined");
    
    // Similar to toString, but returns the primitive value if possible
    // This is a simplified implementation
    return strdup(value);
}

// Generic slice method that works for strings and arrays
char* smash_slice(const char* value, const char* start_str, const char* end_str) {
    if (value == NULL) return strdup("");
    
    // Try to determine if this is a string or an array
    if (value[0] == '[' && value[1] == 'A' && value[2] == 'r') {
        // Looks like an array
        return smash_array_slice(value, start_str, end_str);
    } else {
        // Assume it's a string
        return smash_string_slice(value, start_str, end_str);
    }
}

// Match a regex pattern against a string
char* smash_regex_match(SmashRegex* regex, const char* str) {
    // Use our simple regex implementation
    return simple_regex_match(regex, str);
}

// Replace matches in a string with a replacement string
char* smash_regex_replace(SmashRegex* regex, const char* str, const char* replacement) {
    // Use our simple regex implementation
    return simple_regex_replace(regex, str, replacement);
}

// Load regex library
int load_regex_library(void) {
    // No need to load external library, using embedded implementation
    return 1;  // Always succeeds with embedded implementation
}

// Create a deep copy of a SmashValue
SmashValue* smash_value_clone(SmashValue* value) {
    if (!value) {
        return smash_value_create_null();
    }
    
    switch (value->type) {
        case SMASH_TYPE_NULL:
            return smash_value_create_null();
            
        case SMASH_TYPE_BOOLEAN:
            return smash_value_create_boolean(value->data.boolean);
            
        case SMASH_TYPE_NUMBER:
            return smash_value_create_number(value->data.number);
            
        case SMASH_TYPE_STRING:
            return smash_value_create_string(value->data.string);
            
        case SMASH_TYPE_ARRAY:
            {
                // Create a new array
                SmashValue* new_array = smash_value_create_array(value->data.array->size);
                if (!new_array) return NULL;
                
                // Clone each element
                for (int i = 0; i < value->data.array->size; i++) {
                    SmashValue* element_clone = smash_value_clone(value->data.array->elements[i]);
                    if (!element_clone) {
                        // Handle error: free the array we've created so far
                        smash_value_free(new_array);
                        return NULL;
                    }
                    
                    // Add the cloned element to the new array
                    new_array->data.array->elements[i] = element_clone;
                }
                
                return new_array;
            }
            
        case SMASH_TYPE_OBJECT:
            {
                // For now, just return a simple null value for objects
                // In a complete implementation, we would clone all properties
                return smash_value_create_null();
            }
            
        default:
            return smash_value_create_null();
    }
}

// Implementation for object creation and property access
SmashValue* smash_value_create_object() {
    SmashValue* obj = (SmashValue*)malloc(sizeof(SmashValue));
    if (!obj) {
        return NULL;
    }
    
    obj->type = SMASH_TYPE_OBJECT;
    obj->data.object = (SmashObject*)malloc(sizeof(SmashObject));
    if (!obj->data.object) {
        free(obj);
        return NULL;
    }
    
    obj->data.object->properties = NULL;  // Start with no properties
    obj->data.object->size = 0;
    
    return obj;
}

SmashValue* smash_object_get(SmashValue* obj, const char* property) {
    if (!obj || obj->type != SMASH_TYPE_OBJECT) {
        // Return null for non-object values
        return smash_value_create_null();
    }
    
    SmashObject* object = obj->data.object;
    
    // Search for the property in the object
    for (int i = 0; i < object->size; i++) {
        if (strcmp(object->properties[i].key, property) == 0) {
            // Return a copy of the found property value
            return smash_value_clone(object->properties[i].value);
        }
    }
    
    // Property not found, return null
    return smash_value_create_null();
}

// Logical NOT operation
SmashValue* smash_value_logical_not(SmashValue* value) {
    if (!value) {
        return smash_value_create_boolean(true); // !null is true
    }
    
    // Convert value to boolean and negate it
    bool is_truthy = smash_value_is_truthy(value);
    return smash_value_create_boolean(!is_truthy);
}

// Helper function to determine if a value is truthy
bool smash_value_is_truthy(SmashValue* value) {
    if (!value) {
        return false;
    }
    
    switch (value->type) {
        case SMASH_TYPE_NULL:
            return false;
            
        case SMASH_TYPE_BOOLEAN:
            return value->data.boolean;
            
        case SMASH_TYPE_NUMBER:
            return value->data.number != 0;
            
        case SMASH_TYPE_STRING:
            return value->data.string && value->data.string[0] != '\0';
            
        case SMASH_TYPE_ARRAY:
            return value->data.array && value->data.array->size > 0;
            
        case SMASH_TYPE_OBJECT:
            return true; // Objects are always truthy
            
        default:
            return false;
    }
}

// Set a property on an object
void smash_object_set(SmashValue* obj, const char* property, SmashValue* value) {
    if (!obj || obj->type != SMASH_TYPE_OBJECT || !property || !value) {
        return; // Invalid input
    }

    // Create a copy of the value to store in the object
    SmashValue* value_copy = smash_value_clone(value);
    if (!value_copy) {
        return; // Failed to clone value
    }
    
    // Get the object structure
    SmashObject* object = obj->data.object;
    
    // Check if property already exists
    for (int i = 0; i < object->size; i++) {
        if (strcmp(object->properties[i].key, property) == 0) {
            // Free the old value
            smash_value_free(object->properties[i].value);
            // Set the new value (use the clone we already created)
            object->properties[i].value = value_copy;
            return;
        }
    }
    
    // Property doesn't exist, add it
    int new_size = object->size + 1;
    object->properties = realloc(object->properties, new_size * sizeof(SmashProperty));
    if (!object->properties) {
        smash_value_free(value_copy);
        return; // Memory allocation failed
    }
    
    // Set the new property
    object->properties[object->size].key = strdup(property);
    object->properties[object->size].value = value_copy;
    object->size = new_size;
}

// Get all property names from an object as an array
SmashValue* smash_object_get_keys(SmashValue* obj) {
    if (!obj || obj->type != SMASH_TYPE_OBJECT) {
        return smash_value_create_array(0); // Return empty array for non-objects
    }

    SmashObject* object = obj->data.object;
    int key_count = object->size;
    
    // Create an array to hold all keys
    SmashValue* keys_array = smash_value_create_array(key_count);
    if (!keys_array) {
        return smash_value_create_array(0); // Return empty array on error
    }
    
    // Add all keys to the array
    for (int i = 0; i < object->size; i++) {
        SmashValue* key = smash_value_create_string(object->properties[i].key);
        if (key) {
            smash_array_push(keys_array, key);
        }
    }
    
    return keys_array;
}

// --- Promise Implementation ---

// Create a new Promise
SmashValue* smash_promise_create() {
    SmashValue* promise_value = (SmashValue*)malloc(sizeof(SmashValue));
    if (!promise_value) {
        return NULL; // Memory allocation failed
    }
    
    SmashPromise* promise = (SmashPromise*)malloc(sizeof(SmashPromise));
    if (!promise) {
        free(promise_value);
        return NULL; // Memory allocation failed
    }
    
    // Initialize the promise
    promise->status = PROMISE_PENDING;
    promise->result = NULL;
    promise->on_fulfill = NULL;
    promise->on_reject = NULL;
    promise->callback_data = NULL;
    
    // Set up the SmashValue
    promise_value->type = SMASH_TYPE_PROMISE;
    promise_value->data.promise = promise;
    
    return promise_value;
}

// Resolve a promise with a value
void smash_promise_resolve(SmashValue* promise_value, SmashValue* value) {
    if (!promise_value || promise_value->type != SMASH_TYPE_PROMISE) {
        return; // Not a promise
    }
    
    SmashPromise* promise = promise_value->data.promise;
    if (promise->status != PROMISE_PENDING) {
        return; // Promise already settled
    }
    
    // Update promise status and result
    promise->status = PROMISE_FULFILLED;
    promise->result = smash_value_clone(value);
    
    // Execute the onFulfill callback if set
    if (promise->on_fulfill) {
        promise->on_fulfill(promise->result, promise->callback_data);
    }
}

// Reject a promise with a reason
void smash_promise_reject(SmashValue* promise_value, SmashValue* reason) {
    if (!promise_value || promise_value->type != SMASH_TYPE_PROMISE) {
        return; // Not a promise
    }
    
    SmashPromise* promise = promise_value->data.promise;
    if (promise->status != PROMISE_PENDING) {
        return; // Promise already settled
    }
    
    // Update promise status and reason
    promise->status = PROMISE_REJECTED;
    promise->result = smash_value_clone(reason);
    
    // Execute the onReject callback if set
    if (promise->on_reject) {
        promise->on_reject(promise->result, promise->callback_data);
    }
}

// Helper struct for promise chaining
typedef struct {
    SmashValue* next_promise;
    SmashValue* handler;
} PromiseChainData;

// Callback for promise chaining (fulfill)
void promise_chain_fulfill(SmashValue* value, void* data) {
    PromiseChainData* chain_data = (PromiseChainData*)data;
    if (!chain_data) return;
    
    SmashValue* next_promise = chain_data->next_promise;
    SmashValue* on_fulfilled = chain_data->handler;
    
    if (on_fulfilled && on_fulfilled->type == SMASH_TYPE_FUNCTION) {
        // Call the handler function with the value
        SmashValue* args[1] = { value };
        SmashValue* result = on_fulfilled->data.function(NULL, 1, args);
        
        // Resolve the next promise with the result
        smash_promise_resolve(next_promise, result);
        smash_value_free(result);
    } else {
        // No handler, just pass through the value
        smash_promise_resolve(next_promise, value);
    }
    
    // Clean up
    free(chain_data);
}

// Callback for promise chaining (reject)
void promise_chain_reject(SmashValue* reason, void* data) {
    PromiseChainData* chain_data = (PromiseChainData*)data;
    if (!chain_data) return;
    
    SmashValue* next_promise = chain_data->next_promise;
    SmashValue* on_rejected = chain_data->handler;
    
    if (on_rejected && on_rejected->type == SMASH_TYPE_FUNCTION) {
        // Call the handler function with the reason
        SmashValue* args[1] = { reason };
        SmashValue* result = on_rejected->data.function(NULL, 1, args);
        
        // Resolve the next promise with the result
        smash_promise_resolve(next_promise, result);
        smash_value_free(result);
    } else {
        // No handler, propagate the rejection
        smash_promise_reject(next_promise, reason);
    }
    
    // Clean up
    free(chain_data);
}

// Chain promises with then
SmashValue* smash_promise_then(SmashValue* promise_value, SmashValue* on_fulfilled, SmashValue* on_rejected) {
    if (!promise_value || promise_value->type != SMASH_TYPE_PROMISE) {
        return smash_promise_create(); // Return a new rejected promise?
    }
    
    SmashPromise* promise = promise_value->data.promise;
    SmashValue* next_promise = smash_promise_create();
    
    if (promise->status == PROMISE_PENDING) {
        // Set up callbacks for when the promise resolves/rejects
        PromiseChainData* fulfill_data = (PromiseChainData*)malloc(sizeof(PromiseChainData));
        fulfill_data->next_promise = next_promise;
        fulfill_data->handler = on_fulfilled;
        
        PromiseChainData* reject_data = (PromiseChainData*)malloc(sizeof(PromiseChainData));
        reject_data->next_promise = next_promise;
        reject_data->handler = on_rejected;
        
        promise->on_fulfill = promise_chain_fulfill;
        promise->on_reject = promise_chain_reject;
        promise->callback_data = fulfill_data; // This is a simplification, we'd need separate data for each
    } else if (promise->status == PROMISE_FULFILLED) {
        // Promise already fulfilled, call handler immediately
        if (on_fulfilled && on_fulfilled->type == SMASH_TYPE_FUNCTION) {
            SmashValue* args[1] = { promise->result };
            SmashValue* result = on_fulfilled->data.function(NULL, 1, args);
            smash_promise_resolve(next_promise, result);
            smash_value_free(result);
        } else {
            smash_promise_resolve(next_promise, promise->result);
        }
    } else if (promise->status == PROMISE_REJECTED) {
        // Promise already rejected, call handler immediately
        if (on_rejected && on_rejected->type == SMASH_TYPE_FUNCTION) {
            SmashValue* args[1] = { promise->result };
            SmashValue* result = on_rejected->data.function(NULL, 1, args);
            smash_promise_resolve(next_promise, result);
            smash_value_free(result);
        } else {
            smash_promise_reject(next_promise, promise->result);
        }
    }
    
    return next_promise;
}

// Add catch handler to a promise
SmashValue* smash_promise_catch(SmashValue* promise_value, SmashValue* on_rejected) {
    return smash_promise_then(promise_value, NULL, on_rejected);
}

// --- Fetch API Implementation ---

// Struct to hold HTTP response data
typedef struct {
    char* body;
    int status_code;
    char* status_text;
    SmashValue* headers;  // Object containing response headers
} HttpResponse;

// Free an HTTP response
void free_http_response(HttpResponse* response) {
    if (!response) return;
    
    if (response->body) free(response->body);
    if (response->status_text) free(response->status_text);
    if (response->headers) smash_value_free(response->headers);
    free(response);
}

// Create a Response object from an HttpResponse
SmashValue* create_response_object(HttpResponse* http_response) {
    SmashValue* response = smash_value_create_object();
    
    // Add status code
    SmashValue* status = smash_value_create_number(http_response->status_code);
    smash_object_set(response, "status", status);
    smash_value_free(status);
    
    // Add status text
    SmashValue* status_text = smash_value_create_string(http_response->status_text);
    smash_object_set(response, "statusText", status_text);
    smash_value_free(status_text);
    
    // Add headers
    smash_object_set(response, "headers", http_response->headers);
    
    // Add body (as a property, not directly accessible)
    SmashValue* body_str = smash_value_create_string(http_response->body);
    smash_object_set(response, "_body", body_str);
    smash_value_free(body_str);
    
    // Add methods as function pointers (simplified for now)
    // In a real implementation, we'd need to set up proper function objects
    
    return response;
}

// Helper function to perform HTTP request (simplified mock implementation)
HttpResponse* perform_http_request(const char* url, const char* method, const char* body, SmashValue* headers) {
    // This is a mock implementation that would be replaced with a real HTTP client
    // In a real implementation, we'd use libcurl or another HTTP client library
    
    HttpResponse* response = (HttpResponse*)malloc(sizeof(HttpResponse));
    if (!response) return NULL;
    
    // For demonstration, return a mock successful response
    response->status_code = 200;
    response->status_text = strdup("OK");
    
    // Create mock response body - in a real implementation this would come from the HTTP request
    if (strstr(url, "example.com")) {
        response->body = strdup("{\"message\": \"Hello from the API\", \"success\": true}");
    } else {
        response->body = strdup("{\"error\": \"Not found\", \"success\": false}");
    }
    
    // Create headers object
    response->headers = smash_value_create_object();
    SmashValue* content_type = smash_value_create_string("application/json");
    smash_object_set(response->headers, "Content-Type", content_type);
    smash_value_free(content_type);
    
    return response;
}

// Callback data for fetch
typedef struct {
    SmashValue* promise;
    char* url;
    char* method;
    char* body;
    SmashValue* headers;
} FetchData;

// Thread function for async fetch (simplified)
void* fetch_thread_func(void* arg) {
    FetchData* data = (FetchData*)arg;
    if (!data) return NULL;
    
    // Perform the HTTP request
    HttpResponse* http_response = perform_http_request(data->url, data->method, data->body, data->headers);
    
    if (http_response) {
        // Create response object
        SmashValue* response = create_response_object(http_response);
        
        // Resolve the promise with the response
        smash_promise_resolve(data->promise, response);
        
        // Clean up
        smash_value_free(response);
        free_http_response(http_response);
    } else {
        // Create error object
        SmashValue* error = smash_value_create_object();
        SmashValue* message = smash_value_create_string("Network error");
        smash_object_set(error, "message", message);
        smash_value_free(message);
        
        // Reject the promise with the error
        smash_promise_reject(data->promise, error);
        
        // Clean up
        smash_value_free(error);
    }
    
    // Clean up fetch data
    free(data->url);
    if (data->method) free(data->method);
    if (data->body) free(data->body);
    if (data->headers) smash_value_free(data->headers);
    free(data);
    
    return NULL;
}

// Fetch API implementation
SmashValue* smash_fetch(const char* url, SmashValue* options) {
    // Create a promise to return
    SmashValue* promise = smash_promise_create();
    
    // Prepare fetch data
    FetchData* data = (FetchData*)malloc(sizeof(FetchData));
    if (!data) {
        SmashValue* error = smash_value_create_object();
        SmashValue* message = smash_value_create_string("Memory allocation failed");
        smash_object_set(error, "message", message);
        smash_value_free(message);
        
        smash_promise_reject(promise, error);
        smash_value_free(error);
        return promise;
    }
    
    data->promise = promise;
    data->url = strdup(url);
    data->method = NULL;
    data->body = NULL;
    data->headers = NULL;
    
    // Parse options if provided
    if (options && options->type == SMASH_TYPE_OBJECT) {
        // Get method
        SmashValue* method = smash_object_get(options, "method");
        if (method && method->type == SMASH_TYPE_STRING) {
            data->method = strdup(method->data.string);
        } else {
            data->method = strdup("GET");  // Default to GET
        }
        
        // Get body
        SmashValue* body = smash_object_get(options, "body");
        if (body) {
            if (body->type == SMASH_TYPE_STRING) {
                data->body = strdup(body->data.string);
            } else {
                // Convert body to JSON string if it's an object
                // This would require a JSON stringify function
                // For now, we'll just use a placeholder
                data->body = strdup("{}");
            }
        }
        
        // Get headers
        SmashValue* headers = smash_object_get(options, "headers");
        if (headers && headers->type == SMASH_TYPE_OBJECT) {
            data->headers = smash_value_clone(headers);
        }
    } else {
        // Default to GET method
        data->method = strdup("GET");
    }
    
    // In a real implementation, we would spawn a thread to perform the fetch
    // For simplicity, we'll just call the function directly
    fetch_thread_func(data);
    
    return promise;
}

// Parse JSON from response
SmashValue* smash_response_json(SmashValue* response) {
    if (!response || response->type != SMASH_TYPE_OBJECT) {
        return smash_value_create_null();
    }
    
    // Get the body from the response
    SmashValue* body = smash_object_get(response, "_body");
    if (!body || body->type != SMASH_TYPE_STRING) {
        return smash_value_create_null();
    }
    
    // In a real implementation, we would parse the JSON string
    // For simplicity, we'll create a mock object based on the body content
    SmashValue* json = smash_value_create_object();
    
    // Check if the body contains success: true
    if (strstr(body->data.string, "\"success\": true")) {
        SmashValue* success = smash_value_create_boolean(true);
        smash_object_set(json, "success", success);
        smash_value_free(success);
        
        // Extract message if present
        if (strstr(body->data.string, "\"message\"")) {
            SmashValue* message = smash_value_create_string("Hello from the API");
            smash_object_set(json, "message", message);
            smash_value_free(message);
        }
    } else {
        SmashValue* success = smash_value_create_boolean(false);
        smash_object_set(json, "success", success);
        smash_value_free(success);
        
        // Extract error if present
        if (strstr(body->data.string, "\"error\"")) {
            SmashValue* error = smash_value_create_string("Not found");
            smash_object_set(json, "error", error);
            smash_value_free(error);
        }
    }
    
    return json;
}

// Get text from response
SmashValue* smash_response_text(SmashValue* response) {
    if (!response || response->type != SMASH_TYPE_OBJECT) {
        return smash_value_create_string("");
    }
    
    // Get the body from the response
    SmashValue* body = smash_object_get(response, "_body");
    if (!body || body->type != SMASH_TYPE_STRING) {
        return smash_value_create_string("");
    }
    
    // Return a copy of the body string
    return smash_value_create_string(body->data.string);
}

// --- Timer Implementation ---
#include <pthread.h>
#include <time.h>

// Struct to hold timer callback data
typedef struct {
    SmashValue* promise;
    SmashValue* callback;
    SmashValue** args;  // Fixed: Changed from SmashValue* to SmashValue**
    int num_args;
    unsigned long delay_ms;
} TimerData;

// Promise resolver function for setTimeout
SmashValue* promise_resolver(SmashValue* this_val, int argc, SmashValue** args) {
    // This function is called when the timer expires
    // It resolves the promise with a value (or null if no value provided)
    if (this_val && this_val->type == SMASH_TYPE_OBJECT) {
        SmashValue* promise = smash_object_get(this_val, "promise");
        if (promise && promise->type == SMASH_TYPE_PROMISE) {
            // Resolve the promise with the first argument or null
            if (argc > 0 && args && args[0]) {
                smash_promise_resolve(promise, args[0]);
            } else {
                SmashValue* null_val = smash_value_create_null();
                smash_promise_resolve(promise, null_val);
                smash_value_free(null_val);
            }
        }
    }
    return smash_value_create_null();
}

// Thread function for setTimeout
void* timer_thread_func(void* arg) {
    TimerData* data = (TimerData*)arg;
    if (!data) return NULL;
    
    // Sleep for the specified delay
    struct timespec ts;
    ts.tv_sec = data->delay_ms / 1000;
    ts.tv_nsec = (data->delay_ms % 1000) * 1000000;
    nanosleep(&ts, NULL);
    
    // If we have a callback, call it
    if (data->callback && data->callback->type == SMASH_TYPE_FUNCTION) {
        // Create a value to pass to the callback
        SmashValue* value = smash_value_create_number(data->delay_ms);
        
        // Create an array to hold the arguments
        SmashValue** args = NULL;
        int argc = 0;
        
        if (data->args && data->num_args > 0) {
            // Use the provided arguments
            args = data->args;
            argc = data->num_args;
        } else {
            // Use the delay value as the argument
            args = &value;
            argc = 1;
        }
        
        // Call the function with the arguments
        SmashValue* result = data->callback->data.function(NULL, argc, args);
        
        // If we have a promise, resolve it with the result
        if (data->promise) {
            smash_promise_resolve(data->promise, result);
        }
        
        // Clean up
        smash_value_free(result);
        smash_value_free(value);
    } else if (data->promise) {
        // If we don't have a callback but we do have a promise, resolve it with null
        SmashValue* null_value = smash_value_create_null();
        smash_promise_resolve(data->promise, null_value);
        smash_value_free(null_value);
    }
    
    // Clean up
    if (data->callback) smash_value_free(data->callback);
    if (data->args) {
        for (int i = 0; i < data->num_args; i++) {
            smash_value_free(data->args[i]);
        }
        free(data->args);
    }
    free(data);
    
    return NULL;
}

// setTimeout implementation
SmashValue* smash_set_timeout(SmashValue* callback, unsigned long delay_ms, int argc, SmashValue** args) {
    // Create a promise to return
    SmashValue* promise = smash_promise_create();
    
    // Prepare timer data
    TimerData* data = (TimerData*)malloc(sizeof(TimerData));
    if (!data) {
        SmashValue* error = smash_value_create_object();
        SmashValue* message = smash_value_create_string("Memory allocation failed");
        smash_object_set(error, "message", message);
        smash_value_free(message);
        
        smash_promise_reject(promise, error);
        smash_value_free(error);
        return promise;
    }
    
    data->promise = promise;
    data->callback = callback ? smash_value_clone(callback) : NULL;
    data->delay_ms = delay_ms;
    
    // Copy arguments
    if (argc > 0 && args) {
        data->args = (SmashValue**)malloc(sizeof(SmashValue*) * argc);
        if (!data->args) {
            free(data);
            SmashValue* error = smash_value_create_object();
            SmashValue* message = smash_value_create_string("Memory allocation failed");
            smash_object_set(error, "message", message);
            smash_value_free(message);
            
            smash_promise_reject(promise, error);
            smash_value_free(error);
            return promise;
        }
        
        for (int i = 0; i < argc; i++) {
            data->args[i] = smash_value_clone(args[i]);
        }
        data->num_args = argc;
    } else {
        data->args = NULL;
        data->num_args = 0;
    }
    
    // Create a thread to handle the timer
    pthread_t thread;
    if (pthread_create(&thread, NULL, timer_thread_func, data) != 0) {
        // Failed to create thread
        if (data->callback) smash_value_free(data->callback);
        if (data->args) {
            for (int i = 0; i < data->num_args; i++) {
                smash_value_free(data->args[i]);
            }
            free(data->args);
        }
        free(data);
        
        SmashValue* error = smash_value_create_object();
        SmashValue* message = smash_value_create_string("Failed to create timer thread");
        smash_object_set(error, "message", message);
        smash_value_free(message);
        
        smash_promise_reject(promise, error);
        smash_value_free(error);
        return promise;
    }
    
    // Detach the thread so it can clean up itself
    pthread_detach(thread);
    
    return promise;
}

// Function to create a SmashValue that contains a function
SmashValue* smash_value_create_function(SmashFunction func) {
    SmashValue* value = malloc(sizeof(SmashValue));
    value->type = SMASH_TYPE_FUNCTION;
    value->data.function = func;
    return value;
}

// Sleep function implementation using setTimeout with Promise
SmashValue* smash_sleep(unsigned long ms) {
    // Create a new Promise
    SmashValue* promise = smash_promise_create();
    
    // Create a context object to hold the promise
    SmashValue* context = smash_value_create_object();
    smash_object_set(context, "promise", promise);
    
    // Create a function value for the resolver callback
    SmashValue* resolver = smash_value_create_function(promise_resolver);
    
    // Use setTimeout to delay the resolution
    smash_set_timeout(resolver, ms, 0, NULL);
    
    // Clean up
    smash_value_free(resolver);
    
    return promise;
}

// Match a string against a pattern
char* smash_string_match(const char* str, const char* pattern) {
    if (!str || !pattern) return NULL;
    
    // Extract pattern and flags
    char* pattern_copy = strdup(pattern);
    if (!pattern_copy) return NULL;
    
    char* flags = "";
    char* slash_pos = strrchr(pattern_copy, '/');
    if (slash_pos && slash_pos > pattern_copy) {
        *slash_pos = '\0';  // Split the string at the slash
        flags = slash_pos + 1;  // Flags start after the slash
    }
    
    // Create a regex object with the pattern and flags
    SmashRegex* regex = smash_regex_create(pattern_copy, flags);
    if (!regex) {
        free(pattern_copy);
        return NULL;
    }
    
    // Perform the match
    char* result = smash_regex_match(regex, str);
    
    // Free the regex object and pattern copy
    smash_regex_free(regex);
    free(pattern_copy);
    
    return result;
}

// Replace matches in a string with a replacement string
char* smash_string_replace(const char* str, const char* pattern, const char* replacement) {
    if (!str || !pattern || !replacement) return NULL;
    
    // Extract pattern and flags
    char* pattern_copy = strdup(pattern);
    if (!pattern_copy) return NULL;
    
    char* flags = "g";  // Default to global replacement
    char* slash_pos = strrchr(pattern_copy, '/');
    if (slash_pos && slash_pos > pattern_copy) {
        *slash_pos = '\0';  // Split the string at the slash
        flags = slash_pos + 1;  // Flags start after the slash
        
        // Make sure 'g' is included in flags for replacements
        if (!strchr(flags, 'g')) {
            char* new_flags = malloc(strlen(flags) + 2);
            if (new_flags) {
                strcpy(new_flags, flags);
                strcat(new_flags, "g");
                flags = new_flags;
            }
        }
    }
    
    // Create a regex object with the pattern and flags
    SmashRegex* regex = smash_regex_create(pattern_copy, flags);
    if (!regex) {
        free(pattern_copy);
        if (strcmp(flags, "g") != 0 && strchr(flags, 'g')) {
            free((void*)flags);
        }
        return NULL;
    }
    
    // Perform the replacement
    char* result = smash_regex_replace(regex, str, replacement);
    
    // Free the regex object and pattern copy
    smash_regex_free(regex);
    free(pattern_copy);
    if (strcmp(flags, "g") != 0 && strchr(flags, 'g')) {
        free((void*)flags);
    }
    
    return result;
}
