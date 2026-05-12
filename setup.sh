#!/usr/bin/env bash
# SUMMARY: Universal idempotent dependency provisioner for AETHER-NODE.

set -euo pipefail

if [ "$EUID" -ne 0 ]; then
  echo "Error: Provisioning requires root privileges."
  exit 1
fi

detect_pkg_manager() {
    if command -v apt-get >/dev/null 2>&1; then echo "apt-get";
    elif command -v dnf >/dev/null 2>&1; then echo "dnf";
    elif command -v pacman >/dev/null 2>&1; then echo "pacman";
    else echo "unknown"; fi
}

PKG_MGR=$(detect_pkg_manager)

case "$PKG_MGR" in
    apt-get)
        apt-get update -qq
        apt-get install -y -qq clang rustc cargo libpcap-dev make rocm-hip-sdk
        ;;
    dnf)
        dnf install -y clang rust cargo libpcap-devel make rocm-hip-sdk
        ;;
    pacman)
        pacman -Sy --noconfirm clang rust cargo libpcap make hip-runtime-amd
        ;;
    *)
        echo "Error: Unsupported package manager. Install clang, rustc, cargo, libpcap, make, and HIP manually."
        exit 1
        ;;
esac

echo "Provisioning complete."
