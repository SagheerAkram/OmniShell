# 🛡️ Tactical Features (Protocol Level 5)

OmniShell is not just a chat app; it is a **Cyber-Physical Operating System**. Most advanced features are **Default Off** to maintain operational security (OPSEC) and battery life.

To enable these features, edit the `features.toml` file in your installation directory.

## ⚠️ Warning
These features are designed for high-risk environments. Understand the capabilities before activating them.

---

## 🔒 Physical Security

### Sentry Mode (`sentry_mode`)
**Scenario**: You are sleeping in a hostile environment (hotel, temporary base) and cannot watch your laptop.
**Function**: Turns your device into a physical perimeter alarm.
- **Sensors**: Monitors Lid Open events, AC Power disconnection, and Motion (accelerometer).
- **Trigger**: If tampered with, it wipes ephemeral encryption keys and captures a photo of the intruder.
**Usage**: 
```bash
omni sentry --arm
```

### The Hydra (`the_hydra`)
**Scenario**: You need to store critical intel (nuclear codes, wallet seeds) but cannot trust a single device to hold it.
**Function**: Splits a secret into $N$ mathematical shards using **Shamir's Secret Sharing**.
- **Math**: You need $K$ shards to reconstruct the secret. Anything less yields zero data.
- **Usage**:
```bash
# Split a secret into 5 shards, requiring 3 to recover
omni hydra split --secret "MyCriticalIntel" -n 5 -k 3

# Recover (need 3 valid shards)
omni hydra recover "shard_1_hex..." "shard_2_hex..." "shard_3_hex..."
```

### Sonar (`sonar`)
**Scenario**: You are in an air-gapped room or Wi-Fi is monitored.
**Function**: Transmits data via **near-ultrasonic sound waves** (18.5kHz - 19kHz).
- **Usage**:
```bash
# Sender
omni sonar --send "Here is the key"

# Receiver
omni sonar --listen
```

### Mirage (`mirage`)
**Scenario**: You are in a public place and someone is watching your screen.
**Function**: Instantly replaces the UI with a fake, benign screen.
- **Modes**:
    - `update`: Fake Windows Update progress bar.
    - `logs`: Fake Linux system logs.
    - `code`: Fake random coding session.
- **Usage**:
```bash
omni mirage --mode update
```

### The Mole (`the_mole`)
**Scenario**: You are behind a restrictive firewall that blocks everything except Ping.
**Function**: Exfiltrates data by hiding it inside **ICMP Ping packets**.
- **Usage**:
```bash
# Send hidden data to a listening server
omni mole --target 8.8.8.8 --data "Secret Payload"
```
*Note: Requires root/sudo privileges to send raw sockets.*

### Spectrum Agility (`spectrum_agility`)
**Scenario**: Adversaries are jamming or monitoring specific frequencies.
**Function**: Simulates **Frequency Hopping Spread Spectrum (FHSS)**. Radios hop to a new frequency every second based on a shared seed.
- **Usage**:
```bash
# Start hopping using "OperationAlpha" as the key
omni agility --monitor --seed "OperationAlpha"
```

### Ghost Reckoning (`ghost_reckoning`)
**Scenario**: GPS jamming or denied environment (indoor/tunnel).
**Function**: Navigation via **Dead Reckoning**. Uses Inertial Measurement Unit (IMU) simulation to estimate position from a known starting point.
- **Usage**:
```bash
# Start DR navigation from last known GPS coordinates
omni ghost --start --lat 34.05 --lon -118.25
```

### Hunter Mode (`hunter_mode`)
**Scenario**: Locating a rogue transmitter (jammer/beacon).
**Function**: Simulates **TDOA (Time Difference of Arrival)** Geolocation using distributed sensors.
- **Usage**:
```bash
# Scan and triangulate signals
omni hunter --scan
```

---

## 📡 Electronic Warfare (SIGINT)

### Passive SIGINT (`passive_sigint`)
**Scenario**: You suspect you are being jammed or tracked.
**Function**: Background process that analyzes radio spectrum data (from internal Wi-Fi/Bluetooth or external SDR).
- **Jamming Detection**: Alerts on sudden noise floor spikes (900MHz/2.4GHz).
- **Surveillance**: Detects unknown Bluetooth LE beacons following you.
**Usage**: 
Enabled automatically in background. Alerts appear in `omni tactical` dashboard.

### Triangulate (`triangulate`)
*Currently Disabled by Default*
**Scenario**: Finding the physical location of a jammer or lost team member.
**Function**: Collaborative TDOA (Time Difference of Arrival) using multiple mesh nodes.

---

## 🧠 Intelligence & Extensibility

### Cortex Plugin Engine (`cortex_plugins`)
**Scenario**: You need custom functionality (weather bots, dead-drop logic) without recompiling the core.
**Function**: Runs sandboxed WebAssembly (WASM) modules.
**Usage**:
```bash
omni plugin install my_script.wasm
```

### The Mule (`the_mule`)
*Currently Disabled by Default*
**Scenario**: Sneakernet bridge for air-gapped data transfer.
**Function**: Auto-ingests/exgests encrypted packets from specific USB drives.

---

## 🔧 Configuration

Edit `features.toml` to toggle capabilities:

```toml
[features]
sentry_mode = true
the_hydra = true
passive_sigint = true
cortex_plugins = true
```
