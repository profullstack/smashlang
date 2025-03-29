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

# Default values for command line arguments
DEFAULT_COMMAND="install"
DEFAULT_USE_MASTER=false

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
    
    # Copy binaries from the repository or build them
    echo -e "${BLUE}Installing SmashLang binaries...${NC}"
    if [ -f "$temp_dir/bin/smash" ]; then
      # Use pre-built binary
      cp "$temp_dir/bin/smash" "$LINUX_INSTALL_DIR/"
      chmod +x "$LINUX_INSTALL_DIR/smash"
    elif [ -f "$temp_dir/Cargo.toml" ] && command -v cargo &> /dev/null; then
      # Build from source
      echo -e "${BLUE}Building SmashLang from source...${NC}"
      cd "$temp_dir"
      
      # Capture git hash for version info
      GIT_HASH=$(git rev-parse --short HEAD)
      echo "$GIT_HASH" > "$temp_dir/src/git_hash.txt"
      echo -e "${BLUE}Captured git hash: $GIT_HASH${NC}"
      
      # Make sure the git hash file gets copied to the installation directory
      mkdir -p "$LINUX_INSTALL_DIR/src"
      cp "$temp_dir/src/git_hash.txt" "$LINUX_INSTALL_DIR/git_hash.txt"
      cp "$temp_dir/src/git_hash.txt" "$LINUX_INSTALL_DIR/src/git_hash.txt"
      
      cargo build --release
      
      if [ -f "$temp_dir/target/release/smash" ]; then
        cp "$temp_dir/target/release/smash" "$LINUX_INSTALL_DIR/"
        chmod +x "$LINUX_INSTALL_DIR/smash"
        
        # Copy documentation directory
        # Always try to copy to user's home directory first as it doesn't require root permissions
        mkdir -p "$HOME/.local/share/smashlang"
        if [ -d "$temp_dir/docs" ]; then
          echo -e "${BLUE}Installing documentation...${NC}"
          cp -r "$temp_dir/docs" "$HOME/.local/share/smashlang/"
          echo -e "${GREEN}Documentation installed to $HOME/.local/share/smashlang/docs${NC}"
          
          # Also try system-wide installation if we have permissions
          if [ -w "/usr/local/share" ]; then
            mkdir -p "/usr/local/share/smashlang"
            cp -r "$temp_dir/docs" "/usr/local/share/smashlang/"
            echo -e "${GREEN}Documentation also installed to /usr/local/share/smashlang/docs${NC}"
          fi
        else
          echo -e "${YELLOW}Warning: Documentation directory not found in the repository${NC}"
        fi
        echo -e "${GREEN}Successfully built and installed SmashLang binary.${NC}"
        
        # Check for smashc compiler binary
        if [ -f "$temp_dir/target/release/smashc" ]; then
          cp "$temp_dir/target/release/smashc" "$LINUX_INSTALL_DIR/"
          chmod +x "$LINUX_INSTALL_DIR/smashc"
          echo -e "${GREEN}Successfully installed smashc compiler binary.${NC}"
        else
          echo -e "${YELLOW}Warning: smashc compiler binary not found, creating placeholder...${NC}"
          cat > "$LINUX_INSTALL_DIR/smashc" << 'EOF'
#!/bin/bash

# Colors for output
RED="\033[0;31m"
GREEN="\033[0;32m"
YELLOW="\033[0;33m"
BLUE="\033[0;34m"
CYAN="\033[0;36m"
NC="\033[0m" # No Color

# Check for command line arguments
if [[ "$1" == "--help" || "$1" == "-h" ]]; then
  echo -e "${BLUE}SmashLang Compiler v0.1.0-dev${NC} - The SmashLang Compiler"
  echo ""
  echo -e "${YELLOW}Usage:${NC}"
  echo -e "  smashc [options] <file.smash>"
  echo ""
  echo -e "${YELLOW}Options:${NC}"
  echo -e "  -h, --help                 Show this help message"
  echo -e "  -v, --version              Show version information"
  echo -e "  -o, --output <file>        Specify output file name"
  echo -e "  --target <platform>        Target platform (linux, macos, windows)"
  echo -e "  --wasm                     Compile to WebAssembly"
  echo -e "  --debug                    Include debug information"
  echo -e "  --release                  Optimize for release"
  echo -e "  --lib                      Compile as a library"
  echo ""
  echo -e "${YELLOW}Examples:${NC}"
  echo -e "  smashc hello.smash                   Compile hello.smash to default output"
  echo -e "  smashc hello.smash -o hello          Specify output filename"
  echo -e "  smashc hello.smash --target windows  Cross-compile for Windows"
  echo -e "  smashc hello.smash --wasm            Compile to WebAssembly"
  echo ""
  echo -e "${YELLOW}Documentation:${NC}"
  echo -e "  Visit ${CYAN}https://smashlang.com/docs/compiler${NC} for full documentation"
  exit 0
fi

if [[ "$1" == "--version" || "$1" == "-v" ]]; then
  echo -e "${BLUE}SmashLang Compiler v0.1.0-dev${NC}"
  exit 0
fi

if [[ -n "$1" && "$1" == *.smash ]]; then
  echo -e "${BLUE}SmashLang Compiler v0.1.0-dev${NC} (placeholder)"
  echo -e "Would compile file: $1 (not yet implemented)"
  echo -e "This is a placeholder. The actual compiler is not yet implemented."
else
  echo -e "${RED}Error:${NC} No input file specified or file is not a .smash file"
  echo -e "Run ${CYAN}smashc --help${NC} for usage information"
  exit 1
fi
EOF
          chmod +x "$LINUX_INSTALL_DIR/smashc"
        fi
      else
        echo -e "${YELLOW}Warning: Failed to build smash binary, creating placeholder...${NC}"
        cat > "$LINUX_INSTALL_DIR/smash" << 'EOF'
#!/bin/bash

# Colors for output
RED="\033[0;31m"
GREEN="\033[0;32m"
YELLOW="\033[0;33m"
BLUE="\033[0;34m"
CYAN="\033[0;36m"
NC="\033[0m" # No Color

# Check for command line arguments
if [[ "$1" == "--help" || "$1" == "-h" ]]; then
  echo -e "${BLUE}SmashLang v0.1.0-dev${NC} - A JavaScript-inspired programming language"
  echo ""
  echo -e "${YELLOW}Usage:${NC}"
  echo -e "  smash [options] <file.smash>    Run a SmashLang program"
  echo -e "  smash                          Start interactive REPL mode"
  echo ""
  echo -e "${YELLOW}Options:${NC}"
  echo -e "  -h, --help                     Show this help message"
  echo -e "  -v, --version                  Show version information"
  echo -e "  -c, --compile <file.smash>     Compile a SmashLang program to binary"
  echo -e "  -o, --output <file>            Specify output file for compilation"
  echo -e "  --wasm                         Compile to WebAssembly (see docs/wasm_support.md)"
  echo -e "  --target <platform>            Specify target platform (linux, macos, windows)"
  echo -e "  --debug                        Enable debug mode"
  echo ""
  echo -e "${YELLOW}Examples:${NC}"
  echo -e "  smash                           Start interactive REPL"
  echo -e "  smash hello.smash               Run a SmashLang program"
  echo -e "  smash -c hello.smash -o hello  Compile a program to binary"
  echo -e "  smash --wasm hello.smash       Compile a program to WebAssembly"
  echo ""
  echo -e "${YELLOW}Documentation:${NC}"
  echo -e "  Visit ${CYAN}https://smashlang.com/docs${NC} for full documentation"
  exit 0
elif [[ "$1" == "--version" || "$1" == "-v" ]]; then
  echo -e "${BLUE}SmashLang v0.1.0-dev${NC}"
  exit 0
elif [[ "$1" == "repl" || -z "$1" ]]; then
  echo -e "${YELLOW}SmashLang REPL v0.1.0-dev${NC}"
  echo -e "${BLUE}Type .help for available commands or .exit to quit${NC}"
  echo -e "${YELLOW}> ${NC}This is a placeholder. The actual REPL is not yet implemented."
  exit 0
else
  if [[ -n "$1" && "$1" == *.smash ]]; then
    echo -e "${YELLOW}SmashLang v0.1.0-dev (placeholder)${NC}"
    echo -e "Would run file: $1 (not yet implemented)"
  else
    echo -e "${YELLOW}SmashLang v0.1.0-dev (placeholder)${NC}"
    echo -e "Unknown command or file: $1"
    echo -e "Run ${CYAN}smash --help${NC} for usage information"
  fi
fi
EOF
        chmod +x "$LINUX_INSTALL_DIR/smash"
      fi
    else
      # Create placeholder if no binary and can't build
      echo -e "${YELLOW}Warning: smash binary not found, creating placeholder...${NC}"
      cat > "$LINUX_INSTALL_DIR/smash" << 'EOF'
#!/bin/bash

# Colors for output
RED="\033[0;31m"
GREEN="\033[0;32m"
YELLOW="\033[0;33m"
BLUE="\033[0;34m"
CYAN="\033[0;36m"
NC="\033[0m" # No Color

# Check for command line arguments
if [[ "$1" == "--help" || "$1" == "-h" ]]; then
  echo -e "${BLUE}SmashLang v0.1.0-dev${NC} - A JavaScript-inspired programming language"
  echo ""
  echo -e "${YELLOW}Usage:${NC}"
  echo -e "  smash [options] <file.smash>    Run a SmashLang program"
  echo -e "  smash                          Start interactive REPL mode"
  echo ""
  echo -e "${YELLOW}Options:${NC}"
  echo -e "  -h, --help                     Show this help message"
  echo -e "  -v, --version                  Show version information"
  echo -e "  -c, --compile <file.smash>     Compile a SmashLang program to binary"
  echo -e "  -o, --output <file>            Specify output file for compilation"
  echo -e "  --wasm                         Compile to WebAssembly (see docs/wasm_support.md)"
  echo -e "  --target <platform>            Specify target platform (linux, macos, windows)"
  echo -e "  --debug                        Enable debug mode"
  echo ""
  echo -e "${YELLOW}Examples:${NC}"
  echo -e "  smash                           Start interactive REPL"
  echo -e "  smash hello.smash               Run a SmashLang program"
  echo -e "  smash -c hello.smash -o hello  Compile a program to binary"
  echo -e "  smash --wasm hello.smash       Compile a program to WebAssembly"
  echo ""
  echo -e "${YELLOW}Documentation:${NC}"
  echo -e "  Visit ${CYAN}https://smashlang.com/docs${NC} for full documentation"
  exit 0
elif [[ "$1" == "--version" || "$1" == "-v" ]]; then
  echo -e "${BLUE}SmashLang v0.1.0-dev${NC}"
  exit 0
elif [[ "$1" == "repl" || -z "$1" ]]; then
  echo -e "${YELLOW}SmashLang REPL v0.1.0-dev${NC}"
  echo -e "${BLUE}Type .help for available commands or .exit to quit${NC}"
  echo -e "${YELLOW}> ${NC}This is a placeholder. The actual REPL is not yet implemented."
  exit 0
else
  if [[ -n "$1" && "$1" == *.smash ]]; then
    echo -e "${YELLOW}SmashLang v0.1.0-dev (placeholder)${NC}"
    echo -e "Would run file: $1 (not yet implemented)"
  else
    echo -e "${YELLOW}SmashLang v0.1.0-dev (placeholder)${NC}"
    echo -e "Unknown command or file: $1"
    echo -e "Run ${CYAN}smash --help${NC} for usage information"
  fi
fi
EOF
      chmod +x "$LINUX_INSTALL_DIR/smash"
    fi
    
    # Install smashpkg binary or create placeholder
    if [ -f "$temp_dir/bin/smashpkg" ]; then
      # Use pre-built binary
      cp "$temp_dir/bin/smashpkg" "$LINUX_INSTALL_DIR/"
      chmod +x "$LINUX_INSTALL_DIR/smashpkg"
    elif [ -f "$temp_dir/target/release/smashpkg" ]; then
      # Use binary built from source
      cp "$temp_dir/target/release/smashpkg" "$LINUX_INSTALL_DIR/"
      chmod +x "$LINUX_INSTALL_DIR/smashpkg"
      echo -e "${GREEN}Successfully installed SmashLang package manager binary.${NC}"
    else
      # Create placeholder
      echo -e "${YELLOW}Warning: smashpkg binary not found, creating placeholder...${NC}"
      cat > "$LINUX_INSTALL_DIR/smashpkg" << 'EOF'
#!/bin/bash

# Colors for output
RED="\033[0;31m"
GREEN="\033[0;32m"
YELLOW="\033[0;33m"
BLUE="\033[0;34m"
CYAN="\033[0;36m"
NC="\033[0m" # No Color

# Check for command line arguments
if [[ "$1" == "--help" || "$1" == "-h" ]]; then
  echo -e "${BLUE}SmashLang Package Manager v0.1.0-dev${NC}"
  echo ""
  echo -e "${YELLOW}Usage:${NC}"
  echo -e "  smashpkg [command] [options]"
  echo ""
  echo -e "${YELLOW}Commands:${NC}"
  echo -e "  install <package>       Install a package"
  echo -e "  remove <package>        Remove a package"
  echo -e "  update <package>        Update a package"
  echo -e "  list                   List installed packages"
  echo -e "  search <query>          Search for packages"
  echo -e "  info <package>          Show package information"
  echo ""
  echo -e "${YELLOW}Options:${NC}"
  echo -e "  -h, --help              Show this help message"
  echo -e "  -v, --version           Show version information"
  echo -e "  -g, --global            Install/remove packages globally"
  echo ""
  echo -e "${YELLOW}Examples:${NC}"
  echo -e "  smashpkg install networking/hono    Install the Hono package"
  echo -e "  smashpkg list                      List installed packages"
  echo -e "  smashpkg search http               Search for HTTP-related packages"
  echo ""
  echo -e "${YELLOW}Documentation:${NC}"
  echo -e "  Visit ${CYAN}https://smashlang.com/docs/packages${NC} for full documentation"
  exit 0
elif [[ "$1" == "--version" || "$1" == "-v" ]]; then
  echo -e "${BLUE}SmashLang Package Manager v0.1.0-dev${NC}"
  exit 0
else
  echo -e "${YELLOW}SmashLang Package Manager v0.1.0-dev (placeholder)${NC}"
  if [[ -n "$1" ]]; then
    echo -e "Command: $1 (not yet implemented)"
  fi
  echo -e "Run ${CYAN}smashpkg --help${NC} for usage information"
fi
EOF
      chmod +x "$LINUX_INSTALL_DIR/smashpkg"
    fi
    
    # Copy packages from the repository
    echo -e "${BLUE}Installing SmashLang packages...${NC}"
    if [ -d "$temp_dir/smashlang_packages" ]; then
      cp -r "$temp_dir/smashlang_packages"/* "$LINUX_PACKAGES_DIR/"
    else
      echo -e "${YELLOW}Warning: smashlang_packages directory not found in repository${NC}"
    fi
    
    # Copy assets directory if it exists
    if [ -d "$temp_dir/assets" ]; then
      echo -e "${BLUE}Copying assets directory...${NC}"
      mkdir -p "$HOME/.local/share/smashlang/assets"
      cp -r "$temp_dir/assets"/* "$HOME/.local/share/smashlang/assets/"
      
      # Create logo.txt and favicon.txt if they don't exist
      if [ -f "$temp_dir/assets/logo.ascii" ] && [ ! -f "$HOME/.local/share/smashlang/assets/logo.txt" ]; then
        cp "$temp_dir/assets/logo.ascii" "$HOME/.local/share/smashlang/assets/logo.txt"
      fi
      
      if [ -f "$temp_dir/assets/favicon.ascii" ] && [ ! -f "$HOME/.local/share/smashlang/assets/favicon.txt" ]; then
        cp "$temp_dir/assets/favicon.ascii" "$HOME/.local/share/smashlang/assets/favicon.txt"
      fi
    else
      echo -e "${YELLOW}Warning: assets directory not found in repository${NC}"
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
    # Detect current shell
    CURRENT_SHELL=$(basename "$SHELL")
    RC_FILE=""
    
    case "$CURRENT_SHELL" in
      bash)
        RC_FILE="$HOME/.bashrc"
        ;;
      zsh)
        RC_FILE="$HOME/.zshrc"
        ;;
      fish)
        RC_FILE="$HOME/.config/fish/config.fish"
        FISH_PATH_EXPORT="set -gx PATH $LINUX_INSTALL_DIR \$PATH"
        ;;
      *)
        # Default to bashrc if shell is unknown
        RC_FILE="$HOME/.bashrc"
        ;;
    esac
    
    # Check if PATH entry already exists in the rc file
    if [ -f "$RC_FILE" ]; then
      if ! grep -q "$LINUX_INSTALL_DIR" "$RC_FILE"; then
        echo -e "${YELLOW}Adding SmashLang to PATH in $RC_FILE...${NC}"
        if [ "$CURRENT_SHELL" = "fish" ]; then
          echo "$FISH_PATH_EXPORT" >> "$RC_FILE"
        else
          echo "export PATH=\"$LINUX_INSTALL_DIR:\$PATH\"" >> "$RC_FILE"
        fi
        echo -e "${YELLOW}Please run 'source $RC_FILE' or start a new terminal to use SmashLang.${NC}"
      fi
    else
      echo -e "${YELLOW}Creating $RC_FILE and adding SmashLang to PATH...${NC}"
      mkdir -p "$(dirname "$RC_FILE")"
      if [ "$CURRENT_SHELL" = "fish" ]; then
        echo "$FISH_PATH_EXPORT" > "$RC_FILE"
      else
        echo "export PATH=\"$LINUX_INSTALL_DIR:\$PATH\"" > "$RC_FILE"
      fi
      echo -e "${YELLOW}Please run 'source $RC_FILE' or start a new terminal to use SmashLang.${NC}"
    fi
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
      cat > "$MACOS_INSTALL_DIR/smash" << 'EOF'
#!/bin/bash
echo "SmashLang v0.1.0-dev (placeholder)"
EOF
      chmod +x "$MACOS_INSTALL_DIR/smash"
    fi
    
    if [ -f "$temp_dir/bin/smashpkg" ]; then
      cp "$temp_dir/bin/smashpkg" "$MACOS_INSTALL_DIR/"
      chmod +x "$MACOS_INSTALL_DIR/smashpkg"
    else
      echo -e "${YELLOW}Warning: smashpkg binary not found in repository, creating placeholder...${NC}"
      cat > "$MACOS_INSTALL_DIR/smashpkg" << 'EOF'
#!/bin/bash
echo "SmashLang Package Manager v0.1.0-dev (placeholder)"
EOF
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
      cat > "$WINDOWS_INSTALL_DIR/smash.bat" << 'EOF'
@echo off
echo SmashLang v0.1.0-dev (placeholder)
EOF
    fi
    
    if [ -f "$temp_dir/bin/smashpkg.exe" ]; then
      cp "$temp_dir/bin/smashpkg.exe" "$WINDOWS_INSTALL_DIR/"
    else
      echo -e "${YELLOW}Warning: smashpkg.exe binary not found in repository, creating placeholder...${NC}"
      cat > "$WINDOWS_INSTALL_DIR/smashpkg.bat" << 'EOF'
@echo off
echo SmashLang Package Manager v0.1.0-dev (placeholder)
EOF
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
  USE_MASTER=false

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
      --master)
        USE_MASTER=true
        shift
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
  
  # Debug output for command-line arguments
  if [ "$USE_MASTER" = true ]; then
    echo -e "${YELLOW}Debug: Using GitHub master branch for installation${NC}"
  else
    echo -e "${YELLOW}Debug: Using release packages for installation${NC}"
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
    
    # Skip package assets generation for regular installation
    # This is only needed when preparing packages for publishing
    if [ "$GENERATE_LOGOS" = true ]; then
      echo -e "${BLUE}Note: Package assets generation is only needed when publishing packages.${NC}"
      echo -e "${BLUE}Run 'scripts/generate_package_logo.sh' and 'scripts/generate_favicon.sh' manually if needed.${NC}"
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
  echo "  --master        Use GitHub master branch instead of releases"
  echo "  --no-logos      Skip generation of package asset files (logo.txt, favicon.txt)"
  echo "  --help, -h      Show this help message"
}

# Generate logo.txt and favicon.txt files for packages
generate_package_assets() {
  local script_dir=$(get_script_dir)
  
  # Try to find assets in multiple locations
  local assets_dir=""
  
  # Check possible asset locations
  for dir in "$script_dir/assets" "$script_dir/../assets" "$(pwd)/assets" "$HOME/.local/share/smashlang/assets"; do
    if [ -d "$dir" ]; then
      assets_dir="$dir"
      break
    fi
  done
  
  if [ -z "$assets_dir" ]; then
    echo -e "${YELLOW}Warning: Could not find assets directory${NC}"
    return
  fi
  
  echo -e "${BLUE}Found assets directory at: $assets_dir${NC}"
  
  # Use pre-generated logo.txt and favicon.txt files
  if [ -f "$assets_dir/logo.txt" ]; then
    echo -e "${BLUE}Using pre-generated logo.txt file...${NC}"
    
    # Create package template directory if it doesn't exist
    local template_dir="$script_dir/smashlang_packages/__package__template/assets"
    mkdir -p "$template_dir"
    
    # Copy logo.txt to template directory
    cp "$assets_dir/logo.txt" "$template_dir/logo.txt"
    
    echo -e "${GREEN}Package logo.txt file copied successfully.${NC}"
  elif [ -f "$script_dir/scripts/generate_package_logo.sh" ]; then
    echo -e "${BLUE}Generating logo.txt files for packages...${NC}"
    
    # Make sure the script is executable
    chmod +x "$script_dir/scripts/generate_package_logo.sh"
    
    # Update the package template logo.txt
    "$script_dir/scripts/generate_package_logo.sh" --template
    
    echo -e "${GREEN}Package logo.txt files generated successfully.${NC}"
  else
    echo -e "${YELLOW}Warning: Neither pre-generated logo.txt nor generator script found.${NC}"
  fi
  
  # Use pre-generated favicon.txt file
  if [ -f "$assets_dir/favicon.txt" ]; then
    echo -e "${BLUE}Using pre-generated favicon.txt file...${NC}"
    
    # Create package template directory if it doesn't exist
    local template_dir="$script_dir/smashlang_packages/__package__template/assets"
    mkdir -p "$template_dir"
    
    # Copy favicon.txt to template directory
    cp "$assets_dir/favicon.txt" "$template_dir/favicon.txt"
    
    echo -e "${GREEN}Package favicon.txt file copied successfully.${NC}"
  elif [ -f "$script_dir/scripts/generate_favicon.sh" ]; then
    echo -e "${BLUE}Generating favicon.txt files for packages...${NC}"
    
    # Make sure the script is executable
    chmod +x "$script_dir/scripts/generate_favicon.sh"
    
    # Update the package template favicon.txt
    "$script_dir/scripts/generate_favicon.sh" --template
    
    echo -e "${GREEN}Package favicon.txt files generated successfully.${NC}"
  else
    echo -e "${YELLOW}Warning: Neither pre-generated favicon.txt nor generator script found.${NC}"
  fi
}

# Check if this is a direct download from GitHub
if [ -z "$DOWNLOADED_INSTALLER" ] && [ "$1" = "--master" ]; then
  # This is a direct download and we want to use master branch
  # Download the script to a temporary file and run it with the proper arguments
  TEMP_SCRIPT="/tmp/smashlang_installer_$$.sh"
  echo "Downloading installer script to temporary file..."
  if command -v curl &> /dev/null; then
    curl -fsSL "https://raw.githubusercontent.com/profullstack/smashlang/master/install.sh" > "$TEMP_SCRIPT"
  else
    wget -q -O "$TEMP_SCRIPT" "https://raw.githubusercontent.com/profullstack/smashlang/master/install.sh"
  fi
  chmod +x "$TEMP_SCRIPT"
  
  # Run the downloaded script with the --master flag and mark it as downloaded
  DOWNLOADED_INSTALLER=true "$TEMP_SCRIPT" --master
  
  # Clean up
  rm -f "$TEMP_SCRIPT"
  exit 0
fi

# Run the main function
main "$@"
