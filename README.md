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

```bash
# Linux/macOS
chmod +x install.sh
./install.sh

# Windows
powershell -ExecutionPolicy Bypass -File install.ps1

# From source
cargo build --release
```

### First Commands

```bash
# 1. Initialize
omnishell init

# 2. View your identity
omnishell whoami

# 3. Add a contact
omnishell add alice omni:PUBLIC_KEY_HERE

# 4. Send encrypted message
omnishell msg @alice "Hello, secure world!"

# 5. Read messages
omnishell read @alice
```

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

## 💡 Why OmniShell?

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

- **🎤 Journalism** - Anonymous sources, emergency broadcast, panic mode
- **👥 Remote Teams** - Group chat, automation, REST API integration
- **🚨 Disaster Recovery** - LoRa mesh, offline queue, location sharing
- **✊ Activism** - I2P routing, duress password, dead man's switch
- **🏢 Corporate** - 2FA, geofencing, security audit tools
- **🎖️ Military** - Multi-hop relay, satellite, PFS
- **🏥 Healthcare** - HIPAA compliance, encrypted backups, audit trails
- **💰 Finance** - Low-latency P2P, webhooks, templates

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
