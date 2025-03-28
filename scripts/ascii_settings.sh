#!/bin/bash

# ASCII Art Generator Settings

# Which generator to use: 'jp2a' or 'ascii-image-converter'
ASCII_GENERATOR="jp2a"

# Width of the ASCII art output
ASCII_WIDTH=60

# Height of the ASCII art output (leave empty for auto)
ASCII_HEIGHT=""

# Color output (true/false)
ASCII_COLOR=false

# Character set to use (only for jp2a)
# Options: default, vt100, vt100-bw, or custom
JP2A_CHARSET="custom"

# Custom character set for jp2a (only used if JP2A_CHARSET="custom")
JP2A_CUSTOM_CHARS=" .,:;+*#%@"

# Output style (only for ascii-image-converter)
# Options: standard, colored, braille
ASCII_CONVERTER_STYLE="braille"

# Generate ASCII art from an image file
generate_ascii_art() {
  local input_file="$1"
  local output_file="$2"
  
  # Create directory if it doesn't exist
  mkdir -p "$(dirname "$output_file")"
  
  # Check if the input file exists
  if [ ! -f "$input_file" ]; then
    echo "Error: Input file '$input_file' not found."
    return 1
  fi
  
  # Generate ASCII art based on the configured generator
  if [ "$ASCII_GENERATOR" = "jp2a" ]; then
    # Check if jp2a is installed
    if ! command -v jp2a &> /dev/null; then
      echo "Error: jp2a is not installed. Please install it with 'yay -S jp2a'."
      return 1
    fi
    
    # Build jp2a command
    local cmd="jp2a --width=$ASCII_WIDTH"
    
    if [ -n "$ASCII_HEIGHT" ]; then
      cmd="$cmd --height=$ASCII_HEIGHT"
    fi
    
    if [ "$ASCII_COLOR" = true ]; then
      cmd="$cmd --colors"
    fi
    
    if [ "$JP2A_CHARSET" = "custom" ]; then
      cmd="$cmd --chars=\"$JP2A_CUSTOM_CHARS\""
    else
      cmd="$cmd --charset=$JP2A_CHARSET"
    fi
    
    # Run jp2a and save output
    eval "$cmd '$input_file' > '$output_file'"
    
  elif [ "$ASCII_GENERATOR" = "ascii-image-converter" ]; then
    # Check if ascii-image-converter is installed
    if ! command -v ascii-image-converter &> /dev/null; then
      echo "Error: ascii-image-converter is not installed. Please install it with 'yay -S ascii-image-converter'."
      return 1
    fi
    
    # Build ascii-image-converter command
    local cmd="ascii-image-converter '$input_file' --width $ASCII_WIDTH"
    
    if [ -n "$ASCII_HEIGHT" ]; then
      cmd="$cmd --height $ASCII_HEIGHT"
    fi
    
    if [ "$ASCII_CONVERTER_STYLE" = "colored" ] && [ "$ASCII_COLOR" = true ]; then
      cmd="$cmd --color"
    elif [ "$ASCII_CONVERTER_STYLE" = "braille" ]; then
      cmd="$cmd --braille"
    fi
    
    # Run ascii-image-converter and save output
    eval "$cmd > '$output_file'"
  else
    echo "Error: Unknown ASCII generator '$ASCII_GENERATOR'."
    return 1
  fi
  
  echo "ASCII art generated and saved to '$output_file'."
  return 0
}

# Example usage:
# generate_ascii_art "assets/logo.png" "assets/logo.ascii"
