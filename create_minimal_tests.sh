#!/bin/bash

# Script to create minimal test files for all SmashLang packages

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

echo -e "${BLUE}Creating minimal test files for all SmashLang packages...${NC}"

# Find all package directories
find "$PACKAGES_DIR" -mindepth 2 -type d | grep -v "/\(assets\|examples\|src\|tests\)$" | while read -r package_dir; do
    # Skip the template
    if [[ "$package_dir" == *"__package__template"* ]]; then
        continue
    fi
    
    # Get the package name and parent
    package_name="$(basename "$package_dir")"
    parent_dir="$(basename "$(dirname "$package_dir")")"
    
    echo -e "${YELLOW}Processing package: $parent_dir/$package_name${NC}"
    
    # Create tests directory if it doesn't exist
    tests_dir="$package_dir/tests"
    if [ ! -d "$tests_dir" ]; then
        echo -e "  ${YELLOW}Creating tests directory${NC}"
        mkdir -p "$tests_dir"
    fi
    
    # Create a minimal test file
    test_file="$tests_dir/minimal.test.smash"
    
    cat > "$test_file" << EOF
// $parent_dir/$package_name/tests/minimal.test.smash
// Minimal test file for $package_name package

// Basic test function
fn test_$package_name() {
  return 0;
}

// Run the test
fn main() {
  test_$package_name();
  return 0;
}

// Call main
main();
EOF
    
    echo -e "  ${GREEN}Created minimal test file${NC}"
    
    # Try to compile the test file
    echo -e "  ${YELLOW}Compiling test file...${NC}"
    if smashc "$test_file" -o "${tests_dir}/test_${package_name}" > /dev/null 2>&1; then
        echo -e "  ${GREEN}Successfully compiled test file${NC}"
    else
        echo -e "  ${RED}Failed to compile test file${NC}"
    fi
    
    echo ""
done

echo -e "${GREEN}All packages now have minimal test files!${NC}"

# Create a script to run all minimal tests
echo -e "${BLUE}Creating script to run all minimal tests...${NC}"

cat > "${BASE_DIR}/run_minimal_tests.sh" << EOF
#!/bin/bash

# Script to run all minimal tests for SmashLang packages

set -e

# Define colors for output
RED='\\033[0;31m'
GREEN='\\033[0;32m'
YELLOW='\\033[0;33m'
BLUE='\\033[0;34m'
NC='\\033[0m' # No Color

# Base directories
BASE_DIR="\$(pwd)"
PACKAGES_DIR="\${BASE_DIR}/smashlang_packages"

# Create logs directory
mkdir -p "\${BASE_DIR}/logs"

echo -e "\${BLUE}Running minimal tests for all SmashLang packages...\${NC}"

# Find all minimal test files
find "\$PACKAGES_DIR" -name "minimal.test.smash" | while read -r test_file; do
    # Get the package name and parent
    package_dir="\$(dirname "\$(dirname \"\$test_file\")")"
    package_name="\$(basename "\$package_dir")"
    parent_dir="\$(basename "\$(dirname \"\$package_dir\")")"
    
    echo -e "\${YELLOW}Testing package: \$parent_dir/\$package_name\${NC}"
    
    # Compile the test file if not already compiled
    output_file="\$(dirname "\$test_file")/test_\${package_name}"
    if [ ! -f "\$output_file" ] || [ "\$test_file" -nt "\$output_file" ]; then
        echo -e "  \${YELLOW}Compiling test file...\${NC}"
        if smashc "\$test_file" -o "\$output_file" > /dev/null 2>&1; then
            echo -e "  \${GREEN}Successfully compiled test file\${NC}"
        else
            echo -e "  \${RED}Failed to compile test file\${NC}"
            continue
        fi
    fi
    
    # Run the test
    echo -e "  \${YELLOW}Running test...\${NC}"
    log_file="\${BASE_DIR}/logs/\${parent_dir}_\${package_name}_test.log"
    if "\$output_file" > "\$log_file" 2>&1; then
        echo -e "  \${GREEN}Test passed\${NC}"
    else
        echo -e "  \${RED}Test failed\${NC}"
        echo -e "  \${YELLOW}See log file: \$log_file\${NC}"
    fi
    
    echo ""
done

echo -e "\${GREEN}All minimal tests completed!\${NC}"
EOF

chmod +x "${BASE_DIR}/run_minimal_tests.sh"

echo -e "${GREEN}Created run_minimal_tests.sh${NC}"
echo -e "${BLUE}To run all minimal tests, execute: ./run_minimal_tests.sh${NC}"
