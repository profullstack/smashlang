# VS Code SmashLang Formatter

This extension provides formatting support for SmashLang files in Visual Studio Code using the `smashier` package.

## Features

- Format SmashLang (`.smash`) files using the smashier formatter
- Syntax highlighting for SmashLang code
- Customizable formatting options
- Format on save support

## Requirements

- SmashLang installed and available in your PATH
- The `tools/smashier` package installed (`smashpkg install tools/smashier`)

## Extension Settings

This extension contributes the following settings:

* `smashier.useTabs`: Use tabs instead of spaces
* `smashier.tabWidth`: Number of spaces per tab
* `smashier.printWidth`: Maximum line length
* `smashier.singleQuote`: Use single quotes instead of double quotes
* `smashier.semi`: Add semicolons at the end of statements
* `smashier.trailingComma`: Print trailing commas wherever possible

## How to Use

1. Open a SmashLang file (`.smash` extension)
2. Format the document using:
   - Keyboard shortcut: `Alt+Shift+F` (or your configured format shortcut)
   - Command Palette: `Format Document`
   - Right-click menu: `Format Document`

## How It Works

This extension communicates with the SmashLang interpreter to run the `smashier` package for formatting. When you format a document:

1. The extension creates a temporary SmashLang script that imports the `smashier` package
2. The script reads your document content, formats it using `smashier`, and writes the result
3. The extension reads the formatted result and applies it to your document

## Building the Extension

```bash
# Install dependencies
npm install

# Compile the extension
npm run compile

# Package the extension
npm run package
```

## Installation from VSIX

1. Build the extension to generate a `.vsix` file
2. In VS Code, open the Extensions view
3. Click the `...` menu and select `Install from VSIX...`
4. Select the generated `.vsix` file

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT
