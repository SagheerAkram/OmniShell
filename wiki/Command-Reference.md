# Command Reference

Complete reference for all 120+ OmniShell commands.

---

## Table of Contents

- [Identity & Setup](#identity--setup)
- [Contact Management](#contact-management)
- [Messaging](#messaging)
- [Advanced Messaging](#advanced-messaging)
- [Group Chat](#group-chat)
- [File Transfer](#file-transfer)
- [Network Protocols](#network-protocols)
- [Security & Privacy](#security--privacy)
- [Automation](#automation)
- [Emergency Features](#emergency-features)
- [Analytics & Statistics](#analytics--statistics)
- [System & Utilities](#system--utilities)

---

## Identity & Setup

### `omnishell init`
Initialize OmniShell and generate encryption keys.

```bash
omnishell init
omnishell init --force  # Re-initialize (overwrites existing keys)
```

**Example Output:**
```
╔════════════════════════════════════════════════════════════════╗
║                   INITIALIZING OMNISHELL                       ║
╚════════════════════════════════════════════════════════════════╝

→ Generating Ed25519 keypair...
✓ Keypair generated

Your Public Key:
  omni:abc123def456...

Your Fingerprint:
  1234:5678:90AB:CDEF

QR Code: [QR code displayed]
```

### `omnishell whoami`
Display your identity information.

```bash
omnishell whoami
```

---

## Contact Management

### `omnishell add`
Add a new contact.

```bash
omnishell add <name> <public_key>
omnishell add alice omni:abc123...
omnishell add bob omni:def456... --scan  # Scan QR code
omnishell add charlie --nearby           # Discover nearby devices
```

### `omnishell list`
List all contacts.

```bash
omnishell list
omnishell list --online  # Show only online contacts
```

### `omnishell info`
Show detailed contact information.

```bash
omnishell info @alice
```

### `omnishell verify`
Verify contact's key fingerprint.

```bash
omnishell verify @alice
```

### `omnishell remove`
Remove a contact.

```bash
omnishell remove @alice
omnishell remove @bob --delete-history  # Also delete message history
```

---

## Messaging

### `omnishell msg`
Send an encrypted message.

```bash
omnishell msg @alice "Hello!"
omnishell msg @bob "Secret" --stealth                    # Maximum privacy
omnishell msg @charlie "Urgent!" --priority urgent       # High priority
omnishell msg @dave "Disappear" --ttl 5m                # Disappearing message
omnishell msg @eve "Anonymous" --protocol tor            # Via Tor
```

**Options:**
- `--protocol <p2p|tor|i2p>` - Force specific protocol
- `--priority <low|normal|high|urgent>` - Message priority
- `--ttl <duration>` - Time to live (e.g., 5m, 1h, 1d)
- `--stealth` - Maximum anonymity (ChaCha20 + Tor)

### `omnishell read`
Read messages.

```bash
omnishell read                    # All messages
omnishell read @alice             # Messages from Alice
omnishell read @bob --last 10     # Last 10 messages
omnishell read --unread           # Only unread messages
omnishell read --since 2024-01-01 # Messages since date
```

---

## Advanced Messaging

### `omnishell reply`
Reply to a message.

```bash
omnishell reply msg_abc123 "Thanks!"
```

### `omnishell edit`
Edit a sent message.

```bash
omnishell edit msg_abc123 "Corrected text"
```

### `omnishell delete`
Delete a message.

```bash
omnishell delete msg_abc123
omnishell delete msg_def456 --for-everyone  # Delete for all recipients
```

### `omnishell forward`
Forward a message.

```bash
omnishell forward msg_abc123 @charlie
omnishell forward msg_def456 @dave --strip-metadata  # Don't reveal original sender
```

### `omnishell react`
React to a message with emoji.

```bash
omnishell react msg_abc123 👍
omnishell react msg_def456 ❤️
```

### `omnishell star`
Star/bookmark a message.

```bash
omnishell star msg_abc123
omnishell unstar msg_abc123
omnishell starred  # List all starred messages
```

### `omnishell search`
Search messages.

```bash
omnishell search "keyword"
omnishell search "project" --contact @alice
omnishell search "meeting" --date 2024-01
```

---

## Group Chat

### `omnishell group create`
Create a new group.

```bash
omnishell group create team @alice @bob @charlie
```

### `omnishell group list`
List all groups.

```bash
omnishell group list
```

### `omnishell group info`
Show group information.

```bash
omnishell group info team
```

### `omnishell group add`
Add member to group.

```bash
omnishell group add team @dave
```

### `omnishell group remove`
Remove member from group.

```bash
omnishell group remove team @eve
```

### `omnishell group msg`
Send message to group.

```bash
omnishell group msg team "Hello everyone!"
```

---

## File Transfer

### `omnishell send`
Send a file.

```bash
omnishell send @alice document.pdf
omnishell send @bob large-file.zip --compress
```

### `omnishell transfers`
View file transfers.

```bash
omnishell transfers
```

### `omnishell resume`
Resume interrupted transfer.

```bash
omnishell resume transfer_abc123
```

### `omnishell image`
Send image with compression.

```bash
omnishell image @alice photo.jpg
```

### `omnishell voice`
Record and send voice message.

```bash
omnishell voice record 30  # Record for 30 seconds
omnishell voice send @alice voice_123.opus
omnishell voice play voice_123.opus
```

### `omnishell location`
Share location.

```bash
omnishell location share @alice
omnishell location share @bob --live  # Live location sharing
omnishell location stop               # Stop live sharing
```

---

## Network Protocols

### P2P
```bash
omnishell p2p listen 8888
omnishell p2p connect 192.168.1.100:8888
omnishell p2p discover
omnishell p2p list
```

### Tor
```bash
omnishell tor init
omnishell tor start
omnishell tor stop
omnishell tor status
omnishell tor address      # Get .onion address
omnishell tor circuit      # New circuit
```

### I2P
```bash
omnishell i2p init
omnishell i2p status
omnishell i2p tunnel
omnishell i2p destination  # Get .i2p destination
```

### LoRa
```bash
omnishell lora init
omnishell lora status
omnishell lora scan
omnishell lora send @alice "Message"
```

### Bluetooth
```bash
omnishell bluetooth init
omnishell bluetooth status
omnishell bluetooth scan
omnishell bluetooth send @nearby "Hi!"
```

### SMS
```bash
omnishell sms init
omnishell sms send +1234567890 "Message"
```

### Satellite
```bash
omnishell satellite init
omnishell satellite status
omnishell satellite send @remote "Message"
```

### Relay Nodes
```bash
omnishell relay register --port 8888 --capacity 100
omnishell relay list
omnishell relay start
```

---

## Security & Privacy

### `omnishell backup`
Create encrypted backup.

```bash
omnishell backup
omnishell backup --output backup.tar.gz.enc --password mypass
```

### `omnishell restore`
Restore from backup.

```bash
omnishell restore backup.tar.gz.enc --password mypass
```

### `omnishell export`
Export contacts.

```bash
omnishell export contacts.json
```

### `omnishell import`
Import contacts.

```bash
omnishell import contacts.json
```

### `omnishell rotate-keys`
Rotate encryption keys.

```bash
omnishell rotate-keys
```

### `omnishell cleanup`
Clean up old data.

```bash
omnishell cleanup --days 90
omnishell cleanup --days 30 --dry-run  # Preview what will be deleted
```

### Security Features
```bash
omnishell security password      # Set master password
omnishell security 2fa           # Enable 2FA
omnishell security honeypot      # Enable honeypot mode
omnishell security duress        # Set duress password
omnishell security geofence      # Setup geofencing
omnishell security screenshot    # Enable screenshot detection
```

---

## Automation

### `omnishell filter`
Create message filters.

```bash
omnishell filter create spam --pattern ".*spam.*"
omnishell filter list
```

### `omnishell schedule`
Schedule messages.

```bash
omnishell schedule @alice "Meeting reminder" 09:00
```

### `omnishell autoreply`
Set auto-reply.

```bash
omnishell autoreply "I'm away, will respond later"
omnishell autoreply off
```

### `omnishell template`
Message templates.

```bash
omnishell template create greeting "Hello {{name}}!"
omnishell template list
omnishell template use greeting --name Alice
```

### `omnishell webhook`
Register webhooks.

```bash
omnishell webhook register https://example.com/hook --events message.received
omnishell webhook list
```

---

## Emergency Features

### `omnishell emergency`
Emergency broadcast.

```bash
omnishell emergency "Need help at coordinates..."
```

### `omnishell panic`
Panic mode (secure wipe).

```bash
omnishell panic
```

### `omnishell deadman`
Dead man's switch.

```bash
omnishell deadman setup --timeout 24h
omnishell deadman reset
omnishell deadman status
```

---

## Analytics & Statistics

### `omnishell stats`
Show statistics.

```bash
omnishell stats
omnishell stats @alice  # Per-contact stats
```

### `omnishell analytics`
Detailed analytics dashboard.

```bash
omnishell analytics
```

### `omnishell timeline`
Activity timeline.

```bash
omnishell timeline
```

---

## System & Utilities

### `omnishell status`
Show network status.

```bash
omnishell status
```

### `omnishell config`
Configuration management.

```bash
omnishell config                    # Interactive config
omnishell config set key value
omnishell config get key
```

### `omnishell test`
Run tests.

```bash
omnishell test
```

### `omnishell benchmark`
Run benchmarks.

```bash
omnishell benchmark
```

### `omnishell audit`
Security audit.

```bash
omnishell audit
```

### `omnishell help`
Show help.

```bash
omnishell help
omnishell help msg      # Help for specific command
```

### `omnishell version`
Show version.

```bash
omnishell version
```

### `omnishell tutorial`
Interactive tutorials.

```bash
omnishell tutorial basics
omnishell tutorial security
omnishell tutorial groups
omnishell tutorial protocols
```

---

## REST API

### `omnishell api`
API server management.

```bash
omnishell api init
omnishell api start
```

---

## Queue Management

### `omnishell queue`
Message queue operations.

```bash
omnishell queue show
omnishell queue process
omnishell queue clear
```

---

## Plugin System

### `omnishell plugin`
Plugin management.

```bash
omnishell plugin list
omnishell plugin install <name>
omnishell plugin enable <name>
omnishell plugin disable <name>
omnishell plugin uninstall <name>
```

---

## Experimental Features

### `omnishell experimental`
Experimental features.

```bash
omnishell experimental list
omnishell experimental pqc      # Post-quantum crypto
omnishell experimental ai       # AI assistant
omnishell experimental ipfs     # IPFS integration
```

---

## Quick Reference Card

```
┌─────────────────────────────────────────────────────────────┐
│ OMNISHELL QUICK REFERENCE                                   │
├─────────────────────────────────────────────────────────────┤
│ SETUP                                                       │
│   init, whoami, config                                      │
│                                                             │
│ CONTACTS                                                    │
│   add, list, info, verify, remove                          │
│                                                             │
│ MESSAGING                                                   │
│   msg, read, reply, edit, delete, forward                  │
│   react, star, search                                       │
│                                                             │
│ GROUPS                                                      │
│   group create/list/info/add/remove/msg                    │
│                                                             │
│ FILES                                                       │
│   send, transfers, resume, image, voice, location          │
│                                                             │
│ NETWORK                                                     │
│   p2p, tor, i2p, lora, bluetooth, sms, satellite, relay    │
│                                                             │
│ SECURITY                                                    │
│   backup, restore, export, import, rotate-keys, cleanup    │
│   security (password, 2fa, honeypot, duress)               │
│                                                             │
│ AUTOMATION                                                  │
│   filter, schedule, autoreply, template, webhook           │
│                                                             │
│ EMERGENCY                                                   │
│   emergency, panic, deadman                                 │
│                                                             │
│ SYSTEM                                                      │
│   status, stats, analytics, help, version, tutorial        │
└─────────────────────────────────────────────────────────────┘
```

---

**Total Commands: 120+**

For more details, see the [full documentation](https://github.com/sagheerakram/omnishell/wiki).
