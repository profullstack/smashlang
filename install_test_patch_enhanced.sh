#!/bin/bash

# This script adds enhanced test functionality to the SmashLang install.sh script
# It modifies the install.sh script to run tests for all packages when the --master flag is used

set -e

# Check if install.sh exists
if [ ! -f "install.sh" ]; then
  echo "Error: install.sh not found in the current directory"
  exit 1
fi

# Create a backup of the original install.sh
cp install.sh install.sh.bak
echo "Created backup of install.sh as install.sh.bak"

# Create the enhanced run_tests function
cat > run_tests_function.txt << 'EOF'
# Run tests for SmashLang
run_tests() {
  local repo_dir="$1"
  
  echo -e "${BLUE}Running tests for SmashLang...${NC}"
  cd "$repo_dir"
  
  if command -v cargo &> /dev/null; then
    # Run main crate tests
    echo -e "${BLUE}Running main crate tests...${NC}"
    cargo test
    local main_test_result=$?
    
    # Run tests for all workspace packages
    echo -e "${BLUE}Running tests for all packages...${NC}"
    cargo test --all
    local all_test_result=$?
    
    # Run tests with all features enabled
    echo -e "${BLUE}Running tests with all features enabled...${NC}"
    cargo test --all-features
    local features_test_result=$?
    
    # Check if any tests failed
    if [ $main_test_result -eq 0 ] && [ $all_test_result -eq 0 ] && [ $features_test_result -eq 0 ]; then
      echo -e "${GREEN}All tests passed successfully!${NC}"
    else
      echo -e "${YELLOW}Warning: Some tests failed. Continuing with installation...${NC}"
    fi
    
    # Run example tests if they exist
    if [ -d "docs/getting-started" ] && [ -f "docs/getting-started/run_all_examples.sh" ]; then
      echo -e "${BLUE}Running example tests...${NC}"
      chmod +x "docs/getting-started/run_all_examples.sh"
      ./docs/getting-started/run_all_examples.sh
    fi
    
    # Test all packages in smashlang_packages directory if it exists
    if [ -d "smashlang_packages" ]; then
      echo -e "${BLUE}Testing SmashLang packages...${NC}"
      for pkg_dir in smashlang_packages/*; do
        if [ -d "$pkg_dir" ] && [ -f "$pkg_dir/Cargo.toml" ]; then
          echo -e "${BLUE}Testing package: $(basename "$pkg_dir")${NC}"
          (cd "$pkg_dir" && cargo test)
        fi
      done
    fi
  else
    echo -e "${YELLOW}Warning: Cargo not found, skipping tests.${NC}"
  fi
}
EOF

# Add the enhanced run_tests function before the get_script_dir function
sed -i '/# Get script directory/i \' install.sh
cat run_tests_function.txt >> install.sh
echo '' >> install.sh

# Add test call to Linux installation
sed -i '/# Capture git hash for version info/,/cargo build --release/ { s/cargo build --release/# Run tests when using master branch\n      run_tests "$temp_dir"\n      \n      # Build release version\n      cargo build --release/g }' install.sh

# Add test call to macOS installation
sed -i '/git clone --depth 1 "$REPO_URL" "$temp_dir"/a \
    # Run tests when using master branch\n    run_tests "$temp_dir"' install.sh

# Add test call to Windows installation
sed -i '/git clone --depth 1 "$REPO_URL" "$temp_dir"/a \
    # Run tests when using master branch\n    run_tests "$temp_dir"' install.sh

# Clean up temporary file
rm run_tests_function.txt

echo "Successfully updated install.sh to run comprehensive tests when using the --master flag"
echo "This includes testing all packages, all features, and running example tests"
echo "To apply these changes, run: chmod +x install_test_patch_enhanced.sh && ./install_test_patch_enhanced.sh"
