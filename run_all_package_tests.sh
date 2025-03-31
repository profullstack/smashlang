#!/bin/bash

# Script to run all SmashLang package tests

set -e

# Define colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Base directory
BASE_DIR="$(pwd)"
PACKAGES_DIR="${BASE_DIR}/smashlang_packages"

# Create logs directory
mkdir -p "${BASE_DIR}/logs"

# Function to run tests for a package
run_package_tests() {
    local package_dir="$1"
    local package_name="$(basename "$package_dir")"
    local parent_package="$(basename "$(dirname "$package_dir")")"
    
    # Skip standard directories
    if [ "$package_name" == "assets" ] || [ "$package_name" == "examples" ] ||        [ "$package_name" == "src" ] || [ "$package_name" == "tests" ]; then
        return
    fi
    
    # Skip the template
    if [ "$package_name" == "__package__template" ]; then
        return
    fi
    
    # Determine the package path
    if [ "$parent_package" == "smashlang_packages" ]; then
        package_path="$package_name"
    else
        package_path="$parent_package/$package_name"
    fi
    
    echo -e "${BLUE}Running tests for package: $package_path\033[0m"
    
    # Check if tests directory exists
    if [ ! -d "$package_dir/tests" ]; then
        echo -e "  ${YELLOW}No tests directory found\033[0m"
        return
    fi
    
    # Find all test files
    test_files=($(find "$package_dir/tests" -name "*.test.smash"))
    
    if [ ${#test_files[@]} -eq 0 ]; then
        echo -e "  ${YELLOW}No test files found\033[0m"
        return
    fi
    
    # Run each test file
    for test_file in "${test_files[@]}"; do
        test_name="$(basename "$test_file")"
        log_file="${BASE_DIR}/logs/$package_path-$test_name.log"
        
        echo -e "  ${YELLOW}Running $test_name\033[0m"
        
        # Run the test and capture output
        if smash "$test_file" --test > "$log_file" 2>&1; then
            echo -e "  ${GREEN}✓ Test passed: $test_name\033[0m"
        else
            echo -e "  ${RED}✗ Test failed: $test_name\033[0m"
            echo -e "  ${YELLOW}See log file for details: $log_file\033[0m"
        fi
    done
    
    echo ""
}

# Function to process all packages in a directory
process_packages() {
    local dir="$1"
    
    # Process all packages in the directory
    for package_dir in "$dir"/*/; do
        if [ -d "$package_dir" ]; then
            # Get the package name
            local package_name="$(basename "$package_dir")"
            
            # Skip standard directories
            if [ "$package_name" != "assets" ] && [ "$package_name" != "examples" ] &&                [ "$package_name" != "src" ] && [ "$package_name" != "tests" ]; then
                # Run tests for this package
                run_package_tests "$package_dir"
                
                # Process subpackages recursively
                process_packages "$package_dir"
            fi
        fi
    done
}

# Process all top-level packages
echo -e "${BLUE}Starting package tests...\033[0m"
echo ""

process_packages "$PACKAGES_DIR"

echo -e "${GREEN}All package tests completed!\033[0m"
