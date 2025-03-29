<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="assets/logo.dark.svg">
    <img src="assets/logo.light.svg" alt="SmashLang logo" width="400" />
  </picture>
</p>

# SmashLang

**SmashLang** is a bold, high-performance, JavaScript-inspired general-purpose programming language that compiles to native binaries. With strong syntax, a modern standard library, PCRE regex support, REPL, and built-in modules, SmashLang is made for developers who want the power of C/Rust but the clarity of JavaScript — without the bloat.

---

[![GitHub](https://img.shields.io/github/license/profullstack/smashlang)](https://github.com/profullstack/smashlang/blob/master/LICENSE)
[![GitHub commit activity](https://img.shields.io/github/commit-activity/m/profullstack/smashlang)](https://github.com/profullstack/smashlang/pulse)
[![GitHub last commit](https://img.shields.io/github/last-commit/profullstack/smashlang)](https://github.com/profullstack/smashlang/commits/master)


## ✨ Features

- 🔥 JavaScript-inspired syntax with modern improvements
- 💥 First-class support for arrays, objects, strings, regex, dates
- 📦 Module system and `smashpkg` package manager
- 🧠 Pattern matching and function expression sugar
- 🚀 Compiles to native binaries (via LLVM + Clang)
- 💬 REPL and full CLI compiler (`smashc`)
- 🛠️ Language Server Protocol (LSP) support
- 💪 Written in Rust with an embedded runtime (`libsmashrt`)

---

## 📦 Use Cases

- **CLI tools** with native speed
- **Data processing pipelines**
- **Cross-platform scripting**
- **WebAssembly** (WASM) targets in future versions
- **Educational tools** with readable syntax and REPL
- **Regex-heavy parsing applications**

---

## 🖥️ Operating System Support

SmashLang compiles to native binaries for:

- ✅ Linux (x64, ARM)
- ✅ macOS (Intel & Apple Silicon)
- ✅ Windows (via MinGW)
- ✅ Android (NDK/Clang)
- ✅ iOS (Xcode SDK)

---

## 🚀 Getting Started

### Quick Installation

Install SmashLang with a single command:

```bash
# Using curl
curl -fsSL https://raw.githubusercontent.com/profullstack/smashlang/master/install.sh | bash -s -- --master

# Or using wget
wget -O- https://raw.githubusercontent.com/profullstack/smashlang/master/install.sh | bash -s -- --master
```

> **Note:** The `--master` option installs directly from the GitHub master branch instead of using release packages. This is recommended until official releases are available.

This will automatically:
- Detect your operating system (Windows, macOS, or Linux)
- Download the appropriate binaries
- Set up the package repository
- Configure your environment
- Add SmashLang to your PATH

After installation, verify it works:

```bash
smash --version
```

### Build the Compiler & Runtime

```bash
./build.sh example.smash
```

This compiles:
- `std.smash` (standard library)
- `libsmashrt` runtime (Rust)
- Your `.smash` file to an executable

Use `--target` to cross-compile:
```bash
./build.sh hello.smash --target x86_64-w64-windows-gnu
```

### Run the REPL

```bash
smash repl
```

### Example Program

```js
const name = "SmashLang";
let nums = [1, 2, 3];
let doubled = nums.map(fn(x) => x * 2);
print("Hello from " + name);
```

---

## 🌐 HTTP/HTTPS API

SmashLang includes a fetch-compatible API for making HTTP requests with both Promise-based and async/await approaches:

### Promise-based API

```js
// Simple GET request
fetch("https://api.example.com/data")
    .then(fn(response) {
        return response.json();
    })
    .then(fn(data) {
        print("Received data:", data);
    })
    .catch(fn(error) {
        print("Error fetching data:", error);
    });

// POST request with JSON body
post("https://api.example.com/submit", {
    name: "SmashLang",
    version: "1.0.0",
    features: ["native compilation", "JavaScript-like syntax"]
}, {
    headers: {
        "Content-Type": "application/json",
        "Accept": "application/json"
    }
});
```

### Async/Await API

```js
// Top-level async function
async fn fetchData() {
    try {
        // Simple GET request with async/await
        const response = await fetch("https://api.example.com/data");
        const data = await response.json();
        print("Received data:", data);
        
        // POST request with async/await
        const postResponse = await post("https://api.example.com/submit", {
            name: "SmashLang",
            version: "1.0.0",
            features: ["native compilation", "async/await"]
        }, {
            headers: {
                "Content-Type": "application/json"
            }
        });
        const result = await postResponse.json();
        print("Response:", result);
    } catch (error) {
        print("Error:", error);
    }
}

// Run the async function
fetchData();
```

---

## 🔌 Networking

Low-level TCP/IP and UDP networking capabilities with both traditional and async/await approaches:

### Traditional API

```js
// Import the networking module
import "std/net";

// TCP Client example
let client = createTcpClient();
let conn = client.connect("example.com", 80);
client.send(conn, "GET / HTTP/1.1\r\nHost: example.com\r\n\r\n");
let response = client.receive(conn);
print(response);
client.close(conn);

// TCP Server example
let server = createTcpServer();
let s = server.bind("127.0.0.1", 8080);
server.listen(s);

let client = server.accept(s);
let data = createTcpClient().receive(client);
createTcpClient().send(client, "HTTP/1.1 200 OK\r\n\r\nHello!");
createTcpClient().close(client);
server.close(s);

// UDP example
let socket = createUdpSocket();
let udpSocket = socket.bind("127.0.0.1", 8081);
socket.sendTo(udpSocket, "Hello UDP", "127.0.0.1", 8081);
let [data, sender] = socket.receiveFrom(udpSocket);
print(data + " from " + sender.address + ":" + sender.port);
socket.close(udpSocket);
```

### Async/Await API

```js
// Import the networking module
import "std/net";

// Async TCP client example
async fn fetchWebPage(host, port, path) {
    try {
        const client = createTcpClient();
        const conn = await client.connect(host, port);
        
        await client.send(conn, `GET ${path} HTTP/1.1\r\nHost: ${host}\r\n\r\n`);
        const response = await client.receive(conn);
        
        await client.close(conn);
        return response;
    } catch (error) {
        print("Network error:", error);
        throw error;
    }
}

// Async TCP server example
async fn startServer(address, port) {
    const server = createTcpServer();
    
    try {
        const s = await server.bind(address, port);
        await server.listen(s);
        print(`Server listening on ${address}:${port}`);
        
        // Accept and handle a connection
        const client = await server.accept(s);
        const data = await createTcpClient().receive(client);
        await createTcpClient().send(client, "HTTP/1.1 200 OK\r\n\r\nHello!");
        await createTcpClient().close(client);
        await server.close(s);
    } catch (error) {
        print("Server error:", error);
    }
}

// Async UDP example
async fn sendAndReceiveUdp() {
    const socket = createUdpSocket();
    
    try {
        const udpSocket = await socket.bind("127.0.0.1", 8081);
        
        await socket.sendTo(udpSocket, "Hello UDP", "127.0.0.1", 8081);
        const [data, sender] = await socket.receiveFrom(udpSocket);
        print(`${data} from ${sender.address}:${sender.port}`);
        
        await socket.close(udpSocket);
    } catch (error) {
        print("UDP error:", error);
    }
}

// Run the async functions
fetchWebPage("example.com", 80, "/").then(response => print(response));
startServer("127.0.0.1", 8080);
sendAndReceiveUdp();
```

---

## 🧪 Pattern Matching

```js
match age {
  0 => "newborn",
  1 => "baby",
  _ => "child"
}
```

---

## 📦 Package Manager

SmashLang uses a Homebrew-inspired package management system with all packages contained within the `smashlang_packages` directory in the main codebase.

### Installing Packages

```bash
# Install a package
smashpkg install math

# Install a specific version
smashpkg install sqlite@3.36.0

# Install multiple packages
smashpkg install math crypto json
```

Packages are installed to `smash_modules/` and available via `import`.

```js
// Import a package
import "math";

// Use the package
let result = math.sin(0.5) + math.cos(0.5);
```

### Package Directory Structure

All SmashLang packages are organized in the `smashlang_packages` directory:

```
smashlang_packages/
├u2500u2500 core/          # Essential libraries maintained by the SmashLang team
├u2500u2500 networking/    # Libraries for HTTP, WebSockets, and other network protocols
├u2500u2500 database/      # Database drivers and ORM tools
├u2500u2500 community/     # Third-party packages contributed by the community
```

This approach keeps the codebase self-contained and ensures that even with a million packages, the download size remains manageable.

### Creating Packages

SmashLang provides a convenient way to create new packages with the proper structure:

```bash
# Create a simple package
smashpkg create my-package

# Create a package in a category
smashpkg create utils/array-utils
```

This will create a new package in the `./smashlang_packages` directory with the following structure:

```
smashlang_packages/my-package/
├── package.json         # Package metadata
├── README.md            # Documentation
├── .gitignore           # Git ignore rules
├── src/
│   └── index.smash      # Main package code
├── examples/
│   └── example.smash    # Usage examples
└── tests/
    └── index.test.smash # Test cases
```

### Contributing Packages

To contribute a new package:

1. Create a new package using `smashpkg create <name>`
2. Develop and test your package locally
3. Test your formula using `smashpkg test <formula>`
4. Submit a pull request

See the [smashlang_packages/README.md](./smashlang_packages/README.md) for more details.

---

## 🧪 Testing Framework

SmashLang includes a built-in testing framework inspired by Jest and Mocha, making it easy to write and run tests for your code.

### Writing Tests

```javascript
// Import the testing framework
import { test, describe, expect, beforeEach, afterEach } from 'std/testing';

// Simple test
test('addition works correctly', () => {
  expect(2 + 2).toBe(4);
});

// Grouped tests with setup and teardown
describe('String operations', () => {
  let testString;
  
  beforeEach(() => {
    testString = 'SmashLang';
  });
  
  test('string length is correct', () => {
    expect(testString.length).toBe(9);
  });
  
  test('string includes method works', () => {
    expect(testString.includes('Smash')).toBeTrue();
  });
});
```

### Running Tests

Use the `smashtest` command to run your tests:

```bash
# Run all tests in a directory
smashtest ./tests

# Run a specific test file
smashtest ./tests/unit.test.smash

# Run tests with a specific tag
smashtest ./tests --tag=unit
```

### Testing Packages

When you create a package with `smashpkg create`, it automatically includes test files that work with the testing framework.

```bash
smashtest ./smashlang_packages/my-package/tests
```

## 🔧 Tooling

- `smashc` — CLI compiler
- `smash repl` — interactive shell
- `smash-lang-server` — LSP integration
- `smashpkg` — package manager
- `smashtest` — test runner

---

## 🖤 Logo

The SmashLang logo represents resistance, speed, and clarity. The raised fist reflects a new era in programming — strong, expressive, and free. We are also inspired by Bernie Sanders and his relentless fight for the people.

---

## Contact
For technical contributions or questions: opensource@smashlang.com

[![Discord](https://img.shields.io/discord/1011308539819597844?label=Discord&logo=Discord&style=for-the-badge)](https://discord.gg/U7dEXfBA3s)
[![Reddit](https://img.shields.io/badge/Reddit-FF4500?logo=reddit&logoColor=fff&style=for-the-badge)](https://www.reddit.com/r/smashlang/)

## License

ISC © 2025 SmashLang.com