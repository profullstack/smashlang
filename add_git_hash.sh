#!/bin/bash

# Get the current git hash
GIT_HASH=$(git rev-parse --short HEAD)
echo "Git hash: $GIT_HASH"

# Create the git_hash.txt file in the installation directory
echo "$GIT_HASH" > /home/ettinger/.local/bin/git_hash.txt

# Make a simple patch to main.rs to read the git hash
cat > /tmp/version_patch.rs << 'EOF'
        "--version" | "-v" => {
            // Try to read git hash if available
            let git_hash = std::fs::read_to_string(std::env::current_exe().ok().and_then(|p| p.parent().map(|d| d.join("git_hash.txt"))).unwrap_or("git_hash.txt".into())).ok();
            
            if let Some(hash) = git_hash {
                let hash = hash.trim();
                if !hash.is_empty() {
                    println!("SmashLang version {} (git: {})", VERSION, hash);
                    return;
                }
            }
            println!("SmashLang version {}", VERSION);
            return;
        },
EOF

# Create a temporary directory for building
TEMP_DIR=$(mktemp -d)
cd "$TEMP_DIR"

# Clone the repository
git clone --depth 1 https://github.com/profullstack/smashlang .

# Apply the patch
sed -i '/"--version" | "-v" => {/,/return;/c\'
