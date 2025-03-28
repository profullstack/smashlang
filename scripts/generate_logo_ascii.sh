#!/bin/bash

# Script to generate ASCII art logo for SmashLang

# Source the ASCII settings
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/ascii_settings.sh"

# Logo paths
LOGO_SVG="$SCRIPT_DIR/../assets/logo.svg"
LOGO_PNG="$SCRIPT_DIR/../assets/logo.png"
LOGO_JPG="/tmp/smashlang_logo.jpg"
LOGO_ASCII="$SCRIPT_DIR/../assets/logo.ascii"

# Check if ImageMagick is installed
if ! command -v convert &> /dev/null; then
  echo "Error: ImageMagick is not installed. Please install it with 'yay -S imagemagick'."
  exit 1
fi

# Convert logo to JPG for jp2a
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

# Generate ASCII art
generate_ascii_art "$LOGO_JPG" "$LOGO_ASCII"

# Display the result
echo "ASCII logo generated at $LOGO_ASCII"
echo "Preview:"
echo "-----------------------------------"
cat "$LOGO_ASCII"
echo "-----------------------------------"

# Clean up temporary files
rm -f "$LOGO_JPG"

echo "Done! You can include this ASCII art in your installer script."
