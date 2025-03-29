#!/bin/bash

# Get the current git hash
GIT_HASH=$(git rev-parse --short HEAD)
echo "Git hash: $GIT_HASH"

# Create the git_hash.txt file in the installation directory
echo "$GIT_HASH" > /home/ettinger/.local/bin/git_hash.txt
echo "Git hash file created at /home/ettinger/.local/bin/git_hash.txt"

echo "Now let's patch the main.rs file to read the git hash"

# Create a temporary directory
TEMP_DIR=$(mktemp -d)
echo "Created temporary directory: $TEMP_DIR"

# Clone the repository
echo "Cloning repository..."
git clone --depth 1 https://github.com/profullstack/smashlang "$TEMP_DIR"
cd "$TEMP_DIR"

# Modify the main.rs file
echo "Modifying main.rs..."
sed -i '/"--version" | "-v" => {/,/return;/c\
        "--version" | "-v" => {\
            // Try to read git hash if available\
            let git_hash = std::fs::read_to_string("git_hash.txt").ok();\
            \
            if let Some(hash) = git_hash {\
                let hash = hash.trim();\
                if !hash.is_empty() {\
                    println!("SmashLang version {} (git: {})", VERSION, hash);\
                    return;\
                }\
            }\
            println!("SmashLang version {}", VERSION);\
            return;\
        },' src/main.rs

# Build the modified version
echo "Building modified version..."
cargo build --release

# Copy the new binary to replace the old one
echo "Installing modified binary..."
cp target/release/smash /home/ettinger/.local/bin/

echo "Done! Try running 'smash --version' now."
