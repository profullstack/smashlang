use crate::parser::AstNode;

// Helper function to parse a regex pattern string into pattern and flags
fn parse_regex_pattern(regex: &str) -> (&str, &str) {
    // Regex pattern format is typically /pattern/flags
    // We need to extract the pattern and flags
    
    // First, check if the pattern has flags
    if let Some(last_slash_pos) = regex.rfind('/') {
        if last_slash_pos > 1 && last_slash_pos < regex.len() - 1 {
            // There are flags after the last slash
            let pattern = &regex[1..last_slash_pos];
            let flags = &regex[last_slash_pos+1..];
            return (pattern, flags);
        }
    }
    
    // If we get here, either there are no flags or the format is unusual
    // Try to extract just the pattern between slashes
    if regex.starts_with('/') && regex.ends_with('/') && regex.len() >= 2 {
        let pattern = &regex[1..regex.len()-1];
        return (pattern, "");
    }
    
    // If all else fails, return the whole string as the pattern with no flags
    (regex, "")
}

// Simple structure to represent a target machine for C code generation
pub struct TargetMachine {
    pub target_triple: String, // Made public to avoid unused field warning
}

impl TargetMachine {
    pub fn new(target_triple: &str) -> Self {
        TargetMachine {
            target_triple: target_triple.to_string(),
        }
    }
    
    // Write C code to a file
    pub fn write_to_file(&self, module: &Module, _file_type: FileType, output_path: &str) -> Result<(), String> {
        // We could use self.target_triple here for target-specific code generation
        // For now, we're using a simplified approach
        use std::fs;
        use std::io::Write;
        
        let c_code = module.to_c_code();
        
        match fs::File::create(output_path) {
            Ok(mut file) => {
                match file.write_all(c_code.as_bytes()) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(format!("Failed to write to file: {}", e)),
                }
            },
            Err(e) => Err(format!("Failed to create file: {}", e)),
        }
    }
}

// Simple enum to represent file types
pub enum FileType {
    Object,
    Assembly,
}

// Simple module structure to hold generated code
pub struct Module<'a> {
    ast: &'a [AstNode],
}

impl<'a> Module<'a> {
    pub fn new(ast: &'a [AstNode]) -> Self {
        Module { ast }
    }
    
    // Convert AST to C code
    pub fn to_c_code(&self) -> String {
        let mut code = String::new();
        let mut main_code = String::new();
        
        // Add standard includes
        code.push_str("#include <stdio.h>\n");
        code.push_str("#include <stdlib.h>\n");
        code.push_str("#include <string.h>\n");
        code.push_str("#include <stdbool.h>\n");
        code.push_str("#include <ctype.h>\n");
        code.push_str("#include <dlfcn.h>\n\n");  // For dynamic loading of our Rust regex library
        
        // Add helper function for string concatenation
        code.push_str("// Helper function for string concatenation\n");
        code.push_str("char* smash_string_concat(const char* a, const char* b) {\n");
        code.push_str("    size_t len_a = strlen(a);\n");
        code.push_str("    size_t len_b = strlen(b);\n");
        code.push_str("    char* result = (char*)malloc(len_a + len_b + 1);\n");
        code.push_str("    if (result) {\n");
        code.push_str("        strcpy(result, a);\n");
        code.push_str("        strcat(result, b);\n");
        code.push_str("    }\n");
        code.push_str("    return result;\n");
        code.push_str("}\n\n");
        
        // Add helper function for getting string length
        code.push_str("// Helper function for getting string length\n");
        code.push_str("char* smash_get_length(const char* str) {\n");
        code.push_str("    size_t len = strlen(str);\n");
        code.push_str("    char buffer[32];\n");
        code.push_str("    snprintf(buffer, sizeof(buffer), \"%zu\", len);\n");
        code.push_str("    char* result = (char*)malloc(strlen(buffer) + 1);\n");
        code.push_str("    if (result) {\n");
        code.push_str("        strcpy(result, buffer);\n");
        code.push_str("    }\n");
        code.push_str("    return result;\n");
        code.push_str("}\n\n");
        
        // Add helper functions for string methods
        code.push_str("// Helper function for converting string to uppercase\n");
        code.push_str("char* smash_string_to_upper(const char* str) {\n");
        code.push_str("    size_t len = strlen(str);\n");
        code.push_str("    char* result = (char*)malloc(len + 1);\n");
        code.push_str("    if (result) {\n");
        code.push_str("        for (size_t i = 0; i < len; i++) {\n");
        code.push_str("            result[i] = toupper(str[i]);\n");
        code.push_str("        }\n");
        code.push_str("        result[len] = '\0';\n");
        code.push_str("    }\n");
        code.push_str("    return result;\n");
        code.push_str("}\n\n");
        
        code.push_str("// Helper function for converting string to lowercase\n");
        code.push_str("char* smash_string_to_lower(const char* str) {\n");
        code.push_str("    size_t len = strlen(str);\n");
        code.push_str("    char* result = (char*)malloc(len + 1);\n");
        code.push_str("    if (result) {\n");
        code.push_str("        for (size_t i = 0; i < len; i++) {\n");
        code.push_str("            result[i] = tolower(str[i]);\n");
        code.push_str("        }\n");
        code.push_str("        result[len] = '\0';\n");
        code.push_str("    }\n");
        code.push_str("    return result;\n");
        code.push_str("}\n\n");
        
        code.push_str("// Helper function for trimming whitespace from a string\n");
        code.push_str("char* smash_string_trim(const char* str) {\n");
        code.push_str("    size_t len = strlen(str);\n");
        code.push_str("    size_t start = 0, end = len;\n");
        code.push_str("    \n");
        code.push_str("    // Find start index (first non-whitespace)\n");
        code.push_str("    while (start < len && isspace(str[start])) {\n");
        code.push_str("        start++;\n");
        code.push_str("    }\n");
        code.push_str("    \n");
        code.push_str("    // Find end index (last non-whitespace)\n");
        code.push_str("    while (end > start && isspace(str[end - 1])) {\n");
        code.push_str("        end--;\n");
        code.push_str("    }\n");
        code.push_str("    \n");
        code.push_str("    // Create result string\n");
        code.push_str("    size_t result_len = end - start;\n");
        code.push_str("    char* result = (char*)malloc(result_len + 1);\n");
        code.push_str("    if (result) {\n");
        code.push_str("        strncpy(result, str + start, result_len);\n");
        code.push_str("        result[result_len] = '\0';\n");
        code.push_str("    }\n");
        code.push_str("    return result;\n");
        code.push_str("}\n\n");
        
        // Add helper functions for array methods
        code.push_str("// Note: These are simplified implementations for demonstration purposes\n");
        code.push_str("// In a real implementation, we would need proper array handling\n");
        
        code.push_str("// Helper function for array map operation\n");
        code.push_str("char* smash_array_map(const char* array, const char* callback) {\n");
        code.push_str("    // In a real implementation, this would parse the array, apply the callback to each element,\n");
        code.push_str("    // and return a new array. For now, we'll just return a placeholder.\n");
        code.push_str("    return strdup(\"[Mapped Array]\");\n");
        code.push_str("}\n\n");
        
        code.push_str("// Helper function for array filter operation\n");
        code.push_str("char* smash_array_filter(const char* array, const char* callback) {\n");
        code.push_str("    // In a real implementation, this would parse the array, apply the callback to each element,\n");
        code.push_str("    // and return a new array with elements that pass the test. For now, we'll just return a placeholder.\n");
        code.push_str("    return strdup(\"[Filtered Array]\");\n");
        code.push_str("}\n\n");
        
        code.push_str("// Helper function for array push operation\n");
        code.push_str("char* smash_array_push(const char* array, const char* item) {\n");
        code.push_str("    // In a real implementation, this would parse the array, add the item, and return the new array.\n");
        code.push_str("    // For now, we'll just return a placeholder.\n");
        code.push_str("    return strdup(\"[Array with pushed item]\");\n");
        code.push_str("}\n\n");
        
        code.push_str("// Helper function for array pop operation\n");
        code.push_str("char* smash_array_pop(const char* array) {\n");
        code.push_str("    // In a real implementation, this would parse the array, remove the last item, and return that item.\n");
        code.push_str("    // For now, we'll just return a placeholder.\n");
        code.push_str("    return strdup(\"[Popped item]\");\n");
        code.push_str("}\n\n");
        
        // Add embedded regex implementation with full JavaScript-like functionality
        code.push_str("// Embedded regex implementation for JavaScript-like support\n");
        code.push_str("#include <pcre.h>  // Use PCRE for JavaScript-compatible regex\n\n");
        
        code.push_str("// Structure to hold regex pattern and compiled regex\n");
        code.push_str("typedef struct {\n");
        code.push_str("    char* pattern;     // Original pattern string\n");
        code.push_str("    char* flags;       // Flags (i, g, m, etc.)\n");
        code.push_str("    pcre* re;          // Compiled regex\n");
        code.push_str("    pcre_extra* extra; // Optimized regex data\n");
        code.push_str("} SmashRegex;\n\n");
        
        code.push_str("// Create a new regex pattern\n");
        code.push_str("SmashRegex* smash_regex_create(const char* pattern, const char* flags) {\n");
        code.push_str("    if (!pattern) return NULL;\n");
        code.push_str("    \n");
        code.push_str("    SmashRegex* regex = (SmashRegex*)malloc(sizeof(SmashRegex));\n");
        code.push_str("    if (!regex) return NULL;\n");
        
        code.push_str("    // Initialize to NULL so we can safely free in case of error\n");
        code.push_str("    regex->pattern = NULL;\n");
        code.push_str("    regex->flags = NULL;\n");
        code.push_str("    regex->re = NULL;\n");
        code.push_str("    regex->extra = NULL;\n");
        
        code.push_str("    // Copy pattern and flags\n");
        code.push_str("    regex->pattern = strdup(pattern);\n");
        code.push_str("    regex->flags = flags ? strdup(flags) : strdup(\"\");\n");
        
        code.push_str("    if (!regex->pattern || !regex->flags) {\n");
        code.push_str("        smash_regex_free(regex);\n");
        code.push_str("        return NULL;\n");
        code.push_str("    }\n");
        
        code.push_str("    // Build PCRE options based on flags\n");
        code.push_str("    int options = 0;\n");
        code.push_str("    if (strchr(regex->flags, 'i')) options |= PCRE_CASELESS;\n");
        code.push_str("    if (strchr(regex->flags, 'm')) options |= PCRE_MULTILINE;\n");
        code.push_str("    if (strchr(regex->flags, 's')) options |= PCRE_DOTALL;\n");
        
        code.push_str("    // Compile the regex\n");
        code.push_str("    const char* error;\n");
        code.push_str("    int erroffset;\n");
        code.push_str("    regex->re = pcre_compile(pattern, options, &error, &erroffset, NULL);\n");
        
        code.push_str("    if (!regex->re) {\n");
        code.push_str("        fprintf(stderr, \"Regex compilation failed at offset %d: %s\\n\", erroffset, error);\n");
        code.push_str("        smash_regex_free(regex);\n");
        code.push_str("        return NULL;\n");
        code.push_str("    }\n");
        
        code.push_str("    // Study the pattern for optimization\n");
        code.push_str("    regex->extra = pcre_study(regex->re, 0, &error);\n");
        code.push_str("    if (error) {\n");
        code.push_str("        fprintf(stderr, \"Regex study failed: %s\\n\", error);\n");
        code.push_str("        // Continue without the optimization\n");
        code.push_str("    }\n");
        
        code.push_str("    return regex;\n");
        code.push_str("}\n\n");
        
        code.push_str("// Free a regex pattern\n");
        code.push_str("void smash_regex_free(SmashRegex* regex) {\n");
        code.push_str("    if (!regex) return;\n");
        code.push_str("    \n");
        code.push_str("    if (regex->pattern) free(regex->pattern);\n");
        code.push_str("    if (regex->flags) free(regex->flags);\n");
        code.push_str("    if (regex->extra) pcre_free(regex->extra);\n");
        code.push_str("    if (regex->re) pcre_free(regex->re);\n");
        code.push_str("    free(regex);\n");
        code.push_str("}\n\n");
        
        code.push_str("// Test if a string matches a regex pattern\n");
        code.push_str("int smash_regex_test(SmashRegex* regex, const char* str) {\n");
        code.push_str("    if (!regex || !str || !regex->re) return 0;\n");
        code.push_str("    \n");
        code.push_str("    int ovector[30];  // Output vector for match offsets\n");
        code.push_str("    int rc = pcre_exec(regex->re, regex->extra, str, strlen(str), 0, 0, ovector, 30);\n");
        code.push_str("    \n");
        code.push_str("    // Return 1 for match, 0 for no match\n");
        code.push_str("    return (rc >= 0) ? 1 : 0;\n");
        code.push_str("}\n\n");
        
        code.push_str("// Find matches in a string (returns JSON array of matches)\n");
        code.push_str("char* smash_regex_match(SmashRegex* regex, const char* str) {\n");
        code.push_str("    if (!regex || !str || !regex->re) return NULL;\n");
        code.push_str("    \n");
        code.push_str("    int ovector[30];  // Output vector for match offsets\n");
        code.push_str("    int str_len = strlen(str);\n");
        code.push_str("    int start_offset = 0;\n");
        code.push_str("    int global = strchr(regex->flags, 'g') != NULL;\n");
        code.push_str("    \n");
        code.push_str("    // Buffer to build JSON array of matches\n");
        code.push_str("    char* result = strdup(\"[]\");\n");
        code.push_str("    if (!result) return NULL;\n");
        code.push_str("    \n");
        code.push_str("    // Find all matches\n");
        code.push_str("    int match_count = 0;\n");
        code.push_str("    while (start_offset < str_len) {\n");
        code.push_str("        int rc = pcre_exec(regex->re, regex->extra, str, str_len, start_offset, 0, ovector, 30);\n");
        code.push_str("        if (rc < 0) break;  // No more matches\n");
        code.push_str("        \n");
        code.push_str("        // Extract the matched substring\n");
        code.push_str("        int match_len = ovector[1] - ovector[0];\n");
        code.push_str("        char* match = (char*)malloc(match_len + 1);\n");
        code.push_str("        if (!match) {\n");
        code.push_str("            free(result);\n");
        code.push_str("            return NULL;\n");
        code.push_str("        }\n");
        code.push_str("        \n");
        code.push_str("        strncpy(match, str + ovector[0], match_len);\n");
        code.push_str("        match[match_len] = '\0';\n");
        code.push_str("        \n");
        code.push_str("        // Add to JSON array\n");
        code.push_str("        if (match_count == 0) {\n");
        code.push_str("            // First match, replace empty array\n");
        code.push_str("            free(result);\n");
        code.push_str("            result = (char*)malloc(match_len + 5);  // Array format\n");
        code.push_str("            if (!result) {\n");
        code.push_str("                free(match);\n");
        code.push_str("                return NULL;\n");
        code.push_str("            }\n");
        code.push_str("            sprintf(result, \"[\\\"%s\\\"]\", match);\n");
        code.push_str("        } else {\n");
        code.push_str("            // Append to existing array\n");
        code.push_str("            int old_len = strlen(result);\n");
        code.push_str("            char* new_result = (char*)malloc(old_len + match_len + 5);  // For appending\n");
        code.push_str("            if (!new_result) {\n");
        code.push_str("                free(match);\n");
        code.push_str("                free(result);\n");
        code.push_str("                return NULL;\n");
        code.push_str("            }\n");
        code.push_str("            // Remove trailing ]\n");
        code.push_str("            result[old_len - 1] = '\0';\n");
        code.push_str("            sprintf(new_result, \"%s,\\\"%s\\\"]\", result, match);\n");
        code.push_str("            free(result);\n");
        code.push_str("            result = new_result;\n");
        code.push_str("        }\n");
        code.push_str("        \n");
        code.push_str("        free(match);\n");
        code.push_str("        match_count++;\n");
        code.push_str("        \n");
        code.push_str("        // If not global, stop after first match\n");
        code.push_str("        if (!global) break;\n");
        code.push_str("        \n");
        code.push_str("        // Move to position after the match\n");
        code.push_str("        start_offset = ovector[1];\n");
        code.push_str("        // Avoid infinite loop on zero-length matches\n");
        code.push_str("        if (ovector[0] == ovector[1]) start_offset++;\n");
        code.push_str("    }\n");
        code.push_str("    \n");
        code.push_str("    return result;\n");
        code.push_str("}\n\n");
        
        code.push_str("// Replace matches in a string\n");
        code.push_str("char* smash_regex_replace(SmashRegex* regex, const char* str, const char* replacement) {\n");
        code.push_str("    if (!regex || !str || !replacement || !regex->re) return NULL;\n");
        code.push_str("    \n");
        code.push_str("    int ovector[30];  // Output vector for match offsets\n");
        code.push_str("    int str_len = strlen(str);\n");
        code.push_str("    int repl_len = strlen(replacement);\n");
        code.push_str("    int start_offset = 0;\n");
        code.push_str("    int global = strchr(regex->flags, 'g') != NULL;\n");
        code.push_str("    \n");
        code.push_str("    // Initial buffer size estimate\n");
        code.push_str("    int buffer_size = str_len * 2;  // Start with twice the input size\n");
        code.push_str("    if (buffer_size < 1024) buffer_size = 1024;  // Minimum buffer size\n");
        
        code.push_str("    char* result = (char*)malloc(buffer_size);\n");
        code.push_str("    if (!result) return NULL;\n");
        code.push_str("    \n");
        code.push_str("    // Copy the input string to start with\n");
        code.push_str("    strcpy(result, str);\n");
        code.push_str("    \n");
        code.push_str("    // Find and replace all matches\n");
        code.push_str("    int match_count = 0;\n");
        code.push_str("    char* current = result;\n");
        code.push_str("    \n");
        code.push_str("    while (1) {\n");
        code.push_str("        int rc = pcre_exec(regex->re, regex->extra, current, strlen(current), 0, 0, ovector, 30);\n");
        code.push_str("        if (rc < 0) break;  // No more matches\n");
        code.push_str("        \n");
        code.push_str("        // Calculate new string length\n");
        code.push_str("        int match_len = ovector[1] - ovector[0];\n");
        code.push_str("        int new_len = strlen(current) - match_len + repl_len;\n");
        code.push_str("        \n");
        code.push_str("        // Create temporary buffer for the replacement\n");
        code.push_str("        char* temp = (char*)malloc(new_len + 1);\n");
        code.push_str("        if (!temp) {\n");
        code.push_str("            free(result);\n");
        code.push_str("            return NULL;\n");
        code.push_str("        }\n");
        code.push_str("        \n");
        code.push_str("        // Copy parts before match\n");
        code.push_str("        strncpy(temp, current, ovector[0]);\n");
        code.push_str("        temp[ovector[0]] = '\0';\n");
        code.push_str("        \n");
        code.push_str("        // Copy replacement\n");
        code.push_str("        strcat(temp, replacement);\n");
        code.push_str("        \n");
        code.push_str("        // Copy parts after match\n");
        code.push_str("        strcat(temp, current + ovector[1]);\n");
        code.push_str("        \n");
        code.push_str("        // Replace current with the new string\n");
        code.push_str("        if (current == result) {\n");
        code.push_str("            free(result);\n");
        code.push_str("            result = temp;\n");
        code.push_str("            current = result;\n");
        code.push_str("        } else {\n");
        code.push_str("            free(current);\n");
        code.push_str("            current = temp;\n");
        code.push_str("        }\n");
        code.push_str("        \n");
        code.push_str("        match_count++;\n");
        code.push_str("        \n");
        code.push_str("        // If not global, stop after first replacement\n");
        code.push_str("        if (!global) break;\n");
        code.push_str("    }\n");
        code.push_str("    \n");
        code.push_str("    // If no replacements were made, ensure we return a copy of the original\n");
        code.push_str("    if (match_count == 0) {\n");
        code.push_str("        free(result);\n");
        code.push_str("        return strdup(str);\n");
        code.push_str("    }\n");
        code.push_str("    \n");
        code.push_str("    return result;\n");
        code.push_str("}\n\n");
        
        code.push_str("// Free a string returned by regex functions\n");
        code.push_str("void smash_free_string(char* str) {\n");
        code.push_str("    free(str);\n");
        code.push_str("}\n\n");
        
        code.push_str("// No need to load external library, using embedded implementation\n");
        code.push_str("int load_regex_library() {\n");
        code.push_str("    return 1;  // Always succeeds with embedded implementation\n");
        code.push_str("}\n");
        

        
        // Helper functions for string methods that use regex
        code.push_str("// Helper function for string.match with regex\n");
        code.push_str("char* smash_string_match(const char* str, const char* pattern) {\n");
        code.push_str("    // If pattern is a regex object (starts with 'SmashRegex:'), use it directly\n");
        code.push_str("    // Otherwise, create a new regex object\n");
        code.push_str("    SmashRegex* regex;\n");
        code.push_str("    int should_free = 0;\n");
        code.push_str("    \n");
        code.push_str("    if (strncmp(pattern, \"SmashRegex:\", 11) == 0) {\n");
        code.push_str("        // Extract the regex pointer from the string\n");
        code.push_str("        sscanf(pattern + 11, \"%p\", &regex);\n");
        code.push_str("    } else {\n");
        code.push_str("        // Create a new regex object from the pattern string\n");
        code.push_str("        regex = smash_regex_create(pattern, \"\");\n");
        code.push_str("        should_free = 1;\n");
        code.push_str("    }\n");
        code.push_str("    \n");
        code.push_str("    // Match the regex against the string\n");
        code.push_str("    char* result = smash_regex_match(regex, str);\n");
        code.push_str("    \n");
        code.push_str("    // Free the regex if we created it\n");
        code.push_str("    if (should_free) {\n");
        code.push_str("        smash_regex_free(regex);\n");
        code.push_str("    }\n");
        code.push_str("    \n");
        code.push_str("    return result;\n");
        code.push_str("}\n\n");
        
        code.push_str("// Helper function for string.replace with regex\n");
        code.push_str("char* smash_string_replace(const char* str, const char* pattern, const char* replacement) {\n");
        code.push_str("    // If pattern is a regex object (starts with 'SmashRegex:'), use it directly\n");
        code.push_str("    // Otherwise, create a new regex object\n");
        code.push_str("    SmashRegex* regex;\n");
        code.push_str("    int should_free = 0;\n");
        code.push_str("    \n");
        code.push_str("    if (strncmp(pattern, \"SmashRegex:\", 11) == 0) {\n");
        code.push_str("        // Extract the regex pointer from the string\n");
        code.push_str("        sscanf(pattern + 11, \"%p\", &regex);\n");
        code.push_str("    } else {\n");
        code.push_str("        // Create a new regex object from the pattern string\n");
        code.push_str("        regex = smash_regex_create(pattern, \"\");\n");
        code.push_str("        should_free = 1;\n");
        code.push_str("    }\n");
        code.push_str("    \n");
        code.push_str("    // Replace matches in the string\n");
        code.push_str("    char* result = smash_regex_replace(regex, str, replacement);\n");
        code.push_str("    \n");
        code.push_str("    // Free the regex if we created it\n");
        code.push_str("    if (should_free) {\n");
        code.push_str("        smash_regex_free(regex);\n");
        code.push_str("    }\n");
        code.push_str("    \n");
        code.push_str("    return result;\n");
        code.push_str("}\n\n");
        
        // Process each AST node
        let mut has_main_function = false;
        // We'll track if we have a main function
        
        for node in self.ast {
            match node {
                AstNode::Function { name, params, body } => {
                    if name == "main" {
                        // Remember that we found a main function
                        has_main_function = true;
                        
                        // Instead of generating a char* main(), we'll call it smash_main()
                        // and call it from our C main function
                        let mut modified_function = format!("char* smash_main(\n");
                        
                        // Add parameters
                        for (i, param) in params.iter().enumerate() {
                            if i > 0 {
                                modified_function.push_str(", ");
                            }
                            modified_function.push_str(&format!("char* {}", param));
                        }
                        modified_function.push_str(") {\n");
                        
                        // Add function body
                        for stmt in body {
                            modified_function.push_str(&self.generate_c_code_for_node(stmt, 1));
                        }
                        
                        // Add default return if none is present
                        modified_function.push_str("    return \"\";\n");
                        
                        // Close function
                        modified_function.push_str("}\n\n");
                        
                        code.push_str(&modified_function);
                    } else {
                        // Other function definitions go outside main
                        code.push_str(&self.generate_c_code_for_node(node, 0));
                    }
                },
                AstNode::FunctionCall { name, args } => {
                    if name == "main" && args.is_empty() {
                        // Skip the main function call as we'll add it automatically
                    } else {
                        // Other function calls go inside main
                        main_code.push_str(&self.generate_c_code_for_node(node, 1));
                    }
                },
                _ => {
                    // Everything else goes inside main
                    main_code.push_str(&self.generate_c_code_for_node(node, 1));
                }
            }
        }
        
        // If we have a main function, add a call to smash_main in our C main
        if has_main_function {
            main_code.push_str("    char* result = smash_main();\n");
            main_code.push_str("    if (result && strlen(result) > 0) {\n");
            main_code.push_str("        printf(\"%s\\n\", result);\n");
            main_code.push_str("    }\n");
        }
        
        // Add C main function with the collected code
        code.push_str("int main(int argc, char** argv) {\n");
        code.push_str("    // Initialize the regex library\n");
        code.push_str("    if (!load_regex_library()) {\n");
        code.push_str("        fprintf(stderr, \"Failed to load regex library. Regex operations will not work.\\n\");\n");
        code.push_str("    }\n\n");
        code.push_str(&main_code);
        code.push_str("    return 0;\n");
        code.push_str("};\n");
        
        code
    }
    
    // Helper method to generate C code for a specific AST node
    fn generate_c_code_for_node(&self, node: &AstNode, indent_level: usize) -> String {
        let indent = "    ".repeat(indent_level);
        let mut code = String::new();
        
        match node {
            AstNode::Block(statements) => {
                code.push_str("\n");
                for stmt in statements {
                    code.push_str(&self.generate_c_code_for_node(stmt, indent_level));
                }
            },
            AstNode::LetDecl { name, value } => {
                code.push_str(&format!("{indent}char* {name} = "));
                code.push_str(&self.generate_c_code_for_expr(value, indent_level));
                code.push_str(";\n");
            },
            AstNode::ConstDecl { name, value } => {
                code.push_str(&format!("{indent}const char* {name} = "));
                code.push_str(&self.generate_c_code_for_expr(value, indent_level));
                code.push_str(";\n");
            },
            AstNode::FunctionCall { name, args } => {
                if name == "print" {
                    // Special handling for print function
                    if args.len() > 0 {
                        match &args[0] {
                            AstNode::String(s) => {
                                // For string literals, print them directly
                                let escaped = s.replace("\"", "\\\"");
                                code.push_str(&format!("{indent}printf(\"%s\\n\", \"{}\");\n", escaped));
                            },
                            _ => {
                                // For other expressions, evaluate them
                                code.push_str(&format!("{indent}printf(\"%s\\n\", "));
                                code.push_str(&self.generate_c_code_for_expr(&args[0], indent_level));
                                code.push_str(");\n");
                            }
                        }
                    }
                } else {
                    // Regular function call
                    code.push_str(&format!("{indent}{name}("));
                    for (i, arg) in args.iter().enumerate() {
                        if i > 0 {
                            code.push_str(", ");
                        }
                        code.push_str(&self.generate_c_code_for_expr(arg, indent_level));
                    }
                    code.push_str(");\n");
                }
            },
            AstNode::If { condition, then_branch, else_branch } => {
                code.push_str(&format!("{indent}if ("));
                code.push_str(&self.generate_c_code_for_expr(condition, indent_level));
                code.push_str(") {\n");
                
                // then_branch is a Box<AstNode>, not an Option
                code.push_str(&self.generate_c_code_for_node(then_branch, indent_level + 1));
                
                code.push_str(&format!("{}}}\n", indent));
                
                if let Some(else_node) = else_branch {
                    code.push_str(&format!("{}else {{\n", indent));
                    code.push_str(&self.generate_c_code_for_node(else_node, indent_level + 1));
                    code.push_str(&format!("{}}}\n", indent));
                }
            },
            AstNode::Return(expr) => {
                code.push_str(&format!("{indent}return "));
                code.push_str(&self.generate_c_code_for_expr(expr, indent_level));
                code.push_str(";\n");
            },
            AstNode::Function { name, params, body } => {
                // Generate function declaration
                code.push_str(&format!("{indent}char* {name}("));
                
                // Add parameters
                for (i, param) in params.iter().enumerate() {
                    if i > 0 {
                        code.push_str(", ");
                    }
                    code.push_str(&format!("char* {param}"));
                }
                code.push_str(") {\n");
                
                // Add function body
                for stmt in body {
                    code.push_str(&self.generate_c_code_for_node(stmt, indent_level + 1));
                }
                
                // Add default return if none is present
                code.push_str(&format!("{indent}    return \"\";\n"));
                
                // Close function
                code.push_str(&format!("{}}}\n\n", indent));
            },
            _ => {
                code.push_str(&format!("{indent}// Unimplemented AST node type\n"));
            }
        }
        
        code
    }
    
    // Helper method to generate C code for expressions
    fn generate_c_code_for_expr(&self, expr: &AstNode, indent_level: usize) -> String {
        match expr {
            AstNode::Number(n) => {
                // Convert number to string
                format!("(char*)\"{}\"" , n)
            },
            AstNode::String(s) => {
                // Escape quotes in the string
                let escaped = s.replace("\"", "\\\"");
                format!("\"{}\"" , escaped)
            },
            AstNode::Boolean(b) => {
                if *b {
                    "\"true\"".to_string()
                } else {
                    "\"false\"".to_string()
                }
            },
            AstNode::Identifier(name) => {
                name.clone()
            },
            AstNode::BinaryOp { left, op, right } => {
                let left_code = self.generate_c_code_for_expr(left, indent_level);
                let right_code = self.generate_c_code_for_expr(right, indent_level);
                
                match op.as_str() {
                    "+" => format!("smash_string_concat({}, {})", left_code, right_code),
                    "<" => format!("(strcmp({}, {}) < 0 ? \"true\" : \"false\")", left_code, right_code),
                    ">" => format!("(strcmp({}, {}) > 0 ? \"true\" : \"false\")", left_code, right_code),
                    "<=" => format!("(strcmp({}, {}) <= 0 ? \"true\" : \"false\")", left_code, right_code),
                    ">=" => format!("(strcmp({}, {}) >= 0 ? \"true\" : \"false\")", left_code, right_code),
                    "==" => format!("(strcmp({}, {}) == 0 ? \"true\" : \"false\")", left_code, right_code),
                    "!=" => format!("(strcmp({}, {}) != 0 ? \"true\" : \"false\")", left_code, right_code),
                    "&&" => format!("(strcmp({}, \"true\") == 0 && strcmp({}, \"true\") == 0 ? \"true\" : \"false\")", left_code, right_code),
                    "||" => format!("(strcmp({}, \"true\") == 0 || strcmp({}, \"true\") == 0 ? \"true\" : \"false\")", left_code, right_code),
                    _ => format!("/* Unsupported operator: {} */ \"{} {} {}\"" , op, left_code, op, right_code)
                }
            },
            AstNode::UnaryOp { op, expr } => {
                let expr_code = self.generate_c_code_for_expr(expr, indent_level);
                
                match op.as_str() {
                    "!" => format!("(strcmp({}, \"true\") == 0 ? \"false\" : \"true\")", expr_code),
                    _ => format!("/* Unsupported unary operator: {} */ \"{}{}\"", op, op, expr_code)
                }
            },
            AstNode::TernaryOp { condition, true_expr, false_expr } => {
                let cond_code = self.generate_c_code_for_expr(condition, indent_level);
                let then_code = self.generate_c_code_for_expr(true_expr, indent_level);
                let else_code = self.generate_c_code_for_expr(false_expr, indent_level);
                
                format!("(strcmp({}, \"true\") == 0 ? {} : {})", cond_code, then_code, else_code)
            },
            AstNode::TemplateLiteral(parts) => {
                // For empty template literals, return an empty string
                if parts.is_empty() {
                    return "\"\"".to_string();
                }
                
                // Start with an empty string if we need to concatenate multiple parts
                let mut result = String::new();
                let mut is_first = true;
                
                for part in parts {
                    let part_code = self.generate_c_code_for_expr(part, indent_level);
                    
                    if is_first {
                        // First part doesn't need concatenation
                        result = part_code;
                        is_first = false;
                    } else {
                        // Concatenate with previous parts
                        result = format!("smash_string_concat({}, {})", result, part_code);
                    }
                }
                
                result
            },
            AstNode::FunctionCall { name, args } => {
                let mut call = format!("{name}(");
                
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        call.push_str(", ");
                    }
                    call.push_str(&self.generate_c_code_for_expr(arg, indent_level));
                }
                
                call.push_str(")");
                call
            },
            AstNode::PropertyAccess { object, property } => {
                let obj_code = self.generate_c_code_for_expr(object, indent_level);
                
                // Handle common properties based on the object type
                // For now, we'll only implement a few common properties like length
                if property == "length" {
                    // For strings, length is the string length
                    format!("smash_get_length({})", obj_code)
                } else {
                    // For other properties, we'll just return a placeholder
                    format!("\"Property {} not implemented\"", property)
                }
            },
            AstNode::MethodCall { object, method, args } => {
                let obj_code = self.generate_c_code_for_expr(object, indent_level);
                
                // Generate code for the arguments
                let mut arg_codes = Vec::new();
                for arg in args {
                    arg_codes.push(self.generate_c_code_for_expr(arg, indent_level));
                }
                
                // Handle common methods based on the object type and method name
                match method.as_str() {
                    // Regex methods
                    "test" => {
                        if arg_codes.len() < 1 {
                            format!("\"Error: test requires a string to test\"")
                        } else {
                            format!("(smash_regex_test({}, {}) ? \"true\" : \"false\")", obj_code, arg_codes[0])
                        }
                    },
                    "match" => {
                        if arg_codes.len() < 1 {
                            format!("\"Error: match requires a string to match\"")
                        } else {
                            format!("smash_regex_match({}, {})", obj_code, arg_codes[0])
                        }
                    },
                    "replace" => {
                        if arg_codes.len() < 2 {
                            format!("\"Error: replace requires a pattern and replacement\"")
                        } else {
                            format!("smash_regex_replace({}, {}, {})", obj_code, arg_codes[0], arg_codes[1])
                        }
                    },
                    "map" => {
                        // For array.map, we need to implement a map function in C
                        // This is a simplified implementation that assumes the first argument is a function
                        if arg_codes.len() < 1 {
                            format!("\"Error: map requires a callback function\"")
                        } else {
                            format!("smash_array_map({}, {})", obj_code, arg_codes[0])
                        }
                    },
                    "filter" => {
                        // For array.filter, we need to implement a filter function in C
                        // This is a simplified implementation that assumes the first argument is a function
                        if arg_codes.len() < 1 {
                            format!("\"Error: filter requires a callback function\"")
                        } else {
                            format!("smash_array_filter({}, {})", obj_code, arg_codes[0])
                        }
                    },
                    "push" => {
                        // For array.push, add an element to the array
                        if arg_codes.len() < 1 {
                            format!("\"Error: push requires at least one argument\"")
                        } else {
                            let args_str = arg_codes.join(", ");
                            format!("smash_array_push({}, {})", obj_code, args_str)
                        }
                    },
                    "pop" => {
                        // For array.pop, remove the last element from the array
                        format!("smash_array_pop({})", obj_code)
                    },
                    // String methods
                    "toUpperCase" => {
                        format!("smash_string_to_upper({})", obj_code)
                    },
                    "toLowerCase" => {
                        format!("smash_string_to_lower({})", obj_code)
                    },
                    "trim" => {
                        format!("smash_string_trim({})", obj_code)
                    },
                    "trimStart" => {
                        format!("smash_string_trim_start({})", obj_code)
                    },
                    "trimEnd" => {
                        format!("smash_string_trim_end({})", obj_code)
                    },
                    "charAt" => {
                        if arg_codes.len() < 1 {
                            format!("\"Error: charAt requires an index\"")
                        } else {
                            format!("smash_string_char_at({}, {})", obj_code, arg_codes[0])
                        }
                    },
                    "concat" => {
                        if arg_codes.len() < 1 {
                            format!("\"Error: concat requires a string\"")
                        } else {
                            format!("smash_string_concat({}, {})", obj_code, arg_codes[0])
                        }
                    },
                    "includes" => {
                        if arg_codes.len() < 1 {
                            format!("\"Error: includes requires a search string\"")
                        } else {
                            format!("smash_string_includes({}, {})", obj_code, arg_codes[0])
                        }
                    },
                    "indexOf" => {
                        if arg_codes.len() < 1 {
                            format!("\"Error: indexOf requires a search string\"")
                        } else {
                            format!("smash_string_index_of({}, {})", obj_code, arg_codes[0])
                        }
                    },
                    "split" => {
                        if arg_codes.len() < 1 {
                            format!("\"Error: split requires a delimiter\"")
                        } else {
                            format!("smash_string_split({}, {})", obj_code, arg_codes[0])
                        }
                    },
                    "repeat" => {
                        if arg_codes.len() < 1 {
                            format!("\"Error: repeat requires a count\"")
                        } else {
                            format!("smash_string_repeat({}, {})", obj_code, arg_codes[0])
                        }
                    },
                    
                    // Number methods
                    "toFixed" => {
                        if arg_codes.len() < 1 {
                            format!("\"Error: toFixed requires decimal places\"")
                        } else {
                            format!("smash_number_to_fixed({}, {})", obj_code, arg_codes[0])
                        }
                    },
                    "toPrecision" => {
                        if arg_codes.len() < 1 {
                            format!("\"Error: toPrecision requires precision\"")
                        } else {
                            format!("smash_number_to_precision({}, {})", obj_code, arg_codes[0])
                        }
                    },
                    "toExponential" => {
                        if arg_codes.len() < 1 {
                            format!("\"Error: toExponential requires decimal places\"")
                        } else {
                            format!("smash_number_to_exponential({}, {})", obj_code, arg_codes[0])
                        }
                    },
                    
                    // Array methods (some already implemented above)
                    "forEach" => {
                        if arg_codes.len() < 1 {
                            format!("\"Error: forEach requires a callback function\"")
                        } else {
                            format!("smash_array_for_each({}, {})", obj_code, arg_codes[0])
                        }
                    },
                    "find" => {
                        if arg_codes.len() < 1 {
                            format!("\"Error: find requires a callback function\"")
                        } else {
                            format!("smash_array_find({}, {})", obj_code, arg_codes[0])
                        }
                    },
                    "join" => {
                        let separator = if arg_codes.len() > 0 { arg_codes[0].clone() } else { "\",\"" .to_string() };
                        format!("smash_array_join({}, {})", obj_code, separator)
                    },
                    "reverse" => {
                        format!("smash_array_reverse({})", obj_code)
                    },
                    
                    // Object methods
                    "hasOwnProperty" => {
                        if arg_codes.len() < 1 {
                            format!("\"Error: hasOwnProperty requires a property name\"")
                        } else {
                            format!("smash_object_has_own_property({}, {})", obj_code, arg_codes[0])
                        }
                    },
                    "keys" => {
                        format!("smash_object_keys({})", obj_code)
                    },
                    "values" => {
                        format!("smash_object_values({})", obj_code)
                    },
                    "entries" => {
                        format!("smash_object_entries({})", obj_code)
                    },
                    
                    // String methods that use regex
                    
                    // Common methods that might be used by different types
                    "toString" => {
                        // Handle toString based on object type
                        format!("smash_to_string({})", obj_code)
                    },
                    "valueOf" => {
                        // Handle valueOf based on object type
                        format!("smash_value_of({})", obj_code)
                    },
                    "slice" => {
                        // Handle slice based on object type (string or array)
                        if arg_codes.len() < 2 {
                            format!("\"Error: slice requires start and end indices\"")
                        } else {
                            format!("smash_slice({}, {}, {})", obj_code, arg_codes[0], arg_codes[1])
                        }
                    },
                    _ => {
                        // For other methods, we'll just return a placeholder
                        format!("\"Method {} not implemented\"", method)
                    }
                }
            },
            AstNode::Regex(pattern) => {
                // Parse the regex pattern and flags
                let (pattern_str, flags) = parse_regex_pattern(pattern);
                
                // Create a regex object using our Rust implementation
                format!("smash_regex_create(\"{}\", \"{}\")", pattern_str, flags)
            },
            _ => {
                format!("\"Unsupported expression type\"" )
            }
        }
    }
}

// Generate LLVM IR (simplified to C code for now)
pub fn generate_llvm_ir<'a>(
    ast: &'a [AstNode],
    target: Option<&str>,
) -> (Module<'a>, TargetMachine) {
    // Create a module with the AST
    let module = Module::new(ast);
    
    // Create a target machine with the specified target triple
    let target_triple = match target {
        Some(t) => t,
        None => "x86_64-unknown-linux-gnu", // Default target
    };
    
    let target_machine = TargetMachine::new(target_triple);
    
    (module, target_machine)
}
