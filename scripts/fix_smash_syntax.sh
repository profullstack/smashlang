#!/bin/bash

# fix_smash_syntax.sh - Script to fix SmashLang syntax inconsistencies across all packages

echo "===== SmashLang Syntax Fixer ====="
echo "Fixing JavaScript-like syntax to proper SmashLang syntax in all .smash files"

# Find all .smash files in the packages directory
SMASH_FILES=$(find /home/ettinger/src/profullstack.com/smashlang/smashlang_packages -name "*.smash")

TOTAL_FILES=$(echo "$SMASH_FILES" | wc -l)
echo "Found $TOTAL_FILES .smash files to process"

FIXED_COUNT=0

# Process each file
for file in $SMASH_FILES; do
    echo "Processing: $file"
    
    # Replace console.log with print
    if grep -q "console\.log" "$file"; then
        sed -i 's/console\.log(/print(/g' "$file"
        echo "  - Fixed console.log → print"
        FIXED_COUNT=$((FIXED_COUNT + 1))
    fi
    
    # Replace function with fn
    if grep -q "function " "$file"; then
        sed -i 's/function /fn /g' "$file"
        echo "  - Fixed function → fn"
        FIXED_COUNT=$((FIXED_COUNT + 1))
    fi
    
    # Replace export function with export fn
    if grep -q "export function " "$file"; then
        sed -i 's/export function /export fn /g' "$file"
        echo "  - Fixed export function → export fn"
        FIXED_COUNT=$((FIXED_COUNT + 1))
    fi
    
    # Replace async function with async fn
    if grep -q "async function " "$file"; then
        sed -i 's/async function /async fn /g' "$file"
        echo "  - Fixed async function → async fn"
        FIXED_COUNT=$((FIXED_COUNT + 1))
    fi
    
    # Replace export async function with export async fn
    if grep -q "export async function " "$file"; then
        sed -i 's/export async function /export async fn /g' "$file"
        echo "  - Fixed export async function → export async fn"
        FIXED_COUNT=$((FIXED_COUNT + 1))
    fi
    
    # Replace process.cwd() with std.cwd()
    if grep -q "process\.cwd()" "$file"; then
        sed -i 's/process\.cwd()/std.cwd()/g' "$file"
        echo "  - Fixed process.cwd() → std.cwd()"
        FIXED_COUNT=$((FIXED_COUNT + 1))
    fi
    
    # Replace require() with import
    if grep -q "require(" "$file"; then
        echo "  - Found require() statements that may need manual conversion to import"
    fi
done

echo "===== Summary ====="
echo "Processed $TOTAL_FILES .smash files"
echo "Fixed $FIXED_COUNT syntax issues"
echo "Done!"
