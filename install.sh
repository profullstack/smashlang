#!/bin/bash

# SmashLang Installer Script
# This script installs SmashLang on Windows, macOS, and Linux systems

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
    
    # Copy the test results log to the installation directory
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
  if [ -f "$1/test_log_path.txt" ]; then
    local log_file=$(cat "$1/test_log_path.txt")
    if [ -f "$log_file" ]; then
      echo -e "\n${BLUE}Test Results Summary${NC}"
      echo -e "${BLUE}-------------------${NC}"
      echo -e "A detailed test log has been saved to: $log_file"
      if grep -q "TEST SUMMARY: All tests passed successfully!" "$log_file"; then
        echo -e "${GREEN}All tests passed successfully!${NC}"
      else
        echo -e "${YELLOW}Some tests failed. See the log file for details.${NC}"
      fi
    fi
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
  
  # Generate package assets if needed
  generate_package_assets
  
  echo -e "${GREEN}SmashLang has been successfully installed on Linux!${NC}"
  echo -e "Run 'smash --version' to verify the installation."
  echo -e "Note: Package assets generation is only needed when publishing packages."
  echo -e "Run 'scripts/generate_package_logo.sh' and 'scripts/generate_favicon.sh' manually if needed."
}

# Check if this is a direct download from GitHub
if [ -z "$DOWNLOADED_INSTALLER" ] && [ "$1" = "--master" ]; then
  # This is a direct download and we want to use master branch
  # Create a temporary script with our functions and then download the installer
  TEMP_SCRIPT="/tmp/smashlang_installer_$$.sh"
  
  # First, create the temporary script with our run_tests function
  cat > "$TEMP_SCRIPT" << 'EOFSCRIPT'
#!/bin/bash

# SmashLang Installer Script
# This script installs SmashLang on Windows, macOS, and Linux systems

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
    
    # Save the log file path for later use
    echo "$log_file" > "$repo_dir/test_log_path.txt"
  else
    echo -e "${YELLOW}Warning: Cargo not found, skipping tests.${NC}"
    echo "Cargo not found, tests were skipped." >> "$log_file"
  fi
}

# Display test results at the end of installation
display_test_results() {
  if [ -f "$1/test_log_path.txt" ]; then
    local log_file=$(cat "$1/test_log_path.txt")
    if [ -f "$log_file" ]; then
      echo -e "\n${BLUE}Test Results Summary${NC}"
      echo -e "${BLUE}-------------------${NC}"
      echo -e "A detailed test log has been saved to: $log_file"
      if grep -q "TEST SUMMARY: All tests passed successfully!" "$log_file"; then
        echo -e "${GREEN}All tests passed successfully!${NC}"
      else
        echo -e "${YELLOW}Some tests failed. See the log file for details.${NC}"
      fi
    fi
  fi
}
EOFSCRIPT

  # Now append the installer script from GitHub
  echo "Downloading installer script to temporary file..."
  if command -v curl &> /dev/null; then
    curl -fsSL "https://raw.githubusercontent.com/profullstack/smashlang/master/install.sh" >> "$TEMP_SCRIPT"
  else
    wget -q -O - "https://raw.githubusercontent.com/profullstack/smashlang/master/install.sh" >> "$TEMP_SCRIPT"
  fi
  chmod +x "$TEMP_SCRIPT"
  
  # Run the combined script with the --master flag and mark it as downloaded
  DOWNLOADED_INSTALLER=true "$TEMP_SCRIPT" --master
  
  # Clean up
  rm -f "$TEMP_SCRIPT"
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
