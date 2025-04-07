#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <pcre.h>
#include "runtime.h"

// Forward declaration for smash_regex_free
void smash_regex_free(SmashRegex* regex);

struct SmashRegex {
    char* pattern;     // Original pattern string
    char* flags;       // Flags (i, g, m, etc.)
    pcre* re;          // Compiled regex
    pcre_extra* extra; // Optimized regex data
};

// Create a new regex pattern
SmashRegex* smash_regex_create(const char* pattern, const char* flags) {
    if (!pattern) return NULL;
    
    SmashRegex* regex = (SmashRegex*)malloc(sizeof(SmashRegex));
    if (!regex) return NULL;

    regex->pattern = NULL;
    regex->flags = NULL;
    regex->re = NULL;
    regex->extra = NULL;

    regex->pattern = strdup(pattern);
    regex->flags = flags ? strdup(flags) : strdup("");
    if (!regex->pattern || !regex->flags) {
        smash_regex_free(regex);
        return NULL;
    }

    int options = 0;
    if (strchr(regex->flags, 'i')) options |= PCRE_CASELESS;
    if (strchr(regex->flags, 'm')) options |= PCRE_MULTILINE;
    if (strchr(regex->flags, 's')) options |= PCRE_DOTALL;

    const char* error;
    int erroffset;
    regex->re = pcre_compile(pattern, options, &error, &erroffset, NULL);
    if (!regex->re) {
        fprintf(stderr, "Regex compilation failed at offset %d: %s\n", erroffset, error);
        smash_regex_free(regex);
        return NULL;
    }

    regex->extra = pcre_study(regex->re, 0, &error);
    if (error) {
        fprintf(stderr, "Regex study failed: %s\n", error);
    }

    return regex;
}

// Free a regex pattern
void smash_regex_free(SmashRegex* regex) {
    if (!regex) return;
    
    if (regex->pattern) free(regex->pattern);
    if (regex->flags) free(regex->flags);
    if (regex->extra) pcre_free(regex->extra);
    if (regex->re) pcre_free(regex->re);
    free(regex);
}

// Test if a string matches a regex pattern
int smash_regex_test(SmashRegex* regex, const char* str) {
    if (!regex || !str || !regex->re) return 0;
    
    int ovector[30];
    int rc = pcre_exec(regex->re, regex->extra, str, strlen(str), 0, 0, ovector, 30);
    return (rc >= 0) ? 1 : 0;
}

// Find matches in a string (returns JSON array of matches)
char* smash_regex_match(SmashRegex* regex, const char* str) {
    if (!regex || !str || !regex->re) return NULL;
    
    int ovector[30];
    int str_len = strlen(str);
    int start_offset = 0;
    int global = strchr(regex->flags, 'g') != NULL;
    
    char* result = strdup("[]");
    if (!result) return NULL;
    
    int match_count = 0;
    while (start_offset < str_len) {
        int rc = pcre_exec(regex->re, regex->extra, str, str_len, start_offset, 0, ovector, 30);
        if (rc < 0) break;
        
        int match_len = ovector[1] - ovector[0];
        char* match = (char*)malloc(match_len + 1);
        if (!match) {
            free(result);
            return NULL;
        }
        
        strncpy(match, str + ovector[0], match_len);
        match[match_len] = '\0';
        
        if (match_count == 0) {
            free(result);
            result = (char*)malloc(match_len + 5);
            if (!result) {
                free(match);
                return NULL;
            }
            sprintf(result, "[\"%s\"]", match);
        } else {
            int old_len = strlen(result);
            char* new_result = (char*)malloc(old_len + match_len + 5);
            if (!new_result) {
                free(match);
                free(result);
                return NULL;
            }
            result[old_len - 1] = '\0';
            sprintf(new_result, "%s,\"%s\"]", result, match);
            free(result);
            result = new_result;
        }
        
        free(match);
        match_count++;
        
        if (!global) break;
        
        start_offset = ovector[1];
        if (ovector[0] == ovector[1]) start_offset++;
    }
    
    return result;
}

// Replace matches in a string
char* smash_regex_replace(SmashRegex* regex, const char* str, const char* replacement) {
    if (!regex || !str || !replacement || !regex->re) return NULL;
    
    int ovector[30];
    int str_len = strlen(str);
    int repl_len = strlen(replacement);
    int start_offset = 0;
    int global = strchr(regex->flags, 'g') != NULL;
    
    int buffer_size = str_len * 2;
    if (buffer_size < 1024) buffer_size = 1024;
    char* result = (char*)malloc(buffer_size);
    if (!result) return NULL;
    
    strcpy(result, str);
    
    int match_count = 0;
    char* current = result;
    
    while (1) {
        int rc = pcre_exec(regex->re, regex->extra, current, strlen(current), 0, 0, ovector, 30);
        if (rc < 0) break;
        
        int match_len = ovector[1] - ovector[0];
        int new_len = strlen(current) - match_len + repl_len;
        
        char* temp = (char*)malloc(new_len + 1);
        if (!temp) {
            free(result);
            return NULL;
        }
        
        strncpy(temp, current, ovector[0]);
        temp[ovector[0]] = '\0';
        
        strcat(temp, replacement);
        strcat(temp, current + ovector[1]);
        
        if (current == result) {
            free(result);
            result = temp;
            current = result;
        } else {
            free(current);
            current = temp;
        }
        
        match_count++;
        
        if (!global) break;
    }
    
    if (match_count == 0) {
        free(result);
        return strdup(str);
    }
    
    return result;
}

// Helper function for string.match with regex
char* smash_string_match(const char* str, const char* pattern) {
    SmashRegex* regex;
    int should_free = 0;
    
    if (strncmp(pattern, "SmashRegex:", 11) == 0) {
        sscanf(pattern + 11, "%p", &regex);
    } else {
        regex = smash_regex_create(pattern, "");
        should_free = 1;
    }
    
    char* result = smash_regex_match(regex, str);
    
    if (should_free) {
        smash_regex_free(regex);
    }
    
    return result;
}

// Helper function for string.replace with regex
char* smash_string_replace(const char* str, const char* pattern, const char* replacement) {
    SmashRegex* regex;
    int should_free = 0;
    
    if (strncmp(pattern, "SmashRegex:", 11) == 0) {
        sscanf(pattern + 11, "%p", &regex);
    } else {
        regex = smash_regex_create(pattern, "");
        should_free = 1;
    }
    
    char* result = smash_regex_replace(regex, str, replacement);
    
    if (should_free) {
        smash_regex_free(regex);
    }
    
    return result;
}

// Free a string returned by regex functions
void smash_free_string(char* str) {
    free(str);
}

// No need to load external library, using embedded implementation
int load_regex_library(void) {
    return 1;
}
