#!/bin/bash

# This script downloads and fixes the SmashLang installer

set -e

# Set color variables
RED="\033[0;31m"
GREEN="\033[0;32m"
YELLOW="\033[0;33m"
BLUE="\033[0;34m"
NC="\033[0m" # No Color

echo -e "${BLUE}Downloading SmashLang installer...${NC}"
INSTALLER_SCRIPT="fixed_install.sh"
if command -v curl &> /dev/null; then
  curl -fsSL "https://raw.githubusercontent.com/profullstack/smashlang/master/install.sh" > "$INSTALLER_SCRIPT"
else
  wget -q -O "$INSTALLER_SCRIPT" "https://raw.githubusercontent.com/profullstack/smashlang/master/install.sh"
fi

# Fix the heredoc issue at line 579
echo -e "${BLUE}Fixing installer script...${NC}"

# Add the missing EOF after line 587 (after the JSON config)
sed -i '587a\n}\nEOF' "$INSTALLER_SCRIPT"

# Make the installer script executable
chmod +x "$INSTALLER_SCRIPT"

echo -e "${GREEN}Installer script has been fixed!${NC}"
echo -e "${BLUE}You can now run:${NC} ./fixed_install.sh --master"
