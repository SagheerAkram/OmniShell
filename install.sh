#!/bin/bash
# OmniShell Installation Script for Linux/macOS

set -e

echo "╔════════════════════════════════════════════════════════════════╗"
echo "║              OmniShell Installation Script                     ║"
echo "╚════════════════════════════════════════════════════════════════╝"
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Check if running as root
if [ "$EUID" -eq 0 ]; then 
    echo -e "${RED}✗ Please do not run as root${NC}"
    exit 1
fi

# Detect OS
OS="$(uname -s)"
case "${OS}" in
    Linux*)     MACHINE=Linux;;
    Darwin*)    MACHINE=Mac;;
    *)          MACHINE="UNKNOWN:${OS}"
esac

echo -e "${CYAN}→${NC} Detected OS: ${MACHINE}"
echo ""

# Check for Rust
echo -e "${CYAN}→${NC} Checking for Rust..."
if ! command -v cargo &> /dev/null; then
    echo -e "${YELLOW}⚠${NC}  Rust not found. Installing..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
    echo -e "${GREEN}✓${NC} Rust installed"
else
    echo -e "${GREEN}✓${NC} Rust found: $(rustc --version)"
fi
echo ""

# Install system dependencies
echo -e "${CYAN}→${NC} Installing system dependencies..."
if [ "$MACHINE" = "Linux" ]; then
    if command -v apt-get &> /dev/null; then
        sudo apt-get update
        sudo apt-get install -y build-essential pkg-config libssl-dev sqlite3 tor i2pd
    elif command -v dnf &> /dev/null; then
        sudo dnf install -y gcc pkg-config openssl-devel sqlite tor i2pd
    elif command -v pacman &> /dev/null; then
        sudo pacman -S --noconfirm base-devel openssl sqlite tor i2pd
    fi
elif [ "$MACHINE" = "Mac" ]; then
    if ! command -v brew &> /dev/null; then
        echo -e "${YELLOW}⚠${NC}  Homebrew not found. Please install from https://brew.sh"
        exit 1
    fi
    brew install openssl sqlite tor i2p
fi

# Verify Tor installation
if command -v tor &> /dev/null; then
    echo -e "${GREEN}✓${NC} Tor is installed"
else
    echo -e "${YELLOW}⚠${NC}  Tor installation failed or not found in PATH"
fi

# Verify I2P installation
if command -v i2pd &> /dev/null || command -v i2prouter &> /dev/null; then
    echo -e "${GREEN}✓${NC} I2P is installed"
else
    echo -e "${YELLOW}⚠${NC}  I2P installation failed. You may need to install it manually."
fi

echo -e "${GREEN}✓${NC} Dependencies installed"
echo ""

# Build OmniShell
echo -e "${CYAN}→${NC} Building OmniShell..."
cargo build --release
echo -e "${GREEN}✓${NC} Build complete"
echo ""

# Install binary
echo -e "${CYAN}→${NC} Installing binary..."
INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"
cp target/release/omnishell "$INSTALL_DIR/"
chmod +x "$INSTALL_DIR/omnishell"
echo -e "${GREEN}✓${NC} Binary installed to $INSTALL_DIR/omnishell"
echo ""

# Add to PATH if not already
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo -e "${YELLOW}⚠${NC}  Adding $INSTALL_DIR to PATH..."
    
    SHELL_RC="$HOME/.bashrc"
    if [ -f "$HOME/.zshrc" ]; then
        SHELL_RC="$HOME/.zshrc"
    fi
    
    echo "" >> "$SHELL_RC"
    echo "# OmniShell" >> "$SHELL_RC"
    echo "export PATH=\"\$PATH:$INSTALL_DIR\"" >> "$SHELL_RC"
    
    echo -e "${GREEN}✓${NC} Added to PATH in $SHELL_RC"
    echo -e "${YELLOW}⚠${NC}  Please run: source $SHELL_RC"
fi
echo ""

# Initialize OmniShell
echo -e "${CYAN}→${NC} Initializing OmniShell..."
"$INSTALL_DIR/omnishell" init || true
echo ""

echo "╔════════════════════════════════════════════════════════════════╗"
echo "║              Installation Complete!                            ║"
echo "╚════════════════════════════════════════════════════════════════╝"
echo ""
echo -e "${GREEN}✓${NC} OmniShell is now installed!"
echo ""
echo "Quick Start:"
echo "  omnishell whoami          # View your identity"
echo "  omnishell add <name> <key> # Add a contact"
echo "  omnishell msg @alice \"Hi!\" # Send a message"
echo "  omnishell help            # Show all commands"
echo ""
echo "Documentation:"
echo "  README.md     - Overview and features"
echo "  INSTALL.md    - Detailed installation guide"
echo "  COMMANDS.md   - Command reference"
echo "  SECURITY.md   - Security whitepaper"
echo ""
