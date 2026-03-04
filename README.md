Gemini said
This README is designed to look as sharp as your terminal. I've added rich Markdown formatting, status badges, and clear visual hierarchy to help your project stand out on GitHub.

README.md
<p align="center">🍱 bento-fetch</p>
<p align="center">
<img src="https://img.shields.io/badge/Language-Rust-orange.svg" alt="Language">
<img src="https://img.shields.io/badge/Speed-%3C%2025ms-brightgreen.svg" alt="Speed">
<img src="https://img.shields.io/badge/Platform-Linux%20%7C%20macOS-blue.svg" alt="Platform">
<img src="https://img.shields.io/badge/Style-Modern%20Bento-magenta.svg" alt="Style">
</p>

bento-fetch is a high-performance, designer-centric system information tool. While traditional fetch tools prioritize a "logo-on-the-left" approach, bento-fetch treats your terminal like a high-end dashboard, organizing your system's soul into a clean, balanced Bento Box grid.

✨ Key Features
Bento Grid Layout: A 6x6 grid that separates hardware vitals from software environments for maximum scannability.

Dynamic UX 🚦: Box borders dynamically change color based on usage:

🟢 Green: Safe (Usage < 60%)

🟡 Yellow: Moderate (Usage 60% - 85%)

🔴 Red: Critical (Usage > 85%)

Compile-Time Assets: Over 400+ ASCII logos are baked directly into the binary at compile-time using a custom build.rs script—zero runtime disk I/O.

Gestalt Grouping: Metrics are grouped logically (CPU/RAM/GPU/Load) so your eyes don't have to hunt for information.

Designer Aesthetic: Features rounded borders, NerdFont icon support, and responsive central margins for a "website" feel in the TUI.

🛠 Installation
1. Prerequisites
Ensure you have the following installed on your system:

Rust Toolchain: rustup

Nerd Fonts: Required for glyph rendering (e.g., JetBrainsMono)

PCI Hardware Database: Usually provided by the hwdata or pciutils package on Linux.

2. Setup
Bash
# Clone the repository
git clone https://github.com/yourusername/bento-fetch.git
cd bento-fetch

# Download the ASCII library (optional if folder exists)
git clone --depth 1 https://github.com/fastfetch-cli/fastfetch.git temp_repo
mv temp_repo/src/logo/ascii ./
rm -rf temp_repo

# Build for maximum performance
cargo build --release

# Install globally
sudo cp target/release/bento-fetch /usr/local/bin/
🚀 Configuration
bento-fetch is designed to work out of the box with zero configuration files.

[!TIP]
To add to your Fish shell startup:

Code snippet
# ~/.config/fish/config.fish
function fish_greeting
    bento-fetch
end
📸 Component Breakdown
Top Row: The Engine (Hardware)
CPU: Processor name and real-time usage color.

RAM: Used vs. Total capacity.

GPU: Detected Graphics Card name.

GPU%: Real-time VRAM/Utilization.

DISK: Aggregated usage across all mounted drives.

LOAD: 1-minute system load average.

Bottom Row: The Environment (Software)
OS: Distribution name (CachyOS, Arch, Debian, etc.).

KERNEL: System kernel version.

UPTIME: How long you've been grinding.

SHELL: Your active shell environment.

TERM: Current terminal emulator.

IP: Primary local network address.

📄 License
This project is licensed under the MIT License. Feel free to fork, tweak, and rice it to your heart's content.

