#!/bin/bash

# Script to refactor the hardware package, converting source files to proper package structure

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
HARDWARE_DIR="${PACKAGES_DIR}/hardware"

# Check if directories exist
if [ ! -d "$TEMPLATE_DIR" ]; then
    echo -e "${RED}Error: Template directory not found at $TEMPLATE_DIR${NC}"
    exit 1
fi

if [ ! -d "$HARDWARE_DIR" ]; then
    echo -e "${RED}Error: Hardware directory not found at $HARDWARE_DIR${NC}"
    exit 1
fi

if [ ! -d "$HARDWARE_DIR/src" ]; then
    echo -e "${RED}Error: Hardware src directory not found at $HARDWARE_DIR/src${NC}"
    exit 1
fi

# Create a package for each source file in hardware/src
for source_file in "$HARDWARE_DIR/src"/*.smash; do
    # Skip if not a file or if it's index.smash
    if [ ! -f "$source_file" ] || [ "$(basename "$source_file")" = "index.smash" ]; then
        continue
    fi
    
    # Get the base filename without extension
    filename=$(basename "$source_file")
    package_name="${filename%.*}"
    
    echo -e "${BLUE}Creating package: hardware/$package_name from $filename${NC}"
    
    # Create the package directory
    package_dir="${HARDWARE_DIR}/${package_name}"
    mkdir -p "$package_dir"
    
    # Create standard directories
    mkdir -p "$package_dir/assets"
    mkdir -p "$package_dir/examples"
    mkdir -p "$package_dir/src"
    mkdir -p "$package_dir/tests"
    
    # Add README files to each directory
    echo "# hardware/$package_name - assets" > "$package_dir/assets/README.md"
    echo "# hardware/$package_name - examples" > "$package_dir/examples/README.md"
    echo "# hardware/$package_name - src" > "$package_dir/src/README.md"
    echo "# hardware/$package_name - tests" > "$package_dir/tests/README.md"
    
    # Copy the source file to the src directory
    cp "$source_file" "$package_dir/src/index.smash"
    
    # Create package.smash
    cp "$TEMPLATE_DIR/package.smash" "$package_dir/package.smash"
    sed -i "s/package_name/$package_name/g" "$package_dir/package.smash"
    
    # Create package_config.json
    cp "$TEMPLATE_DIR/package_config.json" "$package_dir/package_config.json"
    sed -i "s/\"name\":[[:space:]]*\"package_name\"/\"name\": \"$package_name\"/g" "$package_dir/package_config.json"
    
    # Create a display name from package name (convert underscores to spaces, capitalize words)
    display_name=$(echo "$package_name" | sed 's/_/ /g' | awk '{for(i=1;i<=NF;i++) $i=toupper(substr($i,1,1)) substr($i,2)} 1')
    sed -i "s/\"display_name\":[[:space:]]*\"Package Name\"/\"display_name\": \"$display_name\"/g" "$package_dir/package_config.json"
    
    # Copy install scripts
    cp "$TEMPLATE_DIR/preinstall.smash" "$package_dir/preinstall.smash"
    cp "$TEMPLATE_DIR/postinstall.smash" "$package_dir/postinstall.smash"
    sed -i "s/__package__template/$package_name/g" "$package_dir/preinstall.smash"
    sed -i "s/__package__template/$package_name/g" "$package_dir/postinstall.smash"
    
    # Create README.md
    cat > "$package_dir/README.md" << EOF
# $display_name

A SmashLang package for $display_name hardware functionality.

## Package Structure

This package follows the standard SmashLang package structure:

- \`package.smash\`: Build and installation configuration
- \`package_config.json\`: Theme and presentation configuration
- \`assets/\`: Package assets (logos, icons, etc.)
- \`src/\`: Source code
- \`examples/\`: Example code
- \`tests/\`: Test files

## Installation

\`\`\`bash
smashpkg install hardware/$package_name
\`\`\`

## Usage

\`\`\`javascript
import { ... } from 'hardware/$package_name';

// Your code here
\`\`\`

## License

MIT

## Created

$(date +"%Y-%m-%d")
EOF
    
    # Copy assets from hardware package if they exist
    if [ -d "${HARDWARE_DIR}/assets" ]; then
        cp -r "${HARDWARE_DIR}/assets"/* "$package_dir/assets/" 2>/dev/null || true
    fi
    
    echo -e "${GREEN}Created package: hardware/$package_name${NC}"
    echo ""
done

# Create index.smash for hardware package
echo -e "${YELLOW}Creating index.smash for hardware package${NC}"

# Create src directory if it doesn't exist
mkdir -p "${HARDWARE_DIR}/src"

# Start with a header
cat > "${HARDWARE_DIR}/src/index.smash" << EOF
// hardware/index.smash - Main entry point for hardware package
// This file imports and re-exports functionality from all hardware subpackages

// Imports from subpackages
EOF

# Add imports for each subpackage
for subpackage in "${HARDWARE_DIR}"/*; do
    if [ -d "$subpackage" ] && [ "$(basename "$subpackage")" != "assets" ] && \
       [ "$(basename "$subpackage")" != "examples" ] && [ "$(basename "$subpackage")" != "src" ] && \
       [ "$(basename "$subpackage")" != "tests" ]; then
        subpackage_name="$(basename "$subpackage")"
        echo "import * as $subpackage_name from './$subpackage_name';" >> "${HARDWARE_DIR}/src/index.smash"
    fi
done

# Add exports
cat >> "${HARDWARE_DIR}/src/index.smash" << EOF

// Re-export all imports
export {
EOF

# Add each subpackage to the exports
for subpackage in "${HARDWARE_DIR}"/*; do
    if [ -d "$subpackage" ] && [ "$(basename "$subpackage")" != "assets" ] && \
       [ "$(basename "$subpackage")" != "examples" ] && [ "$(basename "$subpackage")" != "src" ] && \
       [ "$(basename "$subpackage")" != "tests" ]; then
        subpackage_name="$(basename "$subpackage")"
        echo "    $subpackage_name," >> "${HARDWARE_DIR}/src/index.smash"
    fi
done

# Close the export block
echo "};" >> "${HARDWARE_DIR}/src/index.smash"

echo -e "${GREEN}Created index.smash for hardware package${NC}"

echo -e "${GREEN}Hardware package refactoring complete!${NC}"
