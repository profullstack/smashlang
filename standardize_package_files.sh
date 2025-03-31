#!/bin/bash

# Script to standardize SmashLang package files according to the __package__template
# Converts package.json to package.smash and ensures package_config.json exists

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

# Function to standardize package files
standardize_package_files() {
    local package_dir="$1"
    local package_name="$(basename "$package_dir")"
    
    # Skip the template itself
    if [ "$package_name" == "__package__template" ]; then
        return
    fi
    
    echo -e "${BLUE}Processing package: $package_name${NC}"
    
    # Check for package.json and convert to package.smash if needed
    if [ -f "$package_dir/package.json" ]; then
        echo -e "  ${YELLOW}Found package.json, converting to package.smash format${NC}"
        
        # Extract information from package.json
        local pkg_name=$(grep '"name"' "$package_dir/package.json" | head -1 | sed -E 's/.*"name"[[:space:]]*:[[:space:]]*"([^"]+)".*/\1/')
        local pkg_version=$(grep '"version"' "$package_dir/package.json" | head -1 | sed -E 's/.*"version"[[:space:]]*:[[:space:]]*"([^"]+)".*/\1/')
        local pkg_description=$(grep '"description"' "$package_dir/package.json" | head -1 | sed -E 's/.*"description"[[:space:]]*:[[:space:]]*"([^"]+)".*/\1/')
        local pkg_license=$(grep '"license"' "$package_dir/package.json" | head -1 | sed -E 's/.*"license"[[:space:]]*:[[:space:]]*"([^"]+)".*/\1/')
        local pkg_author=$(grep '"author"' "$package_dir/package.json" | head -1 | sed -E 's/.*"author"[[:space:]]*:[[:space:]]*"([^"]+)".*/\1/')
        
        # If any values are empty, use defaults
        [ -z "$pkg_name" ] && pkg_name="$package_name"
        [ -z "$pkg_version" ] && pkg_version="0.1.0"
        [ -z "$pkg_description" ] && pkg_description="A SmashLang package"
        [ -z "$pkg_license" ] && pkg_license="MIT"
        [ -z "$pkg_author" ] && pkg_author=""
        
        # Create package.smash based on template
        if [ ! -f "$package_dir/package.smash" ]; then
            echo -e "  ${YELLOW}Creating package.smash from template${NC}"
            cp "$TEMPLATE_DIR/package.smash" "$package_dir/package.smash"
            
            # Update package.smash with information from package.json
            sed -i "s/\"name\":[[:space:]]*\"package_name\"/\"name\": \"$pkg_name\"/g" "$package_dir/package.smash"
            sed -i "s/\"version\":[[:space:]]*\"0\.1\.0\"/\"version\": \"$pkg_version\"/g" "$package_dir/package.smash"
            sed -i "s/\"description\":[[:space:]]*\"A brief description of what your package does\"/\"description\": \"$pkg_description\"/g" "$package_dir/package.smash"
            sed -i "s/\"license\":[[:space:]]*\"MIT\"/\"license\": \"$pkg_license\"/g" "$package_dir/package.smash"
            
            # Update authors array
            if [ ! -z "$pkg_author" ]; then
                sed -i "/\"authors\":[[:space:]]*\[/,/\]/c\  \"authors\": [\n    \"$pkg_author\"\n  ]," "$package_dir/package.smash"
            fi
            
            echo -e "  ${GREEN}Created package.smash with data from package.json${NC}"
        else
            echo -e "  ${GREEN}package.smash already exists, not overwriting${NC}"
        fi
        
        # Rename package.json to package.json.bak
        mv "$package_dir/package.json" "$package_dir/package.json.bak"
        echo -e "  ${GREEN}Renamed package.json to package.json.bak${NC}"
    elif [ ! -f "$package_dir/package.smash" ]; then
        echo -e "  ${YELLOW}No package.json or package.smash found, creating package.smash from template${NC}"
        cp "$TEMPLATE_DIR/package.smash" "$package_dir/package.smash"
        sed -i "s/package_name/$package_name/g" "$package_dir/package.smash"
        echo -e "  ${GREEN}Created package.smash from template${NC}"
    else
        echo -e "  ${GREEN}package.smash already exists${NC}"
    fi
    
    # Ensure package_config.json exists
    if [ ! -f "$package_dir/package_config.json" ]; then
        echo -e "  ${YELLOW}Creating package_config.json from template${NC}"
        cp "$TEMPLATE_DIR/package_config.json" "$package_dir/package_config.json"
        
        # Update package_config.json with package name
        sed -i "s/\"name\":[[:space:]]*\"package_name\"/\"name\": \"$package_name\"/g" "$package_dir/package_config.json"
        
        # Create a display name from package name (convert underscores to spaces, capitalize words)
        local display_name=$(echo "$package_name" | sed 's/_/ /g' | awk '{for(i=1;i<=NF;i++) $i=toupper(substr($i,1,1)) substr($i,2)} 1')
        sed -i "s/\"display_name\":[[:space:]]*\"Package Name\"/\"display_name\": \"$display_name\"/g" "$package_dir/package_config.json"
        
        echo -e "  ${GREEN}Created package_config.json${NC}"
    else
        echo -e "  ${GREEN}package_config.json already exists${NC}"
    fi
    
    # Ensure preinstall.smash and postinstall.smash exist
    for script in "preinstall.smash" "postinstall.smash"; do
        if [ ! -f "$package_dir/$script" ]; then
            echo -e "  ${YELLOW}Creating $script from template${NC}"
            cp "$TEMPLATE_DIR/$script" "$package_dir/$script"
            sed -i "s/__package__template/$package_name/g" "$package_dir/$script"
            echo -e "  ${GREEN}Created $script${NC}"
        else
            echo -e "  ${GREEN}$script already exists${NC}"
        fi
    done
    
    echo ""
}

# Process all packages
echo -e "${BLUE}Starting package file standardization...${NC}"
echo ""

# First, process top-level packages
for package_dir in "$PACKAGES_DIR"/*/; do
    # Skip README.md and other files
    if [ -d "$package_dir" ]; then
        standardize_package_files "$package_dir"
    fi
done

# Then, process subpackages (packages within packages)
for package_dir in "$PACKAGES_DIR"/*/*/; do
    # Skip if not a directory or if it's a standard directory
    if [ -d "$package_dir" ]; then
        package_name="$(basename "$package_dir")"
        if [ "$package_name" != "assets" ] && [ "$package_name" != "examples" ] && [ "$package_name" != "src" ] && [ "$package_name" != "tests" ]; then
            standardize_package_files "$package_dir"
        fi
    fi
done

echo -e "${GREEN}All package files have been standardized!${NC}"

# Update README.md files to mention the package structure
echo -e "${BLUE}Updating documentation to reflect standard package structure...${NC}"

find "$PACKAGES_DIR" -name "README.md" -type f | while read -r readme_file; do
    # Skip if the README already mentions package.smash and package_config.json
    if ! grep -q 'package\.smash' "$readme_file" || ! grep -q 'package_config\.json' "$readme_file"; then
        echo -e "${YELLOW}Updating documentation in: $(basename $(dirname "$readme_file"))/README.md${NC}"
        # Add a note about the package structure
        sed -i '/# /a \
## Package Structure\n\nThis package follows the standard SmashLang package structure:\n\n- `package.smash`: Build and installation configuration\n- `package_config.json`: Theme and presentation configuration\n- `assets/`: Package assets (logos, icons, etc.)\n- `src/`: Source code\n- `examples/`: Example code\n- `tests/`: Test files\n' "$readme_file"
    fi
done

echo -e "${GREEN}Package file standardization complete!${NC}"
