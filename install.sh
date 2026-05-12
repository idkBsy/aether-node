#!/usr/bin/env bash
# SUMMARY: Compiles AETHER-NODE and registers the global 'aether' command.

set -euo pipefail

if [ "$EUID" -ne 0 ]; then
  echo "Error: Installation requires root privileges for /usr/local/bin."
  exit 1
fi

# Build release
echo "Building AETHER-NODE release..."
make build

# Install
INSTALL_DIR="/usr/local/bin"
TARGET_BIN="./target/release/aether-node"

if [ -f "$TARGET_BIN" ]; then
    cp "$TARGET_BIN" "$INSTALL_DIR/aether"
    chmod 755 "$INSTALL_DIR/aether"
    echo "Successfully installed to $INSTALL_DIR/aether"
else
    echo "Error: Build artifact not found at $TARGET_BIN"
    exit 1
fi
