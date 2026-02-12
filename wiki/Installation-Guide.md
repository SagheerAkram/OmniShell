# Installation Guide

## Prerequisites
- **Rust**: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- **Build Tools**: `build-essential` (Linux), `Visual Studio Build Tools` (Windows)

## Steps
1. **Clone**:
   ```bash
   git clone https://github.com/sagheerakram/omnishell
   cd omnishell
   ```

2. **Build**:
   ```bash
   cargo build --release
   ```

3. **Install**:
   ```bash
   # Linux
   sudo cp target/release/omnishell /usr/local/bin/

   # Windows
   # Add target\release\omnishell.exe to PATH
   ```

## Troubleshooting
- **Missing OpenSSL**: Install `libssl-dev` or `openssl-devel`.
- **Linker Errors**: Ensure C compiler (gcc/clang/msvc) is installed.
