# Welcome to OmniShell Wiki

<div align="center">

![OmniShell](https://img.shields.io/badge/OmniShell-v1.0.0-blue?style=for-the-badge)
![Rust](https://img.shields.io/badge/Rust-1.70+-orange?style=for-the-badge&logo=rust)
![License](https://img.shields.io/badge/License-MIT-green?style=for-the-badge)
![Security](https://img.shields.io/badge/Security-Military%20Grade-red?style=for-the-badge)

**Military-Grade Encrypted Messaging CLI**

[Installation](#installation) • [Quick Start](#quick-start) • [Commands](Command-Reference) • [Security](Security-Guide)

</div>

---

## 📚 Table of Contents

### Getting Started
- [Installation Guide](Installation-Guide)
- [Quick Start Tutorial](Quick-Start)
- [First Steps](First-Steps)

### Core Features
- [Command Reference](Command-Reference) - All 120+ commands
- [Messaging Guide](Messaging-Guide)
- [File Transfer](File-Transfer)
- [Group Chat](Group-Chat)

### Network & Security
- [Network Protocols](Network-Protocols) - 7 protocols explained
- [Security Features](Security-Features)
- [Encryption Guide](Encryption-Guide)
- [Perfect Forward Secrecy](Perfect-Forward-Secrecy)

### Advanced Features
- [Automation & Scripting](Automation)
- [REST API](REST-API)
- [Plugin System](Plugins)
- [Emergency Features](Emergency-Features)

### Comparisons & Use Cases
- [OmniShell vs BitChat](OmniShell-vs-BitChat)
- [Use Cases](Use-Cases)
- [Best Practices](Best-Practices)

### Technical Documentation
- [Architecture](Architecture)
- [Protocol Specifications](Protocol-Specifications)
- [Security Whitepaper](Security-Whitepaper)
- [Development Guide](Development-Guide)

---

## 🚀 Quick Start

### Installation

```bash
# Linux/macOS
chmod +x install.sh
./install.sh

# Windows
powershell -ExecutionPolicy Bypass -File install.ps1
```

### First Commands

```bash
# 1. Initialize OmniShell
omnishell init

# 2. View your identity
omnishell whoami

# 3. Add a contact
omnishell add alice omni:PUBLIC_KEY_HERE

# 4. Send a message
omnishell msg @alice "Hello, secure world!"

# 5. Read messages
omnishell read @alice
```

---

## 🎯 Why OmniShell?

### ✅ What Makes OmniShell Unique

| Feature | OmniShell | BitChat | Signal | Telegram |
|---------|-----------|---------|--------|----------|
| **CLI-First** | ✅ | ✅ | ❌ | ❌ |
| **Protocols** | 7 | 1 | 1 | 1 |
| **Perfect Forward Secrecy** | ✅ | ❌ | ✅ | ❌ |
| **Tor Support** | ✅ | ❌ | ❌ | ❌ |
| **I2P Support** | ✅ | ❌ | ❌ | ❌ |
| **Offline Queue** | ✅ | ❌ | ❌ | ❌ |
| **Relay Nodes** | ✅ | ❌ | ❌ | ❌ |
| **Emergency Features** | ✅ | ❌ | ❌ | ❌ |
| **REST API** | ✅ | ❌ | ❌ | ❌ |
| **Plugin System** | ✅ | ❌ | ❌ | ❌ |
| **Open Source** | ✅ | ✅ | ✅ | ❌ |

---

## 📊 Statistics

- **Total Commands**: 120+
- **Network Protocols**: 7 (P2P, Tor, I2P, LoRa, Bluetooth, SMS, Satellite)
- **Encryption**: AES-256-GCM, ChaCha20-Poly1305
- **Lines of Code**: 21,000+
- **Modules**: 37+
- **Documentation**: 160KB+

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
│ ✓ Zero-Knowledge Architecture       │
│ ✓ Tor/I2P Anonymous Routing         │
│ ✓ Multi-Hop Relay Network           │
└─────────────────────────────────────┘
```

---

## 🌟 Key Features

### 💬 Messaging
- End-to-end encrypted messages
- Group chat with encrypted keys
- Message reactions and starring
- Full-text search
- Disappearing messages

### 📁 File Transfer
- Encrypted file transfer
- Resume interrupted transfers
- Voice messages (Opus)
- Image compression
- Location sharing

### 🌐 Network Protocols
1. **P2P** - Direct connections
2. **Tor** - Anonymous .onion routing
3. **I2P** - Garlic routing
4. **LoRa** - Long-range mesh
5. **Bluetooth** - Nearby devices
6. **SMS** - Cellular backup
7. **Satellite** - Global coverage

### 🔒 Security
- Master password + 2FA
- Honeypot & duress modes
- Panic mode (secure wipe)
- Emergency broadcast
- Dead man's switch
- Geofencing

### 🤖 Automation
- Message filters
- Auto-reply
- Scheduled messages
- Templates
- Webhooks
- REST API

---

## 📖 Documentation

- **[Command Reference](Command-Reference)** - Complete command list
- **[Security Guide](Security-Guide)** - Security best practices
- **[API Documentation](REST-API)** - REST API reference
- **[Protocol Specs](Protocol-Specifications)** - Technical protocols

---

## 🆚 OmniShell vs BitChat

See our detailed [comparison](OmniShell-vs-BitChat) to understand why OmniShell is the superior choice for secure CLI messaging.

---

## 🤝 Contributing

We welcome contributions! See our [Development Guide](Development-Guide) for details.

---

## 📜 License

MIT License - See [LICENSE](https://github.com/sagheerakram/omnishell/blob/main/LICENSE)

---

## 📧 Contact

- **GitHub**: [github.com/sagheerakram/omnishell](https://github.com/sagheerakram/omnishell)
- **Issues**: [Report a bug](https://github.com/sagheerakram/omnishell/issues)
- **Security**: security@omnishell.dev

---

<div align="center">

**Made with ❤️ and Rust**

⭐ Star us on [GitHub](https://github.com/sagheerakram/omnishell)!

</div>
