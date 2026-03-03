<div align="center">

# 🔐 OmniShell

**Military-Grade Encrypted Messaging CLI**

**Cross-Platform: Windows • Linux • macOS**

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-green?style=for-the-badge)](LICENSE)
[![Security](https://img.shields.io/badge/Security-Military%20Grade-red?style=for-the-badge)](wiki/Security-Whitepaper)
[![Version](https://img.shields.io/badge/Version-1.0.0-blue?style=for-the-badge)](wiki/Release-Notes)
[![Windows](https://img.shields.io/badge/Windows-10%2F11-blue?style=for-the-badge&logo=windows)](wiki/Cross-Platform)
[![Linux](https://img.shields.io/badge/Linux-All%20Distros-yellow?style=for-the-badge&logo=linux)](wiki/Cross-Platform)
[![macOS](https://img.shields.io/badge/macOS-10.15+-black?style=for-the-badge&logo=apple)](wiki/Cross-Platform)

**The most advanced secure messaging CLI with 7 network protocols, Perfect Forward Secrecy, and 120+ commands**

[Features](#-features) • [Installation](#-installation) • [Quick Start](#-quick-start) • [Wiki](../../wiki) • [Commands](wiki/Command-Reference)

</div>

---

## 🌟 Features

### 💬 **Secure Messaging**
- **End-to-End Encryption**: AES-256-GCM & ChaCha20-Poly1305
- **Perfect Forward Secrecy**: Double Ratchet algorithm
- **Group Chat**: Encrypted group keys
- **Advanced Operations**: Reply, edit, delete, forward, react, star, search

### 🌐 **7 Network Protocols**
1. **P2P** - Direct peer-to-peer connections
2. **Tor** - Anonymous .onion routing
3. **I2P** - Garlic routing with .i2p destinations
4. **LoRa** - Long-range mesh networking (15km+)
5. **Bluetooth** - Nearby device communication
6. **SMS** - Cellular gateway integration
7. **Satellite** - High-latency global coverage

### 📁 **File Transfer**
- Encrypted file transfer with chunking
- **Resume interrupted transfers**
- Voice messages (Opus format)
- Image compression
- Location sharing

### 🔒 **Security & Privacy**
- Master password + 2FA (TOTP)
- Honeypot & duress modes
- Panic mode (secure wipe)
- Emergency broadcast
- Dead man's switch
- Geofencing & screenshot detection
- Web of Trust

### 🤖 **Automation & Integration**
- Message filters & auto-reply
- Scheduled messages
- Templates with variables
- Webhooks
- **REST API** with authentication
- Plugin system

### 📊 **Analytics**
- Activity dashboard
- Per-contact statistics
- Protocol usage analytics
- Activity timeline

---

## 🚀 Quick Start

### Installation

The fastest way to install and run the OmniShell interactive dashboard is using Cargo (Rust's package manager).

```bash
# Clone the repository and run immediately
git clone https://github.com/SagheerAkram/OmniShell.git
cd OmniShell
cargo run --release
```

This single command will compile the executable and launch the **Tactical Interface Dashboard** automatically, giving you access to the background nodes, Sonar audio modems, and stealth messaging features without any configuration required.

### First Commands

```bash
# 1. Initialize OmniShell
omnishell init

# 2. View your identity (includes QR code)
omnishell whoami

# 3. Add a contact
omnishell add alice omni:PUBLIC_KEY_HERE

# 4. List your contacts
omnishell list

# 5. Send encrypted message
omnishell msg @alice "Hello, secure world!"

# 6. Read messages
omnishell read @alice

# 7. Check statistics
omnishell stats

# 8. Create backup
omnishell backup
```

> **Note:** After installation, you can use `omnishell` from anywhere on your system!

---

## 📖 Documentation

- **[📚 Wiki Home](../../wiki)** - Complete documentation
- **[⚡ Quick Start](wiki/Quick-Start)** - Get started in 5 minutes
- **[📋 Command Reference](wiki/Command-Reference)** - All 120+ commands
- **[🎯 Use Cases](wiki/Use-Cases)** - Real-world scenarios
- **[🔐 Security Guide](wiki/Security-Whitepaper)** - Security whitepaper
- **[🆚 vs BitChat](wiki/OmniShell-vs-BitChat)** - Detailed comparison
- **[🔧 Installation Guide](wiki/Installation-Guide)** - Detailed setup

---

## � No Internet? No Problem!

OmniShell is designed to work even when you have **zero internet connection**. Here is how you can use it:

### 🏠 **At Home / Office / Dorm (Local Wi-Fi)**
**Scenario**: The internet is down, or you are on a restricted college/office Wi-Fi that blocks messaging apps.
- **How it works**: OmniShell finds other users connected to the *same* Wi-Fi router.
- **What you can do**: Chat, send huge files (movies, projects) at super fast speeds directly between laptops. No data usage!

### 🚌 **On the Bus / Train / Plane (Bluetooth)**
**Scenario**: You are travelling with friends and have no signal.
- **How it works**: Uses your device's Bluetooth to create a mesh network.
- **What you can do**: Text friends sitting nearby. Messages can "hop" from one phone to another to reach someone further away.

### 🏕️ **Hiking / Remote Areas (LoRa)**
**Scenario**: You are kilometers away from civilization.
- **How it works**: Uses tiny, cheap radio transmitters (LoRa) that plug into your laptop.
- **What you can do**: Send texts up to **15km (9 miles)** away without any cell towers or satellites.

---

## �💡 Why OmniShell?

### **Superior to BitChat**

| Feature | OmniShell | BitChat |
|---------|-----------|---------|
| **Protocols** | 7 | 1 |
| **Perfect Forward Secrecy** | ✅ | ❌ |
| **Tor/I2P Support** | ✅ | ❌ |
| **Relay Nodes** | ✅ | ❌ |
| **Emergency Features** | ✅ | ❌ |
| **REST API** | ✅ | ❌ |
| **Commands** | 120+ | ~20 |
| **Active Development** | ✅ 2024 | ❌ Abandoned |

**[See full comparison →](wiki/OmniShell-vs-BitChat)**

---

## 🎯 Use Cases

- **🎓 College/Campus** - Bypass restrictive firewalls using Tor/I2P or P2P mesh networking.
- **🏕️ Off-Grid/Camping** - Communicate when there is no cell service using LoRa or Bluetooth mesh.
- **🎤 Journalism** - Protect anonymous sources with potential deniability and hidden services.
- **👥 Remote Teams** - Secure, encrypted group chat for sensitive project discussions.
- **🚨 Disaster Response** - Coordinate rescue efforts when infrastructure is down using LoRa/Satellite.
- **✊ Activism & Protests** - Organize safely in high-risk environments with panic buttons and duress passwords.
- **🏢 Corporate Security** - Internal secure communication channels with strict audit logs.
- **🏥 Healthcare/HIPAA** - Share patient data securely with end-to-end encryption.
- **💰 High-Value Finance** - Discuss transactions securely without fear of interception.

**[See all use cases →](wiki/Use-Cases)**

---

## 🔐 Security Highlights

```
┌─────────────────────────────────────┐
│  Military-Grade Security            │
├─────────────────────────────────────┤
│ ✓ AES-256-GCM Encryption           │
│ ✓ Ed25519 Signatures                │
│ ✓ Perfect Forward Secrecy           │
│ ✓ Double Ratchet Algorithm          │
│ ✓ Argon2id Key Derivation          │
│ ✓ X25519 Key Exchange               │
│ ✓ Tor/I2P Anonymous Routing         │
│ ✓ Multi-Hop Relay Network           │
└─────────────────────────────────────┘
```

**[Read Security Whitepaper →](wiki/Security-Whitepaper)**

---

## 📊 Statistics

- **Commands**: 120+
- **Network Protocols**: 7
- **Modules**: 37+
- **Lines of Code**: 21,000+
- **Documentation**: 160KB+
- **Security Level**: 256-bit

---

## 🛠️ Technology Stack

- **Language**: Rust 1.70+
- **Encryption**: `ring`, `chacha20poly1305`
- **Networking**: `tokio`, `quinn`
- **Database**: SQLite with `sqlx`
- **CLI**: `clap`, `colored`
- **Security**: `argon2`, `ed25519-dalek`, `x25519-dalek`

---

## 📝 Example Commands

```bash
# Secure messaging
omnishell msg @alice "Secret message" --stealth
omnishell msg @bob "Urgent!" --priority urgent --protocol tor

# Group chat
omnishell group create team @alice @bob @charlie
omnishell group msg team "Hello everyone!"

# File transfer
omnishell send @alice document.pdf
omnishell resume transfer_abc123  # Resume interrupted

# Voice & location
omnishell voice record 30
omnishell location share @alice --live

# Emergency
omnishell emergency "Need help!"
omnishell panic  # Secure wipe

# Automation
omnishell filter create spam --pattern ".*spam.*"
omnishell schedule @alice "Reminder" 09:00
omnishell webhook register https://example.com/hook

# Analytics
omnishell stats
omnishell analytics
omnishell timeline
```

---

## 🛡️ Tactical Capabilities

OmniShell includes a suite of "Default Off" features for Electronic Warfare and Physical Security.
**[View Full Tactical Feature Documentation](FEATURES.md)**

## 🤝 Contributing

We welcome contributions! Please see our [Development Guide](wiki/Development-Guide).

---

## 📜 License

MIT License - See [LICENSE](LICENSE) for details.

---

## 📧 Contact & Support

- **GitHub**: [github.com/sagheerakram/omnishell](https://github.com/sagheerakram/omnishell)
- **Issues**: [Report a bug](https://github.com/sagheerakram/omnishell/issues)
- **Wiki**: [Complete Documentation](../../wiki)
- **Security**: security@omnishell.dev

---

<div align="center">

**Made with ❤️ and Rust**

⭐ **Star us on GitHub!** ⭐

[Documentation](../../wiki) • [Quick Start](wiki/Quick-Start) • [Commands](wiki/Command-Reference) • [Security](wiki/Security-Whitepaper)

</div>
