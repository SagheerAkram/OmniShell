# OmniShell Security Whitepaper

## 1. Core Cryptography
OmniShell uses a "Defense in Depth" approach, relying on modern, audited cryptographic primitives.

### Encryption at Rest & In Transit
- **Algorithm**: XChaCha20-Poly1305 (via `sodium` or `rust-crypto`).
- **Properties**: Authenticated Encryption with Associated Data (AEAD).
- **Forward Secrecy**: Ephemeral keys are generated for each session/message.

### Key Derivation
- **Algorithm**: Argon2id.
- **Parameters**: Tuned for high-memory usage to resist GPU/ASIC cracking.
- **Salt**: 32-byte secure random salt per user.

## 2. Network Resilience (MTA)
**Multipath Transport Aggregation** breaks the reliance on a single link.
- **Fragmentation**: Messages are split into `N` chunks.
- **Dispersion**: Chunks are routed via different protocols (e.g., Chunk A via Tor, Chunk B via LoRa).
- **Reassembly**: The receiver validates checksums and reassembles the message.
- **Security Benefit**: An interceptor monitoring only one channel (e.g., WiFi) gets incomplete, useless data.

## 3. Distributed Security ("The Hydra")
Based on **Shamir's Secret Sharing (SSS)**.
- **Concept**: A secret $S$ is divided into $n$ shares such that any $k$ shares can reconstruct $S$.
- **Implementation**: `GF(2^8)` finite field arithmetic.
- **Use Case**: Critical decryption keys are never stored fully on one device. They are reconstructed only in RAM when a quorum of team members is present.

## 4. Physical & Environmental Security
### Sentry Mode
- Uses hardware sensors (Microphone, Accelerometer, Lid Switch).
- Heuristics detect unauthorized physical access.
- **Response**: Immediate memory wipe and shutdown.

### Mirage (Panic Camouflage)
- **Concept**: Social Engineering defense.
- **Mechanism**: Overlays the OS shell with an interactive, fake UI (e.g., Windows Update).
- **Goal**: Provide plausible deniability during "shoulder surfing" or surprise inspections.

## 5. Electronic Warfare (EW) Resilience
- **Spectrum Agility**: Software-Defined Frequency Hopping (FHSS) minimizes jamming impact.
- **The Mole**: Steganographic tunneling over ICMP (Ping) allows communication through strict firewalls that block standard TCP/UDP ports.
- **TDOA**: Time Difference of Arrival algorithms allow users to locate RF sources, turning defense into offense.

## 6. Threat Model
| Threat | Mitigation |
|:---|:---|
| **Deep Packet Inspection (DPI)** | MTA, Tor/I2P, The Mole |
| **Physical Seizure** | Full Disk Encryption, Sentry, Hydra |
| **Signal Jamming** | Spectrum Agility, LoRa Mesh, MTA |
| **Geolocation/Tracking** | Ghost Reckoning (GPS-free nav), Tor |
| **Coercion/Torture** | Mirage (Plausible Deniability), Hidden Volumes |
