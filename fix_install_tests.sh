#!/bin/bash

# This script fixes the install.sh script to properly run tests when using the --master flag

set -e

# Check if install.sh exists
if [ ! -f "install.sh" ]; then
  echo "Error: install.sh not found in the current directory"
  exit 1
fi

# Create a backup of the original install.sh
cp install.sh install.sh.bak
echo "Created backup of install.sh as install.sh.bak"

# Find the line where the run_tests function is defined
RUN_TESTS_LINE=$(grep -n "^run_tests()" install.sh | cut -d':' -f1)

if [ -z "$RUN_TESTS_LINE" ]; then
  echo "Error: run_tests function not found in install.sh"
  exit 1
fi

echo "Found run_tests function at line $RUN_TESTS_LINE"

# Find the Linux installation section where we need to call run_tests
LINUX_SECTION_LINE=$(grep -n "# Install SmashLang on Linux" install.sh | cut -d':' -f1)

if [ -z "$LINUX_SECTION_LINE" ]; then
  echo "Error: Linux installation section not found in install.sh"
  exit 1
fi

echo "Found Linux installation section at line $LINUX_SECTION_LINE"

# Find the macOS installation section where we need to call run_tests
MACOS_SECTION_LINE=$(grep -n "# Install SmashLang on macOS" install.sh | cut -d':' -f1)

if [ -z "$MACOS_SECTION_LINE" ]; then
  echo "Error: macOS installation section not found in install.sh"
  exit 1
fi

echo "Found macOS installation section at line $MACOS_SECTION_LINE"

# Find the Windows installation section where we need to call run_tests
WINDOWS_SECTION_LINE=$(grep -n "# Install SmashLang on Windows" install.sh | cut -d':' -f1)

if [ -z "$WINDOWS_SECTION_LINE" ]; then
  echo "Error: Windows installation section not found in install.sh"
  exit 1
fi

echo "Found Windows installation section at line $WINDOWS_SECTION_LINE"

# Create a temporary file with the fixed content
TMP_FILE=$(mktemp)

# Fix the Linux installation section
LINUX_GIT_CLONE_LINE=$(grep -n "git clone --depth 1 \"\$REPO_URL\" \"\$temp_dir\"" install.sh | head -n 1 | cut -d':' -f1)
LINUX_GIT_CLONE_LINE=$((LINUX_GIT_CLONE_LINE + 1))

sed -n "1,${LINUX_GIT_CLONE_LINE}p" install.sh > "$TMP_FILE"
echo "      # Run tests when using master branch" >> "$TMP_FILE"
echo "      run_tests \"\$temp_dir\"" >> "$TMP_FILE"
sed -n "$((LINUX_GIT_CLONE_LINE + 1)),\$p" install.sh >> "$TMP_FILE"

# Replace the original file with the fixed one
mv "$TMP_FILE" install.sh
chmod +x install.sh

echo "Successfully fixed install.sh to run tests when using the --master flag"
echo "You can now run './install.sh --master' to install SmashLang with tests"
