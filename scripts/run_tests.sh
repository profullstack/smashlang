#!/bin/bash

# Test script for SmashLang
# This script runs all tests for SmashLang

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
    cargo test > "$log_file.tmp" 2>&1
    local main_test_result=$?
    cat "$log_file.tmp"
    cat "$log_file.tmp" >> "$log_file"
    rm "$log_file.tmp"
    echo "" >> "$log_file"
    
    # Run tests for all workspace packages
    echo -e "${BLUE}Running tests for all packages...${NC}"
    echo "All Packages Tests" >> "$log_file"
    echo "-----------------" >> "$log_file"
    cargo test --all > "$log_file.tmp" 2>&1
    local all_test_result=$?
    cat "$log_file.tmp"
    cat "$log_file.tmp" >> "$log_file"
    rm "$log_file.tmp"
    echo "" >> "$log_file"
    
    # Run tests with all features enabled
    echo -e "${BLUE}Running tests with all features enabled...${NC}"
    echo "All Features Tests" >> "$log_file"
    echo "-----------------" >> "$log_file"
    cargo test --all-features > "$log_file.tmp" 2>&1
    local features_test_result=$?
    cat "$log_file.tmp"
    cat "$log_file.tmp" >> "$log_file"
    rm "$log_file.tmp"
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
      ./docs/getting-started/run_all_examples.sh > "$log_file.tmp" 2>&1
      cat "$log_file.tmp"
      cat "$log_file.tmp" >> "$log_file"
      rm "$log_file.tmp"
      echo "" >> "$log_file"
    else
      echo -e "${YELLOW}Warning: Example tests directory not found, skipping example tests.${NC}"
      echo "Example tests directory not found, tests were skipped." >> "$log_file"
    fi
    
    # Test all packages in smashlang_packages directory if it exists
    if [ -d "smashlang_packages" ]; then
      echo -e "${BLUE}Testing SmashLang packages...${NC}"
      echo "SmashLang Packages Tests" >> "$log_file"
      echo "----------------------" >> "$log_file"
      
      # First, run a combined test of all packages if possible
      if [ -f "smashlang_packages/Cargo.toml" ]; then
        echo -e "${BLUE}Running combined test of all packages...${NC}"
        echo "Combined Package Tests" >> "$log_file"
        echo "---------------------" >> "$log_file"
        (cd "smashlang_packages" && cargo test --all) > "$log_file.tmp" 2>&1
        local combined_test_result=$?
        cat "$log_file.tmp"
        cat "$log_file.tmp" >> "$log_file"
        rm "$log_file.tmp"
        if [ $combined_test_result -eq 0 ]; then
          echo -e "${GREEN}Combined package tests passed!${NC}"
        else
          echo -e "${YELLOW}Some combined package tests failed. Running individual package tests...${NC}"
        fi
        echo "" >> "$log_file"
      fi
      
      # Find and test all packages, including those in subdirectories
      local all_packages_passed=true
      echo -e "${BLUE}Testing individual packages...${NC}"
      echo "Individual Package Tests" >> "$log_file"
      echo "----------------------" >> "$log_file"
      
      # Find all Cargo.toml files in the packages directory
      find "smashlang_packages" -name "Cargo.toml" | while read -r package_file; do
        local package_dir=$(dirname "$package_file")
        local package_name=$(basename "$package_dir")
        
        echo -e "${BLUE}Testing package: $package_name${NC}"
        echo "Testing package: $package_name" >> "$log_file"
        (cd "$package_dir" && cargo test) > "$log_file.tmp" 2>&1
        local package_test_result=$?
        cat "$log_file.tmp"
        cat "$log_file.tmp" >> "$log_file"
        rm "$log_file.tmp"
        
        if [ $package_test_result -ne 0 ]; then
          all_packages_passed=false
          echo -e "${YELLOW}Package $package_name tests failed.${NC}"
          echo "Package $package_name tests failed." >> "$log_file"
        fi
        echo "" >> "$log_file"
      done
      
      if [ "$all_packages_passed" == "true" ]; then
        echo -e "${GREEN}All package tests passed successfully!${NC}"
        echo "All package tests passed successfully!" >> "$log_file"
      else
        echo -e "${YELLOW}Some package tests failed. See log for details.${NC}"
        echo "Some package tests failed. See details above." >> "$log_file"
      fi
    else
      echo -e "${YELLOW}No SmashLang packages directory found, skipping package tests.${NC}"
      echo "No SmashLang packages directory found, tests were skipped." >> "$log_file"
    fi
    
    echo -e "\nTest Results Summary\n-------------------"
    echo -e "A detailed test log has been saved to: $log_file"
    
    return 0
  else
    echo -e "${RED}Error: Cargo not found, unable to run tests.${NC}"
    echo "Error: Cargo not found, unable to run tests." >> "$log_file"
    return 1
  fi
}

# If this script is run directly, execute the run_tests function
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
  # Check if the repository directory is provided
  if [ -z "$1" ]; then
    echo -e "${RED}Error: Repository directory not provided.${NC}"
    echo -e "Usage: $0 <repository_directory>"
    exit 1
  fi
  
  # Run tests
  run_tests "$1"
  exit $?
fi
