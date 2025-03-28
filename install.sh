#!/bin/bash

# SmashLang Installer Script
# This script installs SmashLang on Windows, macOS, and Linux systems

set -e

# Colors for output
RED="\033[0;31m"
GREEN="\033[0;32m"
YELLOW="\033[0;33m"
BLUE="\033[0;34m"
NC="\033[0m" # No Color

# SmashLang version
VERSION="0.1.0"

# Default version for upgrades
DEFAULT_VERSION="0.1.0"

# GitHub repository
REPO="profullstack/smashlang"
REPO_URL="https://github.com/$REPO"
RELEASE_URL="$REPO_URL/releases/download/v$VERSION"

# Installation directories
LINUX_INSTALL_DIR="$HOME/.local/bin"
MACOS_INSTALL_DIR="$HOME/.local/bin"
WINDOWS_INSTALL_DIR="$HOME/AppData/Local/SmashLang"

# Package directories
LINUX_PACKAGES_DIR="$HOME/.local/share/smashlang/packages"
MACOS_PACKAGES_DIR="$HOME/Library/Application Support/SmashLang/packages"
WINDOWS_PACKAGES_DIR="$HOME/AppData/Local/SmashLang/packages"

# Command line arguments
COMMAND="install"
TARGET_VERSION="$DEFAULT_VERSION"
USE_MASTER=false

if [[ "$1" == "upgrade" ]]; then
  COMMAND="upgrade"
  shift
  
  # Check for version flag
  if [[ "$1" == "--version" && -n "$2" ]]; then
    TARGET_VERSION="$2"
    shift 2
  fi
elif [[ "$1" == "--master" ]]; then
  USE_MASTER=true
  shift
elif [[ "$1" == "--help" || "$1" == "-h" ]]; then
  echo "Usage: ./install.sh [command] [options]"
  echo "Commands:"
  echo "  install         Install SmashLang (default)"
  echo "  upgrade         Upgrade or downgrade SmashLang"
  echo ""
  echo "Options:"
  echo "  --version VER   Specify version for upgrade (default: latest)"
  echo "  --master        Use GitHub master branch instead of releases"
  echo "  --help, -h      Show this help message"
  exit 0
fi

# Detect operating system
detect_os() {
  if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    echo "linux"
  elif [[ "$OSTYPE" == "darwin"* ]]; then
    echo "macos"
  elif [[ "$OSTYPE" == "msys"* || "$OSTYPE" == "cygwin"* || "$OSTYPE" == "win32"* ]]; then
    echo "windows"
  else
    echo "unknown"
  fi
}

# Check for required tools
check_requirements() {
  local os=$1
  
  # Check for curl or wget
  if ! command -v curl &> /dev/null && ! command -v wget &> /dev/null; then
    echo -e "${RED}Error: Neither curl nor wget found. Please install one of them and try again.${NC}"
    exit 1
  fi
  
  # Check for git if using master branch
  if [ "$USE_MASTER" = true ] && ! command -v git &> /dev/null; then
    echo -e "${RED}Error: git not found. Please install git and try again.${NC}"
    exit 1
  fi
  
  # Check for unzip (needed for Windows)
  if [[ "$os" == "windows" ]] && ! command -v unzip &> /dev/null; then
    echo -e "${RED}Error: unzip not found. Please install unzip and try again.${NC}"
    exit 1
  fi
  
  # Check for tar (needed for Linux and macOS)
  if [[ "$os" != "windows" ]] && ! command -v tar &> /dev/null; then
    echo -e "${RED}Error: tar not found. Please install tar and try again.${NC}"
    exit 1
  fi
}

# Download a file
download() {
  local url=$1
  local output=$2
  
  echo -e "${BLUE}Downloading $url...${NC}"
  
  if command -v curl &> /dev/null; then
    curl -L -o "$output" "$url"
  else
    wget -O "$output" "$url"
  fi
}

# Create directory if it doesn't exist
create_dir() {
  local dir=$1
  
  if [[ ! -d "$dir" ]]; then
    echo -e "${BLUE}Creating directory $dir...${NC}"
    mkdir -p "$dir"
  fi
}

# Install SmashLang on Linux
install_linux() {
  echo -e "${GREEN}Installing SmashLang on Linux...${NC}"
  
  # Set up directories
  create_dir "$LINUX_INSTALL_DIR"
  create_dir "$LINUX_PACKAGES_DIR"
  
  if [ "$USE_MASTER" = true ]; then
    echo -e "${YELLOW}Using GitHub master branch for installation...${NC}"
    
    # Clone the repository
    local temp_dir="/tmp/smashlang-master"
    rm -rf "$temp_dir"
    echo -e "${BLUE}Cloning SmashLang repository...${NC}"
    git clone --depth 1 "$REPO_URL" "$temp_dir"
    
    # Copy binaries from the repository
    echo -e "${BLUE}Installing SmashLang binaries...${NC}"
    if [ -f "$temp_dir/bin/smash" ]; then
      cp "$temp_dir/bin/smash" "$LINUX_INSTALL_DIR/"
      chmod +x "$LINUX_INSTALL_DIR/smash"
    else
      echo -e "${YELLOW}Warning: smash binary not found in repository, creating placeholder...${NC}"
      echo '#!/bin/bash\necho "SmashLang v0.1.0-dev (placeholder)"' > "$LINUX_INSTALL_DIR/smash"
      chmod +x "$LINUX_INSTALL_DIR/smash"
    fi
    
    if [ -f "$temp_dir/bin/smashpkg" ]; then
      cp "$temp_dir/bin/smashpkg" "$LINUX_INSTALL_DIR/"
      chmod +x "$LINUX_INSTALL_DIR/smashpkg"
    else
      echo -e "${YELLOW}Warning: smashpkg binary not found in repository, creating placeholder...${NC}"
      echo '#!/bin/bash\necho "SmashLang Package Manager v0.1.0-dev (placeholder)"' > "$LINUX_INSTALL_DIR/smashpkg"
      chmod +x "$LINUX_INSTALL_DIR/smashpkg"
    fi
    
    # Copy packages from the repository
    echo -e "${BLUE}Installing SmashLang packages...${NC}"
    if [ -d "$temp_dir/smashlang_packages" ]; then
      cp -r "$temp_dir/smashlang_packages"/* "$LINUX_PACKAGES_DIR/"
    else
      echo -e "${YELLOW}Warning: smashlang_packages directory not found in repository${NC}"
    fi
  else
    # Download SmashLang binary
    local binary_url="$RELEASE_URL/smashlang-linux-x64.tar.gz"
    local binary_file="/tmp/smashlang-linux-x64.tar.gz"
    download "$binary_url" "$binary_file"
    
    # Extract binary
    echo -e "${BLUE}Extracting SmashLang binary...${NC}"
    tar -xzf "$binary_file" -C "$LINUX_INSTALL_DIR"
    
    # Download packages
    local packages_url="$RELEASE_URL/smashlang_packages.tar.gz"
    local packages_file="/tmp/smashlang_packages.tar.gz"
    download "$packages_url" "$packages_file"
    
    # Extract packages
    echo -e "${BLUE}Extracting SmashLang packages...${NC}"
    tar -xzf "$packages_file" -C "$LINUX_PACKAGES_DIR"
  fi
  
  # Create configuration file
  create_config_linux
  
  # Make binary executable
  chmod +x "$LINUX_INSTALL_DIR/smash"
  chmod +x "$LINUX_INSTALL_DIR/smashpkg"
  
  # Add to PATH if not already there
  if [[ ":$PATH:" != *":$LINUX_INSTALL_DIR:"* ]]; then
    echo -e "${YELLOW}Adding SmashLang to PATH in ~/.bashrc...${NC}"
    echo "export PATH=\"$LINUX_INSTALL_DIR:\$PATH\"" >> "$HOME/.bashrc"
    echo -e "${YELLOW}Please run 'source ~/.bashrc' or start a new terminal to use SmashLang.${NC}"
  fi
  
  echo -e "${GREEN}SmashLang has been successfully installed on Linux!${NC}"
  echo -e "${GREEN}Run 'smash --version' to verify the installation.${NC}"
}

# Install SmashLang on macOS
install_macos() {
  echo -e "${GREEN}Installing SmashLang on macOS...${NC}"
  
  # Set up directories
  create_dir "$MACOS_INSTALL_DIR"
  create_dir "$MACOS_PACKAGES_DIR"
  
  if [ "$USE_MASTER" = true ]; then
    echo -e "${YELLOW}Using GitHub master branch for installation...${NC}"
    
    # Clone the repository
    local temp_dir="/tmp/smashlang-master"
    rm -rf "$temp_dir"
    echo -e "${BLUE}Cloning SmashLang repository...${NC}"
    git clone --depth 1 "$REPO_URL" "$temp_dir"
    
    # Copy binaries from the repository
    echo -e "${BLUE}Installing SmashLang binaries...${NC}"
    if [ -f "$temp_dir/bin/smash" ]; then
      cp "$temp_dir/bin/smash" "$MACOS_INSTALL_DIR/"
      chmod +x "$MACOS_INSTALL_DIR/smash"
    else
      echo -e "${YELLOW}Warning: smash binary not found in repository, creating placeholder...${NC}"
      echo '#!/bin/bash\necho "SmashLang v0.1.0-dev (placeholder)"' > "$MACOS_INSTALL_DIR/smash"
      chmod +x "$MACOS_INSTALL_DIR/smash"
    fi
    
    if [ -f "$temp_dir/bin/smashpkg" ]; then
      cp "$temp_dir/bin/smashpkg" "$MACOS_INSTALL_DIR/"
      chmod +x "$MACOS_INSTALL_DIR/smashpkg"
    else
      echo -e "${YELLOW}Warning: smashpkg binary not found in repository, creating placeholder...${NC}"
      echo '#!/bin/bash\necho "SmashLang Package Manager v0.1.0-dev (placeholder)"' > "$MACOS_INSTALL_DIR/smashpkg"
      chmod +x "$MACOS_INSTALL_DIR/smashpkg"
    fi
    
    # Copy packages from the repository
    echo -e "${BLUE}Installing SmashLang packages...${NC}"
    if [ -d "$temp_dir/smashlang_packages" ]; then
      cp -r "$temp_dir/smashlang_packages"/* "$MACOS_PACKAGES_DIR/"
    else
      echo -e "${YELLOW}Warning: smashlang_packages directory not found in repository${NC}"
    fi
  else
    # Download SmashLang binary
    local binary_url="$RELEASE_URL/smashlang-macos-x64.tar.gz"
    local binary_file="/tmp/smashlang-macos-x64.tar.gz"
    download "$binary_url" "$binary_file"
    
    # Extract binary
    echo -e "${BLUE}Extracting SmashLang binary...${NC}"
    tar -xzf "$binary_file" -C "$MACOS_INSTALL_DIR"
    
    # Download packages
    local packages_url="$RELEASE_URL/smashlang_packages.tar.gz"
    local packages_file="/tmp/smashlang_packages.tar.gz"
    download "$packages_url" "$packages_file"
    
    # Extract packages
    echo -e "${BLUE}Extracting SmashLang packages...${NC}"
    tar -xzf "$packages_file" -C "$MACOS_PACKAGES_DIR"
  fi
  
  # Create configuration file
  create_config_macos
  
  # Make binary executable
  chmod +x "$MACOS_INSTALL_DIR/smash"
  chmod +x "$MACOS_INSTALL_DIR/smashpkg"
  
  # Add to PATH if not already there
  if [[ ":$PATH:" != *":$MACOS_INSTALL_DIR:"* ]]; then
    echo -e "${YELLOW}Adding SmashLang to PATH in ~/.zshrc...${NC}"
    echo "export PATH=\"$MACOS_INSTALL_DIR:\$PATH\"" >> "$HOME/.zshrc"
    echo -e "${YELLOW}Please run 'source ~/.zshrc' or start a new terminal to use SmashLang.${NC}"
  fi
  
  echo -e "${GREEN}SmashLang has been successfully installed on macOS!${NC}"
  echo -e "${GREEN}Run 'smash --version' to verify the installation.${NC}"
}

# Install SmashLang on Windows
install_windows() {
  echo -e "${GREEN}Installing SmashLang on Windows...${NC}"
  
  # Set up directories
  create_dir "$WINDOWS_INSTALL_DIR"
  create_dir "$WINDOWS_PACKAGES_DIR"
  
  if [ "$USE_MASTER" = true ]; then
    echo -e "${YELLOW}Using GitHub master branch for installation...${NC}"
    
    # Clone the repository
    local temp_dir="/tmp/smashlang-master"
    rm -rf "$temp_dir"
    echo -e "${BLUE}Cloning SmashLang repository...${NC}"
    git clone --depth 1 "$REPO_URL" "$temp_dir"
    
    # Copy binaries from the repository
    echo -e "${BLUE}Installing SmashLang binaries...${NC}"
    if [ -f "$temp_dir/bin/smash.exe" ]; then
      cp "$temp_dir/bin/smash.exe" "$WINDOWS_INSTALL_DIR/"
    else
      echo -e "${YELLOW}Warning: smash.exe binary not found in repository, creating placeholder...${NC}"
      echo '@echo off\necho SmashLang v0.1.0-dev (placeholder)' > "$WINDOWS_INSTALL_DIR/smash.bat"
    fi
    
    if [ -f "$temp_dir/bin/smashpkg.exe" ]; then
      cp "$temp_dir/bin/smashpkg.exe" "$WINDOWS_INSTALL_DIR/"
    else
      echo -e "${YELLOW}Warning: smashpkg.exe binary not found in repository, creating placeholder...${NC}"
      echo '@echo off\necho SmashLang Package Manager v0.1.0-dev (placeholder)' > "$WINDOWS_INSTALL_DIR/smashpkg.bat"
    fi
    
    # Copy packages from the repository
    echo -e "${BLUE}Installing SmashLang packages...${NC}"
    if [ -d "$temp_dir/smashlang_packages" ]; then
      cp -r "$temp_dir/smashlang_packages"/* "$WINDOWS_PACKAGES_DIR/"
    else
      echo -e "${YELLOW}Warning: smashlang_packages directory not found in repository${NC}"
    fi
  else
    # Download SmashLang binary
    local binary_url="$RELEASE_URL/smashlang-windows-x64.zip"
    local binary_file="/tmp/smashlang-windows-x64.zip"
    download "$binary_url" "$binary_file"
    
    # Extract binary
    echo -e "${BLUE}Extracting SmashLang binary...${NC}"
    unzip -o "$binary_file" -d "$WINDOWS_INSTALL_DIR"
    
    # Download packages
    local packages_url="$RELEASE_URL/smashlang_packages.zip"
    local packages_file="/tmp/smashlang_packages.zip"
    download "$packages_url" "$packages_file"
    
    # Extract packages
    echo -e "${BLUE}Extracting SmashLang packages...${NC}"
    unzip -o "$packages_file" -d "$WINDOWS_PACKAGES_DIR"
  fi
  
  # Create configuration file
  create_config_windows
  
  # Add to PATH
  echo -e "${YELLOW}Please add $WINDOWS_INSTALL_DIR to your PATH manually.${NC}"
  echo -e "${YELLOW}You can do this by editing your system environment variables.${NC}"
  
  echo -e "${GREEN}SmashLang has been successfully installed on Windows!${NC}"
  echo -e "${GREEN}Run 'smash --version' to verify the installation.${NC}"
}

# Create configuration file for Linux
create_config_linux() {
  local config_dir="$HOME/.config/smashlang"
  local config_file="$config_dir/config.json"
  
  create_dir "$config_dir"
  
  echo -e "${BLUE}Creating configuration file...${NC}"
  cat > "$config_file" << EOF
{
  "version": "$VERSION",
  "packagesDir": "$LINUX_PACKAGES_DIR",
  "modulesDir": "./smash_modules",
  "autoUpdate": true,
  "logLevel": "info"
}
EOF
}

# Create configuration file for macOS
create_config_macos() {
  local config_dir="$HOME/Library/Application Support/SmashLang"
  local config_file="$config_dir/config.json"
  
  create_dir "$config_dir"
  
  echo -e "${BLUE}Creating configuration file...${NC}"
  cat > "$config_file" << EOF
{
  "version": "$VERSION",
  "packagesDir": "$MACOS_PACKAGES_DIR",
  "modulesDir": "./smash_modules",
  "autoUpdate": true,
  "logLevel": "info"
}
EOF
}

# Create configuration file for Windows
create_config_windows() {
  local config_dir="$HOME/AppData/Local/SmashLang"
  local config_file="$config_dir/config.json"
  
  create_dir "$config_dir"
  
  echo -e "${BLUE}Creating configuration file...${NC}"
  cat > "$config_file" << EOF
{
  "version": "$VERSION",
  "packagesDir": "$WINDOWS_PACKAGES_DIR",
  "modulesDir": "./smash_modules",
  "autoUpdate": true,
  "logLevel": "info"
}
EOF
}

# Upgrade SmashLang on Linux
upgrade_linux() {
  local version="$1"
  echo -e "${GREEN}Upgrading SmashLang to version $version on Linux...${NC}"
  
  # Set up directories if they don't exist
  create_dir "$LINUX_INSTALL_DIR"
  create_dir "$LINUX_PACKAGES_DIR"
  
  # Download SmashLang binary for the specified version
  local binary_url="$REPO_URL/releases/download/v$version/smashlang-linux-x64.tar.gz"
  local binary_file="/tmp/smashlang-linux-x64-$version.tar.gz"
  download "$binary_url" "$binary_file"
  
  # Extract binary
  echo -e "${BLUE}Extracting SmashLang binary...${NC}"
  tar -xzf "$binary_file" -C "$LINUX_INSTALL_DIR"
  
  # Download packages for the specified version
  local packages_url="$REPO_URL/releases/download/v$version/smashlang_packages.tar.gz"
  local packages_file="/tmp/smashlang_packages-$version.tar.gz"
  download "$packages_url" "$packages_file"
  
  # Extract packages
  echo -e "${BLUE}Extracting SmashLang packages...${NC}"
  tar -xzf "$packages_file" -C "$LINUX_PACKAGES_DIR"
  
  # Update configuration file with new version
  update_config_linux "$version"
  
  # Make binary executable
  chmod +x "$LINUX_INSTALL_DIR/smash"
  chmod +x "$LINUX_INSTALL_DIR/smashpkg"
  
  echo -e "${GREEN}SmashLang has been successfully upgraded to version $version on Linux!${NC}"
  echo -e "${GREEN}Run 'smash --version' to verify the upgrade.${NC}"
}

# Upgrade SmashLang on macOS
upgrade_macos() {
  local version="$1"
  echo -e "${GREEN}Upgrading SmashLang to version $version on macOS...${NC}"
  
  # Set up directories if they don't exist
  create_dir "$MACOS_INSTALL_DIR"
  create_dir "$MACOS_PACKAGES_DIR"
  
  # Download SmashLang binary for the specified version
  local binary_url="$REPO_URL/releases/download/v$version/smashlang-macos-x64.tar.gz"
  local binary_file="/tmp/smashlang-macos-x64-$version.tar.gz"
  download "$binary_url" "$binary_file"
  
  # Extract binary
  echo -e "${BLUE}Extracting SmashLang binary...${NC}"
  tar -xzf "$binary_file" -C "$MACOS_INSTALL_DIR"
  
  # Download packages for the specified version
  local packages_url="$REPO_URL/releases/download/v$version/smashlang_packages.tar.gz"
  local packages_file="/tmp/smashlang_packages-$version.tar.gz"
  download "$packages_url" "$packages_file"
  
  # Extract packages
  echo -e "${BLUE}Extracting SmashLang packages...${NC}"
  tar -xzf "$packages_file" -C "$MACOS_PACKAGES_DIR"
  
  # Update configuration file with new version
  update_config_macos "$version"
  
  # Make binary executable
  chmod +x "$MACOS_INSTALL_DIR/smash"
  chmod +x "$MACOS_INSTALL_DIR/smashpkg"
  
  echo -e "${GREEN}SmashLang has been successfully upgraded to version $version on macOS!${NC}"
  echo -e "${GREEN}Run 'smash --version' to verify the upgrade.${NC}"
}

# Upgrade SmashLang on Windows
upgrade_windows() {
  local version="$1"
  echo -e "${GREEN}Upgrading SmashLang to version $version on Windows...${NC}"
  
  # Set up directories if they don't exist
  create_dir "$WINDOWS_INSTALL_DIR"
  create_dir "$WINDOWS_PACKAGES_DIR"
  
  # Download SmashLang binary for the specified version
  local binary_url="$REPO_URL/releases/download/v$version/smashlang-windows-x64.zip"
  local binary_file="/tmp/smashlang-windows-x64-$version.zip"
  download "$binary_url" "$binary_file"
  
  # Extract binary
  echo -e "${BLUE}Extracting SmashLang binary...${NC}"
  unzip -o "$binary_file" -d "$WINDOWS_INSTALL_DIR"
  
  # Download packages for the specified version
  local packages_url="$REPO_URL/releases/download/v$version/smashlang_packages.zip"
  local packages_file="/tmp/smashlang_packages-$version.zip"
  download "$packages_url" "$packages_file"
  
  # Extract packages
  echo -e "${BLUE}Extracting SmashLang packages...${NC}"
  unzip -o "$packages_file" -d "$WINDOWS_PACKAGES_DIR"
  
  # Update configuration file with new version
  update_config_windows "$version"
  
  echo -e "${GREEN}SmashLang has been successfully upgraded to version $version on Windows!${NC}"
  echo -e "${GREEN}Run 'smash --version' to verify the upgrade.${NC}"
}

# Update configuration file for Linux with new version
update_config_linux() {
  local version="$1"
  local config_dir="$HOME/.config/smashlang"
  local config_file="$config_dir/config.json"
  
  create_dir "$config_dir"
  
  echo -e "${BLUE}Updating configuration file...${NC}"
  cat > "$config_file" << EOF
{
  "version": "$version",
  "packagesDir": "$LINUX_PACKAGES_DIR",
  "modulesDir": "./smash_modules",
  "autoUpdate": true,
  "logLevel": "info"
}
EOF
}

# Update configuration file for macOS with new version
update_config_macos() {
  local version="$1"
  local config_dir="$HOME/Library/Application Support/SmashLang"
  local config_file="$config_dir/config.json"
  
  create_dir "$config_dir"
  
  echo -e "${BLUE}Updating configuration file...${NC}"
  cat > "$config_file" << EOF
{
  "version": "$version",
  "packagesDir": "$MACOS_PACKAGES_DIR",
  "modulesDir": "./smash_modules",
  "autoUpdate": true,
  "logLevel": "info"
}
EOF
}

# Update configuration file for Windows with new version
update_config_windows() {
  local version="$1"
  local config_dir="$HOME/AppData/Local/SmashLang"
  local config_file="$config_dir/config.json"
  
  create_dir "$config_dir"
  
  echo -e "${BLUE}Updating configuration file...${NC}"
  cat > "$config_file" << EOF
{
  "version": "$version",
  "packagesDir": "$WINDOWS_PACKAGES_DIR",
  "modulesDir": "./smash_modules",
  "autoUpdate": true,
  "logLevel": "info"
}
EOF
}

# Get script directory
get_script_dir() {
  local source="${BASH_SOURCE[0]}"
  while [ -h "$source" ]; do
    local dir="$(cd -P "$(dirname "$source")" && pwd)"
    source="$(readlink "$source")"
    [[ $source != /* ]] && source="$dir/$source"
  done
  echo "$(cd -P "$(dirname "$source")" && pwd)"
}

# Display welcome message
display_welcome() {
  local script_dir=$(get_script_dir)
  local logo_file="$script_dir/assets/logo.ascii"
  
  # Check if logo file exists, otherwise use default ASCII art
  if [ -f "$logo_file" ]; then
    echo -e "\n${YELLOW}"
    cat "$logo_file" | while read line; do
      echo -e "${YELLOW}$line${NC}"
    done
  else
    # Fallback ASCII art if the file doesn't exist
    echo -e "\n${YELLOW}   _____                      _     _                       ${NC}"
    echo -e "${YELLOW}  / ____|                    | |   | |                      ${NC}"
    echo -e "${YELLOW} | (___  _ __ ___   __ _ ___| |__ | |     __ _ _ __   __ _ ${NC}"
    echo -e "${YELLOW}  \___ \| '_ ' _ \ / _' / __| '_ \| |    / _' | '_ \ / _' |${NC}"
    echo -e "${YELLOW}  ____) | | | | | | (_| \__ \ | | | |___| (_| | | | | (_| |${NC}"
    echo -e "${YELLOW} |_____/|_| |_| |_|\__,_|___/_| |_|______\__,_|_| |_|\__, |${NC}"
    echo -e "${YELLOW}                                                        __/ |${NC}"
    echo -e "${YELLOW}                                                       |___/ ${NC}"
  fi
  echo -e "\n${GREEN}💪 Welcome to SmashLang! 💪${NC}"
  echo -e "${BLUE}A bold, high-performance, JavaScript-inspired general-purpose programming language${NC}"
  echo -e "${BLUE}that compiles to native binaries. Made for developers who want the power of C/Rust${NC}"
  echo -e "${BLUE}but the clarity of JavaScript — without the bloat.${NC}"
  echo -e "\n${YELLOW}Visit https://smashlang.com for documentation and community resources.${NC}"
  echo -e "\n"
}

# Main function
main() {
  # Parse command line arguments
  COMMAND="install"
  TARGET_VERSION="latest"
  GENERATE_LOGOS=true

  while [[ $# -gt 0 ]]; do
    case $1 in
      install)
        COMMAND="install"
        shift
        ;;
      upgrade)
        COMMAND="upgrade"
        shift
        ;;
      --version)
        TARGET_VERSION="$2"
        shift 2
        ;;
      --no-logos)
        GENERATE_LOGOS=false
        shift
        ;;
      -h|--help)
        display_help
        exit 0
        ;;
      *)
        echo -e "${RED}Unknown option: $1${NC}"
        display_help
        exit 1
        ;;
    esac
  done

  display_welcome
  
  if [[ "$COMMAND" == "install" ]]; then
    echo -e "${GREEN}SmashLang Installer v$VERSION${NC}"
  else
    echo -e "${GREEN}SmashLang Upgrader - Target Version: $TARGET_VERSION${NC}"
  fi
  
  # Detect operating system
  local os=$(detect_os)
  
  if [[ "$os" == "unknown" ]]; then
    echo -e "${RED}Error: Unsupported operating system.${NC}"
    echo -e "${RED}This installer supports Linux, macOS, and Windows.${NC}"
    exit 1
  fi
  
  echo -e "${BLUE}Detected operating system: $os${NC}"
  
  # Check requirements
  check_requirements "$os"
  
  # Install or upgrade based on command and OS
  if [[ "$COMMAND" == "install" ]]; then
    case "$os" in
      linux)
        install_linux
        ;;
      macos)
        install_macos
        ;;
      windows)
        install_windows
        ;;
    esac
    
    # Generate package asset files after installation
    if [ "$GENERATE_LOGOS" = true ]; then
      generate_package_assets
    fi
  else
    # Upgrade command
    case "$os" in
      linux)
        upgrade_linux "$TARGET_VERSION"
        ;;
      macos)
        upgrade_macos "$TARGET_VERSION"
        ;;
      windows)
        upgrade_windows "$TARGET_VERSION"
        ;;
    esac
    
    # Generate package asset files after upgrade
    if [ "$GENERATE_LOGOS" = true ]; then
      generate_package_assets
    fi
  fi
}

# Display help message
display_help() {
  echo "Usage: ./install.sh [command] [options]"
  echo "Commands:"
  echo "  install         Install SmashLang (default)"
  echo "  upgrade         Upgrade or downgrade SmashLang"
  echo ""
  echo "Options:"
  echo "  --version VER   Specify version for upgrade (default: latest)"
  echo "  --no-logos      Skip generation of package asset files (logo.txt, favicon.txt)"
  echo "  --help, -h      Show this help message"
}

# Generate logo.txt and favicon.txt files for packages
generate_package_assets() {
  local script_dir=$(get_script_dir)
  
  # Generate logo.txt files
  if [ -f "$script_dir/scripts/generate_package_logo.sh" ]; then
    echo -e "${BLUE}Generating logo.txt files for packages...${NC}"
    
    # Make sure the script is executable
    chmod +x "$script_dir/scripts/generate_package_logo.sh"
    
    # Update the package template logo.txt
    "$script_dir/scripts/generate_package_logo.sh" --template
    
    # Generate logo.txt for all existing packages
    "$script_dir/scripts/generate_package_logo.sh" --all
    
    echo -e "${GREEN}Package logo.txt files generated successfully.${NC}"
  else
    echo -e "${YELLOW}Warning: Package logo generator script not found. Skipping logo.txt generation.${NC}"
  fi
  
  # Generate favicon.txt files
  if [ -f "$script_dir/scripts/generate_favicon.sh" ]; then
    echo -e "${BLUE}Generating favicon.txt files for packages...${NC}"
    
    # Make sure the script is executable
    chmod +x "$script_dir/scripts/generate_favicon.sh"
    
    # Update the package template favicon.txt
    "$script_dir/scripts/generate_favicon.sh" --template
    
    # Generate favicon.txt for all existing packages
    "$script_dir/scripts/generate_favicon.sh" --all
    
    echo -e "${GREEN}Package favicon.txt files generated successfully.${NC}"
  else
    echo -e "${YELLOW}Warning: Package favicon generator script not found. Skipping favicon.txt generation.${NC}"
  fi
}

# Run the main function
main
