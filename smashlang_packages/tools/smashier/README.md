# Smashier

A code formatter and syntax highlighter for SmashLang, inspired by Prettier.

## Installation

```bash
smashpkg install tools/smashier
```

## Features

- **Code Formatting**: Automatically format your SmashLang code according to consistent style guidelines
- **Syntax Highlighting**: Add syntax coloring to your code for better readability
- **Code Linting**: Check your code for style issues and potential problems
- **Multiple Themes**: Choose from several built-in color themes for syntax highlighting
- **Customizable Options**: Configure formatting to match your preferred style

## Usage

```javascript
import { format, highlight, lint, formatAndHighlight } from 'tools/smashier';

// Format code
const formattedCode = format(sourceCode);

// Highlight code with syntax coloring (returns HTML)
const highlightedCode = highlight(sourceCode, 'default');

// Check for style issues
const issues = lint(sourceCode);

// Format and highlight in one step
const formattedAndHighlighted = formatAndHighlight(sourceCode, { theme: 'monokai' });
```

## API Reference

### format(code, options)

Formats SmashLang code according to standard style guidelines.

**Parameters:**
- `code` (string): The SmashLang code to format
- `options` (object, optional): Formatting options

**Returns:**
- (string): The formatted code

### highlight(code, theme)

Highlights SmashLang code with syntax coloring.

**Parameters:**
- `code` (string): The SmashLang code to highlight
- `theme` (string, optional): The color theme to use (default: 'default')

**Returns:**
- (string): HTML string with syntax highlighting

### lint(code, options)

Checks SmashLang code for style issues.

**Parameters:**
- `code` (string): The SmashLang code to check
- `options` (object, optional): Linting options

**Returns:**
- (array): Array of style issues found

### formatAndHighlight(code, options)

Formats and highlights SmashLang code in one operation.

**Parameters:**
- `code` (string): The SmashLang code to process
- `options` (object, optional): Processing options

**Returns:**
- (string): HTML string with formatted and highlighted code

## Configuration Options

You can customize the behavior of Smashier by providing options:

```javascript
const options = {
  // Indentation
  useTabs: false,           // Use spaces instead of tabs
  tabWidth: 2,              // Number of spaces per tab
  indentSize: 2,            // Number of spaces for indentation
  
  // Line wrapping
  printWidth: 80,           // Maximum line length
  
  // Quotes
  singleQuote: true,        // Use single quotes instead of double quotes
  
  // Semicolons
  semi: true,               // Add semicolons at the end of statements
  
  // Trailing commas
  trailingComma: 'es5',     // Add trailing commas where valid in ES5
  
  // SmashLang specific options
  moduleImportStyle: 'es6', // Use ES6 style imports
  functionStyle: 'arrow'    // Prefer arrow functions
};

const formattedCode = format(sourceCode, options);
```

## Themes

Smashier supports multiple color themes for syntax highlighting:

- `default`: Similar to VS Code dark theme
- `light`: Light theme with blue keywords
- `monokai`: Monokai-inspired dark theme
- `github`: GitHub-inspired light theme

Example usage:

```javascript
// Use the monokai theme
const highlightedCode = highlight(sourceCode, 'monokai');
```

## Example

Here's a complete example of using Smashier to format and highlight code:

```javascript
import { format, highlight } from 'tools/smashier';

// Example of poorly formatted code
const code = `
function calculateTotal(items,tax){
  let total=0;for(let i=0;i<items.length;i++){
    const item=items[i];total+=item.price;
}
return total*(1+tax);
}
`;

// Format the code
const formattedCode = format(code);
console.log(formattedCode);

// Highlight the code
const highlightedCode = highlight(formattedCode, 'monokai');
// Use the highlighted code in your application
```

## CLI Usage (Future)

In future versions, Smashier will include a command-line interface for formatting files:

```bash
smash smashier file.smash --write
```

## License

MIT

## Created

2025-03-28