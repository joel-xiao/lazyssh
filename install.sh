#!/bin/bash

# LazySSH Installation Script
# This script installs LazySSH from GitHub releases or builds from source

set -euo pipefail

# Colors for output
readonly RED='\033[0;31m'
readonly GREEN='\033[0;32m'
readonly YELLOW='\033[1;33m'
readonly BLUE='\033[0;34m'
readonly NC='\033[0m' # No Color

# Configuration
readonly REPO="joel-xiao/lazyssh"
readonly INSTALL_DIR="/usr/local/bin"
readonly BINARY_NAME="lazyssh"
readonly DEFAULT_VERSION="v0.2.0"

# Global variables
OS=""
ARCH=""
EXT=""
PLATFORM=""
VERSION=""

# Print colored messages
info() {
    echo -e "${BLUE}$1${NC}"
}

success() {
    echo -e "${GREEN}$1${NC}"
}

warning() {
    echo -e "${YELLOW}$1${NC}"
}

error() {
    echo -e "${RED}$1${NC}" >&2
}

# Check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Detect OS and architecture
detect_platform() {
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
            error "Unsupported OS: $(uname -s)"
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
            error "Unsupported architecture: $(uname -m)"
            exit 1
            ;;
    esac
    
    PLATFORM="${OS}-${ARCH}"
}

# Get latest release version from GitHub API
get_latest_version() {
    local api_url="https://api.github.com/repos/${REPO}/releases/latest"
    local version=""
    local response=""
    
    if command_exists curl; then
        response=$(curl -sSL "$api_url" 2>/dev/null || echo "")
    elif command_exists wget; then
        response=$(wget -qO- "$api_url" 2>/dev/null || echo "")
    else
        error "Error: curl or wget is required"
        exit 1
    fi
    
    if [ -n "$response" ]; then
        if command_exists jq; then
            version=$(echo "$response" | jq -r '.tag_name // empty' 2>/dev/null || echo "")
        else
            version=$(echo "$response" | grep '"tag_name":' | sed -E 's/.*"tag_name":\s*"([^"]+)".*/\1/' | head -1 || echo "")
        fi
    fi
    
    if [ -z "$version" ] || [ "$version" = "null" ]; then
        warning "Could not fetch latest version, using ${DEFAULT_VERSION}"
        VERSION="$DEFAULT_VERSION"
    else
        VERSION="$version"
    fi
}

# Download file using curl or wget
download_file() {
    local url="$1"
    local output="$2"
    
    if command_exists curl; then
        curl -fsSL "$url" -o "$output"
    elif command_exists wget; then
        wget -q "$url" -O "$output"
    else
        error "Error: curl or wget is required"
        return 1
    fi
}

# Install binary to target directory
install_binary() {
    local source_file="$1"
    local target_file="$2"
    
    # Create install directory if it doesn't exist
    [ ! -d "$INSTALL_DIR" ] && sudo mkdir -p "$INSTALL_DIR"
    
    # Determine if sudo is needed
    local sudo_cmd=""
    [ ! -w "$INSTALL_DIR" ] && sudo_cmd="sudo"
    
    # Copy and set permissions
    $sudo_cmd cp "$source_file" "$target_file"
    $sudo_cmd chmod +x "$target_file"
    success "✓ Installed to $target_file"
}

# Extract archive (tar.gz or zip)
extract_archive() {
    local archive_file="$1"
    local extract_dir="$2"
    
    cd "$extract_dir" || exit 1
    
    if [ "$OS" = "windows" ]; then
        if ! command_exists unzip; then
            error "Error: unzip is required for Windows archives"
            exit 1
        fi
        unzip -q "$archive_file"
    else
        tar -xzf "$archive_file"
    fi
}

# Download and install binary from GitHub release
install_from_release() {
    info "Downloading LazySSH ${VERSION}..."
    
    # Determine asset name and URL
    local asset_name
    [ "$OS" = "windows" ] && asset_name="${BINARY_NAME}-${PLATFORM}${EXT}.zip" || asset_name="${BINARY_NAME}-${PLATFORM}${EXT}.tar.gz"
    local download_url="https://github.com/${REPO}/releases/download/${VERSION}/${asset_name}"
    
    info "Downloading from: $download_url"
    
    # Create temporary directory
    local temp_dir
    temp_dir=$(mktemp -d)
    trap "rm -rf $temp_dir" EXIT
    
    # Download the release
    local archive_path="$temp_dir/$asset_name"
    if ! download_file "$download_url" "$archive_path" 2>/dev/null || [ ! -f "$archive_path" ]; then
        warning "Release ${VERSION} not found or download failed"
        info "Attempting to build from source instead..."
        trap - EXIT
        rm -rf "$temp_dir"
        build_from_source_clone
        return
    fi
    
    # Extract archive
    extract_archive "$archive_path" "$temp_dir"
    
    # Find and install binary
    local binary_path="$temp_dir/${BINARY_NAME}${EXT}"
    [ ! -f "$binary_path" ] && { error "Error: Binary not found in archive"; exit 1; }
    
    install_binary "$binary_path" "$INSTALL_DIR/${BINARY_NAME}"
}

# Build from source (when already in repository)
build_from_source() {
    info "Building LazySSH from source..."
    
    command_exists cargo || { error "Error: Rust/Cargo is not installed."; echo "Please install Rust from https://rustup.rs/"; exit 1; }
    
    info "Compiling binary (this may take a few minutes)..."
    cargo build --release || { error "Error: Build failed"; exit 1; }
    
    local binary_path="target/release/${BINARY_NAME}"
    [ ! -f "$binary_path" ] && { error "Error: Build succeeded but binary not found"; exit 1; }
    
    install_binary "$binary_path" "$INSTALL_DIR/${BINARY_NAME}"
}

# Build from source by cloning the repository
build_from_source_clone() {
    info "Building LazySSH from source..."
    
    command_exists cargo || {
        error "Error: Rust/Cargo is not installed."
        echo "Please install Rust from https://rustup.rs/"
        echo ""
        echo "Alternatively, you can manually install:"
        echo "  git clone https://github.com/${REPO}.git"
        echo "  cd lazyssh"
        echo "  cargo build --release"
        echo "  sudo cp target/release/${BINARY_NAME} $INSTALL_DIR/${BINARY_NAME}"
        exit 1
    }
    
    command_exists git || { error "Error: git is required to build from source"; exit 1; }
    
    # Create temporary directory
    local temp_dir
    temp_dir=$(mktemp -d)
    trap "rm -rf $temp_dir" EXIT
    
    info "Cloning repository..."
    git clone --depth 1 "https://github.com/${REPO}.git" "$temp_dir/lazyssh" || { error "Error: Failed to clone repository"; exit 1; }
    
    cd "$temp_dir/lazyssh" || exit 1
    
    info "Building binary (this may take a few minutes)..."
    cargo build --release || { error "Error: Build failed"; exit 1; }
    
    local binary_path="target/release/${BINARY_NAME}"
    [ ! -f "$binary_path" ] && { error "Error: Build succeeded but binary not found"; exit 1; }
    
    install_binary "$binary_path" "$INSTALL_DIR/${BINARY_NAME}"
}

# Configure PATH environment variable
configure_path() {
    # Check if install directory is already in PATH
    if [[ ":$PATH:" == *":$INSTALL_DIR:"* ]]; then
        success "✓ $INSTALL_DIR is already in PATH"
        return
    fi
    
    echo ""
    warning "Adding $INSTALL_DIR to PATH..."
    
    # Detect shell configuration file
    local shell_name
    shell_name=$(basename "$SHELL")
    local shell_rc=""
    
    case "$shell_name" in
        bash)
            shell_rc="$HOME/.bashrc"
            ;;
        zsh)
            shell_rc="$HOME/.zshrc"
            ;;
        fish)
            shell_rc="$HOME/.config/fish/config.fish"
            ;;
        *)
            # Fallback: try common config files
            if [ -f "$HOME/.bashrc" ]; then
                shell_rc="$HOME/.bashrc"
            elif [ -f "$HOME/.zshrc" ]; then
                shell_rc="$HOME/.zshrc"
            elif [ -f "$HOME/.profile" ]; then
                shell_rc="$HOME/.profile"
            fi
            ;;
    esac
    
    # Add PATH export to config file
    if [ -n "$shell_rc" ]; then
        if grep -q "export PATH.*$INSTALL_DIR" "$shell_rc" 2>/dev/null; then
            success "✓ PATH already configured in $shell_rc"
        else
            {
                echo ""
                echo "# LazySSH - Add $INSTALL_DIR to PATH"
                echo "export PATH=\"\$PATH:$INSTALL_DIR\""
            } >> "$shell_rc"
            
            success "✓ Added PATH export to $shell_rc"
            warning "Please run: source $shell_rc"
            warning "Or restart your terminal"
        fi
    else
        warning "Could not detect shell config file"
        echo "Please manually add to your shell config:"
        echo "  export PATH=\"\$PATH:$INSTALL_DIR\""
    fi
}

# Verify installation
verify_installation() {
    local binary_path="$INSTALL_DIR/${BINARY_NAME}"
    
    if [ -f "$binary_path" ] && [ -x "$binary_path" ]; then
        echo ""
        success "✓ LazySSH installed successfully!"
        success "  Location: $binary_path"
        echo ""
        echo "You can now run: ${BINARY_NAME}"
    else
        error "Error: Installation verification failed"
        exit 1
    fi
}

# Main installation function
main() {
    success "LazySSH Installation Script"
    echo "================================"
    echo ""
    
    # Detect platform
    detect_platform
    info "Detected platform: $PLATFORM"
    info "Install directory: $INSTALL_DIR"
    echo ""
    
    # Determine installation method
    if [ -d ".git" ] && [ -f "Cargo.toml" ]; then
        info "Detected source repository, building from source..."
        build_from_source
    else
        info "Installing from GitHub Releases..."
        get_latest_version
        info "Latest version: $VERSION"
        install_from_release
    fi
    
    # Configure PATH
    configure_path
    
    # Verify installation
    verify_installation
}

# Run main function
main "$@"

