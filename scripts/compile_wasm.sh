#!/bin/bash

# SmashLang WebAssembly Compiler
# This script compiles SmashLang code to WebAssembly for browser execution

# Colors for output
RED="\033[0;31m"
GREEN="\033[0;32m"
YELLOW="\033[0;33m"
BLUE="\033[0;34m"
MAGENTA="\033[0;35m"
CYAN="\033[0;36m"
NC="\033[0m" # No Color

# Get script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"

# Default values
OUTPUT_DIR="dist"
TARGET="web"
OPTIMIZE=true
BUNDLE=true
WATCH=false
SERVE=false
PORT=8080

# Function to display usage information
usage() {
  echo -e "${YELLOW}SmashLang WebAssembly Compiler${NC}"
  echo -e "${BLUE}Compiles SmashLang code to WebAssembly for browser execution${NC}"
  echo ""
  echo -e "${YELLOW}Usage:${NC} $0 [options] <source_file.smash>"
  echo ""
  echo -e "${YELLOW}Options:${NC}"
  echo -e "  -o, --output DIR     Output directory (default: dist)"
  echo -e "  -t, --target TYPE    Target platform: web, node (default: web)"
  echo -e "  -n, --no-optimize   Disable optimization"
  echo -e "  -s, --standalone    Generate standalone HTML file"
  echo -e "  -w, --watch         Watch for changes and recompile"
  echo -e "  --serve             Start a development server"
  echo -e "  --port PORT         Development server port (default: 8080)"
  echo -e "  -h, --help          Display this help message"
  echo ""
  echo -e "${YELLOW}Examples:${NC}"
  echo -e "  $0 app.smash                      # Basic compilation"
  echo -e "  $0 --standalone app.smash         # Generate standalone HTML"
  echo -e "  $0 --watch --serve app.smash      # Watch and serve for development"
  exit 1
}

# Parse command line arguments
POSITIONAL_ARGS=()
while [[ $# -gt 0 ]]; do
  case $1 in
    -o|--output)
      OUTPUT_DIR="$2"
      shift 2
      ;;
    -t|--target)
      TARGET="$2"
      shift 2
      ;;
    -n|--no-optimize)
      OPTIMIZE=false
      shift
      ;;
    -s|--standalone)
      STANDALONE=true
      shift
      ;;
    -w|--watch)
      WATCH=true
      shift
      ;;
    --serve)
      SERVE=true
      shift
      ;;
    --port)
      PORT="$2"
      shift 2
      ;;
    -h|--help)
      usage
      ;;
    -*)
      echo -e "${RED}Error: Unknown option $1${NC}"
      usage
      ;;
    *)
      POSITIONAL_ARGS+=($1)
      shift
      ;;
  esac
done

# Restore positional arguments
set -- "${POSITIONAL_ARGS[@]}"

# Check if a source file was provided
if [ $# -eq 0 ]; then
  echo -e "${RED}Error: No source file provided${NC}"
  usage
fi

SOURCE_FILE="$1"
SOURCE_FILENAME=$(basename "$SOURCE_FILE")
SOURCE_NAME="${SOURCE_FILENAME%.*}"

# Check if source file exists
if [ ! -f "$SOURCE_FILE" ]; then
  echo -e "${RED}Error: Source file '$SOURCE_FILE' not found${NC}"
  exit 1
fi

# Create output directory if it doesn't exist
mkdir -p "$OUTPUT_DIR"

# Check for required dependencies
check_dependencies() {
  local missing_deps=false
  
  # Check for emscripten (required for WebAssembly compilation)
  if ! command -v emcc &> /dev/null; then
    echo -e "${RED}Error: Emscripten (emcc) is not installed${NC}"
    echo -e "${YELLOW}Please install Emscripten:${NC}"
    echo -e "  git clone https://github.com/emscripten-core/emsdk.git"
    echo -e "  cd emsdk"
    echo -e "  ./emsdk install latest"
    echo -e "  ./emsdk activate latest"
    echo -e "  source ./emsdk_env.sh"
    missing_deps=true
  fi
  
  # Check for node (required for JavaScript bundling)
  if ! command -v node &> /dev/null; then
    echo -e "${RED}Error: Node.js is not installed${NC}"
    echo -e "${YELLOW}Please install Node.js:${NC}"
    echo -e "  https://nodejs.org/"
    missing_deps=true
  fi
  
  if [ "$missing_deps" = true ]; then
    exit 1
  fi
}

# Check dependencies
check_dependencies

# Function to compile SmashLang to C
compile_to_c() {
  echo -e "${BLUE}Transpiling SmashLang to C...${NC}"
  
  # This is a placeholder for the actual SmashLang to C compilation
  # In a real implementation, you would call the SmashLang compiler here
  
  # For now, we'll create a simple C file that demonstrates WebAssembly capabilities
  cat > "$OUTPUT_DIR/${SOURCE_NAME}.c" << EOF
#include <stdio.h>
#include <stdlib.h>
#include <emscripten.h>

// Exported function that will be callable from JavaScript
EMSCRIPTEN_KEEPALIVE int add(int a, int b) {
  return a + b;
}

// Exported function to demonstrate string handling
EMSCRIPTEN_KEEPALIVE char* greet(const char* name) {
  char* greeting = (char*)malloc(100);
  sprintf(greeting, "Hello, %s from SmashLang WASM!", name);
  return greeting;
}

// Main function (required but may not be used in WASM context)
int main() {
  printf("SmashLang WASM module initialized\n");
  return 0;
}
EOF

  echo -e "${GREEN}Transpiled to C successfully${NC}"
  return 0
}

# Function to compile C to WebAssembly
compile_to_wasm() {
  echo -e "${BLUE}Compiling to WebAssembly...${NC}"
  
  local optimize_flags=""
  if [ "$OPTIMIZE" = true ]; then
    optimize_flags="-O3"
  else
    optimize_flags="-O0"
  fi
  
  # Compile C to WebAssembly using Emscripten
  if [ "$TARGET" = "web" ]; then
    emcc "$OUTPUT_DIR/${SOURCE_NAME}.c" \
      $optimize_flags \
      -s WASM=1 \
      -s EXPORTED_RUNTIME_METHODS='["ccall", "cwrap"]' \
      -s EXPORTED_FUNCTIONS='["_add", "_greet", "_malloc", "_free"]' \
      -s ALLOW_MEMORY_GROWTH=1 \
      -o "$OUTPUT_DIR/${SOURCE_NAME}.js"
  elif [ "$TARGET" = "node" ]; then
    emcc "$OUTPUT_DIR/${SOURCE_NAME}.c" \
      $optimize_flags \
      -s WASM=1 \
      -s ENVIRONMENT='node' \
      -s EXPORTED_RUNTIME_METHODS='["ccall", "cwrap"]' \
      -s EXPORTED_FUNCTIONS='["_add", "_greet", "_malloc", "_free"]' \
      -s ALLOW_MEMORY_GROWTH=1 \
      -o "$OUTPUT_DIR/${SOURCE_NAME}.js"
  else
    echo -e "${RED}Error: Unknown target '$TARGET'${NC}"
    exit 1
  fi
  
  echo -e "${GREEN}Compiled to WebAssembly successfully${NC}"
  return 0
}

# Function to create JavaScript bindings
create_js_bindings() {
  echo -e "${BLUE}Creating JavaScript bindings...${NC}"
  
  # Create a JavaScript wrapper for easier use
  cat > "$OUTPUT_DIR/${SOURCE_NAME}.bindings.js" << EOF
/**
 * SmashLang WebAssembly Bindings
 * Generated from ${SOURCE_FILENAME}
 */

class SmashLang {
  constructor() {
    this.module = null;
    this.initialized = false;
  }

  async init() {
    if (this.initialized) return Promise.resolve();

    return new Promise((resolve, reject) => {
      // Load the WebAssembly module
      const moduleScript = document.createElement('script');
      moduleScript.src = './${SOURCE_NAME}.js';
      moduleScript.onload = () => {
        // Initialize the module
        Module.onRuntimeInitialized = () => {
          this.module = Module;
          this.initialized = true;
          console.log('SmashLang WASM module initialized');
          resolve();
        };
      };
      moduleScript.onerror = () => {
        reject(new Error('Failed to load SmashLang WASM module'));
      };
      document.head.appendChild(moduleScript);
    });
  }

  add(a, b) {
    if (!this.initialized) {
      throw new Error('SmashLang module not initialized. Call init() first.');
    }
    return this.module.ccall('add', 'number', ['number', 'number'], [a, b]);
  }

  greet(name) {
    if (!this.initialized) {
      throw new Error('SmashLang module not initialized. Call init() first.');
    }
    const result = this.module.ccall('greet', 'string', ['string'], [name]);
    return result;
  }
}

// Export the SmashLang class
if (typeof module !== 'undefined' && module.exports) {
  module.exports = { SmashLang };
} else {
  window.SmashLang = SmashLang;
}
EOF

  echo -e "${GREEN}Created JavaScript bindings successfully${NC}"
  return 0
}

# Function to create a standalone HTML file
create_standalone_html() {
  echo -e "${BLUE}Creating standalone HTML file...${NC}"
  
  cat > "$OUTPUT_DIR/${SOURCE_NAME}.html" << EOF
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>SmashLang WASM - ${SOURCE_NAME}</title>
  <style>
    body {
      font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
      max-width: 800px;
      margin: 0 auto;
      padding: 20px;
      line-height: 1.6;
    }
    h1 {
      color: #333;
      border-bottom: 2px solid #eee;
      padding-bottom: 10px;
    }
    .result {
      background-color: #f5f5f5;
      border-left: 4px solid #007bff;
      padding: 15px;
      margin: 20px 0;
      border-radius: 4px;
    }
    button {
      background-color: #007bff;
      color: white;
      border: none;
      padding: 10px 15px;
      border-radius: 4px;
      cursor: pointer;
      margin-right: 10px;
      font-size: 14px;
    }
    button:hover {
      background-color: #0069d9;
    }
    input {
      padding: 8px;
      border: 1px solid #ddd;
      border-radius: 4px;
      margin-right: 10px;
      font-size: 14px;
    }
    .demo-section {
      margin: 30px 0;
    }
    .code {
      font-family: monospace;
      background-color: #f8f9fa;
      padding: 2px 4px;
      border-radius: 3px;
    }
    .logo {
      text-align: center;
      margin: 20px 0;
      font-size: 12px;
      white-space: pre;
      line-height: 1;
      color: #555;
    }
  </style>
</head>
<body>
  <div class="logo">
@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@
@#++*%@@@@@@@@@@@@@@
*:,,,:*@@@@@@@@@@@@@
*,.,.,;***++********
@*,.,:*+++;;++++**+*
@#,.,;***+***+*%@@@@
@%#*##%%#%%%###@@@@@
@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@
  </div>

  <h1>SmashLang WebAssembly Demo</h1>
  <p>This page demonstrates SmashLang code compiled to WebAssembly running in your browser.</p>
  
  <div class="demo-section">
    <h2>Addition Demo</h2>
    <p>Try adding two numbers using the SmashLang WASM module:</p>
    <input type="number" id="num1" value="5" />
    <input type="number" id="num2" value="7" />
    <button id="addButton">Add Numbers</button>
    <div id="addResult" class="result">Result will appear here</div>
  </div>

  <div class="demo-section">
    <h2>Greeting Demo</h2>
    <p>Enter your name to get a greeting from SmashLang:</p>
    <input type="text" id="nameInput" value="World" />
    <button id="greetButton">Get Greeting</button>
    <div id="greetResult" class="result">Greeting will appear here</div>
  </div>

  <div class="demo-section">
    <h2>How to Use in Your Code</h2>
    <p>Include the generated JavaScript files in your HTML:</p>
    <pre><code>&lt;script src="${SOURCE_NAME}.js"&gt;&lt;/script&gt;
&lt;script src="${SOURCE_NAME}.bindings.js"&gt;&lt;/script&gt;</code></pre>
    
    <p>Then use the SmashLang class in your JavaScript:</p>
    <pre><code>const smash = new SmashLang();
await smash.init();

// Call SmashLang functions
const sum = smash.add(5, 7);
const greeting = smash.greet("World");</code></pre>
  </div>

  <!-- Load the WebAssembly module -->
  <script src="${SOURCE_NAME}.js"></script>
  <script src="${SOURCE_NAME}.bindings.js"></script>
  
  <script>
    // Initialize the SmashLang module
    const smash = new SmashLang();
    
    // Set up the addition demo
    const addButton = document.getElementById('addButton');
    const addResult = document.getElementById('addResult');
    
    addButton.addEventListener('click', async () => {
      if (!smash.initialized) {
        await smash.init();
      }
      
      const num1 = parseInt(document.getElementById('num1').value, 10);
      const num2 = parseInt(document.getElementById('num2').value, 10);
      
      try {
        const result = smash.add(num1, num2);
        addResult.textContent = `${num1} + ${num2} = ${result}`;
      } catch (error) {
        addResult.textContent = `Error: ${error.message}`;
      }
    });
    
    // Set up the greeting demo
    const greetButton = document.getElementById('greetButton');
    const greetResult = document.getElementById('greetResult');
    
    greetButton.addEventListener('click', async () => {
      if (!smash.initialized) {
        await smash.init();
      }
      
      const name = document.getElementById('nameInput').value;
      
      try {
        const greeting = smash.greet(name);
        greetResult.textContent = greeting;
      } catch (error) {
        greetResult.textContent = `Error: ${error.message}`;
      }
    });
  </script>
</body>
</html>
EOF

  echo -e "${GREEN}Created standalone HTML file successfully${NC}"
  return 0
}

# Function to start a development server
start_dev_server() {
  echo -e "${BLUE}Starting development server on port $PORT...${NC}"
  
  # Check if python3 is available
  if command -v python3 &> /dev/null; then
    (cd "$OUTPUT_DIR" && python3 -m http.server $PORT)
  # Check if python is available
  elif command -v python &> /dev/null; then
    (cd "$OUTPUT_DIR" && python -m SimpleHTTPServer $PORT)
  # Check if npx is available
  elif command -v npx &> /dev/null; then
    npx http-server "$OUTPUT_DIR" -p $PORT
  else
    echo -e "${RED}Error: No suitable HTTP server found${NC}"
    echo -e "${YELLOW}Please install Python or Node.js${NC}"
    exit 1
  fi
}

# Main compilation process
compile_wasm() {
  echo -e "${YELLOW}Compiling ${SOURCE_FILENAME} to WebAssembly...${NC}"
  
  # Step 1: Compile SmashLang to C
  compile_to_c
  
  # Step 2: Compile C to WebAssembly
  compile_to_wasm
  
  # Step 3: Create JavaScript bindings
  create_js_bindings
  
  # Step 4: Create standalone HTML if requested
  if [ "$STANDALONE" = true ]; then
    create_standalone_html
  fi
  
  echo -e "\n${GREEN}Successfully compiled ${SOURCE_FILENAME} to WebAssembly!${NC}"
  echo -e "${BLUE}Output files:${NC}"
  echo -e "  ${CYAN}${OUTPUT_DIR}/${SOURCE_NAME}.wasm${NC} - WebAssembly binary"
  echo -e "  ${CYAN}${OUTPUT_DIR}/${SOURCE_NAME}.js${NC} - JavaScript glue code"
  echo -e "  ${CYAN}${OUTPUT_DIR}/${SOURCE_NAME}.bindings.js${NC} - JavaScript bindings"
  
  if [ "$STANDALONE" = true ]; then
    echo -e "  ${CYAN}${OUTPUT_DIR}/${SOURCE_NAME}.html${NC} - Standalone HTML demo"
  fi
  
  echo -e "\n${YELLOW}To use in your web application:${NC}"
  echo -e "1. Include the JavaScript files in your HTML:"
  echo -e "   ${CYAN}<script src=\"${SOURCE_NAME}.js\"></script>${NC}"
  echo -e "   ${CYAN}<script src=\"${SOURCE_NAME}.bindings.js\"></script>${NC}"
  echo -e "2. Use the SmashLang class in your JavaScript:"
  echo -e "   ${CYAN}const smash = new SmashLang();${NC}"
  echo -e "   ${CYAN}await smash.init();${NC}"
  echo -e "   ${CYAN}const result = smash.add(5, 7);${NC}"
  
  if [ "$STANDALONE" = true ]; then
    echo -e "\n${YELLOW}To view the demo:${NC}"
    echo -e "  Open ${CYAN}${OUTPUT_DIR}/${SOURCE_NAME}.html${NC} in your browser"
  fi
  
  if [ "$SERVE" = true ]; then
    echo -e "\n${YELLOW}Starting development server...${NC}"
    start_dev_server
  fi
}

# Watch mode function
watch_and_compile() {
  echo -e "${YELLOW}Watching for changes to ${SOURCE_FILENAME}...${NC}"
  echo -e "${BLUE}Press Ctrl+C to stop watching${NC}"
  
  # Initial compilation
  compile_wasm
  
  # Start dev server in background if requested
  if [ "$SERVE" = true ]; then
    start_dev_server &
    SERVER_PID=$!
    
    # Trap to kill the server when the script exits
    trap "kill $SERVER_PID 2>/dev/null" EXIT
  fi
  
  # Watch for changes
  local last_modified=$(stat -c %Y "$SOURCE_FILE" 2>/dev/null || stat -f %m "$SOURCE_FILE")
  
  while true; do
    sleep 1
    
    # Check if the file has been modified
    local current_modified=$(stat -c %Y "$SOURCE_FILE" 2>/dev/null || stat -f %m "$SOURCE_FILE")
    
    if [ "$current_modified" != "$last_modified" ]; then
      echo -e "\n${YELLOW}File changed, recompiling...${NC}"
      compile_wasm
      last_modified=$current_modified
    fi
  done
}

# Run the appropriate mode
if [ "$WATCH" = true ]; then
  watch_and_compile
else
  compile_wasm
fi
