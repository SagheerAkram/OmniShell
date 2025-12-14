# 📖 OmniShell Command Reference

Complete reference for all OmniShell commands with examples and options.

---

## 🆔 Identity & Setup

### `omnishell init`
Initialize OmniShell with key generation and setup.

```bash
omnishell init
omnishell init --force  # Re-initialize (overwrites existing keys)
```

**What it does:**
- Generates Ed25519 key pair
- Creates directory structure (~/.omnishell/)
- Generates device fingerprint
- Creates QR code for public key
- Initializes default configuration

---

### `omnishell whoami`
Display your identity information.

```bash
omnishell whoami
```

**Shows:**
- Public key
- Fingerprint (human-readable)
- Visual hash
- Device name and ID
- DHT username (if set)
- QR code location

---

### `omnishell config`
View and manage configuration.

```bash
omnishell config              # View all settings
omnishell config get <key>    # Get specific value
omnishell config set <key> <value>  # Set value
```

**Examples:**
```bash
omnishell config get encryption.default_cipher
omnishell config set encryption.default_cipher chacha20-poly1305
omnishell config set identity.username alice
```

---

## 👥 Contact Management

### `omnishell add`
Add a new contact.

```bash
omnishell add <name> <public_key>
omnishell add <name> --scan           # Scan QR code (future)
omnishell add <name> --nearby         # Discover nearby (future)
```

**Example:**
```bash
omnishell add alice omni:4f3a9b2c8d1e7a5fc2e9d8b6a1c3e5f7...
```

---

### `omnishell list`
List all contacts.

```bash
omnishell list
omnishell list --online  # Show only online contacts
```

**Shows:**
- Contact names
- Online status
- Last seen time
- Trust level
- Fingerprints

---

### `omnishell info`
Show detailed contact information.

```bash
omnishell info @<contact>
```

**Example:**
```bash
omnishell info @alice
```

**Shows:**
- Public key
- Fingerprint
- Visual hash
- Trust level
- Last seen
- Message count
- Notes

---

### `omnishell verify`
Verify contact's key fingerprint.

```bash
omnishell verify @<contact>
```

**Example:**
```bash
omnishell verify @alice
```

**Displays:**
- Fingerprint for manual verification
- Visual hash for quick comparison
- Verification instructions

---

### `omnishell remove`
Remove a contact.

```bash
omnishell remove @<contact>
omnishell remove @<contact> --delete-history
```

**Example:**
```bash
omnishell remove @alice --delete-history
```

---

## 💬 Messaging

### `omnishell msg`
Send an encrypted message.

```bash
omnishell msg @<contact> "<message>"
```

**Options:**
- `--protocol <protocol>` - Force specific protocol (tor, i2p, p2p)
- `--priority <priority>` - Message priority (urgent, high, normal, low)
- `--ttl <duration>` - Time to live (e.g., 5m, 1h, 1d)
- `--stealth` - Use maximum privacy (ChaCha20 + Tor)

**Examples:**
```bash
omnishell msg @alice "Hey, how are you?"
omnishell msg @bob "Urgent!" --priority urgent
omnishell msg @charlie "Secret message" --stealth
omnishell msg @dave "Quick note" --protocol tor
```

---

### `omnishell read`
Read messages.

```bash
omnishell read                    # Show unread count
omnishell read @<contact>         # Read conversation
omnishell read @<contact> --last 10    # Last N messages
omnishell read @<contact> --unread     # Unread only
```

**Examples:**
```bash
omnishell read @alice
omnishell read @bob --last 20
```

---

### `omnishell reply`
Reply to a specific message (threaded).

```bash
omnishell reply <message_id> "<reply>"
```

**Example:**
```bash
omnishell reply 550e8400-e29b-41d4... "Thanks for the update!"
```

---

### `omnishell edit`
Edit a sent message (1 hour limit).

```bash
omnishell edit <message_id> "<new_text>"
```

**Example:**
```bash
omnishell edit 550e8400-e29b-41d4... "Corrected message"
```

---

### `omnishell delete`
Delete a message.

```bash
omnishell delete <message_id>
omnishell delete <message_id> --for-everyone  # Delete for all (1h limit)
```

**Examples:**
```bash
omnishell delete 550e8400-e29b-41d4...
omnishell delete 550e8400-e29b-41d4... --for-everyone
```

---

### `omnishell forward`
Forward a message to another contact.

```bash
omnishell forward <message_id> @<recipient>
omnishell forward <message_id> @<recipient> --strip-metadata
```

**Example:**
```bash
omnishell forward 550e8400-e29b-41d4... @bob
```

---

### `omnishell react`
React to a message with emoji.

```bash
omnishell react <message_id> <emoji>
```

**Example:**
```bash
omnishell react 550e8400-e29b-41d4... 👍
omnishell react 550e8400-e29b-41d4... ❤️
```

---

### `omnishell unreact`
Remove your reaction from a message.

```bash
omnishell unreact <message_id>
```

---

### `omnishell star`
Bookmark a message.

```bash
omnishell star <message_id>
```

---

### `omnishell unstar`
Remove bookmark from a message.

```bash
omnishell unstar <message_id>
```

---

### `omnishell starred`
List all starred/bookmarked messages.

```bash
omnishell starred
```

---

### `omnishell search`
Search messages with full-text search.

```bash
omnishell search "<query>"
omnishell search "<query>" --contact @<name>
omnishell search "<query>" --date <date>
```

**Examples:**
```bash
omnishell search "meeting"
omnishell search "project alpha" --contact @alice
```

---

## 👨‍👩‍👧‍👦 Group Chat

### `omnishell group create`
Create a new group.

```bash
omnishell group create <name> @<user1> @<user2> ...
```

**Example:**
```bash
omnishell group create team-alpha @alice @bob @charlie
```

---

### `omnishell group list`
List all groups.

```bash
omnishell group list
```

---

### `omnishell group info`
Show group information.

```bash
omnishell group info <name>
```

**Example:**
```bash
omnishell group info team-alpha
```

---

### `omnishell group add`
Add a member to a group.

```bash
omnishell group add <group> @<member>
```

**Example:**
```bash
omnishell group add team-alpha @dave
```

---

### `omnishell group remove`
Remove a member from a group.

```bash
omnishell group remove <group> @<member>
```

**Example:**
```bash
omnishell group remove team-alpha @dave
```

---

### `omnishell group msg`
Send message to a group.

```bash
omnishell group msg <group> "<message>"
```

**Example:**
```bash
omnishell group msg team-alpha "Meeting at 3pm!"
```

---

## 📁 File Transfer

### `omnishell send`
Send a file with encryption.

```bash
omnishell send @<contact> <file_path>
omnishell send @<contact> <file_path> --compress
```

**Examples:**
```bash
omnishell send @alice document.pdf
omnishell send @bob presentation.pptx --compress
omnishell send @charlie /path/to/large-file.zip --compress
```

**Features:**
- End-to-end encryption
- 256KB chunking for large files
- Progress bars
- MD5 checksums
- Optional compression

---

### `omnishell transfers`
View file transfer history.

```bash
omnishell transfers
```

---

### `omnishell image`
Send an image with automatic compression.

```bash
omnishell image @<contact> <image_path>
```

**Example:**
```bash
omnishell image @alice photo.jpg
omnishell image @bob screenshot.png
```

**Supported formats:** JPG, PNG, GIF, WebP, BMP

---

## 🛠️ Utilities

### `omnishell status`
Show comprehensive system status.

```bash
omnishell status
```

**Shows:**
- Identity information
- Network protocol status
- Activity statistics
- Security features
- Storage information

---

### `omnishell stats`
Show detailed statistics.

```bash
omnishell stats
```

**Shows:**
- Activity: contacts, groups, messages (sent/received/unread)
- Starred messages count
- Storage: database size and location
- Most active contact
- Recent activity

---

### `omnishell help`
Show help information.

```bash
omnishell help              # General help
omnishell help <command>    # Specific command help
```

**Examples:**
```bash
omnishell help msg
omnishell help group
omnishell help send
```

---

## 🔐 Security Features

### Encryption
- **Ciphers**: AES-256-GCM, ChaCha20-Poly1305
- **Key Exchange**: X25519 (ECDH)
- **Signatures**: Ed25519 (EdDSA)
- **Key Derivation**: Argon2id, HKDF
- **Hashing**: BLAKE2b, SHA-256

### Privacy Options
- **Stealth Mode**: `--stealth` flag uses ChaCha20 + Tor routing
- **Protocol Selection**: Choose between P2P, Tor, I2P
- **Metadata Protection**: Encrypted timestamps and sender info
- **Perfect Forward Secrecy**: Planned with Double Ratchet

### Security Limits
- **Message Edit**: 1 hour time limit
- **Delete for Everyone**: 1 hour time limit
- **Key Fingerprints**: Human-readable for easy verification

---

## 📊 Command Categories

| Category | Commands | Count |
|----------|----------|-------|
| **Identity** | init, whoami, config | 3 |
| **Contacts** | add, list, info, verify, remove | 5 |
| **Messaging** | msg, read, reply, edit, delete, forward, react, unreact, star, unstar, starred, search | 12 |
| **Groups** | group (create, list, info, add, remove, msg) | 6 |
| **Files** | send, transfers, image | 3 |
| **Utilities** | status, stats, help | 3 |
| **TOTAL** | | **32** |

---

## 🚀 Quick Start Examples

### Setup
```bash
# Initialize
omnishell init

# View your identity
omnishell whoami

# Configure
omnishell config set identity.username alice
```

### Add Contact & Message
```bash
# Add a contact
omnishell add bob omni:4f3a9b2c8d1e7a5fc2e9...

# Send message
omnishell msg @bob "Hello!"

# Read messages
omnishell read @bob
```

### Group Chat
```bash
# Create group
omnishell group create team @bob @charlie @dave

# Send to group
omnishell group msg team "Team meeting tomorrow!"
```

### File Transfer
```bash
# Send file
omnishell send @bob report.pdf

# Send image
omnishell image @charlie photo.jpg
```

### Advanced Features
```bash
# Star a message
omnishell star 550e8400-e29b-41d4...

# Search messages
omnishell search "project"

# View statistics
omnishell stats
```

---

## 💡 Tips & Best Practices

1. **Verify Keys**: Always verify fingerprints with `omnishell verify @contact`
2. **Stealth Mode**: Use `--stealth` for sensitive communications
3. **Backup**: Backup your `~/.omnishell/keys/` directory
4. **Groups**: Use groups for team communication
5. **Search**: Use `omnishell search` to find old messages
6. **Stats**: Run `omnishell stats` to monitor usage

---

## 🆘 Getting Help

- General help: `omnishell help`
- Command help: `omnishell help <command>`
- System status: `omnishell status`
- Statistics: `omnishell stats`

---

***Version: 0.1.0 | Build: MVP Complete***
