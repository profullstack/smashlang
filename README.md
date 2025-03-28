<p align="center">
  <img src="assets/logo.png" alt="SmashLang logo" width="400" />
</p>

# SmashLang

**SmashLang** is a bold, high-performance, JavaScript-inspired general-purpose programming language that compiles to native binaries. With strong syntax, a modern standard library, PCRE regex support, REPL, and built-in modules, SmashLang is made for developers who want the power of C/Rust but the clarity of JavaScript — without the bloat.

---

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

Install packages with:

```bash
smashpkg install std:math
```

Installs to `smash_modules/` and available via `import`.

---

## 🔧 Tooling

- `smashc` — CLI compiler
- `smash repl` — interactive shell
- `smash-lang-server` — LSP integration
- `smashpkg` — package manager

---

## 🖤 Logo

The SmashLang logo represents resistance, speed, and clarity. The raised fist reflects a new era in programming — strong, expressive, and free.

---

## License

MIT © 2025 SmashLang Team
