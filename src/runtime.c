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

// Implementation for the print function
void print(SmashValue* value) {
    char* str = smash_value_to_string(value);
    if (str) {
        printf("%s\n", str);
        free(str); // Free the string returned by smash_value_to_string
    } else {
        printf("(error converting value to string)\n");
    }
    // Note: print does not take ownership or free the 'value' argument itself.
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
