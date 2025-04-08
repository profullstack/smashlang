#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <stdbool.h>
#include "simple_regex.h"

// Create a new regex pattern
SimpleRegex* simple_regex_create(const char* pattern, const char* flags) {
    if (!pattern) return NULL;

    SimpleRegex* regex = (SimpleRegex*)malloc(sizeof(SimpleRegex));
    if (!regex) return NULL;

    // Initialize all fields
    regex->pattern = strdup(pattern);
    regex->flags = flags ? strdup(flags) : strdup("");
    regex->case_insensitive = false;
    regex->global = false;
    
    // Parse flags
    if (regex->flags) {
        for (size_t i = 0; i < strlen(regex->flags); i++) {
            switch (regex->flags[i]) {
                case 'i':
                    regex->case_insensitive = true;
                    break;
                case 'g':
                    regex->global = true;
                    break;
                // Ignore unsupported flags
            }
        }
    }

    return regex;
}

// Free a regex pattern
void simple_regex_free(SimpleRegex* regex) {
    if (!regex) return;
    
    if (regex->pattern) free(regex->pattern);
    if (regex->flags) free(regex->flags);
    free(regex);
}

// Helper function to check if a character matches a pattern character
static bool char_matches(char c, char pattern, bool case_insensitive) {
    if (case_insensitive) {
        return tolower(c) == tolower(pattern);
    }
    return c == pattern;
}

// Helper function to check if a character is a word character (alphanumeric or underscore)
static bool is_word_char(char c) {
    return isalnum(c) || c == '_';
}

// Forward declaration for recursive calls
static bool match_pattern_with_len(const char* str, const char* pattern, bool case_insensitive, size_t* match_len);

// Simple pattern matching for basic regex support
// This is a simplified implementation that supports basic patterns and character classes
static bool match_pattern(const char* str, const char* pattern, bool case_insensitive) {
    size_t match_len = 0;
    return match_pattern_with_len(str, pattern, case_insensitive, &match_len);
}

// Enhanced pattern matching that also returns the match length
static bool match_pattern_with_len(const char* str, const char* pattern, bool case_insensitive, size_t* match_len) {
    while (*str && *pattern) {
        // Handle character classes [...]
        if (*pattern == '[') {
            pattern++; // Skip the opening bracket
            bool negate = false;
            bool matched = false;
            
            // Check if this is a negated character class [^...]
            if (*pattern == '^') {
                negate = true;
                pattern++;
            }
            
            // Empty character class is invalid
            if (*pattern == ']') return false;
            
            // Process the character class
            const char* class_start = pattern;
            while (*pattern && *pattern != ']') {
                // Handle character range a-z
                if (pattern[0] != '\\' && pattern[1] == '-' && pattern[2] && pattern[2] != ']') {
                    // Check if the character is in the range
                    char start = pattern[0];
                    char end = pattern[2];
                    if ((*str >= start && *str <= end) || 
                        (case_insensitive && 
                         ((tolower(*str) >= tolower(start) && tolower(*str) <= tolower(end)) || 
                          (toupper(*str) >= toupper(start) && toupper(*str) <= toupper(end))))) {
                        matched = true;
                        break;
                    }
                    pattern += 3; // Skip the range
                } else {
                    // Check for a direct match
                    if (char_matches(*str, *pattern, case_insensitive)) {
                        matched = true;
                        break;
                    }
                    pattern++;
                }
            }
            
            // Skip to the end of the character class
            while (*pattern && *pattern != ']') pattern++;
            if (*pattern == ']') pattern++; // Skip the closing bracket
            
            // If negated, invert the match result
            if (negate) matched = !matched;
            
            // If no match in the character class, return false
            if (!matched) return false;
        }
        // Handle special pattern characters
        else if (*pattern == '\\') {
            pattern++;
            if (!*pattern) return false; // Escape at end of pattern
            
            // Handle escaped characters
            switch (*pattern) {
                case 'd': // Digit
                    if (!isdigit(*str)) return false;
                    break;
                case 'w': // Word character
                    if (!is_word_char(*str)) return false;
                    break;
                case 's': // Whitespace
                    if (!isspace(*str)) return false;
                    break;
                case 'b': // Word boundary - simplified implementation
                    {
                        // This is a zero-width assertion, so don't consume the character
                        const char* prev = str > (const char*)1 ? str - 1 : NULL;
                        bool prev_is_word = prev ? is_word_char(*prev) : false;
                        bool curr_is_word = is_word_char(*str);
                        
                        // A word boundary exists when the current character and previous character
                        // have different word/non-word status
                        if (prev_is_word == curr_is_word) return false;
                        
                        str--; // Don't consume the character for zero-width assertion
                    }
                    break;
                default: // Literal character after escape
                    if (!char_matches(*str, *pattern, case_insensitive)) return false;
            }
        } else if (*pattern == '.') {
            // Dot matches any character
        } else {
            // Regular character match
            if (!char_matches(*str, *pattern, case_insensitive)) return false;
        }
        
        str++;
        pattern++;
    }
    
    // Match is successful if we've consumed the entire pattern
    return *pattern == '\0';
}

// Find all matches in a string
char* simple_regex_match(SimpleRegex* regex, const char* str) {
    if (!regex || !str) return NULL;
    
    // Allocate a buffer for the result
    size_t result_capacity = 1024; // Initial capacity
    char* result = (char*)malloc(result_capacity);
    if (!result) return NULL;
    result[0] = '\0';
    
    size_t result_len = 0;
    size_t str_len = strlen(str);
    
    // Iterate through the string to find matches
    for (size_t i = 0; i < str_len; i++) {
        if (match_pattern(str + i, regex->pattern, regex->case_insensitive)) {
            // Found a match, determine its length
            // Check if the pattern ends with a quantifier like '+'
            size_t pattern_len = strlen(regex->pattern);
            bool has_plus = pattern_len > 0 && regex->pattern[pattern_len-1] == '+';
            
            // For patterns with '+' quantifier, try to match as many characters as possible
            size_t match_len = 0;
            if (has_plus) {
                // Create a pattern without the '+' quantifier
                char* base_pattern = strdup(regex->pattern);
                if (!base_pattern) continue;
                base_pattern[pattern_len-1] = '\0';
                
                // Try to match multiple occurrences
                size_t j = 0;
                size_t curr_match_len = 0;
                
                // For character classes with +, match consecutive characters in the class
                if (base_pattern[0] == '[') {
                    while (i + j < str_len) {
                        if (match_pattern(str + i + j, base_pattern, regex->case_insensitive)) {
                            j++;
                            match_len = j; // Update total match length
                        } else {
                            break;
                        }
                    }
                } else {
                    // For other patterns, just match once and use the pattern length
                    match_len = pattern_len - 1; // Remove the '+' from length
                }
                
                free(base_pattern);
            } else {
                // For regular patterns without quantifiers
                if (regex->pattern[0] == '[') {
                    match_len = 1; // Just match the single character for character classes
                } else {
                    // For regular patterns, use pattern length as an approximation
                    match_len = pattern_len;
                    // Make sure we don't exceed string length
                    if (i + match_len > str_len) {
                        match_len = str_len - i;
                    }
                }
            }
            
            // Check if we need to resize the result buffer
            if (result_len + match_len + 2 > result_capacity) {
                result_capacity *= 2;
                char* new_result = (char*)realloc(result, result_capacity);
                if (!new_result) {
                    free(result);
                    return NULL;
                }
                result = new_result;
            }
            
            // Copy the match to the result
            if (result_len > 0) {
                result[result_len++] = ',';
                result[result_len] = '\0';
            }
            
            strncpy(result + result_len, str + i, match_len);
            result_len += match_len;
            result[result_len] = '\0';
            
            // If not global, stop after first match
            if (!regex->global) break;
            
            // Skip to the end of this match
            i += match_len - 1;
        }
    }
    
    return result;
}

// Replace matches in a string
char* simple_regex_replace(SimpleRegex* regex, const char* str, const char* replacement) {
    if (!regex || !str || !replacement) return NULL;
    
    // Allocate a buffer for the result
    size_t result_capacity = strlen(str) * 2; // Initial capacity
    char* result = (char*)malloc(result_capacity);
    if (!result) return NULL;
    result[0] = '\0';
    
    size_t result_len = 0;
    size_t str_len = strlen(str);
    size_t replacement_len = strlen(replacement);
    
    // Iterate through the string to find matches
    size_t last_match_end = 0;
    for (size_t i = 0; i < str_len; i++) {
        if (match_pattern(str + i, regex->pattern, regex->case_insensitive)) {
            // Found a match, determine its length
            // Check if the pattern ends with a quantifier like '+'
            size_t pattern_len = strlen(regex->pattern);
            bool has_plus = pattern_len > 0 && regex->pattern[pattern_len-1] == '+';
            
            // For patterns with '+' quantifier, try to match as many characters as possible
            size_t match_len = 0;
            if (has_plus) {
                // Create a pattern without the '+' quantifier
                char* base_pattern = strdup(regex->pattern);
                if (!base_pattern) continue;
                base_pattern[pattern_len-1] = '\0';
                
                // Try to match multiple occurrences
                size_t j = 0;
                size_t curr_match_len = 0;
                
                // For character classes with +, match consecutive characters in the class
                if (base_pattern[0] == '[') {
                    while (i + j < str_len) {
                        if (match_pattern(str + i + j, base_pattern, regex->case_insensitive)) {
                            j++;
                            match_len = j; // Update total match length
                        } else {
                            break;
                        }
                    }
                } else {
                    // For other patterns, just match once and use the pattern length
                    match_len = pattern_len - 1; // Remove the '+' from length
                }
                
                free(base_pattern);
            } else {
                // For regular patterns without quantifiers
                if (regex->pattern[0] == '[') {
                    match_len = 1; // Just match the single character for character classes
                } else {
                    // For regular patterns, use pattern length as an approximation
                    match_len = pattern_len;
                    // Make sure we don't exceed string length
                    if (i + match_len > str_len) {
                        match_len = str_len - i;
                    }
                }
            }
            
            // Copy the text between the last match and this match
            size_t between_len = i - last_match_end;
            if (between_len > 0) {
                // Check if we need to resize the result buffer
                if (result_len + between_len + 1 > result_capacity) {
                    result_capacity *= 2;
                    char* new_result = (char*)realloc(result, result_capacity);
                    if (!new_result) {
                        free(result);
                        return NULL;
                    }
                    result = new_result;
                }
                
                strncpy(result + result_len, str + last_match_end, between_len);
                result_len += between_len;
                result[result_len] = '\0';
            }
            
            // Check if we need to resize the result buffer for the replacement
            if (result_len + replacement_len + 1 > result_capacity) {
                result_capacity = (result_len + replacement_len + 1) * 2;
                char* new_result = (char*)realloc(result, result_capacity);
                if (!new_result) {
                    free(result);
                    return NULL;
                }
                result = new_result;
            }
            
            // Copy the replacement
            strcpy(result + result_len, replacement);
            result_len += replacement_len;
            
            // Update the last match end position
            last_match_end = i + match_len;
            
            // If not global, stop after first match
            if (!regex->global) {
                i = str_len; // Break the loop
            } else {
                i += match_len - 1; // Skip to the end of this match
            }
        }
    }
    
    // Copy any remaining text after the last match
    if (last_match_end < str_len) {
        size_t remaining_len = str_len - last_match_end;
        
        // Check if we need to resize the result buffer
        if (result_len + remaining_len + 1 > result_capacity) {
            result_capacity = (result_len + remaining_len + 1) * 2;
            char* new_result = (char*)realloc(result, result_capacity);
            if (!new_result) {
                free(result);
                return NULL;
            }
            result = new_result;
        }
        
        strcpy(result + result_len, str + last_match_end);
    }
    
    return result;
}

// Test if a pattern matches a string
bool simple_regex_test(SimpleRegex* regex, const char* str) {
    if (!regex || !str) return false;
    
    size_t str_len = strlen(str);
    
    // Iterate through the string to find a match
    for (size_t i = 0; i < str_len; i++) {
        if (match_pattern(str + i, regex->pattern, regex->case_insensitive)) {
            return true;
        }
    }
    
    return false;
}
