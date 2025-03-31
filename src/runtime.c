#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include "runtime.h"

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
    
    if (index < 0 || index >= len) {
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
    if (end > len) end = len;
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

// Push an element to an array
char* smash_array_push(const char* array_str, const char* element) {
    // This is a simplified implementation
    // In a real implementation, you would parse the array, add the element, and return the new array
    return strdup("[Array with pushed element]");
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
