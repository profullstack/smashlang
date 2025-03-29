# Installing the VS Code SmashLang Extension

There are several ways to install the VS Code SmashLang extension, depending on your needs and preferences.

## Prerequisites

Before installing the extension, make sure you have:

1. **SmashLang installed** and available in your PATH
2. **The `tools/smashier` package installed**:
   ```bash
   smashpkg install tools/smashier
   ```
3. **Visual Studio Code** (version 1.60.0 or later)

## Installation Methods

### Method 1: Install from VS Code Marketplace (Recommended)

1. Open VS Code
2. Click on the Extensions view icon in the Activity Bar (or press `Ctrl+Shift+X`)
3. Search for "SmashLang"
4. Click the "Install" button for the "SmashLang Formatter" extension

### Method 2: Install from VSIX File

1. Download the latest `.vsix` file from the [releases page](https://github.com/profullstack/smashlang/releases)
2. Open VS Code
3. Click on the Extensions view icon in the Activity Bar (or press `Ctrl+Shift+X`)
4. Click the `...` menu (top-right) and select "Install from VSIX..."
5. Navigate to and select the downloaded `.vsix` file
6. Click "Install" when prompted
7. If asked to reload VS Code, click "Reload Now"

**Alternative Method Using Command Line:**

```bash
# Install the extension from the command line
code --install-extension path/to/vscode-smashier.vsix
```

**Alternative Method Using Drag and Drop:**

1. Open VS Code
2. Drag the `.vsix` file from your file explorer and drop it onto the VS Code window
3. Click "Install" in the dialog that appears

### Method 3: Build and Install from Source

1. Clone the SmashLang repository:
   ```bash
   git clone https://github.com/profullstack/smashlang.git
   cd smashlang
   ```

2. Navigate to the extension directory:
   ```bash
   cd smashlang_external/vscode-smashier
   ```

3. Install dependencies and build the extension:
   ```bash
   npm install
   npm run compile
   npm run package
   ```
   This will generate a `.vsix` file in the current directory.

4. Install the extension in VS Code:
   - Open VS Code
   - Click on the Extensions view icon
   - Click the `...` menu and select "Install from VSIX..."
   - Navigate to and select the generated `.vsix` file

## Configuring the Extension

The SmashLang Formatter extension integrates with VS Code's settings system. You can customize the formatting behavior through the VS Code settings:

1. Open VS Code Settings (File > Preferences > Settings or `Ctrl+,`)
2. Search for "smashier" to see all available settings
3. Adjust the settings according to your preferences:
   - `smashier.useTabs`: Use tabs instead of spaces (boolean)
   - `smashier.tabWidth`: Number of spaces per tab (number)
   - `smashier.printWidth`: Maximum line length (number)
   - `smashier.singleQuote`: Use single quotes instead of double quotes (boolean)
   - `smashier.semi`: Add semicolons at the end of statements (boolean)
   - `smashier.trailingComma`: Print trailing commas ("none", "es5", or "all")

### Configuring Format On Save

1. Open VS Code Settings
2. Search for "format on save"
3. Check the "Editor: Format On Save" option
4. Make sure "Editor: Default Formatter" is set to "SmashLang Formatter" for `.smash` files

You can also configure format on save for SmashLang files only:

1. Click on "Edit in settings.json"
2. Add the following configuration:
   ```json
   "[smashlang]": {
     "editor.defaultFormatter": "smashlang.formatter",
     "editor.formatOnSave": true
   }
   ```

## Verifying the Installation

To verify that the extension is installed and working correctly:

1. Open a `.smash` file in VS Code
2. Check that syntax highlighting is applied automatically
3. Try formatting the document with `Alt+Shift+F` (or your configured format shortcut)
4. Verify that the code is formatted according to your settings

## Troubleshooting

If you encounter issues with the extension:

1. **Syntax highlighting not working**:
   - Make sure the file has a `.smash` extension
   - Try reloading VS Code (`Ctrl+R` or `F1` > "Reload Window")

2. **Formatting not working**:
   - Check that SmashLang is installed and in your PATH
   - Verify that the `tools/smashier` package is installed
   - Check the VS Code Developer Tools console for errors (`Help` > `Toggle Developer Tools`)

3. **Extension not showing up**:
   - Check the Extensions view to make sure it's installed
   - Look for "SmashLang Formatter" in the list of installed extensions

## Updating the Extension

If you installed from the VS Code Marketplace, updates will be applied automatically.

If you installed from a VSIX file or built from source, you'll need to repeat the installation process with the new version.

## Uninstalling

To uninstall the extension:

1. Open VS Code
2. Click on the Extensions view icon
3. Find "SmashLang Formatter" in your installed extensions
4. Click the gear icon and select "Uninstall"
5. Reload VS Code when prompted
