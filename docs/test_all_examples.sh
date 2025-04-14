#!/bin/bash
# Master script to run all SmashLang examples across all documentation categories

# Set colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Get the directory of this script
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Initialize counters
TOTAL_FILES=0
PASSED=0
FAILED=0

# Function to run a single example file
run_example() {
    local file=$1
    local filename=$(basename "$file")
    local category=$(echo "$file" | sed "s|$SCRIPT_DIR/||" | sed "s|/examples/$filename||")
    
    echo -e "${YELLOW}Testing: ${CYAN}$category/${NC}${filename}"
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

# Function to process a directory of examples
process_directory() {
    local dir=$1
    local examples_dir="$dir/examples"
    
    if [ -d "$examples_dir" ]; then
        echo -e "${BLUE}Processing examples in: ${CYAN}$(echo "$examples_dir" | sed "s|$SCRIPT_DIR/||")${NC}"
        echo -e "${YELLOW}========================================${NC}"
        
        # Find all .smash files in the directory
        local smash_files=$(find "$examples_dir" -name "*.smash" | sort)
        local count=$(echo "$smash_files" | wc -l)
        
        if [ -n "$smash_files" ]; then
            TOTAL_FILES=$((TOTAL_FILES + count))
            echo -e "${BLUE}Found ${count} example files${NC}"
            echo
            
            # Run each example file
            while IFS= read -r file; do
                run_example "$file"
            done <<< "$smash_files"
        else
            echo -e "${YELLOW}No .smash files found in $examples_dir${NC}"
            echo
        fi
    fi
}

# Main execution
echo -e "${CYAN}SmashLang Examples Test Runner${NC}"
echo -e "${YELLOW}========================================${NC}"

# Process each category directory
for category_dir in "$SCRIPT_DIR"/*; do
    if [ -d "$category_dir" ]; then
        process_directory "$category_dir"
    fi
done

# Print summary
echo -e "${YELLOW}========================================${NC}"
echo -e "${CYAN}Test Summary:${NC}"
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