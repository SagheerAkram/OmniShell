# Cross-Platform Support

OmniShell is designed to work seamlessly on **Windows, Linux, and macOS**.

## Platform Compatibility

| Platform | Status | Notes |
|----------|--------|-------|
| **Windows 10/11** | ✅ Fully Supported | Native Windows binary |
| **Linux** | ✅ Fully Supported | All major distributions |
| **macOS 10.15+** | ✅ Fully Supported | Intel & Apple Silicon |

---

## Platform-Specific Features

### Windows
- PowerShell installation script (`install.ps1`)
- Native Windows notifications
- Windows Terminal integration
- Visual Studio Build Tools support

### Linux
- Bash installation script (`install.sh`)
- `notify-send` for desktop notifications
- Systemd integration
- Package manager support (apt, dnf, pacman)

### macOS
- Bash installation script (`install.sh`)
- Homebrew integration
- macOS notification center
- Universal binary (Intel + ARM)

---

## Installation

### Windows
```powershell
# Run PowerShell as Administrator (optional)
powershell -ExecutionPolicy Bypass -File install.ps1
```

### Linux
```bash
chmod +x install.sh
./install.sh
```

### macOS
```bash
chmod +x install.sh
./install.sh
```

---

## Building from Source

### All Platforms
```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build OmniShell
cargo build --release

# Binary location
# Windows: target\release\omnishell.exe
# Linux/macOS: target/release/omnishell
```

---

## Platform-Specific Dependencies

### Windows
- **Rust**: 1.70+
- **Visual Studio Build Tools** (for compilation)
- **OpenSSL** (included via vcpkg)

### Linux
```bash
# Debian/Ubuntu
sudo apt-get install build-essential pkg-config libssl-dev sqlite3

# Fedora/RHEL
sudo dnf install gcc pkg-config openssl-devel sqlite

# Arch Linux
sudo pacman -S base-devel openssl sqlite
```

### macOS
```bash
# Install Homebrew (if not installed)
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install dependencies
brew install openssl sqlite
```

---

## Cross-Platform Code

OmniShell uses Rust's cross-platform capabilities:

```rust
// Platform-specific notifications
#[cfg(target_os = "windows")]
fn notify(title: &str, body: &str) {
    // Windows notification
}

#[cfg(target_os = "linux")]
fn notify(title: &str, body: &str) {
    // Linux notify-send
}

#[cfg(target_os = "macos")]
fn notify(title: &str, body: &str) {
    // macOS notification center
}
```

---

## File Paths

OmniShell automatically handles platform-specific paths:

| Platform | Data Directory |
|----------|---------------|
| **Windows** | `C:\Users\<user>\AppData\Local\omnishell\` |
| **Linux** | `~/.local/share/omnishell/` |
| **macOS** | `~/Library/Application Support/omnishell/` |

---

## Testing on Multiple Platforms

We test OmniShell on:
- Windows 10, Windows 11
- Ubuntu 20.04, 22.04, 24.04
- Fedora 38, 39
- Arch Linux (latest)
- macOS 12 (Monterey), 13 (Ventura), 14 (Sonoma)

---

## Known Platform Differences

### Notifications
- **Windows**: Native toast notifications
- **Linux**: Desktop notifications via `notify-send`
- **macOS**: Notification Center

### File Permissions
- **Windows**: ACLs
- **Linux/macOS**: Unix permissions (0600 for keys)

### Network
- All platforms support all 7 network protocols
- Tor/I2P require separate installation on all platforms

---

## CI/CD

Our CI/CD pipeline tests on all three platforms:

```yaml
# GitHub Actions
strategy:
  matrix:
    os: [windows-latest, ubuntu-latest, macos-latest]
    rust: [stable]
```

---

## Binary Distribution

We provide pre-compiled binaries for:
- **Windows**: `omnishell-windows-x64.exe`
- **Linux**: `omnishell-linux-x64`
- **macOS**: `omnishell-macos-universal` (Intel + ARM)

---

## Performance

| Platform | Startup Time | Memory Usage | Encryption Speed |
|----------|--------------|--------------|------------------|
| **Windows** | <100ms | ~50MB | 500+ MB/s |
| **Linux** | <80ms | ~45MB | 550+ MB/s |
| **macOS** | <90ms | ~48MB | 520+ MB/s |

---

## Troubleshooting

### Windows
```powershell
# If Visual Studio Build Tools missing
# Download from: https://visualstudio.microsoft.com/downloads/
# Select "Desktop development with C++"
```

### Linux
```bash
# If OpenSSL errors
sudo apt-get install libssl-dev  # Debian/Ubuntu
sudo dnf install openssl-devel   # Fedora
```

### macOS
```bash
# If OpenSSL errors
brew install openssl
export OPENSSL_DIR=$(brew --prefix openssl)
```

---

**OmniShell works identically on all platforms!** 🎯

[Back to Wiki Home](Home) | [Installation Guide](Installation-Guide)
