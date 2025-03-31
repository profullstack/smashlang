#!/bin/bash

# Create a temporary directory
TEMP_DIR=$(mktemp -d)
echo "Created temporary directory: $TEMP_DIR"

# Copy the installer script to the temporary directory
cp /home/ettinger/src/profullstack.com/smashlang/install.sh "$TEMP_DIR/"

# Set the DOWNLOADED_INSTALLER flag to true
export DOWNLOADED_INSTALLER=true

# Run the installer with the --master flag in debug mode
cd "$TEMP_DIR"
bash -x ./install.sh --master 2>&1 | grep -A 2 -B 2 "generate_package_assets"

# Clean up
rm -rf "$TEMP_DIR"
