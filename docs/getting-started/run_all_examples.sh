#!/bin/bash

# SmashLang Examples Test Script
# This script compiles and runs all the example programs in the getting-started directory

# Set the directory containing the examples
EXAMPLES_DIR="$(dirname "$0")"
cd "$EXAMPLES_DIR" || exit 1

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
find "$EXAMPLES_DIR" -name "*.smash" | sort | while read -r example; do
    run_example "$example"
    if [ $? -ne 0 ]; then
        echo "Test failed for $example"
        exit 1
    fi
done

echo "All examples compiled and ran successfully!"
