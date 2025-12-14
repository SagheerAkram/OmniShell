# OmniShell Protocol Specifications

## 1. Message Protocol

### 1.1 Message Format

```
OmniShell Message v1.0
┌─────────────────────────────────────┐
│ Header (encrypted)                  │
├─────────────────────────────────────┤
│ - Version: u8                       │
│ - Message Type: u8                  │
│ - Sender ID: [u8; 32]              │
│ - Recipient ID: [u8; 32]           │
│ - Timestamp: i64                    │
│ - Sequence Number: u64              │
│ - Nonce: [u8; 12]                  │
├─────────────────────────────────────┤
│ Payload (encrypted)                 │
├─────────────────────────────────────┤
│ - Content: Vec<u8>                  │
│ - Attachments: Vec<Attachment>      │
│ - Metadata: HashMap<String, String> │
├─────────────────────────────────────┤
│ Signature: [u8; 64]                 │
└─────────────────────────────────────┘
```

### 1.2 Message Types

| Type | Code | Description |
|------|------|-------------|
| TEXT | 0x01 | Text message |
| FILE | 0x02 | File transfer |
| VOICE | 0x03 | Voice message |
| IMAGE | 0x04 | Image message |
| LOCATION | 0x05 | Location share |
| GROUP | 0x10 | Group message |
| SYSTEM | 0xFF | System message |

### 1.3 Encryption Layers

```
┌──────────────────────────────┐
│ Application Layer            │
│ (Message content)            │
├──────────────────────────────┤
│ E2E Encryption Layer         │
│ (AES-256-GCM / ChaCha20)     │
├──────────────────────────────┤
│ Transport Layer              │
│ (Protocol-specific)          │
├──────────────────────────────┤
│ Network Layer                │
│ (P2P / Tor / I2P)           │
└──────────────────────────────┘
```

## 2. Key Exchange Protocol

### 2.1 Initial Handshake

```
Alice                           Bob
  |                              |
  |  1. Send Public Key (Ed25519)|
  |----------------------------->|
  |                              |
  |  2. Send Public Key (Ed25519)|
  |<-----------------------------|
  |                              |
  |  3. Derive Shared Secret     |
  |     (X25519 ECDH)           |
  |                              |
  |  4. Initialize Double Ratchet|
  |                              |
  |  5. Send Encrypted Message   |
  |<---------------------------->|
```

### 2.2 Double Ratchet State

```rust
struct RatchetState {
    // DH Ratchet
    dh_send: KeyPair,
    dh_recv: PublicKey,
    
    // Symmetric Ratchet
    root_key: [u8; 32],
    chain_key_send: [u8; 32],
    chain_key_recv: [u8; 32],
    
    // Message counters
    send_count: u64,
    recv_count: u64,
    prev_send_count: u64,
}
```

## 3. Network Protocols

### 3.1 P2P Protocol

**Connection Establishment:**
```
1. TCP handshake
2. Protocol version exchange
3. Public key exchange
4. Signature verification
5. Encrypted channel established
```

**Message Format:**
```
[4 bytes: length][encrypted payload][16 bytes: auth tag]
```

### 3.2 Tor Protocol

**Hidden Service Setup:**
```
1. Generate Ed25519 key pair
2. Derive .onion address
3. Configure Tor daemon
4. Publish descriptor to HSDir
5. Accept connections via Tor
```

**Message Routing:**
```
Client -> Guard -> Middle -> Exit -> Hidden Service
       (3 hops minimum)
```

### 3.3 I2P Protocol

**Tunnel Creation:**
```
1. Generate I2P destination keys
2. Create inbound tunnel (3 hops)
3. Create outbound tunnel (3 hops)
4. Publish lease set
5. Accept connections via I2P
```

**Garlic Routing:**
```
[Garlic Clove 1][Garlic Clove 2][Garlic Clove 3]
Each clove encrypted for specific hop
```

### 3.4 Relay Protocol

**Relay Node Registration:**
```json
{
  "node_id": "uuid",
  "public_key": "ed25519_key",
  "address": "ip:port",
  "capacity": 100,
  "uptime": 86400
}
```

**Multi-Hop Routing:**
```
Source -> Relay1 -> Relay2 -> Relay3 -> Destination
Each hop: decrypt one layer, forward to next
```

## 4. File Transfer Protocol

### 4.1 Chunking

```
File -> [Chunk 0][Chunk 1][Chunk 2]...[Chunk N]
Each chunk: 256 KB
```

### 4.2 Chunk Format

```json
{
  "chunk_id": 0,
  "total_chunks": 10,
  "checksum": "md5_hash",
  "data": "encrypted_bytes"
}
```

### 4.3 Resume Protocol

```
1. Client requests transfer state
2. Server sends chunks_received bitmap
3. Client resumes from last successful chunk
4. Verify checksum on completion
```

## 5. Group Chat Protocol

### 5.1 Group Key Management

```
Group Key Hierarchy:
┌─────────────────┐
│ Group Master Key│
└────────┬────────┘
         │
    ┌────┴────┬────────┬────────┐
    │         │        │        │
  Member1  Member2  Member3  Member4
  (derived) (derived) (derived) (derived)
```

### 5.2 Group Message Format

```json
{
  "group_id": "uuid",
  "sender": "member_id",
  "encrypted_for": ["member1", "member2", "member3"],
  "content": "encrypted_message",
  "signature": "ed25519_sig"
}
```

## 6. Queue Protocol

### 6.1 Message Queue Entry

```json
{
  "queue_id": "uuid",
  "recipient": "contact_id",
  "priority": "urgent|high|normal|low",
  "encrypted_content": "bytes",
  "retry_count": 0,
  "max_retries": 5,
  "created_at": 1234567890
}
```

### 6.2 Delivery States

```
QUEUED -> SENDING -> SENT -> DELIVERED
   ↓         ↓
 FAILED <- RETRY
```

## 7. API Protocol

### 7.1 REST API Endpoints

```
POST   /api/v1/messages/send
GET    /api/v1/messages/list
GET    /api/v1/contacts/list
POST   /api/v1/contacts/add
GET    /api/v1/status
POST   /api/v1/webhooks/register
```

### 7.2 Authentication

```
Authorization: Bearer omni_api_<64_hex_chars>
```

### 7.3 Webhook Payload

```json
{
  "event": "message.received",
  "timestamp": 1234567890,
  "data": {
    "sender": "contact_name",
    "message_id": "uuid",
    "encrypted": true
  }
}
```

## 8. Storage Protocol

### 8.1 Database Schema

```sql
-- Contacts
CREATE TABLE contacts (
    id INTEGER PRIMARY KEY,
    name TEXT UNIQUE,
    public_key BLOB,
    fingerprint TEXT,
    trust_level TEXT
);

-- Messages
CREATE TABLE messages (
    id INTEGER PRIMARY KEY,
    contact_id INTEGER,
    direction TEXT,
    content_encrypted BLOB,
    timestamp INTEGER,
    status TEXT
);

-- Queue
CREATE TABLE message_queue (
    id INTEGER PRIMARY KEY,
    recipient TEXT,
    encrypted_content BLOB,
    priority TEXT,
    retry_count INTEGER
);
```

### 8.2 Encryption at Rest

```
Database -> SQLite -> Encrypted Blobs (AES-256-GCM)
Keys -> Files -> Permissions 0600 -> Encrypted
```

## 9. Version Compatibility

### 9.1 Protocol Versioning

```
Version Format: MAJOR.MINOR.PATCH
Current: 1.0.0

Compatibility Matrix:
1.0.x <-> 1.0.y : Full compatibility
1.x.y <-> 1.z.w : Backward compatible
2.x.y <-> 1.z.w : Not compatible
```

### 9.2 Feature Negotiation

```json
{
  "protocol_version": "1.0.0",
  "supported_ciphers": ["aes256gcm", "chacha20poly1305"],
  "supported_protocols": ["p2p", "tor", "i2p"],
  "features": ["pfs", "groups", "files", "voice"]
}
```

## 10. Error Codes

| Code | Description |
|------|-------------|
| 1000 | Success |
| 2001 | Invalid message format |
| 2002 | Decryption failed |
| 2003 | Signature verification failed |
| 3001 | Contact not found |
| 3002 | Key verification failed |
| 4001 | Network error |
| 4002 | Timeout |
| 5001 | Internal error |

---

**Specification Version**: 1.0  
**Last Updated**: 2024  
**Status**: Stable
