#!/bin/bash

# Script to move logo and favicon files from package roots to assets folders

set -e

# Define colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Base directories
BASE_DIR="$(pwd)"
PACKAGES_DIR="${BASE_DIR}/smashlang_packages"

# Function to move assets to the assets folder
move_assets() {
    local package_dir="$1"
    local package_name="$(basename "$package_dir")"
    
    echo -e "${BLUE}Processing package: $package_name${NC}"
    
    # Ensure assets directory exists
    if [ ! -d "$package_dir/assets" ]; then
        echo -e "  ${YELLOW}Creating assets directory${NC}"
        mkdir -p "$package_dir/assets"
    fi
    
    # Move logo and favicon files to assets folder
    moved_files=0
    
    # Find all logo and favicon files in the package root
    for asset_type in "logo" "favicon"; do
        for variant in "" ".dark" ".light"; do
            for ext in ".txt" ".svg" ".png"; do
                asset_file="${asset_type}${variant}${ext}"
                if [ -f "$package_dir/$asset_file" ]; then
                    echo -e "  ${YELLOW}Moving $asset_file to assets folder${NC}"
                    mv "$package_dir/$asset_file" "$package_dir/assets/"
                    moved_files=$((moved_files + 1))
                fi
            done
        done
    done
    
    if [ $moved_files -eq 0 ]; then
        echo -e "  ${GREEN}No assets to move${NC}"
    else
        echo -e "  ${GREEN}Moved $moved_files asset files to assets folder${NC}"
    fi
    
    echo ""
}

# Process all packages, including the template
echo -e "${BLUE}Starting asset reorganization...${NC}"
echo ""

# First, process top-level packages (including the template)
for package_dir in "$PACKAGES_DIR"/*/; do
    # Skip README.md and other files
    if [ -d "$package_dir" ]; then
        move_assets "$package_dir"
    fi
done

# Then, process subpackages (packages within packages)
for package_dir in "$PACKAGES_DIR"/*/*/; do
    # Skip if not a directory or if it's the assets directory itself
    if [ -d "$package_dir" ] && [ "$(basename "$package_dir")" != "assets" ]; then
        # Skip standard directories like examples, src, tests
        package_name="$(basename "$package_dir")"
        if [ "$package_name" != "examples" ] && [ "$package_name" != "src" ] && [ "$package_name" != "tests" ]; then
            move_assets "$package_dir"
        fi
    fi
done

echo -e "${GREEN}All assets have been moved to their respective assets folders!${NC}"

# Update package_config.json files to reference the new asset locations
echo -e "${BLUE}Updating package_config.json files to reference new asset locations...${NC}"

find "$PACKAGES_DIR" -name "package_config.json" -type f | while read -r config_file; do
    # Check if the file contains references to logo or favicon files without assets/ prefix
    if grep -q '"logo\|"favicon' "$config_file" && ! grep -q '"assets/' "$config_file"; then
        echo -e "${YELLOW}Updating asset paths in: $(basename $(dirname "$config_file"))/package_config.json${NC}"
        # Update paths to include assets/ prefix
        sed -i 's/"\(logo[^"]*\)"/"assets\/\1"/g' "$config_file"
        sed -i 's/"\(favicon[^"]*\)"/"assets\/\1"/g' "$config_file"
    fi
done

echo -e "${GREEN}Asset reorganization complete!${NC}"
