#!/bin/bash

# SmashLang Examples Test Script
# This script compiles and runs all the example programs in the getting-started directory

# Set the directory containing the examples
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
EXAMPLES_DIR="$SCRIPT_DIR"

# Change to the examples directory
cd "$EXAMPLES_DIR" || { echo "Error: Could not change to directory $EXAMPLES_DIR"; exit 1; }

echo "===== SmashLang Examples Test Script ====="
echo "Testing all examples in $EXAMPLES_DIR"
echo ""

# Function to compile and run an example
run_example() {
    local example=$1
    local output_name=$(basename "$example" .smash)
    
    echo "Testing example: $example"
    echo "-----------------------------------"
    
    # Display the source code
    echo "Source code:"
    cat "$example"
    echo ""
    
    # Compile the example
    echo "Compiling..."
    smashc "$example" -o "$output_name"
    if [ $? -ne 0 ]; then
        echo "Compilation failed!"
        return 1
    fi
    
    # Run the compiled program
    echo "Running..."
    ./$output_name
    if [ $? -ne 0 ]; then
        echo "Execution failed!"
        return 1
    fi
    
    echo "Success!"
    echo ""
    
    # Clean up
    rm -f "$output_name"
    return 0
}

# Find all .smash files and run them
EXAMPLE_FILES=$(find "$EXAMPLES_DIR" -name "*.smash" 2>/dev/null | sort)

# Check if any examples were found
if [ -z "$EXAMPLE_FILES" ]; then
    echo "Warning: No .smash example files found in $EXAMPLES_DIR"
    echo "Please check that the examples are in the correct location"
    exit 0
fi

# Run each example
FAILED=0
while read -r example; do
    if [ -n "$example" ]; then
        run_example "$example"
        if [ $? -ne 0 ]; then
            echo "Test failed for $example"
            FAILED=1
        fi
    fi
done <<< "$EXAMPLE_FILES"

if [ $FAILED -eq 0 ]; then
    echo "All examples compiled and ran successfully!"
    exit 0
else
    echo "Some examples failed to compile or run."
    exit 1
fi
