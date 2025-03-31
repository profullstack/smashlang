#!/bin/bash

# This script fixes the install.sh script to ensure the run_tests function is defined before it's used

set -e

# Check if install.sh exists
if [ ! -f "install.sh" ]; then
  echo "Error: install.sh not found in the current directory"
  exit 1
fi

# Create a backup of the original install.sh
cp install.sh install.sh.bak
echo "Created backup of install.sh as install.sh.bak"

# Extract the run_tests and display_test_results functions
run_tests_function=$(sed -n '/^# Run tests for SmashLang/,/^}/p' install.sh)
display_test_results_function=$(sed -n '/^# Display test results at the end of installation/,/^}/p' install.sh)

# Remove the functions from their current location
sed -i '/^# Run tests for SmashLang/,/^}/d' install.sh
sed -i '/^# Display test results at the end of installation/,/^}/d' install.sh

# Insert the functions at the beginning of the file, after the initial comments
sed -i '1s/^/\n/' install.sh
sed -i "1s/^/$display_test_results_function\n\n/" install.sh
sed -i "1s/^/$run_tests_function\n\n/" install.sh

# Fix the GitHub download section to include the run_tests function in the downloaded script
sed -i '/if \[ -z "\$DOWNLOADED_INSTALLER" \] && \[ "\$1" = "--master" \]; then/,/exit 0/c\
# Check if this is a direct download from GitHub\nif [ -z "$DOWNLOADED_INSTALLER" ] && [ "$1" = "--master" ]; then\n  # This is a direct download and we want to use master branch\n  # Create a temporary script with our functions and then download the installer\n  TEMP_SCRIPT="/tmp/smashlang_installer_$$.sh"\n  \n  # First, create the temporary script with our run_tests function\n  cat > "$TEMP_SCRIPT" << \'EOF\'\n#!/bin/bash\n\n# Run tests for SmashLang\nrun_tests() {\n  local repo_dir="$1"\n  local log_file="$repo_dir/test_results.log"\n  \n  echo -e "\${BLUE}Running tests for SmashLang...\${NC}"\n  cd "$repo_dir"\n  \n  # Create or clear the log file\n  echo "SmashLang Test Results" > "$log_file"\n  echo "=======================" >> "$log_file"\n  echo "Date: $(date)" >> "$log_file"\n  echo "" >> "$log_file"\n  \n  if command -v cargo &> /dev/null; then\n    # Run main crate tests\n    echo -e "\${BLUE}Running main crate tests...\${NC}"\n    echo "Main Crate Tests" >> "$log_file"\n    echo "---------------" >> "$log_file"\n    cargo test 2>&1 | tee -a "$log_file"\n    local main_test_result=$?\n    echo "" >> "$log_file"\n    \n    # Run tests for all workspace packages\n    echo -e "\${BLUE}Running tests for all packages...\${NC}"\n    echo "All Packages Tests" >> "$log_file"\n    echo "-----------------" >> "$log_file"\n    cargo test --all 2>&1 | tee -a "$log_file"\n    local all_test_result=$?\n    echo "" >> "$log_file"\n    \n    # Run tests with all features enabled\n    echo -e "\${BLUE}Running tests with all features enabled...\${NC}"\n    echo "All Features Tests" >> "$log_file"\n    echo "-----------------" >> "$log_file"\n    cargo test --all-features 2>&1 | tee -a "$log_file"\n    local features_test_result=$?\n    echo "" >> "$log_file"\n    \n    # Check if any tests failed\n    if [ $main_test_result -eq 0 ] && [ $all_test_result -eq 0 ] && [ $features_test_result -eq 0 ]; then\n      echo -e "\${GREEN}All tests passed successfully!\${NC}"\n      echo "TEST SUMMARY: All tests passed successfully!" >> "$log_file"\n    else\n      echo -e "\${YELLOW}Warning: Some tests failed. Continuing with installation...\${NC}"\n      echo "TEST SUMMARY: Some tests failed. See details above." >> "$log_file"\n    fi\n    \n    # Run example tests if they exist\n    if [ -d "docs/getting-started" ] && [ -f "docs/getting-started/run_all_examples.sh" ]; then\n      echo -e "\${BLUE}Running example tests...\${NC}"\n      echo "Example Tests" >> "$log_file"\n      echo "-------------" >> "$log_file"\n      chmod +x "docs/getting-started/run_all_examples.sh"\n      ./docs/getting-started/run_all_examples.sh 2>&1 | tee -a "$log_file"\n      echo "" >> "$log_file"\n    fi\n    \n    # Test all packages in smashlang_packages directory if it exists\n    if [ -d "smashlang_packages" ]; then\n      echo -e "\${BLUE}Testing SmashLang packages...\${NC}"\n      echo "SmashLang Packages Tests" >> "$log_file"\n      echo "----------------------" >> "$log_file"\n      for pkg_dir in smashlang_packages/*; do\n        if [ -d "$pkg_dir" ] && [ -f "$pkg_dir/Cargo.toml" ]; then\n          pkg_name=$(basename "$pkg_dir")\n          echo -e "\${BLUE}Testing package: $pkg_name\${NC}"\n          echo "Package: $pkg_name" >> "$log_file"\n          (cd "$pkg_dir" && cargo test) 2>&1 | tee -a "$log_file"\n          echo "" >> "$log_file"\n        fi\n      done\n    fi\n    \n    # Save the log file path for later use\n    echo "$log_file" > "$repo_dir/test_log_path.txt"\n  else\n    echo -e "\${YELLOW}Warning: Cargo not found, skipping tests.\${NC}"\n    echo "Cargo not found, tests were skipped." >> "$log_file"\n  fi\n}\n\n# Display test results at the end of installation\ndisplay_test_results() {\n  if [ -f "$1/test_log_path.txt" ]; then\n    local log_file=$(cat "$1/test_log_path.txt")\n    if [ -f "$log_file" ]; then\n      echo -e "\\n\${BLUE}Test Results Summary\${NC}"\n      echo -e "\${BLUE}-------------------\${NC}"\n      echo -e "A detailed test log has been saved to: $log_file"\n      if grep -q "TEST SUMMARY: All tests passed successfully!" "$log_file"; then\n        echo -e "\${GREEN}All tests passed successfully!\${NC}"\n      else\n        echo -e "\${YELLOW}Some tests failed. See the log file for details.\${NC}"\n      fi\n    fi\n  fi\n}\nEOF\n\n  # Now append the installer script from GitHub\n  echo "Downloading installer script to temporary file..."\n  if command -v curl &> /dev/null; then\n    curl -fsSL "https://raw.githubusercontent.com/profullstack/smashlang/master/install.sh" >> "$TEMP_SCRIPT"\n  else\n    wget -q -O - "https://raw.githubusercontent.com/profullstack/smashlang/master/install.sh" >> "$TEMP_SCRIPT"\n  fi\n  chmod +x "$TEMP_SCRIPT"\n  \n  # Run the combined script with the --master flag and mark it as downloaded\n  DOWNLOADED_INSTALLER=true "$TEMP_SCRIPT" --master\n  \n  # Clean up\n  rm -f "$TEMP_SCRIPT"\n  exit 0\nfi' install.sh

echo "Successfully updated install.sh to ensure the run_tests function is defined before it's used"
echo "You can now run './install.sh --master' to install SmashLang with comprehensive tests"
echo "A detailed test log will be saved and a summary will be displayed at the end of installation"
