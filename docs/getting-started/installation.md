# SmashLang Installation Guide

This guide will walk you through installing SmashLang on various platforms. SmashLang provides a cross-platform installer script that makes the installation process simple and consistent across operating systems.

## Quick Installation

The fastest way to install SmashLang is using our installer script:

```bash
# Using curl
curl -fsSL https://raw.githubusercontent.com/profullstack/smashlang/master/install.sh | bash -s -- --master

# Or using wget
wget -O- https://raw.githubusercontent.com/profullstack/smashlang/master/install.sh | bash -s -- --master
```

> **Note:** The `--master` option installs directly from the GitHub master branch instead of using release packages. This is recommended until official releases are available.

The installer script will:
- Detect your operating system (Windows, macOS, or Linux)
- Download the appropriate binaries
- Set up the package repository
- Configure your environment
- Add SmashLang to your PATH

After installation, verify it works:

```bash
smash --version
```

## Platform-Specific Installation

### Linux

#### Requirements
- A modern Linux distribution (Ubuntu 20.04+, Fedora 34+, etc.)
- curl or wget
- tar and unzip utilities
- GCC or Clang

#### Manual Installation

If you prefer to install manually:

1. Download the latest Linux tarball from the releases page
2. Extract it to a directory of your choice
3. Add the `bin` directory to your PATH

```bash
tar -xzf smashlang-linux-x64.tar.gz -C /usr/local
echo 'export PATH="/usr/local/smashlang/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### macOS

#### Requirements
- macOS 10.15 (Catalina) or later
- Command Line Tools for Xcode

#### Manual Installation

1. Download the latest macOS tarball from the releases page
2. Extract it to a directory of your choice
3. Add the `bin` directory to your PATH

```bash
tar -xzf smashlang-macos.tar.gz -C /usr/local
echo 'export PATH="/usr/local/smashlang/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

### Windows

#### Requirements
- Windows 10 or later
- Git Bash, WSL, or PowerShell

#### Manual Installation

1. Download the latest Windows zip from the releases page
2. Extract it to a directory of your choice (e.g., `C:\Program Files\SmashLang`)
3. Add the `bin` directory to your PATH

Using PowerShell (run as Administrator):

```powershell
$env:Path += ";C:\Program Files\SmashLang\bin"
[Environment]::SetEnvironmentVariable("Path", $env:Path, [EnvironmentVariableTarget]::Machine)
```

## Building from Source

If you want to build SmashLang from source:

1. Clone the repository
2. Install Rust and Cargo
3. Build the project

```bash
git clone https://github.com/profullstack/smashlang.git
cd smashlang
cargo build --release
```

The compiled binaries will be in the `target/release` directory.

## Troubleshooting

### Common Issues

#### Permission Denied

If you get a "Permission denied" error when running the installer:

```bash
sudo curl -fsSL https://raw.githubusercontent.com/profullstack/smashlang/master/install.sh | sudo bash -s -- --master
```

#### Missing Dependencies

If you're missing dependencies, install them using your package manager:

```bash
# Ubuntu/Debian
sudo apt-get install build-essential curl

# Fedora/RHEL
sudo dnf install gcc gcc-c++ curl

# macOS
xcode-select --install
```

#### PATH Issues

If `smash` is not found after installation, make sure the installation directory is in your PATH:

```bash
echo $PATH
```

If it's not, add it manually to your shell profile file (`.bashrc`, `.zshrc`, etc.).

## Next Steps

Now that you have SmashLang installed, check out the [Language Basics](./language-basics.md) guide to start learning how to use it, or try running the [REPL](./repl.md) to experiment with the language interactively.
