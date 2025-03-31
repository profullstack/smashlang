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
  local repo_dir="$1"
  local log_file="$repo_dir/test_results.log"
  
  echo -e "${BLUE}Running tests for SmashLang...${NC}"
  cd "$repo_dir"
  
  # Create or clear the log file
  echo "SmashLang Test Results" > "$log_file"
  echo "======================"  >> "$log_file"
  echo "Date: $(date)" >> "$log_file"
  echo "" >> "$log_file"
  
  if command -v cargo &> /dev/null; then
    # Run main crate tests
    echo -e "${BLUE}Running main crate tests...${NC}"
    echo "Main Crate Tests" >> "$log_file"
    echo "---------------" >> "$log_file"
    cargo test 2>&1 | tee -a "$log_file"
    local main_test_result=$?
    echo "" >> "$log_file"
    
    # Run tests for all workspace packages
    echo -e "${BLUE}Running tests for all packages...${NC}"
    echo "All Packages Tests" >> "$log_file"
    echo "-----------------" >> "$log_file"
    cargo test --all 2>&1 | tee -a "$log_file"
    local all_test_result=$?
    echo "" >> "$log_file"
    
    # Run tests with all features enabled
    echo -e "${BLUE}Running tests with all features enabled...${NC}"
    echo "All Features Tests" >> "$log_file"
    echo "-----------------" >> "$log_file"
    cargo test --all-features 2>&1 | tee -a "$log_file"
    local features_test_result=$?
    echo "" >> "$log_file"
    
    # Check if any tests failed
    if [ $main_test_result -eq 0 ] && [ $all_test_result -eq 0 ] && [ $features_test_result -eq 0 ]; then
      echo -e "${GREEN}All tests passed successfully!${NC}"
      echo "TEST SUMMARY: All tests passed successfully!" >> "$log_file"
    else
      echo -e "${YELLOW}Warning: Some tests failed. Continuing with installation...${NC}"
      echo "TEST SUMMARY: Some tests failed. See details above." >> "$log_file"
    fi
    
    # Run example tests if they exist
    if [ -d "docs/getting-started" ] && [ -f "docs/getting-started/run_all_examples.sh" ]; then
      echo -e "${BLUE}Running example tests...${NC}"
      echo "Example Tests" >> "$log_file"
      echo "-------------" >> "$log_file"
      chmod +x "docs/getting-started/run_all_examples.sh"
      ./docs/getting-started/run_all_examples.sh 2>&1 | tee -a "$log_file"
      echo "" >> "$log_file"
    fi
    
    # Test all packages in smashlang_packages directory if it exists
    if [ -d "smashlang_packages" ]; then
      echo -e "${BLUE}Testing SmashLang packages...${NC}"
      echo "SmashLang Packages Tests" >> "$log_file"
      echo "----------------------" >> "$log_file"
      for pkg_dir in smashlang_packages/*; do
        if [ -d "$pkg_dir" ] && [ -f "$pkg_dir/Cargo.toml" ]; then
          pkg_name=$(basename "$pkg_dir")
          echo -e "${BLUE}Testing package: $pkg_name${NC}"
          echo "Package: $pkg_name" >> "$log_file"
          (cd "$pkg_dir" && cargo test) 2>&1 | tee -a "$log_file"
          echo "" >> "$log_file"
        fi
      done
    fi
    
    # Copy the test results log to the installation directory
    if [ -n "$LINUX_INSTALL_DIR" ]; then
      mkdir -p "$LINUX_INSTALL_DIR/logs"
      cp "$log_file" "$LINUX_INSTALL_DIR/logs/"
    elif [ -n "$MACOS_INSTALL_DIR" ]; then
      mkdir -p "$MACOS_INSTALL_DIR/logs"
      cp "$log_file" "$MACOS_INSTALL_DIR/logs/"
    elif [ -n "$WINDOWS_INSTALL_DIR" ]; then
      mkdir -p "$WINDOWS_INSTALL_DIR/logs"
      cp "$log_file" "$WINDOWS_INSTALL_DIR/logs/"
    fi
    
    # Display test summary at the end of installation
    echo -e "\n${BLUE}Test Results Summary${NC}"
    echo -e "${BLUE}-------------------${NC}"
    echo -e "A detailed test log has been saved to: $log_file"
    if [ $main_test_result -eq 0 ] && [ $all_test_result -eq 0 ] && [ $features_test_result -eq 0 ]; then
      echo -e "${GREEN}All tests passed successfully!${NC}"
    else
      echo -e "${YELLOW}Some tests failed. See the log file for details.${NC}"
    fi
  else
    echo -e "${YELLOW}Warning: Cargo not found, skipping tests.${NC}"
    echo "Cargo not found, tests were skipped." >> "$log_file"
  fi
  
  # Save the log file path for later use
  echo "$log_file" > "$repo_dir/test_log_path.txt"
}
EOF

# Replace the run_tests function in the install.sh script
sed -i '/^# Run tests for SmashLang/,/^}/c\' install.sh
cat run_tests_function.txt >> install.sh

# Add a function to display test results at the end of installation
cat >> install.sh << 'EOF'

# Display test results at the end of installation
display_test_results() {
  if [ -f "$1/test_log_path.txt" ]; then
    local log_file=$(cat "$1/test_log_path.txt")
    if [ -f "$log_file" ]; then
      echo -e "\n${BLUE}Test Results Summary${NC}"
      echo -e "${BLUE}-------------------${NC}"
      echo -e "A detailed test log has been saved to: $log_file"
      if grep -q "TEST SUMMARY: All tests passed successfully!" "$log_file"; then
        echo -e "${GREEN}All tests passed successfully!${NC}"
      else
        echo -e "${YELLOW}Some tests failed. See the log file for details.${NC}"
      fi
    fi
  fi
}
EOF

# Fix the Linux installation section
sed -i '/git clone --depth 1 "$REPO_URL" "$temp_dir"/,/run_tests "$temp_dir"/c\
    git clone --depth 1 "$REPO_URL" "$temp_dir"\n\n    # Run tests when using master branch\n    run_tests "$temp_dir"' install.sh

# Fix the macOS installation section
sed -i '/git clone --depth 1 "$REPO_URL" "$temp_dir"/,/run_tests "$temp_dir"/c\
    git clone --depth 1 "$REPO_URL" "$temp_dir"\n\n    # Run tests when using master branch\n    run_tests "$temp_dir"' install.sh

# Fix the Windows installation section
sed -i '/git clone --depth 1 "$REPO_URL" "$temp_dir"/,/run_tests "$temp_dir"/c\
    git clone --depth 1 "$REPO_URL" "$temp_dir"\n\n    # Run tests when using master branch\n    run_tests "$temp_dir"' install.sh

# Add calls to display_test_results at the end of each installation function
sed -i '/echo -e "${GREEN}SmashLang has been successfully installed on Linux!${NC}"/i\
    # Display test results summary\n    display_test_results "$temp_dir"\n' install.sh

sed -i '/echo -e "${GREEN}SmashLang has been successfully installed on macOS!${NC}"/i\
    # Display test results summary\n    display_test_results "$temp_dir"\n' install.sh

sed -i '/echo -e "${GREEN}SmashLang has been successfully installed on Windows!${NC}"/i\
    # Display test results summary\n    display_test_results "$temp_dir"\n' install.sh

# Clean up temporary file
rm run_tests_function.txt

echo "Successfully updated install.sh to run tests and generate a test results log"
echo "You can now run './install.sh --master' to install SmashLang with comprehensive tests"
echo "A detailed test log will be saved and a summary will be displayed at the end of installation"
