# 🔧 Installation & Build Guide

## Prerequisites

### 1. Install Rust

OmniShell is written in Rust. You need to install the Rust toolchain first.

#### Windows

Download and run the Rust installer from [rustup.rs](https://rustup.rs/):

```powershell
# Download and run rustup-init.exe
# Or use winget:
winget install Rustlang.Rustup
```

After installation, restart your terminal and verify:

```powershell
cargo --version
rustc --version
```

#### Linux / macOS

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

Verify installation:

```bash
cargo --version
rustc --version
```

### 2. Install System Dependencies

#### Windows

No additional dependencies required. The Rust toolchain includes everything needed.

#### Linux (Ubuntu/Debian)

```bash
sudo apt update
sudo apt install build-essential pkg-config libssl-dev sqlite3
```

#### Linux (Fedora/RHEL)

```bash
sudo dnf install gcc openssl-devel sqlite-devel
```

#### macOS

```bash
# Install Xcode Command Line Tools
xcode-select --install

# Install Homebrew (if not already installed)
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install dependencies
brew install openssl sqlite
```

---

## Building OmniShell

### 1. Clone the Repository

```bash
git clone https://github.com/SagheerAkram/OmniShell.git
cd omnishell
```

### 2. Build the Project

#### Debug Build (for development)

```bash
cargo build
```

The binary will be at: `target/debug/omnishell` (or `omnishell.exe` on Windows)

#### Release Build (optimized)

```bash
cargo build --release
```

The optimized binary will be at: `target/release/omnishell`

#### Build with Tor Support

```bash
cargo build --release --features tor
```

### 3. Run Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_encryption
```

### 4. Install Locally

```bash
# Install to ~/.cargo/bin (or %USERPROFILE%\.cargo\bin on Windows)
cargo install --path .
```

After installation, `omnishell` will be available in your PATH.

---

## Automated Installation (Recommended)

### Using Install Scripts

The easiest way to install OmniShell is using the provided install scripts:

#### Windows

```powershell
# Run the install script
powershell -ExecutionPolicy Bypass -File install.ps1
```

The script will:
- Build the release binary
- Copy `omnishell.exe` to `C:\Program Files\OmniShell\`
- Add it to your system PATH
- Make `omnishell` available system-wide

#### Linux / macOS

```bash
# Make the script executable
chmod +x install.sh

# Run the install script
./install.sh
```

The script will:
- Build the release binary
- Copy `omnishell` to `/usr/local/bin/`
- Set proper permissions
- Make `omnishell` available system-wide

### Manual PATH Setup

If you prefer to run from the project directory:

```powershell
# Windows PowerShell
[Environment]::SetEnvironmentVariable("Path", $env:Path + ";C:\path\to\OmniShell", [EnvironmentVariableTarget]::User)

# Then restart PowerShell
```

```bash
# Linux/macOS
echo 'export PATH="$PATH:/path/to/OmniShell"' >> ~/.bashrc
source ~/.bashrc
```

---

## First Run

### Initialize OmniShell

```bash
# Initialize (creates keys, database, and config)
omnishell init
```

This will:
1. Create `~/.omnishell/` directory structure
2. Generate Ed25519 key pair (256-bit)
3. **Initialize SQLite database** ✅
4. Create configuration file
5. Generate QR code for your public key
6. Display your identity

### View Your Identity

```bash
omnishell whoami
```

You'll see:
- Your public key (share this with contacts)
- Fingerprint for verification
- Visual hash
- Device ID
- QR code location

### Add Your First Contact

```bash
omnishell add alice omni:PUBLIC_KEY_HERE
```

### Send a Message

```bash
omnishell msg @alice "Hello, secure world!"
```

### Read Messages

```bash
omnishell read @alice
```

### Check Statistics

```bash
omnishell stats
```

---

## Development Setup

### VS Code Setup

Install recommended extensions:

```json
{
  "recommendations": [
    "rust-lang.rust-analyzer",
    "vadimcn.vscode-lldb",
    "serayuzgur.crates"
  ]
}
```

### Run in Development Mode

```bash
# Run without building
cargo run -- init

# Run with arguments
cargo run -- msg @alice "Hello"

# Run with logging
RUST_LOG=debug cargo run -- status
```

### Code Formatting

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check
```

### Linting

```bash
# Run clippy (Rust linter)
cargo clippy

# Fix clippy warnings
cargo clippy --fix
```

---

## Troubleshooting

### Issue: "cargo: command not found"

**Solution**: Rust is not installed or not in PATH.

- **Windows**: Restart your terminal after installing Rust
- **Linux/macOS**: Run `source $HOME/.cargo/env`

### Issue: "linker 'cc' not found"

**Solution**: Install C compiler.

- **Windows**: Install Visual Studio Build Tools
- **Linux**: `sudo apt install build-essential`
- **macOS**: `xcode-select --install`

### Issue: "failed to run custom build command for openssl-sys"

**Solution**: Install OpenSSL development files.

- **Windows**: OpenSSL is bundled, this shouldn't happen
- **Linux**: `sudo apt install libssl-dev`
- **macOS**: `brew install openssl`

### Issue: SQLite errors

**Solution**: Install SQLite development files.

- **Linux**: `sudo apt install libsqlite3-dev`
- **macOS**: `brew install sqlite`

### Issue: Slow compilation

**Solution**: Use release mode or enable parallel compilation.

```bash
# Set number of parallel jobs
export CARGO_BUILD_JOBS=4
cargo build --release
```

---

## Platform-Specific Notes

### Windows

- Use PowerShell or Windows Terminal for best experience
- Antivirus may flag the binary (false positive) - add exception if needed
- File paths use backslashes: `C:\Users\YourName\.omnishell\`

### Linux

- Ensure `~/.cargo/bin` is in your PATH
- May need to install additional libraries for GUI features (future)
- File permissions: Private key automatically set to 0600

### macOS

- May need to allow the binary in System Preferences > Security
- Use Terminal or iTerm2
- Homebrew recommended for dependencies

---

## Binary Distribution

### Create Standalone Binary

```bash
# Build with all optimizations
cargo build --release --features full

# Strip debug symbols (Linux/macOS)
strip target/release/omnishell

# Create distributable archive
tar -czf omnishell-linux-x64.tar.gz -C target/release omnishell
```

### Cross-Compilation

Install cross-compilation tools:

```bash
cargo install cross

# Build for different targets
cross build --target x86_64-pc-windows-gnu --release
cross build --target x86_64-unknown-linux-musl --release
cross build --target aarch64-unknown-linux-gnu --release
```

---

## Docker Build (Optional)

```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl3 sqlite3 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/omnishell /usr/local/bin/
ENTRYPOINT ["omnishell"]
```

Build and run:

```bash
docker build -t omnishell .
docker run -it -v ~/.omnishell:/root/.omnishell omnishell init
```

---

## Performance Optimization

### Build with Maximum Optimization

Edit `Cargo.toml`:

```toml
[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1
strip = true
panic = "abort"
```

Then build:

```bash
cargo build --release
```

### Reduce Binary Size

```bash
# Install cargo-bloat to analyze binary size
cargo install cargo-bloat

# Analyze what's taking space
cargo bloat --release

# Build with size optimization
cargo build --release --config 'profile.release.opt-level="z"'
```

---

## Next Steps

After successful installation:

1. ✅ Run `omnishell init` to set up your identity
2. ✅ Run `omnishell whoami` to view your public key
3. ✅ Share your public key with contacts
4. ✅ Add contacts with `omnishell add <name> <public_key>`
5. ✅ Start messaging with `omnishell msg @<name> "Hello!"`

For more information, see the [README.md](README.md) and [User Guide](docs/USER_GUIDE.md).

---

## Support

- **Issues**: [GitHub Issues](https://github.com/SagheerAkram/OmniShell/issues)
- **Discussions**: [GitHub Discussions](https://github.com/SagheerAkram/OmniShell/discussions)
- **Documentation**: [docs/](docs/)
