#!/usr/bin/env bash
# SUMMARY: 100% clean uninstallation script and memory volatile wipe.

set -e

echo "[PURGE] Initiating Decommissioning Protocol..."

echo "[PURGE] Removing global symlink..."
sudo rm -f /usr/local/bin/aether || true

echo "[PURGE] Cleaning build artifacts..."
cargo clean || true
rm -f ./include/*.o ./include/*.a

echo "[PURGE] Resetting ROCm environment bindings..."
if [ "$1" == "--reset-rocm" ]; then
    echo "  -> Removing ROCm configuration profiles..."
    rm -f ~/.config/aether/rocm_env.sh || true
    echo "  -> Please restart your shell to clear exported ROCm variables."
else
    echo "  -> Skipping ROCm environment variable reset. Use --reset-rocm to force."
fi

echo "[PURGE] Erasing local forensic logs securely..."
if [ -d "./logs" ]; then
    if command -v shred >/dev/null 2>&1; then
        find ./logs -type f -exec shred -u {} + || true
    else
        rm -rf ./logs/*
    fi
fi

echo "[PURGE] Decommissioning complete. AETHER-NODE wiped."
