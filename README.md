<div align="center">
  <h1>AETHER-NODE</h1>
  <p><strong>Hardware OSINT & Silicon Fingerprinting Engine</strong></p>
  <p><i>An experimental project by <b>idkBsy</b>.</i></p>
</div>

---

## 📌 Overview

**AETHER-NODE** is an advanced, highly experimental, low-level telemetry engine designed for autonomous hardware attribution and **Silicon Fingerprinting**. 

By exploiting microscopic manufacturing variances in semiconductors (such as GPU memory controllers and Network Interface Cards), this system attempts to reliably track and identify individual physical machines across networks. This process bypasses standard software-level obfuscation, MAC randomization, and VPN tunneling, identifying the machine based on its physical properties.

> **Note from the Author (idkBsy):** 
> *This is an experimental project I developed to push the boundaries of low-level hardware interactions, Rust memory safety, and C/HIP FFI bindings. I highly encourage the community to test this system. If you experience crashes, bugs, errors, or if you have ideas for improvement, please open an issue! I am actively looking for reviews and feedback from UNIX/Linux users.*

---

## 🧠 Architecture & Algorithmic Logic

The system is engineered to be 100% stable, modular, and extremely fast. It is structured into three primary layers:

1. **Synapse (C / ASM / FFI)**
   - **Role:** Packet Ingestion.
   - **Logic:** Uses `libpcap` to capture raw packets in promiscuous mode with zero-copy intent. It tags each incoming packet with sub-nanosecond timestamp resolution via raw assembly `RDTSC` (Read Time-Stamp Counter) instructions, avoiding OS-level context switch delays.

2. **Nucleus (HIP / ROCm / Rust Fallback)**
   - **Role:** Mathematical Analysis & Skew Estimation.
   - **Logic:** Computes **Clock-Skew ($\Delta$)** using massively parallel GPU compute kernels. Because no two quartz oscillators are manufactured identically, their speeds drift at a microscopic, predictable rate. We use **Ordinary Least Squares (OLS) Regression** to find the exact slope of this time-drift against the host CPU. We then apply **Shannon Entropy** to stabilize the variance, ensuring the signature is a genuine hardware anomaly, not thermal noise.
   - *Note: If a compatible AMD GPU / ROCm environment is not found, the system dynamically falls back to a simulated calculation, ensuring a **zero-crash** guarantee.*

3. **Cortex (Rust)**
   - **Role:** The Brain & TUI Command Center.
   - **Logic:** An asynchronous, memory-safe control plane built on `tokio`. It processes the bayesian attribution logic, manages process sandboxing (`libc::prctl`), handles the `SIGINT` panic protocol (securely wiping volatile caches via `std::ptr::write_volatile`), and orchestrates the elegant Terminal User Interface (TUI).

---

## ⚙️ Installation Guide

This system requires a Linux/UNIX-like environment. Root privileges are necessary due to promiscuous network card interactions and system-level sandboxing.

### 1. Provision Dependencies
A script is provided to automatically detect your package manager (apt, dnf, pacman) and install the necessary compilers (Rust, Clang, Make, libpcap-dev).
```bash
chmod +x setup.sh install.sh scripts/*.sh
sudo ./setup.sh
```

### 2. Compile & Install
This will build the C components, optionally link the HIP kernel (if ROCm is installed), compile the Rust application, embed the dynamic library paths (`rpath`), and link the binary globally.
```bash
sudo ./install.sh
```

---

## 🚀 Usage & Commands

Once installed, the `aether` binary is available globally. Due to the low-level hardware and NIC access required, you must run these commands with `sudo`.

| Command | Description |
|---|---|
| `sudo aether monitor` | **[RECOMMENDED]** Launches the beautiful, real-time Terminal UI (TUI) dashboard to monitor entropy, telemetry, and live attributions. Press `q` to exit. |
| `sudo aether pulse --interface <iface>` | Starts the hardware pulse ingestion on the specified interface (e.g., `eth0` or `wlp3s0`). |
| `sudo aether trace` | Executes a deep, autonomous OSINT trace, calculating the silicon fingerprint and checking for a Bayesian lock. |
| `aether status` | Queries local ROCm/GPU hardware status. |

---

## 🛡️ Security & Stability Guarantees

As a Linux/UNIX power user, your system's stability is paramount. AETHER-NODE is built with the following guarantees:
- **Zero-Crash Architecture:** Weak linking (`dlsym`) ensures that missing GPU toolchains gracefully degrade to CPU fallbacks rather than crashing the linker or the runtime.
- **Seccomp Sandboxing:** Enforces strict execution boundaries at the kernel level using `prctl`.
- **Anti-Forensic Panic Protocol:** If the process is forcefully terminated (e.g., via `Ctrl+C`), a hook securely overwrites volatile memory buffers before the process exits.

---

## 🗑️ Complete Decommissioning (Uninstallation)

I have provided a specialized script to completely erase AETHER-NODE from your system. It removes the global binary, wipes compiler artifacts, and securely shreds local forensic logs.

```bash
# Standard purge
sudo ./scripts/purge.sh

# Complete purge (includes resetting any modified ROCm environment bindings)
sudo ./scripts/purge.sh --reset-rocm
```

---

## ⚖️ Legal & Ethical Disclaimer

**AETHER-NODE is an experimental dual-use tool.** The capacity to uniquely identify hardware across networks is strictly regulated under global privacy laws (including GDPR and CCPA).

- This software is provided **strictly for authorized cybersecurity research, industrial network auditing, and explicit penetration testing**.
- Any deployment on networks or hardware without the explicit, written consent of the owner is prohibited.
- The author (**idkBsy**) assumes no liability for misuse, unauthorized attribution, or associated damages. **Use responsibly.**
