#!/bin/bash

# This script fixes the install.sh script to properly run tests for all packages when using the --master flag

set -e

# Check if install.sh exists
if [ ! -f "install.sh" ]; then
  echo "Error: install.sh not found in the current directory"
  exit 1
fi

# Create a backup of the original install.sh
cp install.sh install.sh.bak
echo "Created backup of install.sh as install.sh.bak"

# Find the run_tests function and replace it with an enhanced version
sed -i '/^run_tests() {/,/^}/c\
# Run tests for SmashLang\nrun_tests() {\n  local repo_dir="$1"\n  \n  echo -e "${BLUE}Running tests for SmashLang...${NC}"\n  cd "$repo_dir"\n  \n  if command -v cargo &> /dev/null; then\n    # Run main crate tests\n    echo -e "${BLUE}Running main crate tests...${NC}"\n    cargo test\n    local main_test_result=$?\n    \n    # Run tests for all workspace packages\n    echo -e "${BLUE}Running tests for all packages...${NC}"\n    cargo test --all\n    local all_test_result=$?\n    \n    # Run tests with all features enabled\n    echo -e "${BLUE}Running tests with all features enabled...${NC}"\n    cargo test --all-features\n    local features_test_result=$?\n    \n    # Check if any tests failed\n    if [ $main_test_result -eq 0 ] && [ $all_test_result -eq 0 ] && [ $features_test_result -eq 0 ]; then\n      echo -e "${GREEN}All tests passed successfully!${NC}"\n    else\n      echo -e "${YELLOW}Warning: Some tests failed. Continuing with installation...${NC}"\n    fi\n    \n    # Run example tests if they exist\n    if [ -d "docs/getting-started" ] && [ -f "docs/getting-started/run_all_examples.sh" ]; then\n      echo -e "${BLUE}Running example tests...${NC}"\n      chmod +x "docs/getting-started/run_all_examples.sh"\n      ./docs/getting-started/run_all_examples.sh\n    fi\n    \n    # Test all packages in smashlang_packages directory if it exists\n    if [ -d "smashlang_packages" ]; then\n      echo -e "${BLUE}Testing SmashLang packages...${NC}"\n      for pkg_dir in smashlang_packages/*; do\n        if [ -d "$pkg_dir" ] && [ -f "$pkg_dir/Cargo.toml" ]; then\n          echo -e "${BLUE}Testing package: $(basename "$pkg_dir")${NC}"\n          (cd "$pkg_dir" && cargo test)\n        fi\n      done\n    fi\n  else\n    echo -e "${YELLOW}Warning: Cargo not found, skipping tests.${NC}"\n  fi\n}\n' install.sh

# Make sure the run_tests function is called in the Linux installation section
sed -i '/git clone --depth 1 "$REPO_URL" "$temp_dir"/a \
      # Run tests when using master branch\n      run_tests "$temp_dir"' install.sh

# Make sure the run_tests function is called in the macOS installation section
sed -i '/git clone --depth 1 "$REPO_URL" "$temp_dir"/a \
    # Run tests when using master branch\n    run_tests "$temp_dir"' install.sh

# Make sure the run_tests function is called in the Windows installation section
sed -i '/git clone --depth 1 "$REPO_URL" "$temp_dir"/a \
    # Run tests when using master branch\n    run_tests "$temp_dir"' install.sh

echo "Successfully updated install.sh to run comprehensive tests for all packages when using the --master flag"
echo "You can now run './install.sh --master' to install SmashLang with comprehensive tests"
