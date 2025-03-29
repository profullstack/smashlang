#!/bin/bash

# Script to fix icon colors for light and dark themes
# Light theme icons should have black fill (#000000)
# Dark theme icons should have white fill (#FFFFFF)

echo "Fixing icon colors for light and dark themes..."

BASE_DIR="/home/ettinger/src/profullstack.com/smashlang"

# Function to create themed PNG files
create_themed_pngs() {
  local source_dir="$1"
  local target_dir="$2"
  
  echo "Creating themed PNGs in $target_dir..."
  
  # Create light versions (black icons for light themes)
  if [ -f "$source_dir/logo.svg" ]; then
    convert "$source_dir/logo.svg" -background none -fill black -colorize 100 "$target_dir/logo.light.png"
    echo "Created $target_dir/logo.light.png"
  fi
  
  if [ -f "$source_dir/favicon.svg" ]; then
    convert "$source_dir/favicon.svg" -background none -fill black -colorize 100 "$target_dir/favicon.light.png"
    echo "Created $target_dir/favicon.light.png"
  fi
  
  # Create dark versions (white icons for dark themes)
  if [ -f "$source_dir/logo.svg" ]; then
    convert "$source_dir/logo.svg" -background none -fill white -colorize 100 "$target_dir/logo.dark.png"
    echo "Created $target_dir/logo.dark.png"
  fi
  
  if [ -f "$source_dir/favicon.svg" ]; then
    convert "$source_dir/favicon.svg" -background none -fill white -colorize 100 "$target_dir/favicon.dark.png"
    echo "Created $target_dir/favicon.dark.png"
  fi
}

# Fix assets directory icons (SVG)
echo "Fixing assets directory SVG icons..."
sed -i 's/fill="#000000"/fill="#000000"/g' $BASE_DIR/assets/logo.light.svg
sed -i 's/fill="#000000"/fill="#000000"/g' $BASE_DIR/assets/favicon.light.svg
sed -i 's/fill="#000000"/fill="#FFFFFF"/g' $BASE_DIR/assets/logo.dark.svg
sed -i 's/fill="#000000"/fill="#FFFFFF"/g' $BASE_DIR/assets/favicon.dark.svg

# Create PNG versions for assets directory
create_themed_pngs "$BASE_DIR/assets" "$BASE_DIR/assets"

# Fix VS Code extension icons (SVG)
echo "Fixing VS Code extension SVG icons..."
sed -i 's/fill="#FFFFFF"/fill="#000000"/g' $BASE_DIR/smashlang_external/vscode-smashier/icons/favicon.light.svg
sed -i 's/fill="#000000"/fill="#000000"/g' $BASE_DIR/smashlang_external/vscode-smashier/icons/logo.light.svg
sed -i 's/fill="#FFFFFF"/fill="#FFFFFF"/g' $BASE_DIR/smashlang_external/vscode-smashier/icons/favicon.dark.svg
sed -i 's/fill="#FFFFFF"/fill="#FFFFFF"/g' $BASE_DIR/smashlang_external/vscode-smashier/icons/logo.dark.svg

# Create PNG versions for VS Code extension
create_themed_pngs "$BASE_DIR/smashlang_external/vscode-smashier/icons" "$BASE_DIR/smashlang_external/vscode-smashier/icons"

# Fix package template icons
echo "Fixing package template icons..."
cp $BASE_DIR/assets/logo.light.svg $BASE_DIR/smashlang_packages/__package__template/logo.light.svg
cp $BASE_DIR/assets/logo.dark.svg $BASE_DIR/smashlang_packages/__package__template/logo.dark.svg
cp $BASE_DIR/assets/favicon.light.svg $BASE_DIR/smashlang_packages/__package__template/favicon.light.svg
cp $BASE_DIR/assets/favicon.dark.svg $BASE_DIR/smashlang_packages/__package__template/favicon.dark.svg
cp $BASE_DIR/assets/logo.light.png $BASE_DIR/smashlang_packages/__package__template/logo.light.png
cp $BASE_DIR/assets/logo.dark.png $BASE_DIR/smashlang_packages/__package__template/logo.dark.png
cp $BASE_DIR/assets/favicon.light.png $BASE_DIR/smashlang_packages/__package__template/favicon.light.png
cp $BASE_DIR/assets/favicon.dark.png $BASE_DIR/smashlang_packages/__package__template/favicon.dark.png

# Fix all existing packages
echo "Fixing all existing packages..."
for package_dir in $BASE_DIR/smashlang_packages/*/; do
  if [ "$package_dir" != "$BASE_DIR/smashlang_packages/__package__template/" ]; then
    package_name=$(basename "$package_dir")
    echo "Fixing package: $package_name"
    
    # Copy SVG files if they exist in the package
    if [ -f "$package_dir/logo.svg" ]; then
      cp $BASE_DIR/assets/logo.light.svg "$package_dir/logo.light.svg"
      cp $BASE_DIR/assets/logo.dark.svg "$package_dir/logo.dark.svg"
    fi
    
    if [ -f "$package_dir/favicon.svg" ]; then
      cp $BASE_DIR/assets/favicon.light.svg "$package_dir/favicon.light.svg"
      cp $BASE_DIR/assets/favicon.dark.svg "$package_dir/favicon.dark.svg"
    fi
    
    # Copy PNG files if they exist in the package
    if [ -f "$package_dir/logo.png" ]; then
      cp $BASE_DIR/assets/logo.light.png "$package_dir/logo.light.png"
      cp $BASE_DIR/assets/logo.dark.png "$package_dir/logo.dark.png"
    fi
    
    if [ -f "$package_dir/favicon.png" ]; then
      cp $BASE_DIR/assets/favicon.light.png "$package_dir/favicon.light.png"
      cp $BASE_DIR/assets/favicon.dark.png "$package_dir/favicon.dark.png"
    fi
    
    # Copy ASCII files
    cp $BASE_DIR/assets/logo.light.txt "$package_dir/logo.light.txt"
    cp $BASE_DIR/assets/logo.dark.txt "$package_dir/logo.dark.txt"
    cp $BASE_DIR/assets/favicon.light.txt "$package_dir/favicon.light.txt"
    cp $BASE_DIR/assets/favicon.dark.txt "$package_dir/favicon.dark.txt"
    
    # Update README.md to use theme-aware images if it exists
    if [ -f "$package_dir/README.md" ]; then
      sed -i 's/<img src="\.\/assets\\/logo\.svg" alt="Package Logo" width="200" \/>/<picture>\n    <source media="(prefers-color-scheme: dark)" srcset="\.\/assets\\/logo\.dark\.svg">\n    <img src="\.\/assets\\/logo\.light\.svg" alt="Package Logo" width="200" \/>\n  <\/picture>/g' "$package_dir/README.md"
    fi
  fi
done

echo "All icon colors fixed successfully!"

# Now regenerate the ASCII art versions with the correct colors
echo "Regenerating ASCII art versions..."
$BASE_DIR/scripts/generate_themed_ascii.sh

echo "Done!"
