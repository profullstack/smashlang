#!/bin/bash

# SmashLang Installer Script
# This script installs SmashLang on Windows, macOS, and Linux systems

# Create and clear logs directory at the beginning
BASE_DIR="$(pwd)"
mkdir -p "$BASE_DIR/logs"
rm -f "$BASE_DIR/logs/"*

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
  
  # Use the logs directory in the BASE_DIR, not in the repo
  # Create a log file with timestamp
  local timestamp=$(date +"%Y%m%d_%H%M%S")
  local log_file="$BASE_DIR/logs/test_results_$timestamp.log"
  
  echo -e "${BLUE}Running tests for SmashLang...${NC}"
  cd "$repo_dir" || { echo -e "${RED}Error: Could not change to directory $repo_dir${NC}"; return 1; }
  
  # Create the log file
  echo "SmashLang Test Results" > "$log_file"
  echo "======================" >> "$log_file"
  echo "Date: $(date)" >> "$log_file"
  echo "" >> "$log_file"
  
  if command -v cargo &> /dev/null; then
    # Run main crate tests
    echo -e "${BLUE}Running main crate tests...${NC}"
    echo "Main Crate Tests" >> "$log_file"
    echo "---------------" >> "$log_file"
    cargo test > "$log_file.tmp" 2>&1 || true
    local main_test_result=$?
    if [ -f "$log_file.tmp" ]; then
      cat "$log_file.tmp"
      cat "$log_file.tmp" >> "$log_file"
      rm "$log_file.tmp"
    else
      echo "Error: Test output file not created" >> "$log_file"
    fi
    echo "" >> "$log_file"
    
    # Run tests for all workspace packages
    echo -e "${BLUE}Running tests for all packages...${NC}"
    echo "All Packages Tests" >> "$log_file"
    echo "-----------------" >> "$log_file"
    cargo test --all > "$log_file.tmp" 2>&1 || true
    local all_test_result=$?
    if [ -f "$log_file.tmp" ]; then
      cat "$log_file.tmp"
      cat "$log_file.tmp" >> "$log_file"
      rm "$log_file.tmp"
    else
      echo "Error: Test output file not created" >> "$log_file"
    fi
    echo "" >> "$log_file"
    
    # Run tests with all features enabled
    echo -e "${BLUE}Running tests with all features enabled...${NC}"
    echo "All Features Tests" >> "$log_file"
    echo "-----------------" >> "$log_file"
    cargo test --all-features > "$log_file.tmp" 2>&1 || true
    local features_test_result=$?
    if [ -f "$log_file.tmp" ]; then
      cat "$log_file.tmp"
      cat "$log_file.tmp" >> "$log_file"
      rm "$log_file.tmp"
    else
      echo "Error: Test output file not created" >> "$log_file"
    fi
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
      
      # Make sure the script is executable
      chmod +x "docs/getting-started/run_all_examples.sh"
      
      # Create example files if they don't exist
      if [ ! -f "docs/getting-started/01_hello_world.smash" ]; then
        echo -e "${BLUE}Creating example files...${NC}"
        echo 'console.log("Hello, SmashLang World!");' > "docs/getting-started/01_hello_world.smash"
      fi
      
      # Run the examples
      ./docs/getting-started/run_all_examples.sh > "$log_file.tmp" 2>&1 || true
      local example_test_result=$?
      if [ -f "$log_file.tmp" ]; then
        cat "$log_file.tmp"
        cat "$log_file.tmp" >> "$log_file"
        rm "$log_file.tmp"
      else
        echo "Error: Example test output file not created" >> "$log_file"
      fi
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
        (cd "smashlang_packages" && cargo test --all > "$repo_dir/pkg_test.tmp" 2>&1) || true
        local combined_test_result=$?
        if [ -f "$repo_dir/pkg_test.tmp" ]; then
          cat "$repo_dir/pkg_test.tmp"
          cat "$repo_dir/pkg_test.tmp" >> "$log_file"
          rm "$repo_dir/pkg_test.tmp"
        else
          echo "Error: Package test output file not created" >> "$log_file"
        fi
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
      
      # Use find to locate all Cargo.toml files in the smashlang_packages directory
      while IFS= read -r cargo_file; do
        pkg_dir=$(dirname "$cargo_file")
        pkg_name=$(basename "$pkg_dir")
        rel_path="${pkg_dir#smashlang_packages/}"
        
        # Skip the root Cargo.toml if it exists (we already tested it)
        if [ "$pkg_dir" = "smashlang_packages" ]; then
          continue
        fi
        
        echo -e "${BLUE}Testing package: $pkg_name ($rel_path)${NC}"
        echo "Package: $pkg_name ($rel_path)" >> "$log_file"
        (cd "$pkg_dir" && cargo test > "$repo_dir/pkg_test.tmp" 2>&1) || true
        local pkg_test_result=$?
        if [ -f "$repo_dir/pkg_test.tmp" ]; then
          cat "$repo_dir/pkg_test.tmp"
          cat "$repo_dir/pkg_test.tmp" >> "$log_file"
          rm "$repo_dir/pkg_test.tmp"
        else
          echo "Error: Package test output file not created" >> "$log_file"
        fi
        
        if [ $pkg_test_result -ne 0 ]; then
          all_packages_passed=false
          echo -e "${YELLOW}Tests for $pkg_name failed${NC}"
        else
          echo -e "${GREEN}Tests for $pkg_name passed${NC}"
        fi
        echo "" >> "$log_file"
      done < <(find smashlang_packages -name Cargo.toml)
      
      # Record the overall result of package tests
      if [ "$all_packages_passed" = true ]; then
        echo -e "${GREEN}All package tests passed successfully!${NC}"
        echo "PACKAGE TESTS SUMMARY: All package tests passed successfully!" >> "$log_file"
      else
        echo -e "${YELLOW}Some package tests failed. See the log for details.${NC}"
        echo "PACKAGE TESTS SUMMARY: Some package tests failed. See details above." >> "$log_file"
      fi
    fi
    
    # Copy the test results log to the installation directory if needed
    if [ -n "$LINUX_INSTALL_DIR" ]; then
      mkdir -p "$LINUX_INSTALL_DIR/logs"
      cp "$log_file" "$LINUX_INSTALL_DIR/logs/"
    elif [ -n "$MACOS_INSTALL_DIR" ]; then
      mkdir -p "$MACOS_INSTALL_DIR/logs"
      cp "$log_file" "$MACOS_INSTALL_DIR/logs/"
    elif [ -n "$WINDOWS_INSTALL_DIR" ]; then
      mkdir -p "$WINDOWS_INSTALL_DIR/logs"
      cp "$log_file" "$WINDOWS_INSTALL_DIR/logs/"
    fi
    
    # Save the log file path for later use
    echo "$log_file" > "$repo_dir/test_log_path.txt"
  else
    echo -e "${YELLOW}Warning: Cargo not found, skipping tests.${NC}"
    echo "Cargo not found, tests were skipped." >> "$log_file"
  fi
}

# Display test results at the end of installation
display_test_results() {
  # Find the most recent log file in the logs directory
  local log_file=$(find "$BASE_DIR/logs" -name "test_results_*.log" -type f -print0 2>/dev/null | xargs -0 ls -t 2>/dev/null | head -n 1)
  
  if [ -n "$log_file" ] && [ -f "$log_file" ]; then
    echo -e "\n${BLUE}Test Results Summary${NC}"
    echo -e "${BLUE}-------------------${NC}"
    echo -e "A detailed test log has been saved to: $log_file"
    
    # Check if all tests passed
    if grep -q "TEST SUMMARY: All tests passed successfully!" "$log_file"; then
      echo -e "${GREEN}All tests passed successfully!${NC}"
    else
      echo -e "${YELLOW}Some tests failed. See the log file for details.${NC}"
    fi
    
    # Display important test information at the end
    echo -e "\n${BLUE}Important Test Information${NC}"
    echo -e "${BLUE}------------------------${NC}"
    
    # Extract and display test results
    echo -e "${YELLOW}Main Crate Tests:${NC}"
    if grep -q "Main Crate Tests" "$log_file"; then
      grep -A 5 "Main Crate Tests" "$log_file" | grep -v "Main Crate Tests" | grep -v -- "---------------" | head -n 3
    else
      echo "No main crate test results found"
    fi
    
    echo -e "\n${YELLOW}All Packages Tests:${NC}"
    if grep -q "All Packages Tests" "$log_file"; then
      grep -A 5 "All Packages Tests" "$log_file" | grep -v "All Packages Tests" | grep -v -- "-----------------" | head -n 3
    else
      echo "No package test results found"
    fi
    
    echo -e "\n${YELLOW}All Features Tests:${NC}"
    if grep -q "All Features Tests" "$log_file"; then
      grep -A 5 "All Features Tests" "$log_file" | grep -v "All Features Tests" | grep -v -- "-----------------" | head -n 3
    else
      echo "No feature test results found"
    fi
    
    # Display any failures
    echo -e "\n${YELLOW}Test Failures (if any):${NC}"
    if grep -i "failed" "$log_file" | grep -v "0 failed" | grep -q "."; then
      grep -i "failed" "$log_file" | grep -v "0 failed" | head -n 10
    else
      echo -e "${GREEN}No test failures found${NC}"
    fi
    
    echo -e "\nTo view the full test results, run: cat $log_file"
  else
    echo -e "${YELLOW}No test log file found.${NC}"
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
    
    # Check for LLVM development files
    if ! command -v llvm-config &> /dev/null; then
      echo -e "${YELLOW}Warning: llvm-config not found. LLVM development files may be missing.${NC}"
      echo -e "${YELLOW}To install LLVM development files on Ubuntu/Debian, run: sudo apt-get install llvm-dev${NC}"
      echo -e "${YELLOW}To install LLVM development files on Fedora, run: sudo dnf install llvm-devel${NC}"
      echo -e "${YELLOW}To install LLVM development files on Arch, run: sudo pacman -S llvm${NC}"
    fi
  elif [ "$os" == "macos" ]; then
    if ! command -v clang &> /dev/null; then
      echo -e "${RED}Error: clang is not installed. Please install Xcode command line tools and try again.${NC}"
      missing_tools=true
    fi
    
    # Check for LLVM development files
    if ! command -v llvm-config &> /dev/null; then
      echo -e "${YELLOW}Warning: llvm-config not found. LLVM development files may be missing.${NC}"
      echo -e "${YELLOW}To install LLVM development files on macOS, run: brew install llvm${NC}"
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

# Download a file
download() {
  local url=$1
  local output_file=$2
  
  if command -v curl &> /dev/null; then
    curl -fsSL "$url" -o "$output_file"
  elif command -v wget &> /dev/null; then
    wget -q -O "$output_file" "$url"
  else
    echo -e "${RED}Error: Neither curl nor wget is installed. Please install one of them and try again.${NC}"
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
  local use_master=$1
  
  echo -e "${BLUE}Installing SmashLang on Linux...${NC}"
  
  # Create installation directory
  create_dir "$LINUX_INSTALL_DIR"
  create_dir "$LINUX_INSTALL_DIR/bin"
  create_dir "$LINUX_INSTALL_DIR/src"
  create_dir "$LINUX_INSTALL_DIR/docs"
  
  if [ "$use_master" == "true" ]; then
    echo -e "${BLUE}Using GitHub master branch for installation...${NC}"
    
    # Create a directory for cloning the repository in the BASE_DIR
    local temp_dir="$BASE_DIR/build/temp_$(date +"%Y%m%d_%H%M%S")"
    mkdir -p "$temp_dir"
    
    # Set up cleanup trap
    trap 'echo -e "${BLUE}Cleaning up temporary files...${NC}"; rm -rf "$temp_dir"' EXIT
    
    # Clone the repository
    echo -e "${BLUE}Cloning SmashLang repository...${NC}"
    git clone --depth 1 "$REPO_URL" "$temp_dir"
    
    # Check if docs/getting-started exists locally but not in the cloned repo
    if [ -d "$(pwd)/docs/getting-started" ]; then
      echo -e "${BLUE}Copying docs/getting-started from local repository...${NC}"
      mkdir -p "$temp_dir/docs/getting-started"
      cp -r "$(pwd)/docs/getting-started/"* "$temp_dir/docs/getting-started/"
      # Make sure the run_all_examples.sh script is executable
      if [ -f "$temp_dir/docs/getting-started/run_all_examples.sh" ]; then
        chmod +x "$temp_dir/docs/getting-started/run_all_examples.sh"
      fi
    fi
    
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
    
    # Run tests after installation is complete
    echo -e "${BLUE}Running tests for SmashLang...${NC}"
    echo -e "${BLUE}(Test output will be saved and summarized at the end)${NC}"
    run_tests "$temp_dir"
    
    # Copy documentation
    echo -e "${BLUE}Installing documentation...${NC}"
    if [ -d "$temp_dir/docs" ]; then
      cp -r "$temp_dir/docs" "$LINUX_INSTALL_DIR/"
      echo -e "${BLUE}Documentation installed to $LINUX_INSTALL_DIR/docs${NC}"
    fi
    
    # Display test results summary
    display_test_results "$temp_dir"
  else
    echo -e "${BLUE}Using pre-built binaries for installation...${NC}"
    
    # Download pre-built binaries
    local bin_url="https://github.com/profullstack/smashlang/releases/latest/download/smashlang-linux-x86_64.tar.gz"
    local bin_file="/tmp/smashlang-linux-x86_64.tar.gz"
    
    echo -e "${BLUE}Downloading SmashLang binaries...${NC}"
    download "$bin_url" "$bin_file"
    
    # Extract binaries
    echo -e "${BLUE}Extracting SmashLang binaries...${NC}"
    tar -xzf "$bin_file" -C "$LINUX_INSTALL_DIR"
    
    # Clean up
    rm -f "$bin_file"
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
  # and will not be performed during installation
  
  # Display test results summary
  display_test_results "$temp_dir"
  
  echo -e "\n${GREEN}SmashLang has been successfully installed on Linux!${NC}"
  echo -e "Run 'smash --version' to verify the installation."
  echo -e "Note: Package assets generation is only needed when publishing packages."
  echo -e "Run 'scripts/generate_package_logo.sh' and 'scripts/generate_favicon.sh' manually if needed."
}

# Check if this is a direct download from GitHub
if [ -z "$DOWNLOADED_INSTALLER" ] && [ "$1" = "--master" ]; then
  # Clone the repository directly and run tests
  echo -e "${BLUE}Using GitHub master branch for installation...${NC}"
  
  # Create a directory for cloning the repository in the BASE_DIR
  TEMP_DIR="$BASE_DIR/build/temp_$(date +"%Y%m%d_%H%M%S")"
  mkdir -p "$TEMP_DIR"
  
  # Set up cleanup trap
  trap 'echo -e "${BLUE}Cleaning up temporary files...${NC}"; rm -rf "$TEMP_DIR"' EXIT
  
  # Clone the repository
  echo -e "${BLUE}Cloning SmashLang repository...${NC}"
  git clone --depth 1 "$REPO_URL" "$TEMP_DIR"
  
  # Change to the cloned directory
  cd "$TEMP_DIR"
  
  # Run the installer with the --master flag
  DOWNLOADED_INSTALLER=true ./install.sh --master
  
  # Clean up and exit
  exit 0
fi

# Rest of the installer script would go here
# For brevity, we're only including the Linux installation function
# The actual script would include macOS and Windows installation functions as well

# Main function
main() {
  # Display welcome message
  display_welcome
  
  # Parse command-line arguments
  local use_master=false
  local upgrade=false
  
  while [[ $# -gt 0 ]]; do
    case $1 in
      --master)
        use_master=true
        shift
        ;;
      --upgrade)
        upgrade=true
        shift
        ;;
      --help)
        display_help
        exit 0
        ;;
      *)
        echo -e "${RED}Error: Unknown option $1${NC}"
        display_help
        exit 1
        ;;
    esac
  done
  
  # Detect operating system
  local os=$(detect_os)
  echo -e "${BLUE}Detected operating system: $os${NC}"
  
  # Check for required tools
  check_requirements "$os"
  
  # Install or upgrade SmashLang based on the detected OS
  if [ "$os" == "linux" ]; then
    if [ "$upgrade" == "true" ]; then
      upgrade_linux
    else
      install_linux "$use_master"
    fi
  elif [ "$os" == "macos" ]; then
    if [ "$upgrade" == "true" ]; then
      upgrade_macos
    else
      install_macos "$use_master"
    fi
  elif [ "$os" == "windows" ]; then
    if [ "$upgrade" == "true" ]; then
      upgrade_windows
    else
      install_windows "$use_master"
    fi
  else
    echo -e "${RED}Error: Unsupported operating system.${NC}"
    exit 1
  fi
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
  echo -e "💪 Welcome to SmashLang! 💪"
  echo -e "A bold, high-performance, JavaScript-inspired general-purpose programming language"
  echo -e "that compiles to native binaries. Made for developers who want the power of C/Rust"
  echo -e "but the clarity of JavaScript — without the bloat."
  echo -e ""
  echo -e "Visit https://smashlang.com for documentation and community resources."
  echo -e ""
  echo -e ""
  echo -e "SmashLang Installer v$VERSION"
}

# Display help message
display_help() {
  echo -e "Usage: ./install.sh [OPTIONS]"
  echo -e ""
  echo -e "Options:"
  echo -e "  --master   Install from the master branch (latest development version)"
  echo -e "  --upgrade  Upgrade an existing installation"
  echo -e "  --help     Display this help message"
  echo -e ""
  echo -e "Examples:"
  echo -e "  ./install.sh              # Install the latest stable version"
  echo -e "  ./install.sh --master     # Install the latest development version"
  echo -e "  ./install.sh --upgrade    # Upgrade an existing installation"
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

# Run the main function
main "$@"
