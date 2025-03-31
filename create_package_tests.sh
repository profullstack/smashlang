#!/bin/bash

# Script to ensure each SmashLang package has a valid test file

set -e

# Define colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Base directories
BASE_DIR="$(pwd)"
PACKAGES_DIR="${BASE_DIR}/smashlang_packages"
TEMPLATE_DIR="${PACKAGES_DIR}/__package__template"

# Check if the template directory exists
if [ ! -d "$TEMPLATE_DIR" ]; then
    echo -e "${RED}Error: Template directory not found at $TEMPLATE_DIR${NC}"
    exit 1
fi

# Function to create a test file for a package
create_package_test() {
    local package_dir="$1"
    local package_name="$(basename "$package_dir")"
    local parent_package="$(basename "$(dirname "$package_dir")")"
    
    # Skip the template itself
    if [ "$package_name" == "__package__template" ]; then
        return
    fi
    
    # Skip standard directories
    if [ "$package_name" == "assets" ] || [ "$package_name" == "examples" ] || \
       [ "$package_name" == "src" ] || [ "$package_name" == "tests" ]; then
        return
    fi
    
    echo -e "${BLUE}Processing package: $parent_package/$package_name${NC}"
    
    # Create tests directory if it doesn't exist
    if [ ! -d "$package_dir/tests" ]; then
        echo -e "  ${YELLOW}Creating tests directory${NC}"
        mkdir -p "$package_dir/tests"
    fi
    
    # Check if index.test.smash exists
    if [ ! -f "$package_dir/tests/index.test.smash" ]; then
        echo -e "  ${YELLOW}Creating index.test.smash${NC}"
        
        # Determine the import path
        if [ "$parent_package" == "smashlang_packages" ]; then
            import_path="$package_name"
        else
            import_path="$parent_package/$package_name"
        fi
        
        # Create the test file
        cat > "$package_dir/tests/index.test.smash" << EOF
// $import_path/tests/index.test.smash - Test file for $package_name package

import * from '$import_path';
import { test, describe, expect, beforeEach, afterEach } from 'std/testing';

describe('Package: $package_name', () => {
    beforeEach(() => {
        // Setup code for each test
        console.log('Setting up test for $package_name');
    });

    afterEach(() => {
        // Cleanup code for each test
        console.log('Cleaning up after test for $package_name');
    });

    test('package can be imported correctly', () => {
        // This test verifies that the package can be imported
        expect(typeof $package_name).toBe('object');
    });

    // Add more specific tests for the package functionality
    test('package has expected exports', () => {
        // Check for expected exports based on the package
        // This is a basic test that should be customized for each package
        expect($package_name).not.toBe(null);
        expect($package_name).not.toBe(undefined);
    });
});

// Add more specific test cases below
// Example:
// test('specific function works as expected', () => {
//     const result = $package_name.someFunction();
//     expect(result).toBe(expectedValue);
// });
EOF
        
        echo -e "  ${GREEN}Created index.test.smash for $package_name${NC}"
    else
        echo -e "  ${GREEN}index.test.smash already exists${NC}"
    fi
    
    # Check if there's a src/index.smash file to extract exports for testing
    if [ -f "$package_dir/src/index.smash" ]; then
        # Extract exported functions and classes for testing
        local exports=$(grep -E 'export (function|class|const|let|var)' "$package_dir/src/index.smash" | sed -E 's/export (function|class|const|let|var) ([a-zA-Z0-9_]+).*/\2/g')
        
        if [ -n "$exports" ]; then
            echo -e "  ${YELLOW}Found exports in src/index.smash, updating tests${NC}"
            
            # Check if we need to add specific tests for exports
            if ! grep -q "Add specific tests for exports" "$package_dir/tests/index.test.smash"; then
                # Add specific tests for each export
                cat >> "$package_dir/tests/index.test.smash" << EOF

// Specific tests for exported items
describe('Exported items', () => {
EOF
                
                # Add a test for each export
                for export_name in $exports; do
                    cat >> "$package_dir/tests/index.test.smash" << EOF
    test('$export_name is exported correctly', () => {
        expect(typeof $package_name.$export_name).not.toBe('undefined');
    });
EOF
                done
                
                # Close the describe block
                cat >> "$package_dir/tests/index.test.smash" << EOF
});
EOF
                
                echo -e "  ${GREEN}Added specific tests for exports${NC}"
            fi
        fi
    fi
    
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
            if [ "$package_name" != "assets" ] && [ "$package_name" != "examples" ] && \
               [ "$package_name" != "src" ] && [ "$package_name" != "tests" ]; then
                # Create test for this package
                create_package_test "$package_dir"
                
                # Process subpackages recursively
                process_packages "$package_dir"
            fi
        fi
    done
}

# Process all top-level packages
echo -e "${BLUE}Starting package test creation...${NC}"
echo ""

process_packages "$PACKAGES_DIR"

echo -e "${GREEN}Package test creation complete!${NC}"

# Create a script to run all tests
echo -e "${BLUE}Creating script to run all tests...${NC}"

cat > "${BASE_DIR}/run_all_package_tests.sh" << EOF
#!/bin/bash

# Script to run all SmashLang package tests

set -e

# Define colors for output
RED='\\033[0;31m'
GREEN='\\033[0;32m'
YELLOW='\\033[0;33m'
BLUE='\\033[0;34m'
NC='\\033[0m' # No Color

# Base directory
BASE_DIR="\$(pwd)"
PACKAGES_DIR="\${BASE_DIR}/smashlang_packages"

# Create logs directory
mkdir -p "\${BASE_DIR}/logs"

# Function to run tests for a package
run_package_tests() {
    local package_dir="\$1"
    local package_name="\$(basename "\$package_dir")"
    local parent_package="\$(basename "\$(dirname "\$package_dir")")"
    
    # Skip standard directories
    if [ "\$package_name" == "assets" ] || [ "\$package_name" == "examples" ] || \
       [ "\$package_name" == "src" ] || [ "\$package_name" == "tests" ]; then
        return
    fi
    
    # Skip the template
    if [ "\$package_name" == "__package__template" ]; then
        return
    fi
    
    # Determine the package path
    if [ "\$parent_package" == "smashlang_packages" ]; then
        package_path="\$package_name"
    else
        package_path="\$parent_package/\$package_name"
    fi
    
    echo -e "\${BLUE}Running tests for package: \$package_path${NC}"
    
    # Check if tests directory exists
    if [ ! -d "\$package_dir/tests" ]; then
        echo -e "  \${YELLOW}No tests directory found${NC}"
        return
    fi
    
    # Find all test files
    test_files=(\$(find "\$package_dir/tests" -name "*.test.smash"))
    
    if [ \${#test_files[@]} -eq 0 ]; then
        echo -e "  \${YELLOW}No test files found${NC}"
        return
    fi
    
    # Run each test file
    for test_file in "\${test_files[@]}"; do
        test_name="\$(basename "\$test_file")"
        log_file="\${BASE_DIR}/logs/\$package_path-\$test_name.log"
        
        echo -e "  \${YELLOW}Running \$test_name${NC}"
        
        # Run the test and capture output
        if smash "\$test_file" --test > "\$log_file" 2>&1; then
            echo -e "  \${GREEN}✓ Test passed: \$test_name${NC}"
        else
            echo -e "  \${RED}✗ Test failed: \$test_name${NC}"
            echo -e "  \${YELLOW}See log file for details: \$log_file${NC}"
        fi
    done
    
    echo ""
}

# Function to process all packages in a directory
process_packages() {
    local dir="\$1"
    
    # Process all packages in the directory
    for package_dir in "\$dir"/*/; do
        if [ -d "\$package_dir" ]; then
            # Get the package name
            local package_name="\$(basename "\$package_dir")"
            
            # Skip standard directories
            if [ "\$package_name" != "assets" ] && [ "\$package_name" != "examples" ] && \
               [ "\$package_name" != "src" ] && [ "\$package_name" != "tests" ]; then
                # Run tests for this package
                run_package_tests "\$package_dir"
                
                # Process subpackages recursively
                process_packages "\$package_dir"
            fi
        fi
    done
}

# Process all top-level packages
echo -e "\${BLUE}Starting package tests...${NC}"
echo ""

process_packages "\$PACKAGES_DIR"

echo -e "\${GREEN}All package tests completed!${NC}"
EOF

chmod +x "${BASE_DIR}/run_all_package_tests.sh"

echo -e "${GREEN}Created run_all_package_tests.sh${NC}"
echo -e "${BLUE}To run all package tests, execute: ./run_all_package_tests.sh${NC}"
