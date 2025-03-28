#!/bin/bash

# Script to generate logo.txt files for SmashLang packages

# Source the ASCII settings
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/ascii_settings.sh"

# Logo paths
LOGO_SVG="$SCRIPT_DIR/../assets/logo.svg"
LOGO_PNG="$SCRIPT_DIR/../assets/logo.png"
LOGO_JPG="/tmp/smashlang_logo.jpg"
LOGO_ASCII="$SCRIPT_DIR/../assets/logo.ascii"

# Package template directory
PACKAGE_TEMPLATE_DIR="$SCRIPT_DIR/../smashlang_packages/__package__template"

# Function to display usage information
usage() {
  echo "Usage: $0 [options] [package_dir]"
  echo ""
  echo "Options:"
  echo "  -a, --all         Generate logo.txt for all packages"
  echo "  -t, --template    Update the package template logo.txt"
  echo "  -w, --width NUM   Set width of ASCII art (default: 60)"
  echo "  -h, --help        Display this help message"
  echo ""
  echo "If no options are provided, generates logo.txt for the specified package directory."
  exit 1
}

# Parse command line arguments
ALL_PACKAGES=false
UPDATE_TEMPLATE=false

while [[ $# -gt 0 ]]; do
  case $1 in
    -a|--all)
      ALL_PACKAGES=true
      shift
      ;;
    -t|--template)
      UPDATE_TEMPLATE=true
      shift
      ;;
    -w|--width)
      ASCII_WIDTH=$2
      shift 2
      ;;
    -h|--help)
      usage
      ;;
    *)
      PACKAGE_DIR=$1
      shift
      ;;
  esac
done

# Check if ImageMagick is installed
if ! command -v convert &> /dev/null; then
  echo "Error: ImageMagick is not installed. Please install it with 'yay -S imagemagick'."
  exit 1
fi

# Convert logo to JPG for processing
if [ -f "$LOGO_PNG" ]; then
  convert "$LOGO_PNG" "$LOGO_JPG"
  echo "Converted PNG logo to JPG for processing."
elif [ -f "$LOGO_SVG" ]; then
  convert "$LOGO_SVG" "$LOGO_JPG"
  echo "Converted SVG logo to JPG for processing."
else
  echo "Error: No logo file found at $LOGO_PNG or $LOGO_SVG"
  exit 1
fi

# Generate ASCII art if it doesn't exist or if width has changed
if [ ! -f "$LOGO_ASCII" ] || [ "$ASCII_WIDTH" != "60" ]; then
  generate_ascii_art "$LOGO_JPG" "$LOGO_ASCII"
  echo "Generated ASCII logo at $LOGO_ASCII"
fi

# Function to add package information to logo.txt
create_logo_txt() {
  local package_dir=$1
  local package_name=$(basename "$package_dir")
  local logo_txt="$package_dir/logo.txt"
  
  # Get package information if package.smash exists
  local package_title="$package_name"
  local package_version=""
  local package_description=""
  
  if [ -f "$package_dir/package.smash" ]; then
    # Extract package info from package.smash
    package_title=$(grep -o '"name"\s*:\s*"[^"]*"' "$package_dir/package.smash" | head -1 | sed 's/"name"\s*:\s*"\([^"]*\)"/\1/')
    package_version=$(grep -o '"version"\s*:\s*"[^"]*"' "$package_dir/package.smash" | head -1 | sed 's/"version"\s*:\s*"\([^"]*\)"/\1/')
    package_description=$(grep -o '"description"\s*:\s*"[^"]*"' "$package_dir/package.smash" | head -1 | sed 's/"description"\s*:\s*"\([^"]*\)"/\1/')
  fi
  
  # Create logo.txt file
  echo "Creating logo.txt for $package_name..."
  
  # Copy ASCII art to logo.txt
  cat "$LOGO_ASCII" > "$logo_txt"
  
  # Add package information
  echo "" >> "$logo_txt"
  echo "${package_title}${package_version:+ v$package_version}" >> "$logo_txt"
  if [ -n "$package_description" ]; then
    echo "$package_description" >> "$logo_txt"
  fi
  echo "" >> "$logo_txt"
  echo "https://smashlang.com" >> "$logo_txt"
  
  echo "Created $logo_txt"
}

# Update the package template
if [ "$UPDATE_TEMPLATE" = true ]; then
  create_logo_txt "$PACKAGE_TEMPLATE_DIR"
  echo "Updated package template logo.txt"
fi

# Process all packages
if [ "$ALL_PACKAGES" = true ]; then
  find "$SCRIPT_DIR/../smashlang_packages" -mindepth 1 -maxdepth 2 -type d | while read package_dir; do
    # Skip the package template
    if [ "$package_dir" != "$PACKAGE_TEMPLATE_DIR" ]; then
      create_logo_txt "$package_dir"
    fi
  done
  echo "Generated logo.txt for all packages"
# Process a single package
elif [ -n "$PACKAGE_DIR" ]; then
  if [ ! -d "$PACKAGE_DIR" ]; then
    echo "Error: Package directory '$PACKAGE_DIR' not found."
    exit 1
  fi
  create_logo_txt "$PACKAGE_DIR"
else
  # If no specific action was requested, update the template
  create_logo_txt "$PACKAGE_TEMPLATE_DIR"
  echo "Updated package template logo.txt"
fi

# Clean up temporary files
rm -f "$LOGO_JPG"

echo "Done!"
