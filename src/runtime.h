#ifndef SMASH_RUNTIME_H
#define SMASH_RUNTIME_H

#include <stdbool.h>
#include "simple_regex.h"

// String helper functions
char* smash_string_to_upper(const char* str);
char* smash_string_to_lower(const char* str);
char* smash_string_trim(const char* str);
char* smash_string_trim_start(const char* str);
char* smash_string_trim_end(const char* str);
char* smash_string_char_at(const char* str, const char* index_str);
char* smash_string_concat(const char* str1, const char* str2);
char* smash_string_includes(const char* str, const char* search_str);
char* smash_string_index_of(const char* str, const char* search_str);
char* smash_string_slice(const char* str, const char* start_str, const char* end_str);
char* smash_string_split(const char* str, const char* delimiter);
char* smash_string_repeat(const char* str, const char* count_str);
char* smash_get_length(const char* str);

// Number helper functions
char* smash_number_to_string(const char* num_str);
char* smash_number_to_fixed(const char* num_str, const char* decimals_str);
char* smash_number_to_precision(const char* num_str, const char* precision_str);
char* smash_number_to_exponential(const char* num_str, const char* decimals_str);

// Array helper functions
char* smash_array_map(const char* array_str, const char* callback);
char* smash_array_filter(const char* array_str, const char* callback);
char* smash_array_push(const char* array_str, const char* element);
char* smash_array_pop(const char* array_str);
char* smash_array_for_each(const char* array_str, const char* callback);
char* smash_array_find(const char* array_str, const char* callback);
char* smash_array_join(const char* array_str, const char* separator);
char* smash_array_reverse(const char* array_str);
char* smash_array_slice(const char* array_str, const char* start_str, const char* end_str);

// Object helper functions
char* smash_object_has_own_property(const char* object_str, const char* property);
char* smash_object_keys(const char* object_str);
char* smash_object_values(const char* object_str);
char* smash_object_entries(const char* object_str);
char* smash_object_to_string(const char* object_str);

// Generic helper functions for common methods
char* smash_to_string(const char* value);
char* smash_value_of(const char* value);
char* smash_slice(const char* value, const char* start_str, const char* end_str);

// Regex helper functions
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
