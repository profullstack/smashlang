#!/bin/bash

# Script to standardize SmashLang packages based on the __package__template

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
TEMPLATE_DIR="${PACKAGES_DIR}/__package__template"

# Check if the template directory exists
if [ ! -d "$TEMPLATE_DIR" ]; then
    echo -e "${RED}Error: Template directory not found at $TEMPLATE_DIR${NC}"
    exit 1
fi

# Get the list of standard directories from the template
TEMPLATE_DIRS=("assets" "examples" "src" "tests")

# Get the list of standard files from the template (excluding directories)
TEMPLATE_FILES=("package_config.json" "README.md" "GUIDE.md")

# Function to standardize a package
standardize_package() {
    local package_dir="$1"
    local package_name="$(basename "$package_dir")"
    
    # Skip the template itself
    if [ "$package_name" == "__package__template" ]; then
        return
    fi
    
    echo -e "${BLUE}Standardizing package: $package_name${NC}"
    
    # Create standard directories if they don't exist
    for dir in "${TEMPLATE_DIRS[@]}"; do
        if [ ! -d "$package_dir/$dir" ]; then
            echo -e "  ${YELLOW}Creating directory: $dir${NC}"
            mkdir -p "$package_dir/$dir"
            # Add a placeholder README.md in each directory
            echo "# $package_name - $dir" > "$package_dir/$dir/README.md"
        else
            echo -e "  ${GREEN}Directory already exists: $dir${NC}"
        fi
    done
    
    # Copy standard files if they don't exist
    for file in "${TEMPLATE_FILES[@]}"; do
        if [ ! -f "$package_dir/$file" ]; then
            echo -e "  ${YELLOW}Adding file: $file${NC}"
            cp "$TEMPLATE_DIR/$file" "$package_dir/$file"
            # Replace template placeholders with package name
            sed -i "s/__package__template/$package_name/g" "$package_dir/$file"
        else
            echo -e "  ${GREEN}File already exists: $file${NC}"
        fi
    done
    
    # Add logo and favicon files if they don't exist
    for asset_type in "logo" "favicon"; do
        for variant in "" ".dark" ".light"; do
            for ext in ".txt" ".svg" ".png"; do
                asset_file="${asset_type}${variant}${ext}"
                if [ -f "$TEMPLATE_DIR/$asset_file" ] && [ ! -f "$package_dir/$asset_file" ]; then
                    echo -e "  ${YELLOW}Adding asset: $asset_file${NC}"
                    cp "$TEMPLATE_DIR/$asset_file" "$package_dir/$asset_file"
                fi
            done
        done
    done
    
    # Add package.smash if it doesn't exist
    if [ ! -f "$package_dir/package.smash" ]; then
        echo -e "  ${YELLOW}Adding package.smash${NC}"
        cp "$TEMPLATE_DIR/package.smash" "$package_dir/package.smash"
        # Replace template placeholders with package name
        sed -i "s/__package__template/$package_name/g" "$package_dir/package.smash"
    else
        echo -e "  ${GREEN}File already exists: package.smash${NC}"
    fi
    
    # Add install scripts if they don't exist
    for script in "preinstall.smash" "postinstall.smash"; do
        if [ ! -f "$package_dir/$script" ]; then
            echo -e "  ${YELLOW}Adding script: $script${NC}"
            cp "$TEMPLATE_DIR/$script" "$package_dir/$script"
            # Replace template placeholders with package name
            sed -i "s/__package__template/$package_name/g" "$package_dir/$script"
        else
            echo -e "  ${GREEN}File already exists: $script${NC}"
        fi
    done
    
    echo -e "${GREEN}Package $package_name standardized successfully${NC}"
    echo ""
}

# Process all packages
echo -e "${BLUE}Starting package standardization...${NC}"
echo ""

# First, process top-level packages
for package_dir in "$PACKAGES_DIR"/*/; do
    # Skip README.md and other files
    if [ -d "$package_dir" ]; then
        standardize_package "$package_dir"
    fi
done

# Then, process subpackages (packages within packages)
for package_dir in "$PACKAGES_DIR"/*/*/; do
    # Skip if not a directory
    if [ -d "$package_dir" ]; then
        # Skip if this is a standard directory like assets, examples, etc.
        package_parent="$(basename "$(dirname "$package_dir")")"
        package_name="$(basename "$package_dir")"
        if [[ ! " ${TEMPLATE_DIRS[@]} " =~ " $package_name " ]]; then
            standardize_package "$package_dir"
        fi
    fi
done

echo -e "${GREEN}All packages have been standardized!${NC}"
