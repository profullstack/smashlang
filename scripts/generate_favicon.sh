#!/bin/bash

# Script to generate favicon.txt files for SmashLang packages

# Source the ASCII settings
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/ascii_settings.sh"

# Logo paths
LOGO_SVG="$SCRIPT_DIR/../assets/logo.svg"
LOGO_PNG="$SCRIPT_DIR/../assets/logo.png"
LOGO_JPG="/tmp/smashlang_favicon.jpg"
FAVICON_ASCII="$SCRIPT_DIR/../assets/favicon.ascii"

# Package template directory
PACKAGE_TEMPLATE_DIR="$SCRIPT_DIR/../smashlang_packages/__package__template"

# Function to display usage information
usage() {
  echo "Usage: $0 [options] [package_dir]"
  echo ""
  echo "Options:"
  echo "  -a, --all         Generate favicon.txt for all packages"
  echo "  -t, --template    Update the package template favicon.txt"
  echo "  -w, --width NUM   Set width of ASCII art (default: 20)"
  echo "  -h, --help        Display this help message"
  echo ""
  echo "If no options are provided, generates favicon.txt for the specified package directory."
  exit 1
}

# Parse command line arguments
ALL_PACKAGES=false
UPDATE_TEMPLATE=false
FAVICON_WIDTH=20

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
      FAVICON_WIDTH=$2
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
  convert "$LOGO_PNG" -resize ${FAVICON_WIDTH}x${FAVICON_WIDTH} "$LOGO_JPG"
  echo "Converted PNG logo to JPG for favicon processing."
elif [ -f "$LOGO_SVG" ]; then
  convert "$LOGO_SVG" -resize ${FAVICON_WIDTH}x${FAVICON_WIDTH} "$LOGO_JPG"
  echo "Converted SVG logo to JPG for favicon processing."
else
  echo "Error: No logo file found at $LOGO_PNG or $LOGO_SVG"
  exit 1
fi

# Generate favicon ASCII art
echo "Generating favicon ASCII art..."

# Save original settings
ORIGINAL_WIDTH=$ASCII_WIDTH
ORIGINAL_HEIGHT=$ASCII_HEIGHT
ORIGINAL_CHARSET=$JP2A_CHARSET
ORIGINAL_CUSTOM_CHARS=$JP2A_CUSTOM_CHARS

# Set favicon-specific settings
ASCII_WIDTH=$FAVICON_WIDTH
ASCII_HEIGHT=""
JP2A_CHARSET="custom"
JP2A_CUSTOM_CHARS=" .,:;+*#%@"

# Generate the favicon ASCII art
generate_ascii_art "$LOGO_JPG" "$FAVICON_ASCII"

# Restore original settings
ASCII_WIDTH=$ORIGINAL_WIDTH
ASCII_HEIGHT=$ORIGINAL_HEIGHT
JP2A_CHARSET=$ORIGINAL_CHARSET
JP2A_CUSTOM_CHARS=$ORIGINAL_CUSTOM_CHARS

echo "Generated favicon ASCII art at $FAVICON_ASCII"

# Function to create favicon.txt for a package
create_favicon_txt() {
  local package_dir=$1
  local package_name=$(basename "$package_dir")
  local favicon_txt="$package_dir/favicon.txt"
  
  # Create favicon.txt file
  echo "Creating favicon.txt for $package_name..."
  
  # Copy ASCII art to favicon.txt
  cat "$FAVICON_ASCII" > "$favicon_txt"
  
  echo "Created $favicon_txt"
}

# Update the package template
if [ "$UPDATE_TEMPLATE" = true ]; then
  create_favicon_txt "$PACKAGE_TEMPLATE_DIR"
  echo "Updated package template favicon.txt"
fi

# Process all packages
if [ "$ALL_PACKAGES" = true ]; then
  find "$SCRIPT_DIR/../smashlang_packages" -mindepth 1 -maxdepth 2 -type d | while read package_dir; do
    # Skip the package template
    if [ "$package_dir" != "$PACKAGE_TEMPLATE_DIR" ]; then
      create_favicon_txt "$package_dir"
    fi
  done
  echo "Generated favicon.txt for all packages"
# Process a single package
elif [ -n "$PACKAGE_DIR" ]; then
  if [ ! -d "$PACKAGE_DIR" ]; then
    echo "Error: Package directory '$PACKAGE_DIR' not found."
    exit 1
  fi
  create_favicon_txt "$PACKAGE_DIR"
else
  # If no specific action was requested, update the template
  create_favicon_txt "$PACKAGE_TEMPLATE_DIR"
  echo "Updated package template favicon.txt"
fi

# Clean up temporary files
rm -f "$LOGO_JPG"

echo "Done!"
