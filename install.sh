#!/bin/bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}LazySSH Installation Script${NC}"
echo "================================"
echo ""

# Installation directory (same as lazygit default location)
INSTALL_DIR="/usr/local/bin"

echo -e "${GREEN}Will install lazyssh to: $INSTALL_DIR${NC}"
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: Rust/Cargo is not installed.${NC}"
    echo "Please install Rust from https://rustup.rs/"
    exit 1
fi

# Build the project
echo -e "${GREEN}Building LazySSH...${NC}"
cargo build --release

if [ ! -f "target/release/lazyssh" ]; then
    echo -e "${RED}Error: Build failed or binary not found${NC}"
    exit 1
fi

# Check if install directory exists and is writable
if [ ! -d "$INSTALL_DIR" ]; then
    echo -e "${YELLOW}Creating directory: $INSTALL_DIR${NC}"
    sudo mkdir -p "$INSTALL_DIR"
fi

# Check if we need sudo
if [ -w "$INSTALL_DIR" ]; then
    SUDO=""
else
    SUDO="sudo"
    echo -e "${YELLOW}Need sudo permissions to install to $INSTALL_DIR${NC}"
fi

# Install binary
echo -e "${GREEN}Installing lazyssh to $INSTALL_DIR...${NC}"
$SUDO cp target/release/lazyssh "$INSTALL_DIR/lazyssh"
$SUDO chmod +x "$INSTALL_DIR/lazyssh"

# Verify installation
if command -v lazyssh &> /dev/null; then
    INSTALLED_PATH=$(which lazyssh)
    echo ""
    echo -e "${GREEN}✓ LazySSH installed successfully!${NC}"
    echo -e "${GREEN}  Location: $INSTALLED_PATH${NC}"
    echo ""
    echo "You can now run: lazyssh"
else
    echo -e "${YELLOW}Warning: lazyssh not found in PATH${NC}"
    echo "You may need to add $INSTALL_DIR to your PATH"
fi
