#!/bin/bash

# This script wraps the SmashLang installer to ensure tests run properly

set -e

# Set color variables
RED="\033[0;31m"
GREEN="\033[0;32m"
YELLOW="\033[0;33m"
BLUE="\033[0;34m"
NC="\033[0m" # No Color

# Run tests for SmashLang
run_tests() {
  local repo_dir="$1"
  local log_file="$repo_dir/test_results.log"
  
  echo -e "${BLUE}Running tests for SmashLang...${NC}"
  cd "$repo_dir"
  
  # Create or clear the log file
  echo "SmashLang Test Results" > "$log_file"
  echo "======================" >> "$log_file"
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
    
    # Display test summary
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
}

# Download the installer script
echo "Downloading SmashLang installer..."
TEMP_SCRIPT="/tmp/smashlang_installer_$$.sh"
if command -v curl &> /dev/null; then
  curl -fsSL "https://raw.githubusercontent.com/profullstack/smashlang/master/install.sh" > "$TEMP_SCRIPT"
else
  wget -q -O "$TEMP_SCRIPT" "https://raw.githubusercontent.com/profullstack/smashlang/master/install.sh"
fi
chmod +x "$TEMP_SCRIPT"

# Run the installer with the --master flag
echo "Running SmashLang installer with --master flag..."
"$TEMP_SCRIPT" --master

# Clone the repository to run tests
echo "Cloning SmashLang repository to run tests..."
TEMP_DIR=$(mktemp -d)
trap "rm -rf $TEMP_DIR" EXIT
git clone --depth 1 "https://github.com/profullstack/smashlang.git" "$TEMP_DIR"

# Run tests on the cloned repository
run_tests "$TEMP_DIR"

# Clean up
rm -f "$TEMP_SCRIPT"
echo -e "${GREEN}SmashLang installation and testing completed!${NC}"
