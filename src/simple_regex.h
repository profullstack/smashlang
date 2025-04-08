#ifndef SIMPLE_REGEX_H
#define SIMPLE_REGEX_H

#include <stdbool.h>

// Simple regex structure without external dependencies
typedef struct SimpleRegex {
    char* pattern;
    char* flags;
    bool case_insensitive;
    bool global;
} SimpleRegex;

// Regex function declarations
SimpleRegex* simple_regex_create(const char* pattern, const char* flags);
void simple_regex_free(SimpleRegex* regex);
char* simple_regex_match(SimpleRegex* regex, const char* str);
char* simple_regex_replace(SimpleRegex* regex, const char* str, const char* replacement);
bool simple_regex_test(SimpleRegex* regex, const char* str);

#endif // SIMPLE_REGEX_H
