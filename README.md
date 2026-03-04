# <p align="center">🍱 bento-fetch</p>
<p align="center">
  <img src="https://img.shields.io/badge/Language-Rust-orange.svg" alt="Language">
  <img src="https://img.shields.io/badge/Speed-%3C%2025ms-brightgreen.svg" alt="Speed">
  <img src="https://img.shields.io/badge/Platform-Windows%20%7C%20Linux%20%7C%20macOS-blue.svg" alt="Platform">
  <img src="https://img.shields.io/badge/Style-Modern%20Bento-magenta.svg" alt="Style">
</p>



**bento-fetch** is a high-performance, cross-platform, designer-centric system information tool. While traditional fetch tools prioritize a "logo-on-the-left" approach, **bento-fetch** treats your terminal like a high-end dashboard, organizing your system's soul into a clean, balanced **Bento Box grid**.

## ⚡ Performance Leaderboard
In the world of CLI tools, speed is king. **bento-fetch** is engineered to be nearly instantaneous by utilizing parallel background threads to hide the latency of hardware syscalls on both Windows and Linux.

| Tool | Avg. Execution Time | Implementation |
| :--- | :--- | :--- |
| `neofetch` | ~350ms | Bash (Process heavy) |
| `fastfetch` | ~20ms | C (Highly optimized) |
| **`bento-fetch`** | **~3.5ms - 25ms** | **Rust (Multithreaded / Parallel Probing)** |

---

## ✨ Key Features

* **Cross-Platform:** Native support for Windows (WMI/SMI), Linux (lspci/SMI), and macOS. 
* **Bento Grid Layout:** A 6x6 grid that separates hardware vitals from software environments for maximum scannability.
* **Dynamic UX 🚦:** Box borders dynamically change color based on usage:
    * 🟢 **Green:** Safe (Usage < 60%)
    * 🟡 **Yellow:** Moderate (Usage 60% - 85%)
    * 🔴 **Red:** Critical (Usage > 85%)
* **Compile-Time Assets:** Over 400+ ASCII logos are baked directly into the binary at compile-time—zero runtime disk I/O.
* **Smart Metrics:** Automatically switches between `LOAD` average on Linux and global `CPU%` utilization on Windows.

---

## 📸 Component Breakdown

### Top Row: The Engine (Hardware)
* **CPU:** Processor name and real-time usage color.
* **LOAD / CPU%:** 1-minute load average (Linux) or Total CPU Utilization (Windows).
* **RAM:** Used vs. Total capacity.
* **GPU:** Detected Graphics Card name.
* **GPU%:** Real-time VRAM/Utilization.
* **DISK:** Aggregated usage across all mounted drives.

### Bottom Row: The Environment (Software)
* **OS:** Distribution name (Windows 11, Arch, Debian, etc.).
* **KERNEL:** System kernel version or Windows NT Build Number.
* **UPTIME:** Formatted system uptime.
* **SHELL:** Your active shell environment (fish, zsh, pwsh).
* **TERM:** Current terminal emulator.
* **IP:** Primary local network IPv4 address.

---

## 🛠 Installation & Setup

### Prerequisites (All Platforms)
* **Rust Toolchain:** [Install Rust](https://rustup.rs/)
* **Nerd Fonts:** Required for glyph rendering (e.g., [JetBrainsMono](https://www.nerdfonts.com/font-downloads))

### 🐧 Linux Specifics
Ensure you have the PCI hardware database installed (usually standard):
```bash
sudo pacman -S hwdata  # Arch/CachyOS
sudo apt install pciutils # Debian/Ubuntu
🪟 Windows Specifics
Before installing Rust, you must install the Microsoft Visual Studio C++ Build Tools.

Download the Visual Studio Installer.

Select the "Desktop development with C++" workload.

Install and restart your terminal.

🚀 Building from Source
Bash
# 1. Clone the repository
git clone [https://github.com/yourusername/bento-fetch.git](https://github.com/yourusername/bento-fetch.git)
cd bento-fetch

# 2. Download the ASCII library (if not already present)
git clone --depth 1 [https://github.com/fastfetch-cli/fastfetch.git](https://github.com/fastfetch-cli/fastfetch.git) temp_repo
mv temp_repo/src/logo/ascii ./
rm -rf temp_repo

# 3. Build the heavily optimized binary
cargo build --release
To Install Globally:

Linux: sudo cp target/release/bento-fetch /usr/local/bin/

Windows: Move target\release\bento-fetch.exe to a permanent folder and add it to your System PATH, or call the absolute path in your PowerShell profile.

🖥️ Shell Integration
bento-fetch is designed to work out of the box with zero configuration files. Add it to your shell startup to see it every time you open a terminal!

Linux (Fish):

Code snippet
# ~/.config/fish/config.fish
function fish_greeting
    bento-fetch
end
Windows (PowerShell):

PowerShell
# Open profile with: notepad $PROFILE
# Add these lines:
[console]::OutputEncoding = [System.Text.Encoding]::UTF8
& "C:\path\to\your\bento-fetch.exe"
(Note: If Windows Defender slows down your boot time, add your bento-fetch.exe path to your Defender exclusions list).

📄 License
This project is licensed under the MIT License.