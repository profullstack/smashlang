#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

// Helper function for string concatenation
char* smash_string_concat(const char* a, const char* b) {
    size_t len_a = strlen(a);
    size_t len_b = strlen(b);
    char* result = (char*)malloc(len_a + len_b + 1);
    if (result) {
        strcpy(result, a);
        strcat(result, b);
    }
    return result;
}

char* formatName(char* first, char* last) {
    return smash_string_concat(smash_string_concat(smash_string_concat("", first), " "), last);
    return "";
}

char* checkAge(char* age) {
    char* isAdult = (strcmp(age, (char*)"18") >= 0 ? "true" : "false");
    return (strcmp(isAdult, "true") == 0 ? "Adult" : "Minor");
    return "";
}

int main(int argc, char** argv) {
    char* name = "World";
    char* doubleQuoted = "Double quoted string";
    char* singleQuoted = "Single quoted string";
    char* greeting = smash_string_concat(smash_string_concat("Hello, ", name), "!");
    printf("%s\n", greeting);
    char* stringLength = (char*)"24";
    char* templateLiteral = smash_string_concat(smash_string_concat("This is a template literal with ", stringLength), " characters");
    printf("%s\n", templateLiteral);
    char* innerTemplate = smash_string_concat("Inner ", name);
    char* nested = smash_string_concat("Outer ", innerTemplate);
    printf("%s\n", nested);
    char* a = (char*)"10";
    char* b = (char*)"5";
    char* isGreater = (strcmp(a, b) > 0 ? "true" : "false");
    char* max = (strcmp(isGreater, "true") == 0 ? a : b);
    printf("%s\n", smash_string_concat("Maximum value: ", max));
    char* score = (char*)"85";
    char* isB = (strcmp(score, (char*)"80") >= 0 ? "true" : "false");
    char* grade = (strcmp(isB, "true") == 0 ? "B" : "F");
    printf("%s\n", smash_string_concat(smash_string_concat(smash_string_concat("Score: ", score), ", Grade: "), grade));
    char* isLoggedIn = "true";
    char* username = "john_doe";
    char* welcomeMessage = smash_string_concat(smash_string_concat("Welcome back, ", username), "!");
    char* loginMessage = "Please log in";
    char* message = (strcmp(isLoggedIn, "true") == 0 ? welcomeMessage : loginMessage);
    printf("%s\n", message);
    char* fullName = formatName("John", "Doe");
    printf("%s\n", fullName);
    printf("%s\n", smash_string_concat("Status: ", checkAge((char*)"20")));
    printf("%s\n", smash_string_concat("Status: ", checkAge((char*)"15")));
    return 0;
};
