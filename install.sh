#!/usr/bin/env bash

set -e

# Detect OS
OS="$(uname -s)"

# Build the binary
echo "Building ls_plus..."
cargo build --release

BIN_PATH="target/release/ls_plus"

if [ ! -f "$BIN_PATH" ]; then
    echo "Build failed: $BIN_PATH not found."
    exit 1
fi

# Install location
INSTALL_DIR="$HOME/bin"
mkdir -p "$INSTALL_DIR"
cp "$BIN_PATH" "$INSTALL_DIR/"

# Add to PATH if not present
if ! echo "$PATH" | grep -q "$INSTALL_DIR"; then
    SHELL_RC="$HOME/.bashrc"
    if [ -n "$ZSH_VERSION" ]; then
        SHELL_RC="$HOME/.zshrc"
    fi
    echo "export PATH=\"\$HOME/bin:\$PATH\"" >> "$SHELL_RC"
    echo "Added \$HOME/bin to PATH in $SHELL_RC. Please restart your shell."
fi

echo "ls_plus installed to $INSTALL_DIR."
echo "You can now run 'ls_plus'."
echo "You can alias ls to point to ls_plus for easy use using 'alias ls='ls_plus''"
