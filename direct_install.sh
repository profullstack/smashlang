#!/bin/bash

# This script directly installs SmashLang without relying on the GitHub installer

set -e

# Set color variables
RED="\033[0;31m"
GREEN="\033[0;32m"
YELLOW="\033[0;33m"
BLUE="\033[0;34m"
NC="\033[0m" # No Color

# Version information
VERSION="0.1.0"

# Repository URL
REPO_URL="https://github.com/profullstack/smashlang.git"

# Installation directories
LINUX_INSTALL_DIR="$HOME/.local/share/smashlang"
MACOS_INSTALL_DIR="$HOME/Library/Application Support/smashlang"
WINDOWS_INSTALL_DIR="$APPDATA\\smashlang"

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

# Detect operating system
detect_os() {
  if [ "$(uname)" == "Darwin" ]; then
    echo "macos"
  elif [ "$(expr substr $(uname -s) 1 5)" == "Linux" ]; then
    echo "linux"
  elif [ "$(expr substr $(uname -s) 1 10)" == "MINGW32_NT" ] || \
       [ "$(expr substr $(uname -s) 1 10)" == "MINGW64_NT" ] || \
       [ "$(expr substr $(uname -s) 1 9)" == "MSYS_NT" ]; then
    echo "windows"
  else
    echo "unknown"
  fi
}

# Check for required tools
check_requirements() {
  local os=$1
  local missing_tools=false
  
  # Check for common tools
  if ! command -v git &> /dev/null; then
    echo -e "${RED}Error: git is not installed. Please install git and try again.${NC}"
    missing_tools=true
  fi
  
  if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: cargo is not installed. Please install Rust and try again.${NC}"
    missing_tools=true
  fi
  
  # Check for OS-specific tools
  if [ "$os" == "linux" ]; then
    if ! command -v gcc &> /dev/null; then
      echo -e "${RED}Error: gcc is not installed. Please install gcc and try again.${NC}"
      missing_tools=true
    fi
  elif [ "$os" == "macos" ]; then
    if ! command -v clang &> /dev/null; then
      echo -e "${RED}Error: clang is not installed. Please install Xcode command line tools and try again.${NC}"
      missing_tools=true
    fi
  elif [ "$os" == "windows" ]; then
    if ! command -v cl &> /dev/null; then
      echo -e "${YELLOW}Warning: MSVC compiler not found in path. You may need to run this from a Developer Command Prompt.${NC}"
    fi
  fi
  
  if [ "$missing_tools" == "true" ]; then
    exit 1
  fi
}

# Create directory if it doesn't exist
create_dir() {
  local dir=$1
  
  if [ ! -d "$dir" ]; then
    mkdir -p "$dir"
    if [ $? -ne 0 ]; then
      echo -e "${RED}Error: Failed to create directory $dir${NC}"
      exit 1
    fi
  fi
}

# Install SmashLang on Linux
install_linux() {
  echo -e "${BLUE}Installing SmashLang on Linux...${NC}"
  
  # Create installation directory
  create_dir "$LINUX_INSTALL_DIR"
  create_dir "$LINUX_INSTALL_DIR/bin"
  create_dir "$LINUX_INSTALL_DIR/src"
  create_dir "$LINUX_INSTALL_DIR/docs"
  
  echo -e "${BLUE}Using GitHub master branch for installation...${NC}"
  
  # Create a temporary directory for cloning the repository
  local temp_dir=$(mktemp -d)
  trap "rm -rf $temp_dir" EXIT
  
  # Clone the repository
  echo -e "${BLUE}Cloning SmashLang repository...${NC}"
  git clone --depth 1 "$REPO_URL" "$temp_dir"
  
  # Run tests when using master branch
  run_tests "$temp_dir"
  
  # Copy binaries from the repository
  echo -e "${BLUE}Installing SmashLang binaries...${NC}"
  echo -e "${BLUE}Building SmashLang from source...${NC}"
  
  # Capture git hash for version info
  local GIT_HASH=$(cd "$temp_dir" && git rev-parse --short HEAD)
  echo "$GIT_HASH" > "$temp_dir/src/git_hash.txt"
  
  # Make sure the git hash file gets copied to the installation directory
  mkdir -p "$LINUX_INSTALL_DIR/src"
  cp "$temp_dir/src/git_hash.txt" "$LINUX_INSTALL_DIR/git_hash.txt"
  cp "$temp_dir/src/git_hash.txt" "$LINUX_INSTALL_DIR/src/git_hash.txt"
  
  # Build release version
  cd "$temp_dir"
  cargo build --release
  
  # Copy binaries
  cp "$temp_dir/target/release/smash" "$LINUX_INSTALL_DIR/bin/"
  cp "$temp_dir/target/release/smashc" "$LINUX_INSTALL_DIR/bin/"
  cp "$temp_dir/target/release/smashpkg" "$LINUX_INSTALL_DIR/bin/"
  
  # Copy documentation
  echo -e "${BLUE}Installing documentation...${NC}"
  if [ -d "$temp_dir/docs" ]; then
    cp -r "$temp_dir/docs" "$LINUX_INSTALL_DIR/"
    echo -e "${BLUE}Documentation installed to $LINUX_INSTALL_DIR/docs${NC}"
  fi
  
  # Create symbolic links to binaries
  local bin_dir="$HOME/.local/bin"
  create_dir "$bin_dir"
  
  ln -sf "$LINUX_INSTALL_DIR/bin/smash" "$bin_dir/smash"
  ln -sf "$LINUX_INSTALL_DIR/bin/smashc" "$bin_dir/smashc"
  ln -sf "$LINUX_INSTALL_DIR/bin/smashpkg" "$bin_dir/smashpkg"
  
  # Create configuration file
  create_config_linux
  
  echo -e "${GREEN}SmashLang has been successfully installed on Linux!${NC}"
  echo -e "Run 'smash --version' to verify the installation."
}

# Create configuration file for Linux
create_config_linux() {
  local config_dir="$HOME/.config/smashlang"
  local config_file="$config_dir/config.json"
  
  create_dir "$config_dir"
  
  # Create or update configuration file
  cat > "$config_file" << EOF
{
  "version": "$VERSION",
  "install_dir": "$LINUX_INSTALL_DIR",
  "bin_dir": "$LINUX_INSTALL_DIR/bin",
  "docs_dir": "$LINUX_INSTALL_DIR/docs",
  "assets_dir": "$LINUX_INSTALL_DIR/assets",
  "packages_dir": "$HOME/.local/share/smashlang_packages"
}
EOF
  
  echo -e "${BLUE}Creating configuration file...${NC}"
}

# Display welcome message
display_welcome() {
  echo -e ""
  echo -e "   _____                      _     _                       "
  echo -e "  / ____|                    | |   | |                      "
  echo -e " | (___  _ __ ___   __ _ ___| |__ | |     __ _ _ __   __ _ "
  echo -e "  \___ \| '_ ' _ \ / _' / __| '_ \| |    / _' | '_ \ / _' |"
  echo -e "  ____) | | | | | | (_| \__ \ | | | |___| (_| | | | | (_| |"
  echo -e " |_____/|_| |_| |_|\__,_|___/_| |_|______|\__,_|_| |_|\__, |"
  echo -e "                                                        __/ |"
  echo -e "                                                       |___/ "
  echo -e ""
  echo -e "ud83dudcaa Welcome to SmashLang! ud83dudcaa"
  echo -e "A bold, high-performance, JavaScript-inspired general-purpose programming language"
  echo -e "that compiles to native binaries. Made for developers who want the power of C/Rust"
  echo -e "but the clarity of JavaScript u2014 without the bloat."
  echo -e ""
  echo -e "Visit https://smashlang.com for documentation and community resources."
  echo -e ""
  echo -e ""
  echo -e "SmashLang Installer v$VERSION"
}

# Main function
main() {
  # Display welcome message
  display_welcome
  
  # Detect operating system
  local os=$(detect_os)
  echo -e "${BLUE}Detected operating system: $os${NC}"
  
  # Check for required tools
  check_requirements "$os"
  
  # Install SmashLang based on the detected OS
  if [ "$os" == "linux" ]; then
    install_linux
  elif [ "$os" == "macos" ]; then
    echo -e "${YELLOW}macOS installation is not implemented in this script yet.${NC}"
    echo -e "Please use the official installer or install manually."
    exit 1
  elif [ "$os" == "windows" ]; then
    echo -e "${YELLOW}Windows installation is not implemented in this script yet.${NC}"
    echo -e "Please use the official installer or install manually."
    exit 1
  else
    echo -e "${RED}Error: Unsupported operating system.${NC}"
    exit 1
  fi
}

# Run the main function
main
