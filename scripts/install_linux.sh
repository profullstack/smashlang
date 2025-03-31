#!/bin/bash

# Installation script for SmashLang on Linux
# This script installs SmashLang on Linux systems

# Set color variables
RED="\033[0;31m"
GREEN="\033[0;32m"
YELLOW="\033[0;33m"
BLUE="\033[0;34m"
NC="\033[0m" # No Color

# Installation directory
LINUX_INSTALL_DIR="$HOME/.local/share/smashlang"

# Create directory if it doesn't exist
create_dir() {
  local dir="$1"
  if [ ! -d "$dir" ]; then
    mkdir -p "$dir"
  fi
}

# Create configuration file for Linux
create_config_linux() {
  local config_dir="$HOME/.config/smashlang"
  local config_file="$config_dir/config.json"
  
  create_dir "$config_dir"
  
  # Create or update configuration file
  cat > "$config_file" << EOF
{
  "version": "0.1.0",
  "install_dir": "$LINUX_INSTALL_DIR",
  "bin_dir": "$LINUX_INSTALL_DIR/bin",
  "docs_dir": "$LINUX_INSTALL_DIR/docs",
  "assets_dir": "$LINUX_INSTALL_DIR/assets",
  "packages_dir": "$HOME/.local/share/smashlang_packages"
}
EOF
  
  echo -e "${BLUE}Creating configuration file...${NC}"
}

# Install SmashLang on Linux
install_linux() {
  local repo_dir="$1"
  
  echo -e "${BLUE}Installing SmashLang on Linux...${NC}"
  
  # Create installation directories
  create_dir "$LINUX_INSTALL_DIR"
  create_dir "$LINUX_INSTALL_DIR/bin"
  create_dir "$LINUX_INSTALL_DIR/docs"
  
  # Copy binaries
  echo -e "${BLUE}Installing SmashLang binaries...${NC}"
  cp "$repo_dir/target/release/smash" "$LINUX_INSTALL_DIR/bin/"
  cp "$repo_dir/target/release/smashc" "$LINUX_INSTALL_DIR/bin/"
  cp "$repo_dir/target/release/smashpkg" "$LINUX_INSTALL_DIR/bin/"
  
  # Copy documentation
  echo -e "${BLUE}Installing documentation...${NC}"
  if [ -d "$repo_dir/docs" ]; then
    cp -r "$repo_dir/docs" "$LINUX_INSTALL_DIR/"
    echo -e "Documentation installed to $LINUX_INSTALL_DIR/docs"
  else
    echo -e "${YELLOW}Warning: Documentation directory not found, skipping documentation installation.${NC}"
  fi
  
  # Create symbolic links to binaries
  local bin_dir="$HOME/.local/bin"
  create_dir "$bin_dir"
  
  ln -sf "$LINUX_INSTALL_DIR/bin/smash" "$bin_dir/smash"
  ln -sf "$LINUX_INSTALL_DIR/bin/smashc" "$bin_dir/smashc"
  ln -sf "$LINUX_INSTALL_DIR/bin/smashpkg" "$bin_dir/smashpkg"
  
  # Create configuration file
  create_config_linux
  
  # Install SmashLang packages
  echo -e "${BLUE}Installing SmashLang packages...${NC}"
  
  # Copy assets directory
  echo -e "${BLUE}Copying assets directory...${NC}"
  create_dir "$LINUX_INSTALL_DIR/assets"
  
  # Note: Package assets generation is only needed when publishing packages
  
  echo -e "${GREEN}SmashLang has been successfully installed on Linux!${NC}"
  echo -e "Run 'smash --version' to verify the installation."
  echo -e "Note: Package assets generation is only needed when publishing packages."
  echo -e "Run 'scripts/generate_package_logo.sh' and 'scripts/generate_favicon.sh' manually if needed."
  
  return 0
}

# If this script is run directly, execute the install_linux function
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
  # Check if the repository directory is provided
  if [ -z "$1" ]; then
    echo -e "${RED}Error: Repository directory not provided.${NC}"
    echo -e "Usage: $0 <repository_directory>"
    exit 1
  fi
  
  # Install SmashLang
  install_linux "$1"
  exit $?
fi
