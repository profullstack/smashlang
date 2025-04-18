# SmashLang WebAssembly Support

This document explains how to compile SmashLang code to WebAssembly (WASM) for browser execution.

## Overview

SmashLang's WebAssembly support allows you to:

- Compile SmashLang code to run in web browsers
- Create JavaScript bindings for easy integration
- Build standalone web applications
- Share code between server and client

## Requirements

To compile SmashLang code to WebAssembly, you'll need:

- **Emscripten**: The WebAssembly compiler toolchain
- **Node.js**: For JavaScript bundling and testing

### Installing Emscripten

```bash
git clone https://github.com/emscripten-core/emsdk.git
cd emsdk
./emsdk install latest
./emsdk activate latest
source ./emsdk_env.sh  # Add this to your .bashrc or .zshrc for persistence
```

## Basic Usage

To compile a SmashLang file to WebAssembly:

```bash
./scripts/compile_wasm.sh your_app.smash
```

This will create the following files in the `dist` directory:

- `your_app.wasm`: The WebAssembly binary
- `your_app.js`: JavaScript glue code generated by Emscripten
- `your_app.bindings.js`: JavaScript bindings for easier usage

## Advanced Options

### Standalone HTML Demo

Generate a standalone HTML file with a demo interface:

```bash
./scripts/compile_wasm.sh --standalone your_app.smash
```

### Development Mode

Watch for changes and automatically recompile:

```bash
./scripts/compile_wasm.sh --watch --serve your_app.smash
```

This will start a development server and recompile whenever the source file changes.

### All Options

```
Options:
  -o, --output DIR     Output directory (default: dist)
  -t, --target TYPE    Target platform: web, node (default: web)
  -n, --no-optimize   Disable optimization
  -s, --standalone    Generate standalone HTML file
  -w, --watch         Watch for changes and recompile
  --serve             Start a development server
  --port PORT         Development server port (default: 8080)
  -h, --help          Display this help message
```

## Using in Web Applications

After compiling your SmashLang code to WebAssembly, you can use it in your web application:

```html
<!-- Include the generated JavaScript files -->
<script src="your_app.js"></script>
<script src="your_app.bindings.js"></script>

<script>
  // Initialize the SmashLang module
  const smash = new SmashLang();
  
  // Wait for initialization
  async function init() {
    await smash.init();
    
    // Call functions exported from your SmashLang code
    const result = smash.add(5, 7);
    console.log(result);  // 12
    
    const greeting = smash.greet("World");
    console.log(greeting);  // "Hello, World from SmashLang WASM!"
  }
  
  init();
</script>
```

## Example

Here's a simple SmashLang file that can be compiled to WebAssembly:

```smash
// Import the standard library
import { console } from 'stdio';

// A simple function that adds two numbers
export function add(a: number, b: number): number {
  return a + b;
}

// A function that returns a greeting string
export function greet(name: string): string {
  return `Hello, ${name} from SmashLang WASM!`;
}

// Main function that runs when the module is loaded
export function main(): void {
  console.log('SmashLang WASM module initialized');
}
```

Compile this file with:

```bash
./scripts/compile_wasm.sh --standalone examples/wasm/hello_wasm.smash
```

Then open `dist/hello_wasm.html` in your browser to see it in action.

## How It Works

The WebAssembly compilation process involves several steps:

1. **SmashLang to C**: Your SmashLang code is first transpiled to C code
2. **C to WebAssembly**: Emscripten compiles the C code to WebAssembly
3. **JavaScript Bindings**: Wrapper code is generated to make the WASM module easy to use from JavaScript
4. **HTML Demo**: (Optional) A standalone HTML file is created with a demo interface

## Limitations

Current limitations of SmashLang WebAssembly support:

- DOM manipulation requires JavaScript interop
- File system access is limited in browser environments
- Some advanced SmashLang features may not be fully supported in WASM yet

## Future Improvements

- Direct DOM manipulation API
- WebGL/Canvas rendering support
- Web Workers integration
- Better debugging tools
- Smaller WASM binary size optimization

## Resources

- [WebAssembly Official Site](https://webassembly.org/)
- [Emscripten Documentation](https://emscripten.org/docs/index.html)
- [SmashLang Documentation](https://smashlang.com/docs)
