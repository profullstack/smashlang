#!/bin/bash

# Colors for output
GREEN="\033[0;32m"
NC="\033[0m" # No Color

# Create the documentation directory
mkdir -p "$HOME/.local/share/smashlang"

# Copy the documentation
cp -r "/home/ettinger/src/profullstack.com/smashlang/docs" "$HOME/.local/share/smashlang/"

echo -e "${GREEN}Documentation copied to $HOME/.local/share/smashlang/docs${NC}"

# Verify the copy
ls -la "$HOME/.local/share/smashlang/docs"

echo -e "${GREEN}Now try running 'smash docs' again${NC}"
