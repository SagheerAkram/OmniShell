# OmniShell Use Cases

OmniShell is designed to be versatile, serving users ranging from privacy-conscious students to field operators in high-risk environments.

## 1. Academic & Campus Environment
**Scenario**: A university campus with restrictive Wi-Fi monitoring or internet censorship.
- **Solution**:
  - **P2P Mesh**: Students connect via Bluetooth/Wi-Fi Direct to share notes/files without central servers.
  - **Offline Chat**: Communications work even if the campus internet is down or firewalled.
  - **Privacy**: End-to-end encryption ensures administration cannot snoop on student organization planning.

## 2. Disaster Relief & Off-Grid
**Scenario**: Natural disaster strikes, knocking out cell towers and internet.
- **Solution**:
  - **LoRa Mesh**: Responders use long-range (>5km) low-power radios to coordinate.
  - **MTA (Multipath)**: Critical SOS messages are fragmented and sent over every available link (Sat, LoRa, Weak WiFi) to maximize delivery chance.
  - **Ghost Reckoning**: Navigation assistants work even when GPS signals are unreliable or jammed.

## 3. Investigative Journalism & Activism
**Scenario**: Reporting from a hostile regime with surveillance and physical searches.
- **Solution**:
  - **Tor/I2P Integration**: Traffic is anonymized by default.
  - **Mirage**: If detained, a hotkey instantly switches the screen to a fake Windows Update or Code Compiler view.
  - **The Hydra**: Sensitive source material is split and stored across multiple trusted contacts; the reporter carries no single complete file.
  - **Sentry**: A laptop left in a hotel room records audio/lid-opening events to detect tampering.

## 4. Tactical & Field Operations
**Scenario**: A coordinated team operating in a contested electronic warfare environment.
- **Solution**:
  - **Blue Force Tracking**: The TUI Dashboard shows team locations in real-time.
  - **Spectrum Agility**: Radios hop frequencies every second to evade jamming and interception.
  - **Hunter Mode**: The team can triangulate the location of a rogue interference source (TDOA).
  - **Passive SIGINT**: Background processes warn of active jamming attempts.
  - **The Mole**: Data is exfiltrated slowly via ICMP (Ping) packets to bypass strict firewalls.
