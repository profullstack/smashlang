#!/bin/bash

# Script to generate themed ASCII art logos for SmashLang (light and dark versions)

# Source the ASCII settings
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/ascii_settings.sh"

# Logo paths
LOGO_SVG="$SCRIPT_DIR/../assets/logo.svg"
LOGO_PNG="$SCRIPT_DIR/../assets/logo.png"
LOGO_JPG="/tmp/smashlang_logo.jpg"
LOGO_LIGHT_ASCII="$SCRIPT_DIR/../assets/logo.light.txt"
LOGO_DARK_ASCII="$SCRIPT_DIR/../assets/logo.dark.txt"

# Favicon paths
FAVICON_SVG="$SCRIPT_DIR/../assets/favicon.svg"
FAVICON_PNG="$SCRIPT_DIR/../assets/favicon.png"
FAVICON_JPG="/tmp/smashlang_favicon.jpg"
FAVICON_LIGHT_ASCII="$SCRIPT_DIR/../assets/favicon.light.txt"
FAVICON_DARK_ASCII="$SCRIPT_DIR/../assets/favicon.dark.txt"

# Check if ImageMagick is installed
if ! command -v convert &> /dev/null; then
  echo "Error: ImageMagick is not installed. Please install it with 'yay -S imagemagick'."
  exit 1
fi

# Function to generate light and dark ASCII versions
generate_themed_ascii() {
  local image_path="$1"
  local temp_jpg="$2"
  local light_output="$3"
  local dark_output="$4"
  local image_name="$5"
  
  # Convert image to JPG for processing
  if [ -f "$image_path" ]; then
    convert "$image_path" "$temp_jpg"
    echo "Converted $image_name to JPG for processing."
  else
    echo "Error: No $image_name file found at $image_path"
    return 1
  fi
  
  # Save original settings
  local original_chars="$JP2A_CUSTOM_CHARS"
  
  # Generate light theme ASCII (dark characters on light background)
  JP2A_CUSTOM_CHARS=" .,:;+*#%@"
  generate_ascii_art "$temp_jpg" "$light_output"
  echo "Light theme $image_name ASCII generated at $light_output"
  
  # Generate dark theme ASCII (light characters on dark background)
  # Invert the character set for dark theme
  JP2A_CUSTOM_CHARS="@%#*+;:,. "
  generate_ascii_art "$temp_jpg" "$dark_output"
  echo "Dark theme $image_name ASCII generated at $dark_output"
  
  # Restore original settings
  JP2A_CUSTOM_CHARS="$original_chars"
  
  # Clean up temporary files
  rm -f "$temp_jpg"
  
  return 0
}

# Generate themed ASCII for logo
echo "Generating themed ASCII for logo..."
if [ -f "$LOGO_SVG" ]; then
  generate_themed_ascii "$LOGO_SVG" "$LOGO_JPG" "$LOGO_LIGHT_ASCII" "$LOGO_DARK_ASCII" "logo"
elif [ -f "$LOGO_PNG" ]; then
  generate_themed_ascii "$LOGO_PNG" "$LOGO_JPG" "$LOGO_LIGHT_ASCII" "$LOGO_DARK_ASCII" "logo"
else
  echo "Error: No logo file found at $LOGO_SVG or $LOGO_PNG"
fi

# Generate themed ASCII for favicon
echo "\nGenerating themed ASCII for favicon..."
if [ -f "$FAVICON_SVG" ]; then
  generate_themed_ascii "$FAVICON_SVG" "$FAVICON_JPG" "$FAVICON_LIGHT_ASCII" "$FAVICON_DARK_ASCII" "favicon"
elif [ -f "$FAVICON_PNG" ]; then
  generate_themed_ascii "$FAVICON_PNG" "$FAVICON_JPG" "$FAVICON_LIGHT_ASCII" "$FAVICON_DARK_ASCII" "favicon"
else
  echo "Error: No favicon file found at $FAVICON_SVG or $FAVICON_PNG"
fi

# Display the results
echo "\nPreview of light theme logo:"
echo "-----------------------------------"
cat "$LOGO_LIGHT_ASCII"
echo "-----------------------------------"

echo "\nPreview of dark theme logo:"
echo "-----------------------------------"
cat "$LOGO_DARK_ASCII"
echo "-----------------------------------"

echo "\nDone! You can include these ASCII art files in your package."
