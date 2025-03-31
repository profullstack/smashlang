#!/bin/bash

# Script to refactor SmashLang packages, converting source files to proper package structure

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

# Function to create a new package from a source file
create_package_from_source() {
    local source_file="$1"
    local parent_package="$2"
    
    # Get the base filename without extension
    local filename=$(basename "$source_file")
    local package_name="${filename%.*}"
    
    echo -e "${BLUE}Creating package: $parent_package/$package_name from $filename${NC}"
    
    # Create the package directory
    local package_dir="${PACKAGES_DIR}/${parent_package}/${package_name}"
    mkdir -p "$package_dir"
    
    # Create standard directories
    for dir in "assets" "examples" "src" "tests"; do
        mkdir -p "$package_dir/$dir"
        echo "# $parent_package/$package_name - $dir" > "$package_dir/$dir/README.md"
    done
    
    # Copy the source file to the src directory
    cp "$source_file" "$package_dir/src/index.smash"
    
    # Create package.smash
    cp "$TEMPLATE_DIR/package.smash" "$package_dir/package.smash"
    sed -i "s/package_name/$package_name/g" "$package_dir/package.smash"
    
    # Create package_config.json
    cp "$TEMPLATE_DIR/package_config.json" "$package_dir/package_config.json"
    sed -i "s/\"name\":[[:space:]]*\"package_name\"/\"name\": \"$package_name\"/g" "$package_dir/package_config.json"
    
    # Create a display name from package name (convert underscores to spaces, capitalize words)
    local display_name=$(echo "$package_name" | sed 's/_/ /g' | awk '{for(i=1;i<=NF;i++) $i=toupper(substr($i,1,1)) substr($i,2)} 1')
    sed -i "s/\"display_name\":[[:space:]]*\"Package Name\"/\"display_name\": \"$display_name\"/g" "$package_dir/package_config.json"
    
    # Copy install scripts
    cp "$TEMPLATE_DIR/preinstall.smash" "$package_dir/preinstall.smash"
    cp "$TEMPLATE_DIR/postinstall.smash" "$package_dir/postinstall.smash"
    sed -i "s/__package__template/$package_name/g" "$package_dir/preinstall.smash"
    sed -i "s/__package__template/$package_name/g" "$package_dir/postinstall.smash"
    
    # Create README.md
    cat > "$package_dir/README.md" << EOF
# $display_name

A SmashLang package for $display_name functionality.

## Package Structure

This package follows the standard SmashLang package structure:

- `package.smash`: Build and installation configuration
- `package_config.json`: Theme and presentation configuration
- `assets/`: Package assets (logos, icons, etc.)
- `src/`: Source code
- `examples/`: Example code
- `tests/`: Test files

## Installation

```bash
smashpkg install $parent_package/$package_name
```

## Usage

```javascript
import { ... } from '$parent_package/$package_name';

// Your code here
```

## License

MIT

## Created

$(date +"%Y-%m-%d")
EOF
    
    # Copy assets from parent package if they exist
    if [ -d "${PACKAGES_DIR}/${parent_package}/assets" ]; then
        cp -r "${PACKAGES_DIR}/${parent_package}/assets"/* "$package_dir/assets/" 2>/dev/null || true
    fi
    
    echo -e "${GREEN}Created package: $parent_package/$package_name${NC}"
    echo ""
}

# Function to refactor a package directory
refactor_package() {
    local package_dir="$1"
    local package_name="$(basename "$package_dir")"
    
    # Skip the template itself
    if [ "$package_name" == "__package__template" ]; then
        return
    fi
    
    echo -e "${BLUE}Refactoring package: $package_name${NC}"
    
    # Check if there's a src directory with .smash files
    if [ -d "$package_dir/src" ]; then
        # Find all .smash files in the src directory
        for source_file in "$package_dir/src"/*.smash; do
            if [ -f "$source_file" ] && [ "$(basename "$source_file")" != "index.smash" ]; then
                create_package_from_source "$source_file" "$package_name"
            fi
        done
        
        # If there's no index.smash, create one that imports from all subpackages
        if [ ! -f "$package_dir/src/index.smash" ]; then
            echo -e "${YELLOW}Creating index.smash that imports from all subpackages${NC}"
            
            # Start with a header
            cat > "$package_dir/src/index.smash" << EOF
// $package_name/index.smash - Main entry point for $package_name package
// This file imports and re-exports functionality from all subpackages

// Imports from subpackages
EOF
            
            # Add imports for each subpackage
            for subpackage in "$PACKAGES_DIR/$package_name"/*; do
                if [ -d "$subpackage" ] && [ "$(basename "$subpackage")" != "assets" ] && \
                   [ "$(basename "$subpackage")" != "examples" ] && [ "$(basename "$subpackage")" != "src" ] && \
                   [ "$(basename "$subpackage")" != "tests" ]; then
                    local subpackage_name="$(basename "$subpackage")"
                    echo "import * as $subpackage_name from './$subpackage_name';" >> "$package_dir/src/index.smash"
                fi
            done
            
            # Add exports
            cat >> "$package_dir/src/index.smash" << EOF

// Re-export all imports
export {
    // List all subpackages here
EOF
            
            # Add each subpackage to the exports
            for subpackage in "$PACKAGES_DIR/$package_name"/*; do
                if [ -d "$subpackage" ] && [ "$(basename "$subpackage")" != "assets" ] && \
                   [ "$(basename "$subpackage")" != "examples" ] && [ "$(basename "$subpackage")" != "src" ] && \
                   [ "$(basename "$subpackage")" != "tests" ]; then
                    local subpackage_name="$(basename "$subpackage")"
                    echo "    $subpackage_name," >> "$package_dir/src/index.smash"
                fi
            done
            
            # Close the export block
            echo "};" >> "$package_dir/src/index.smash"
            
            echo -e "${GREEN}Created index.smash for $package_name${NC}"
        fi
    fi
    
    echo ""
}

# Process all top-level packages
echo -e "${BLUE}Starting package refactoring...${NC}"
echo ""

for package_dir in "$PACKAGES_DIR"/*/; do
    # Skip README.md and other files
    if [ -d "$package_dir" ]; then
        refactor_package "$package_dir"
    fi
done

echo -e "${GREEN}Package refactoring complete!${NC}"
