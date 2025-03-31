#!/bin/bash

# This script downloads the SmashLang installer, fixes it, and runs it with tests

set -e

# Set color variables
RED="\033[0;31m"
GREEN="\033[0;32m"
YELLOW="\033[0;33m"
BLUE="\033[0;34m"
NC="\033[0m" # No Color

# Download the installer script
echo -e "${BLUE}Downloading SmashLang installer...${NC}"
INSTALLER_SCRIPT="smashlang_installer.sh"
if command -v curl &> /dev/null; then
  curl -fsSL "https://raw.githubusercontent.com/profullstack/smashlang/master/install.sh" > "$INSTALLER_SCRIPT"
else
  wget -q -O "$INSTALLER_SCRIPT" "https://raw.githubusercontent.com/profullstack/smashlang/master/install.sh"
fi

# Fix any potential issues with the installer script
echo -e "${BLUE}Fixing installer script...${NC}"
# Ensure all heredocs are properly closed
sed -i 's/EOF$/EOF\n/g' "$INSTALLER_SCRIPT"

# Make the installer script executable
chmod +x "$INSTALLER_SCRIPT"

# Run the installer with the --master flag
echo -e "${BLUE}Running SmashLang installer with --master flag...${NC}"
./"$INSTALLER_SCRIPT" --master

# Create the test script
echo -e "${BLUE}Creating test script...${NC}"
TEST_SCRIPT="run_all_tests.sh"
cat > "$TEST_SCRIPT" << 'EOF'
#!/bin/bash

# SmashLang Comprehensive Test Runner
# This script runs all tests for the SmashLang project and generates a detailed log

set -e

# Set color variables
RED="\033[0;31m"
GREEN="\033[0;32m"
YELLOW="\033[0;33m"
BLUE="\033[0;34m"
NC="\033[0m" # No Color

# Create a log file
LOG_FILE="smashlang_test_results.log"

# Initialize the log file
echo "SmashLang Test Results" > "$LOG_FILE"
echo "======================" >> "$LOG_FILE"
echo "Date: $(date)" >> "$LOG_FILE"
echo "" >> "$LOG_FILE"

echo -e "${BLUE}Running comprehensive tests for SmashLang...${NC}"

if ! command -v cargo &> /dev/null; then
  echo -e "${RED}Error: Cargo not found. Please install Rust and try again.${NC}"
  echo "Cargo not found, tests were skipped." >> "$LOG_FILE"
  exit 1
fi

# Run main crate tests
echo -e "${BLUE}Running main crate tests...${NC}"
echo "Main Crate Tests" >> "$LOG_FILE"
echo "---------------" >> "$LOG_FILE"
cargo test 2>&1 | tee -a "$LOG_FILE"
MAIN_TEST_RESULT=$?
echo "" >> "$LOG_FILE"

# Run tests for all workspace packages
echo -e "${BLUE}Running tests for all packages...${NC}"
echo "All Packages Tests" >> "$LOG_FILE"
echo "-----------------" >> "$LOG_FILE"
cargo test --all 2>&1 | tee -a "$LOG_FILE"
ALL_TEST_RESULT=$?
echo "" >> "$LOG_FILE"

# Run tests with all features enabled
echo -e "${BLUE}Running tests with all features enabled...${NC}"
echo "All Features Tests" >> "$LOG_FILE"
echo "-----------------" >> "$LOG_FILE"
cargo test --all-features 2>&1 | tee -a "$LOG_FILE"
FEATURES_TEST_RESULT=$?
echo "" >> "$LOG_FILE"

# Run example tests if they exist
if [ -d "docs/getting-started" ] && [ -f "docs/getting-started/run_all_examples.sh" ]; then
  echo -e "${BLUE}Running example tests...${NC}"
  echo "Example Tests" >> "$LOG_FILE"
  echo "-------------" >> "$LOG_FILE"
  chmod +x "docs/getting-started/run_all_examples.sh"
  ./docs/getting-started/run_all_examples.sh 2>&1 | tee -a "$LOG_FILE"
  echo "" >> "$LOG_FILE"
fi

# Test all packages in smashlang_packages directory if it exists
if [ -d "smashlang_packages" ]; then
  echo -e "${BLUE}Testing SmashLang packages...${NC}"
  echo "SmashLang Packages Tests" >> "$LOG_FILE"
  echo "----------------------" >> "$LOG_FILE"
  for pkg_dir in smashlang_packages/*; do
    if [ -d "$pkg_dir" ] && [ -f "$pkg_dir/Cargo.toml" ]; then
      pkg_name=$(basename "$pkg_dir")
      echo -e "${BLUE}Testing package: $pkg_name${NC}"
      echo "Package: $pkg_name" >> "$LOG_FILE"
      (cd "$pkg_dir" && cargo test) 2>&1 | tee -a "$LOG_FILE"
      echo "" >> "$LOG_FILE"
    fi
  done
fi

# Display test summary
echo -e "\n${BLUE}Test Results Summary${NC}"
echo -e "${BLUE}-------------------${NC}"
echo -e "A detailed test log has been saved to: $LOG_FILE"

if [ $MAIN_TEST_RESULT -eq 0 ] && [ $ALL_TEST_RESULT -eq 0 ] && [ $FEATURES_TEST_RESULT -eq 0 ]; then
  echo -e "${GREEN}All tests passed successfully!${NC}"
  echo "TEST SUMMARY: All tests passed successfully!" >> "$LOG_FILE"
else
  echo -e "${YELLOW}Some tests failed. See the log file for details.${NC}"
  echo "TEST SUMMARY: Some tests failed. See details above." >> "$LOG_FILE"
fi
EOF

# Make the test script executable
chmod +x "$TEST_SCRIPT"

# Clone the repository to run tests
echo -e "${BLUE}Cloning SmashLang repository to run tests...${NC}"
TEMP_DIR=$(mktemp -d)
trap "rm -rf $TEMP_DIR" EXIT
git clone --depth 1 "https://github.com/profullstack/smashlang.git" "$TEMP_DIR"

# Run tests on the cloned repository
echo -e "${BLUE}Running comprehensive tests...${NC}"
cd "$TEMP_DIR"
./"$TEST_SCRIPT"

# Clean up
echo -e "${GREEN}SmashLang installation and testing completed!${NC}"
echo -e "${BLUE}You can run tests again at any time with:${NC} ./run_all_tests.sh"
