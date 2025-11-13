#!/bin/bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
REPO="joel-xiao/lazyssh"
INSTALL_DIR="/usr/local/bin"
BINARY_NAME="lazyssh"

# Detect OS and architecture
detect_platform() {
    OS=""
    ARCH=""
    EXT=""
    
    case "$(uname -s)" in
        Linux*)
            OS="linux"
            EXT=""
            ;;
        Darwin*)
            OS="darwin"
            EXT=""
            ;;
        MINGW*|MSYS*|CYGWIN*)
            OS="windows"
            EXT=".exe"
            ;;
        *)
            echo -e "${RED}Unsupported OS: $(uname -s)${NC}"
            exit 1
            ;;
    esac
    
    case "$(uname -m)" in
        x86_64|amd64)
            ARCH="x86_64"
            ;;
        arm64|aarch64)
            ARCH="arm64"
            ;;
        *)
            echo -e "${RED}Unsupported architecture: $(uname -m)${NC}"
            exit 1
            ;;
    esac
    
    PLATFORM="${OS}-${ARCH}"
}

# Get latest release version
get_latest_version() {
    if command -v curl &> /dev/null; then
        VERSION=$(curl -s "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
    elif command -v wget &> /dev/null; then
        VERSION=$(wget -qO- "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
    else
        echo -e "${RED}Error: curl or wget is required${NC}"
        exit 1
    fi
    
    if [ -z "$VERSION" ]; then
        echo -e "${YELLOW}Warning: Could not fetch latest version, using v0.2.0${NC}"
        VERSION="v0.2.0"
    fi
}

# Download and install binary
install_from_release() {
    echo -e "${GREEN}Downloading LazySSH ${VERSION}...${NC}"
    
    if [ "$OS" = "windows" ]; then
        ASSET_NAME="${BINARY_NAME}-${PLATFORM}${EXT}.zip"
        DOWNLOAD_URL="https://github.com/${REPO}/releases/download/${VERSION}/${ASSET_NAME}"
    else
        ASSET_NAME="${BINARY_NAME}-${PLATFORM}${EXT}.tar.gz"
        DOWNLOAD_URL="https://github.com/${REPO}/releases/download/${VERSION}/${ASSET_NAME}"
    fi
    
    TEMP_DIR=$(mktemp -d)
    trap "rm -rf $TEMP_DIR" EXIT
    
    echo -e "${BLUE}Downloading from: $DOWNLOAD_URL${NC}"
    
    if command -v curl &> /dev/null; then
        curl -fsSL "$DOWNLOAD_URL" -o "$TEMP_DIR/${ASSET_NAME}"
    elif command -v wget &> /dev/null; then
        wget -q "$DOWNLOAD_URL" -O "$TEMP_DIR/${ASSET_NAME}"
    else
        echo -e "${RED}Error: curl or wget is required${NC}"
        exit 1
    fi
    
    if [ ! -f "$TEMP_DIR/${ASSET_NAME}" ]; then
        echo -e "${RED}Error: Download failed${NC}"
        exit 1
    fi
    
    # Extract
    cd "$TEMP_DIR"
    if [ "$OS" = "windows" ]; then
        if command -v unzip &> /dev/null; then
            unzip -q "${ASSET_NAME}"
        else
            echo -e "${RED}Error: unzip is required${NC}"
            exit 1
        fi
    else
        tar -xzf "${ASSET_NAME}"
    fi
    
    # Install
    if [ -f "${BINARY_NAME}${EXT}" ]; then
        if [ ! -d "$INSTALL_DIR" ]; then
            sudo mkdir -p "$INSTALL_DIR"
        fi
        
        if [ -w "$INSTALL_DIR" ]; then
            SUDO=""
        else
            SUDO="sudo"
        fi
        
        $SUDO cp "${BINARY_NAME}${EXT}" "$INSTALL_DIR/${BINARY_NAME}"
        $SUDO chmod +x "$INSTALL_DIR/${BINARY_NAME}"
        echo -e "${GREEN}✓ Installed to $INSTALL_DIR/${BINARY_NAME}${NC}"
    else
        echo -e "${RED}Error: Binary not found in archive${NC}"
        exit 1
    fi
}

# Build from source
build_from_source() {
    echo -e "${GREEN}Building LazySSH from source...${NC}"
    
    if ! command -v cargo &> /dev/null; then
        echo -e "${RED}Error: Rust/Cargo is not installed.${NC}"
        echo "Please install Rust from https://rustup.rs/"
        exit 1
    fi
    
    cargo build --release
    
    if [ ! -f "target/release/${BINARY_NAME}" ]; then
        echo -e "${RED}Error: Build failed or binary not found${NC}"
        exit 1
    fi
    
    if [ ! -d "$INSTALL_DIR" ]; then
        sudo mkdir -p "$INSTALL_DIR"
    fi
    
    if [ -w "$INSTALL_DIR" ]; then
        SUDO=""
    else
        SUDO="sudo"
    fi
    
    $SUDO cp "target/release/${BINARY_NAME}" "$INSTALL_DIR/${BINARY_NAME}"
    $SUDO chmod +x "$INSTALL_DIR/${BINARY_NAME}"
    echo -e "${GREEN}✓ Built and installed to $INSTALL_DIR/${BINARY_NAME}${NC}"
}

# Configure PATH
configure_path() {
    if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
        echo ""
        echo -e "${YELLOW}Adding $INSTALL_DIR to PATH...${NC}"
        
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
}

# Main installation function
main() {
    echo -e "${GREEN}LazySSH Installation Script${NC}"
    echo "================================"
    echo ""
    
    detect_platform
    echo -e "${BLUE}Detected platform: $PLATFORM${NC}"
    echo -e "${BLUE}Install directory: $INSTALL_DIR${NC}"
    echo ""
    
    # Check if we're in a git repository (source install)
    if [ -d ".git" ] && [ -f "Cargo.toml" ]; then
        echo -e "${GREEN}Detected source repository, building from source...${NC}"
        build_from_source
    else
        # Try to install from release
        echo -e "${GREEN}Installing from GitHub Releases...${NC}"
        get_latest_version
        echo -e "${BLUE}Latest version: $VERSION${NC}"
        install_from_release
    fi
    
    # Configure PATH
    configure_path
    
    # Verify installation
    if [ -f "$INSTALL_DIR/${BINARY_NAME}" ]; then
        echo ""
        echo -e "${GREEN}✓ LazySSH installed successfully!${NC}"
        echo -e "${GREEN}  Location: $INSTALL_DIR/${BINARY_NAME}${NC}"
        echo ""
        echo "You can now run: ${BINARY_NAME}"
    else
        echo -e "${RED}Error: Installation verification failed${NC}"
        exit 1
    fi
}

# Run main function
main
