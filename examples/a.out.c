#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <ctype.h>
#include <dlfcn.h>
#include "simple_regex.h"
#include "runtime.h"
// Forward declarations
void smash_regex_free(SmashRegex* regex);
SmashRegex* smash_regex_create(const char* pattern, const char* flags);
char* smash_regex_match(SmashRegex* regex, const char* str);
char* smash_regex_replace(SmashRegex* regex, const char* str, const char* replacement);

// Helper functions are defined in runtime.c

int main(int argc, char** argv) {
    // Initialize the regex library
    if (!load_regex_library()) {
        fprintf(stderr, "Failed to load regex library. Regex operations will not work.\n");
    }

    char* text = "Hello, SmashLang! This is a test string with numbers 123 and 456.";
    printf("%s", "Original text:");
    printf(" ");
    printf("%s", text);
    printf("\n");
    printf("%s", "--- Basic Matching ---");
    printf("\n");
    char* basicMatch = smash_string_match(text, "SmashLang");
    printf("%s", "Match 'SmashLang':");
    printf(" ");
    printf("%s", basicMatch);
    printf("\n");
    char* caseMatch = smash_string_match(text, "smashlang/i");
    printf("%s", "Case-insensitive match 'smashlang':");
    printf(" ");
    printf("%s", caseMatch);
    printf("\n");
    printf("%s", "--- Character Classes ---");
    printf("\n");
    char* digitMatch = smash_string_match(text, "[0-9]+");
    printf("%s", "Match digits [0-9]+:");
    printf(" ");
    printf("%s", digitMatch);
    printf("\n");
    printf("%s", "--- Global Matching ---");
    printf("\n");
    char* repeatText = "one two one two one three";
    printf("%s", "Original text for global match:");
    printf(" ");
    printf("%s", repeatText);
    printf("\n");
    printf("%s", "--- Replacement Tests ---");
    printf("\n");
    char* basicReplaced = smash_string_replace(repeatText, "one", "ONE");
    printf("%s", "Replace first 'one':");
    printf(" ");
    printf("%s", basicReplaced);
    printf("\n");
    char* globalReplaced = smash_string_replace(repeatText, "one/g", "ONE");
    printf("%s", "Replace all 'one' (global):");
    printf(" ");
    printf("%s", globalReplaced);
    printf("\n");
    char* mixedText = "Hello hello HELLO world";
    printf("%s", "Original mixed case text:");
    printf(" ");
    printf("%s", mixedText);
    printf("\n");
    char* caseReplaced = smash_string_replace(mixedText, "hello/i", "hi");
    printf("%s", "Replace first 'hello' case-insensitive:");
    printf(" ");
    printf("%s", caseReplaced);
    printf("\n");
    char* combinedReplaced = smash_string_replace(mixedText, "hello/ig", "hi");
    printf("%s", "Replace all 'hello' case-insensitive and global:");
    printf(" ");
    printf("%s", combinedReplaced);
    printf("\n");
    printf("%s", "Final regex test complete!");
    printf("\n");
    return 0;
}
