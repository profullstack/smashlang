#!/bin/bash

# This script adds test functionality to the SmashLang install.sh script
# It modifies the install.sh script to run tests when the --master flag is used

set -e

# Check if install.sh exists
if [ ! -f "install.sh" ]; then
  echo "Error: install.sh not found in the current directory"
  exit 1
fi

# Create a backup of the original install.sh
cp install.sh install.sh.bak
echo "Created backup of install.sh as install.sh.bak"

# Add the run_tests function before the get_script_dir function
sed -i '/# Get script directory/i \
# Run tests for SmashLang\nrun_tests() {\n  local repo_dir="$1"\n  \n  echo -e "${BLUE}Running tests for SmashLang...${NC}"\n  cd "$repo_dir"\n  \n  if command -v cargo &> /dev/null; then\n    echo -e "${BLUE}Running unit tests...${NC}"\n    cargo test\n    local test_result=$?\n    \n    if [ $test_result -eq 0 ]; then\n      echo -e "${GREEN}All tests passed successfully!${NC}"\n    else\n      echo -e "${YELLOW}Warning: Some tests failed. Continuing with installation...${NC}"\n    fi\n    \n    # Run example tests if they exist\n    if [ -d "docs/getting-started" ] && [ -f "docs/getting-started/run_all_examples.sh" ]; then\n      echo -e "${BLUE}Running example tests...${NC}"\n      chmod +x "docs/getting-started/run_all_examples.sh"\n      ./docs/getting-started/run_all_examples.sh\n    fi\n  else\n    echo -e "${YELLOW}Warning: Cargo not found, skipping tests.${NC}"\n  fi\n}\n' install.sh

# Add test call to Linux installation
sed -i '/# Capture git hash for version info/,/cargo build --release/ { s/cargo build --release/# Run tests when using master branch\n      run_tests "$temp_dir"\n      \n      # Build release version\n      cargo build --release/g }' install.sh

# Add test call to macOS installation
sed -i '/git clone --depth 1 "$REPO_URL" "$temp_dir"/a \
    # Run tests when using master branch\n    run_tests "$temp_dir"' install.sh

# Add test call to Windows installation
sed -i '/git clone --depth 1 "$REPO_URL" "$temp_dir"/a \
    # Run tests when using master branch\n    run_tests "$temp_dir"' install.sh

echo "Successfully updated install.sh to run tests when using the --master flag"
echo "To apply these changes, run: chmod +x install_test_patch.sh && ./install_test_patch.sh"
