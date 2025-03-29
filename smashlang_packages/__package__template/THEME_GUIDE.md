# Theme Support Guide for SmashLang Packages

This guide explains how to implement theme support in your SmashLang package, allowing your package to adapt to light and dark themes in various environments.

## Theme-Specific Icons

Your package should include both light and dark versions of icons to ensure good visibility across different themes:

### Required Icon Files

- **Favicon**: A small icon representing your package
  - `assets/favicon.light.svg` - For light themes (black/dark icon)
  - `assets/favicon.dark.svg` - For dark themes (white/light icon)
  
- **Logo**: A larger logo for documentation and branding
  - `assets/logo.light.svg` - For light themes (black/dark logo)
  - `assets/logo.dark.svg` - For dark themes (white/light logo)

### File Format Preferences

SVG format is preferred for all icons as it provides better scalability. However, PNG formats are also supported:

- `assets/favicon.light.png` and `assets/favicon.dark.png`
- `assets/logo.light.png` and `assets/logo.dark.png`

## Package Configuration

The `package_config.json` file contains theme-related settings for your package:

```json
{
  "name": "your_package",
  "display_name": "Your Package",
  "version": "1.0.0",
  "theme": {
    "icons": {
      "favicon": {
        "light": "favicon.light.svg",
        "dark": "favicon.dark.svg"
      },
      "logo": {
        "light": "logo.light.svg",
        "dark": "logo.dark.svg"
      }
    },
    "colors": {
      "primary": "#3498db",
      "secondary": "#2ecc71",
      "accent": "#e74c3c",
      "background": {
        "light": "#ffffff",
        "dark": "#1e1e1e"
      },
      "text": {
        "light": "#333333",
        "dark": "#f5f5f5"
      }
    }
  }
}
```

## Using Theme-Specific Icons in Documentation

In your README.md and other documentation, use the following HTML to display the appropriate logo based on the user's theme preference:

```html
<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="./assets/logo.dark.svg">
    <img src="./assets/logo.light.svg" alt="Package Logo" width="200" />
  </picture>
</p>
```

## ASCII Art Icons

For terminal-based interfaces, you can provide ASCII art versions of your icons:

- `favicon.light.txt` - ASCII art favicon for light terminals
- `favicon.dark.txt` - ASCII art favicon for dark terminals
- `logo.light.txt` - ASCII art logo for light terminals
- `logo.dark.txt` - ASCII art logo for dark terminals

These files will be automatically used when appropriate based on the terminal's theme.

## Integration with Web Interfaces

If your package includes a web interface, you can use CSS media queries to switch between themes:

```css
:root {
  --primary-color: #3498db;
  --text-color: #333333;
  --background-color: #ffffff;
}

@media (prefers-color-scheme: dark) {
  :root {
    --primary-color: #3498db;
    --text-color: #f5f5f5;
    --background-color: #1e1e1e;
  }
}
```

## Integration with VS Code Extensions

If your package includes a VS Code extension, update the `package.json` file to use theme-specific icons:

```json
{
  "icon": "icons/favicon.light.png",
  "languages": [
    {
      "id": "your-language",
      "icon": {
        "light": "./icons/favicon.light.png",
        "dark": "./icons/favicon.dark.png"
      }
    }
  ]
}
```

## Testing Your Theme Support

Test your package with both light and dark themes to ensure good visibility and contrast in all environments:

1. Test in VS Code with both light and dark themes
2. Test in web browsers with light and dark mode
3. Test in terminals with light and dark backgrounds

## Troubleshooting

- If theme-specific icons aren't displaying correctly, check file paths and formats
- Ensure SVG files have appropriate fill colors for their intended theme
- For PNG files, ensure transparent backgrounds for best results
