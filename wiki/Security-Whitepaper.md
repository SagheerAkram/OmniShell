# OmniShell Security Whitepaper

## Executive Summary

OmniShell is a military-grade encrypted messaging CLI application implementing state-of-the-art cryptographic protocols and privacy-preserving technologies. This whitepaper details the security architecture, threat model, and cryptographic implementations.

## 1. Cryptographic Foundations

### 1.1 Key Generation
- **Algorithm**: Ed25519 (Curve25519)
- **Key Size**: 256-bit
- **Random Number Generation**: OS-provided CSPRNG
- **Key Storage**: Encrypted at rest with file permissions 0600

### 1.2 Encryption Algorithms

#### Primary Cipher: AES-256-GCM
- **Mode**: Galois/Counter Mode (authenticated encryption)
- **Key Size**: 256 bits
- **Nonce**: 96-bit random (never reused)
- **Authentication Tag**: 128 bits
- **Security Level**: 256-bit security

#### Alternative Cipher: ChaCha20-Poly1305
- **Stream Cipher**: ChaCha20
- **MAC**: Poly1305
- **Key Size**: 256 bits
- **Nonce**: 96-bit random
- **Security Level**: 256-bit security

### 1.3 Key Exchange
- **Protocol**: X25519 (ECDH on Curve25519)
- **Shared Secret Derivation**: HKDF-SHA256
- **Forward Secrecy**: Implemented via Double Ratchet

### 1.4 Digital Signatures
- **Algorithm**: Ed25519
- **Signature Size**: 64 bytes
- **Verification**: Constant-time operations
- **Use Cases**: Message authentication, identity verification

### 1.5 Key Derivation
- **Password-Based**: Argon2id
  - Memory: 64 MB
  - Iterations: 3
  - Parallelism: 4 threads
- **Key-Based**: HKDF-SHA256

## 2. Perfect Forward Secrecy (PFS)

### 2.1 Double Ratchet Algorithm
OmniShell implements the Signal Protocol's Double Ratchet algorithm:

1. **DH Ratchet**: New ephemeral key pair per message
2. **Symmetric Ratchet**: KDF chains for message keys
3. **Root Key**: Derived from DH ratchet output
4. **Chain Keys**: Separate sending and receiving chains
5. **Message Keys**: One-time keys derived from chain keys

### 2.2 Key Rotation
- **Automatic**: Keys rotate every 1000 messages
- **Manual**: `omnishell rotate-keys` command
- **Archive**: Old keys stored for historical decryption

## 3. Network Security

### 3.1 Protocol Support
1. **P2P**: Direct encrypted connections
2. **Tor**: Onion routing (.onion addresses)
3. **I2P**: Garlic routing (.i2p destinations)
4. **Relay Nodes**: Multi-hop routing

### 3.2 Metadata Protection
- **Encrypted Headers**: All message metadata encrypted
- **Padding**: Random padding to obscure message sizes
- **Timing Obfuscation**: Random delays in relay routing

### 3.3 Anonymous Routing
- **Onion Routing**: Messages wrapped in multiple encryption layers
- **Relay Chain**: Configurable number of hops (default: 3)
- **No Logging**: Relay nodes don't log traffic

## 4. Authentication & Access Control

### 4.1 Multi-Factor Authentication
- **Master Password**: Argon2id-derived key
- **2FA**: TOTP (Time-based One-Time Password)
- **Biometric**: Platform-dependent (future)

### 4.2 Duress Protection
- **Duress Password**: Shows decoy data when entered
- **Honeypot Mode**: Fake conversations and contacts
- **Panic Mode**: Secure wipe of all data

### 4.3 Key Verification
- **Fingerprints**: SHA-256 hash of public keys
- **Visual Hash**: Unique visual representation
- **Out-of-Band**: QR codes for key exchange

## 5. Data Protection

### 5.1 At-Rest Encryption
- **Database**: SQLite with encrypted blobs
- **Files**: AES-256-GCM encryption
- **Backups**: Password-protected tar.gz.enc

### 5.2 Secure Deletion
- **Overwrite**: Multiple passes (DoD 5220.22-M)
- **Panic Mode**: Immediate secure wipe
- **Auto-Cleanup**: Configurable message retention

### 5.3 Memory Safety
- **Language**: Rust (memory-safe by design)
- **Zeroization**: Sensitive data cleared from memory
- **No Swap**: Recommendation to disable swap for keys

## 6. Threat Model

### 6.1 Adversary Capabilities
**Assumed Adversaries:**
1. **Passive Network Observer**: Can monitor all network traffic
2. **Active MITM**: Can intercept and modify traffic
3. **Compromised Relay**: Controls some relay nodes
4. **Local Attacker**: Physical access to device

**NOT Protected Against:**
- Compromised operating system
- Hardware keyloggers
- Malicious plugins (user-installed)

### 6.2 Attack Vectors & Mitigations

| Attack Vector | Mitigation |
|--------------|------------|
| Traffic Analysis | Tor/I2P routing, padding |
| MITM | Public key verification, signatures |
| Replay Attacks | Nonces, timestamps, sequence numbers |
| Key Compromise | Perfect Forward Secrecy |
| Brute Force | Argon2id, rate limiting |
| Side Channels | Constant-time crypto operations |

## 7. Security Features

### 7.1 Emergency Features
- **Emergency Broadcast**: Alert all contacts
- **Dead Man's Switch**: Auto-alert if inactive
- **Geofencing**: Location-based security policies
- **Screenshot Detection**: Alert on screenshot attempts

### 7.2 Privacy Features
- **Stealth Mode**: Maximum anonymity (ChaCha20 + Tor)
- **Disappearing Messages**: Auto-delete after TTL
- **No Metadata**: Minimal metadata collection
- **Offline Mode**: No network required

## 8. Compliance & Standards

### 8.1 Cryptographic Standards
- **FIPS 140-2**: Compliant algorithms
- **NIST**: Recommended curves and parameters
- **RFC Compliance**: Ed25519 (RFC 8032), ChaCha20-Poly1305 (RFC 8439)

### 8.2 Best Practices
- **OWASP**: Secure coding practices
- **CWE**: Common weakness mitigation
- **SANS**: Security implementation guidelines

## 9. Security Audit Recommendations

### 9.1 Regular Audits
1. **Code Review**: Quarterly security code review
2. **Penetration Testing**: Annual pen tests
3. **Cryptographic Review**: Expert cryptographer review
4. **Dependency Audit**: Monthly dependency scanning

### 9.2 Incident Response
1. **Key Rotation**: Immediate on suspected compromise
2. **Notification**: Alert all contacts
3. **Forensics**: Preserve logs for analysis
4. **Patch**: Rapid security patch deployment

## 10. Limitations & Future Work

### 10.1 Known Limitations
- Relay nodes require trust
- No protection against compromised OS
- Voice/video not yet implemented
- Mobile apps not available

### 10.2 Future Enhancements
- Post-quantum cryptography (Kyber, Dilithium)
- Hardware security module (HSM) support
- Secure enclaves (SGX, TrustZone)
- Formal verification of crypto code

## 11. Conclusion

OmniShell provides military-grade security through:
- State-of-the-art cryptography (Ed25519, AES-256-GCM)
- Perfect Forward Secrecy (Double Ratchet)
- Anonymous routing (Tor, I2P, relay nodes)
- Comprehensive threat mitigation
- Privacy-first design

**Security Level**: Suitable for high-security communications requiring confidentiality, integrity, and anonymity.

---

**Version**: 1.0  
**Date**: 2024  
**License**: MIT  
**Contact**: security@omnishell.dev
