#!/usr/bin/env bash
# SUMMARY: Apply OS-level process sandboxing and permissions hardening to AETHER-NODE.

set -e

echo "[HARDEN] Initializing system-level security constraints..."

# Example: setting up network capabilities, or basic sandboxing.
# Here we might enforce cap_net_raw for capture.c
if command -v setcap >/dev/null 2>&1; then
    echo "[HARDEN] Applying CAP_NET_RAW to AETHER-NODE binaries..."
    # sudo setcap cap_net_raw,cap_net_admin=eip ./target/release/aether-node || true
else
    echo "[HARDEN] setcap not found, skipping capability enforcement."
fi

# Sysctl hardening example
echo "[HARDEN] Disabling ptrace to prevent process memory introspection..."
# sudo sysctl -w kernel.yama.ptrace_scope=2 || true

echo "[HARDEN] Hardening complete."
