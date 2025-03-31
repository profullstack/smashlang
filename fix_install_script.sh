#!/bin/bash

# This script fixes the install.sh script to properly run tests and output a test results log

set -e

# Check if install.sh exists
if [ ! -f "install.sh" ]; then
  echo "Error: install.sh not found in the current directory"
  exit 1
fi

# Create a backup of the original install.sh
cp install.sh install.sh.bak
echo "Created backup of install.sh as install.sh.bak"

# Create a clean version of the install.sh script
cp install.sh.bak install.sh

# Create the enhanced run_tests function with logging
cat > run_tests_function.txt << 'EOF'
# Run tests for SmashLang
run_tests() {
  local repo_dir=
