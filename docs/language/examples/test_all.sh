#!/bin/bash
# Script to run all SmashLang example files in the docs/language/examples directory

# Set colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Get the directory of this script
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Find all .smash files in the directory
SMASH_FILES=$(find "$SCRIPT_DIR" -name "*.smash" | sort)

# Count total files
TOTAL_FILES=$(echo "$SMASH_FILES" | wc -l)
PASSED=0
FAILED=0

echo -e "${YELLOW}Running all SmashLang examples ($TOTAL_FILES files)...${NC}"
echo

# Function to run a single example file
run_example() {
    local file=$1
    local filename=$(basename "$file")
    
    echo -e "${YELLOW}Testing: ${filename}${NC}"
    echo -e "${YELLOW}----------------------------------------${NC}"
    
    # Run the file using smashc
    if cargo run --bin smashc "$file" > /tmp/smash_output.log 2>&1; then
        echo -e "${GREEN}✓ PASSED${NC}"
        PASSED=$((PASSED + 1))
    else
        echo -e "${RED}✗ FAILED${NC}"
        echo -e "${RED}Error output:${NC}"
        cat /tmp/smash_output.log
        FAILED=$((FAILED + 1))
    fi
    
    echo
}

# Run each example file
for file in $SMASH_FILES; do
    run_example "$file"
done

# Print summary
echo -e "${YELLOW}----------------------------------------${NC}"
echo -e "${YELLOW}Summary:${NC}"
echo -e "${GREEN}Passed: $PASSED${NC}"
echo -e "${RED}Failed: $FAILED${NC}"
echo -e "${YELLOW}Total: $TOTAL_FILES${NC}"

if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}All examples passed!${NC}"
    exit 0
else
    echo -e "${RED}Some examples failed.${NC}"
    exit 1
fi