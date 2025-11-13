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

# Verify installation and check PATH
INSTALLED_PATH="$INSTALL_DIR/lazyssh"
if [ -f "$INSTALLED_PATH" ]; then
    echo ""
    echo -e "${GREEN}✓ LazySSH installed successfully!${NC}"
    echo -e "${GREEN}  Location: $INSTALLED_PATH${NC}"
    
    # Check if /usr/local/bin is in PATH
    if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
        echo ""
        echo -e "${YELLOW}Adding $INSTALL_DIR to PATH...${NC}"
        
        # Detect shell
        SHELL_NAME=$(basename "$SHELL")
        SHELL_RC=""
        
        case "$SHELL_NAME" in
            bash)
                SHELL_RC="$HOME/.bashrc"
                ;;
            zsh)
                SHELL_RC="$HOME/.zshrc"
                ;;
            fish)
                SHELL_RC="$HOME/.config/fish/config.fish"
                ;;
            *)
                # Try common files
                if [ -f "$HOME/.bashrc" ]; then
                    SHELL_RC="$HOME/.bashrc"
                elif [ -f "$HOME/.zshrc" ]; then
                    SHELL_RC="$HOME/.zshrc"
                elif [ -f "$HOME/.profile" ]; then
                    SHELL_RC="$HOME/.profile"
                fi
                ;;
        esac
        
        if [ -n "$SHELL_RC" ]; then
            # Check if already added
            if ! grep -q "export PATH.*$INSTALL_DIR" "$SHELL_RC" 2>/dev/null; then
                echo "" >> "$SHELL_RC"
                echo "# LazySSH - Add /usr/local/bin to PATH" >> "$SHELL_RC"
                echo "export PATH=\"\$PATH:$INSTALL_DIR\"" >> "$SHELL_RC"
                echo -e "${GREEN}✓ Added PATH export to $SHELL_RC${NC}"
                echo -e "${YELLOW}Please run: source $SHELL_RC${NC}"
                echo -e "${YELLOW}Or restart your terminal${NC}"
            else
                echo -e "${GREEN}✓ PATH already configured in $SHELL_RC${NC}"
            fi
        else
            echo -e "${YELLOW}Could not detect shell config file${NC}"
            echo "Please manually add to your shell config:"
            echo "  export PATH=\"\$PATH:$INSTALL_DIR\""
        fi
    else
        echo -e "${GREEN}✓ $INSTALL_DIR is already in PATH${NC}"
    fi
    
    echo ""
    echo "You can now run: lazyssh"
else
    echo -e "${RED}Error: Installation failed${NC}"
    exit 1
fi
