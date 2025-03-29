#!/bin/bash

# Script to fix SmashLang syntax across the codebase
# Replaces 'function' with 'fn' in function declarations

echo "Starting SmashLang syntax fixer..."

# Statistics
files_scanned=0
files_modified=0
function_replacements=0

# Root directory to scan
root_dir="${1:-.}"
echo "Scanning directory: $(realpath "$root_dir")"

# Find all .smash files
while IFS= read -r file; do
  # Skip files in node_modules or dist directories
  if [[ "$file" == *node_modules* || "$file" == *dist* ]]; then
    continue
  fi
  
  ((files_scanned++))
  
  # Count occurrences before replacement
  function_count=$(grep -c -E '\b(async\s+)?function\s+[a-zA-Z0-9_$]+\s*\(' "$file" || true)
  export_function_count=$(grep -c -E '\bexport\s+(async\s+)?function\s+[a-zA-Z0-9_$]+\s*\(' "$file" || true)
  total_count=$((function_count + export_function_count))
  
  if [ "$total_count" -gt 0 ]; then
    # Create a temporary file
    temp_file="$(mktemp)"
    
    # Replace function declarations
    sed -E 's/\b(async\s+)?function\s+([a-zA-Z0-9_$]+)\s*\(/\1fn \2(/g' "$file" |
    sed -E 's/\bexport\s+(async\s+)?function\s+([a-zA-Z0-9_$]+)\s*\(/export \1fn \2(/g' > "$temp_file"
    
    # Move the temporary file to the original file
    mv "$temp_file" "$file"
    
    ((files_modified++))
    ((function_replacements+=total_count))
    
    echo "Modified $file: replaced $total_count function declarations"
  fi
done < <(find "$root_dir" -name "*.smash" -type f)

# Print summary
echo -e "\nSummary:"
echo "Files scanned: $files_scanned"
echo "Files modified: $files_modified"
echo "Function declarations replaced: $function_replacements"

echo -e "\nDone!"
