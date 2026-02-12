# Development Guide

## Architecture
- `src/main.rs`: Entry point and CLI parsing.
- `src/network/`: Protocol implementations (Tor, I2P, LoRa).
- `src/security/`: Crypto and Tactical modules (Sentry, Hydra).
- `src/plugins/`: WASM constraints and host functions.

## Contributing
1. Fork the repo.
2. Create a feature branch: `git checkout -b feat/my-feature`
3. Commit changes.
4. Push and PR.

## Style
- Run `cargo fmt` before committing.
- Ensure `cargo check` passes.
- Document new commands in `FEATURES.md`.
