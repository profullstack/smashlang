#!/bin/bash

# Script to install a SmashLang package and display information about it

# Colors for output
RED="\033[0;31m"
GREEN="\033[0;32m"
YELLOW="\033[0;33m"
BLUE="\033[0;34m"
MAGENTA="\033[0;35m"
CYAN="\033[0;36m"
NC="\033[0m" # No Color

# Get script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PACKAGES_DIR="$SCRIPT_DIR/../smashlang_packages"

# Function to display usage information
usage() {
  echo -e "${YELLOW}Usage: $0 <package_name>${NC}"
  echo -e "${BLUE}Installs a SmashLang package from the local package repository.${NC}"
  echo ""
  echo -e "${YELLOW}Examples:${NC}"
  echo -e "  $0 networking/http     # Install the HTTP package"
  echo -e "  $0 core/json          # Install the JSON package"
  echo ""
  echo -e "${YELLOW}Available packages:${NC}"
  find "$PACKAGES_DIR" -mindepth 1 -maxdepth 2 -type d -not -path "*/__package__template*" | \
    sed "s|$PACKAGES_DIR/||" | sort | while read pkg; do
    if [ -f "$PACKAGES_DIR/$pkg/package.smash" ]; then
      # Extract package description if available
      description=$(grep -o '"description"\s*:\s*"[^"]*"' "$PACKAGES_DIR/$pkg/package.smash" | \
                   head -1 | sed 's/"description"\s*:\s*"\([^"]*\)"/\1/')
      if [ -n "$description" ]; then
        echo -e "  ${CYAN}$pkg${NC} - $description"
      else
        echo -e "  ${CYAN}$pkg${NC}"
      fi
    else
      echo -e "  ${CYAN}$pkg${NC}"
    fi
  done
  exit 1
}

# Check if a package name was provided
if [ $# -eq 0 ]; then
  usage
fi

PACKAGE_NAME="$1"
PACKAGE_PATH="$PACKAGES_DIR/$PACKAGE_NAME"

# Check if the package exists
if [ ! -d "$PACKAGE_PATH" ]; then
  echo -e "${RED}Error: Package '$PACKAGE_NAME' not found.${NC}"
  echo -e "${YELLOW}Use '$0' without arguments to see available packages.${NC}"
  exit 1
fi

# Check if package.smash exists
if [ ! -f "$PACKAGE_PATH/package.smash" ]; then
  echo -e "${RED}Error: Invalid package. Missing package.smash file.${NC}"
  exit 1
fi

# Extract package information
PACKAGE_TITLE=$(grep -o '"name"\s*:\s*"[^"]*"' "$PACKAGE_PATH/package.smash" | \
                head -1 | sed 's/"name"\s*:\s*"\([^"]*\)"/\1/')
PACKAGE_VERSION=$(grep -o '"version"\s*:\s*"[^"]*"' "$PACKAGE_PATH/package.smash" | \
                  head -1 | sed 's/"version"\s*:\s*"\([^"]*\)"/\1/')
PACKAGE_DESCRIPTION=$(grep -o '"description"\s*:\s*"[^"]*"' "$PACKAGE_PATH/package.smash" | \
                      head -1 | sed 's/"description"\s*:\s*"\([^"]*\)"/\1/')
PACKAGE_AUTHOR=$(grep -o '"author"\s*:\s*"[^"]*"' "$PACKAGE_PATH/package.smash" | \
                 head -1 | sed 's/"author"\s*:\s*"\([^"]*\)"/\1/')

# Use package name if title is not available
if [ -z "$PACKAGE_TITLE" ]; then
  PACKAGE_TITLE=$(basename "$PACKAGE_PATH")
fi

# Perform installation (this is where you would add your actual installation logic)
echo -e "${GREEN}Installing $PACKAGE_TITLE${NC}${YELLOW}${PACKAGE_VERSION:+ v$PACKAGE_VERSION}${NC}..."

# TODO: Add actual installation logic here
# For now, we'll just simulate installation with a sleep
sleep 1

# Display ASCII art and package information
echo -e "\n${GREEN}✅ Successfully installed $PACKAGE_TITLE${NC}${YELLOW}${PACKAGE_VERSION:+ v$PACKAGE_VERSION}${NC}\n"

# Display logo if available
if [ -f "$PACKAGE_PATH/logo.txt" ]; then
  cat "$PACKAGE_PATH/logo.txt" | while read line; do
    echo -e "${YELLOW}$line${NC}"
  done
  echo ""
fi

# Display package information
echo -e "${BLUE}$PACKAGE_DESCRIPTION${NC}\n"

# Display links to documentation and examples
echo -e "${MAGENTA}📚 Resources:${NC}"

# Check for README.md
if [ -f "$PACKAGE_PATH/README.md" ]; then
  echo -e "  ${CYAN}• README:${NC} $PACKAGE_PATH/README.md"
fi

# Check for docs directory
if [ -d "$PACKAGE_PATH/docs" ]; then
  echo -e "  ${CYAN}• Documentation:${NC} $PACKAGE_PATH/docs/"
fi

# Check for examples directory
if [ -d "$PACKAGE_PATH/examples" ]; then
  echo -e "  ${CYAN}• Examples:${NC} $PACKAGE_PATH/examples/"
  
  # List example files
  find "$PACKAGE_PATH/examples" -type f -name "*.smash" | sort | while read example; do
    example_name=$(basename "$example")
    echo -e "    ${YELLOW}→ $example_name:${NC} $example"
  done
fi

# Display author information if available
if [ -n "$PACKAGE_AUTHOR" ]; then
  echo -e "\n${CYAN}Author:${NC} $PACKAGE_AUTHOR"
fi

echo -e "\n${GREEN}To use this package in your SmashLang code:${NC}"
echo -e "${BLUE}import { $PACKAGE_TITLE } from '$PACKAGE_NAME';${NC}"

echo -e "\n${YELLOW}Visit https://smashlang.com for more information and documentation.${NC}"
