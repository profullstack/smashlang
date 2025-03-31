#!/bin/bash

# Build script for SmashLang
# This script builds SmashLang from source

# Set color variables
RED="\033[0;31m"
GREEN="\033[0;32m"
YELLOW="\033[0;33m"
BLUE="\033[0;34m"
NC="\033[0m" # No Color

# Build SmashLang from source
build_smashlang() {
  local repo_dir="$1"
  local build_type="$2"
  
  echo -e "${BLUE}Building SmashLang from source...${NC}"
  cd "$repo_dir"
  
  # Capture git hash for version info if in a git repository
  if [ -d ".git" ]; then
    local GIT_HASH=$(git rev-parse --short HEAD)
    echo "$GIT_HASH" > "src/git_hash.txt"
  fi
  
  # Build based on build type
  if [ "$build_type" == "release" ]; then
    echo -e "${BLUE}Building release version...${NC}"
    cargo build --release
    if [ $? -ne 0 ]; then
      echo -e "${RED}Error: Failed to build SmashLang.${NC}"
      return 1
    fi
  else
    echo -e "${BLUE}Building debug version...${NC}"
    cargo build
    if [ $? -ne 0 ]; then
      echo -e "${RED}Error: Failed to build SmashLang.${NC}"
      return 1
    fi
  fi
  
  echo -e "${GREEN}SmashLang built successfully!${NC}"
  return 0
}

# If this script is run directly, execute the build function
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
  # Check if the repository directory is provided
  if [ -z "$1" ]; then
    echo -e "${RED}Error: Repository directory not provided.${NC}"
    echo -e "Usage: $0 <repository_directory> [release|debug]"
    exit 1
  fi
  
  # Check if the build type is provided
  build_type="release"
  if [ ! -z "$2" ]; then
    build_type="$2"
  fi
  
  # Build SmashLang
  build_smashlang "$1" "$build_type"
  exit $?
fi
