**English** | [中文](README.zh.md)

# LazySSH

A cross-platform SSH management tool written in **Rust**, inspired by `lazygit`.  
It provides a **graphical TUI interface** for managing SSH hosts with support for **auto-login and command execution**.

## Features

- **Graphical TUI interface** for managing SSH hosts
- **Add, edit, delete hosts** with an intuitive form editor
- **Password storage** (local config file, optional sshpass auto-login)
- **Multi-line command support** with interactive shell after execution
- **Copy/paste functionality**: Quickly copy SSH commands, paste from clipboard
- **Delete confirmation**: Prevents accidental deletion of host configurations
- **Uses system SSH** - no additional SSH libraries required
- **Cross-platform** - Linux/macOS/Windows support

## Installation

### Quick Install (Recommended)

```bash
curl -fsSL https://raw.githubusercontent.com/joel-xiao/lazyssh/main/install.sh | bash
```

The script automatically detects your platform, downloads the latest release, installs to `/usr/local/bin`, and configures PATH. Falls back to building from source if release is unavailable.

### Pre-built Binaries

Download from [Releases](https://github.com/joel-xiao/lazyssh/releases) and extract:

```bash
tar -xzf lazyssh-linux-x86_64.tar.gz
sudo mv lazyssh /usr/local/bin/
sudo chmod +x /usr/local/bin/lazyssh
```

### Build from Source

**Prerequisites:** Rust 1.70+ ([rustup.rs](https://rustup.rs/))

```bash
git clone https://github.com/joel-xiao/lazyssh.git
cd lazyssh
cargo build --release
sudo cp target/release/lazyssh /usr/local/bin/
```

**Optional:** Install `sshpass` for auto-login:
- Debian/Ubuntu: `sudo apt install sshpass`
- macOS: `brew install sshpass`
- Arch: `sudo pacman -S sshpass`
- Fedora: `sudo dnf install sshpass`

## Quick Start

1. Run `lazyssh`
2. Press `a` to add a host, fill in the form, press `Enter` to save
3. Use `↑/↓` or `j/k` to navigate, press `Enter` to connect

## Configuration

Configuration file: `~/.lazyssh/config.toml`

### Example

```toml
[[hosts]]
name = "web-server"
user = "deploy"
host = "192.0.2.10"
port = 22
password = "your_password_here"  # Optional, requires sshpass
command = "cd /var/www && ls -la"  # Optional, supports multi-line

[[hosts]]
name = "monitoring"
user = "monitor"
host = "monitor.example.com"
command = """
cd /var/log
tail -f application.log
"""
```

### Fields

- `name`: Host display name (required)
- `user`: SSH username (required)
- `host`: IP or domain (required)
- `port`: SSH port (optional, default: 22)
- `password`: Password for auto-login (optional, requires sshpass)
- `command`: Commands to execute after login (optional, multi-line supported)

> ⚠️ **Security**: Passwords are stored in plain text. Use `chmod 600 ~/.lazyssh/config.toml`.  
> **Recommended**: Use SSH Key authentication and leave password empty.

## Usage

### Keyboard Shortcuts

**Main Interface:**
- `↑/↓` or `j/k`: Navigate hosts
- `Enter`: Connect to selected host
- `a`: Add host, `e`: Edit, `d`: Delete, `q`: Quit
- `y`: Copy selected host's SSH command to clipboard
- `p`: Paste SSH command from clipboard (format must be correct: `ssh user@host` or `ssh -p port user@host`)
- `Ctrl+C` / `Cmd+C`: Quit application

**Form Editor:**
- `Tab/↓`: Next field, `Shift+Tab/↑`: Previous field
- `Enter`: Save, `Esc`: Cancel
- `Shift+Enter`: New line (in command field)

**Delete Confirmation:**
- Press `d` to delete a host, confirmation prompt will appear
- Type `y` to confirm, any other key to cancel

### Behavior

- Auto-login if password configured and `sshpass` available
- Commands execute sequentially, then interactive shell starts
- Continue working in SSH session after commands complete

## Troubleshooting

**Installation fails:**
- Ensure `curl`/`wget` installed and internet connected
- Script falls back to source build if download fails

**Binary not found:**
- Check PATH: `echo $PATH | grep /usr/local/bin`
- Restart terminal or `source ~/.bashrc` (or `~/.zshrc`)

**sshpass not found:**
- Install sshpass (see Installation) or use SSH Key authentication

**Shift+Enter doesn't work:**
- Edit config file directly or use terminal that supports it (iTerm2, Alacritty)

**Permission denied:**
```bash
chmod +x lazyssh
chmod 600 ~/.lazyssh/config.toml
```

## Development

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Cross-compilation
cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin
cargo build --release --target x86_64-pc-windows-msvc
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test file
cargo test --test ui_test
cargo test --test config_test
```

Test files are located in the `tests/` directory:
- `tests/ui_test.rs` - UI module tests
- `tests/config_test.rs` - Config module tests

## Contributing

Contributions welcome! Fork, create a feature branch, commit changes, and open a Pull Request.

## Version

Current version: **v0.3.1**

### Changelog

#### v0.3.0
- ✨ New `y` shortcut: Copy selected host's SSH command to clipboard
- ✨ New `p` shortcut: Paste SSH command from clipboard (with format validation)
- ✨ New `Ctrl+C` / `Cmd+C` shortcut to quit
- ✨ Delete confirmation prompt to prevent accidental deletion
- 🧪 Refactored test structure: Moved unit tests to separate `tests/` directory
- 🔧 Code optimization: Extracted common functions, reduced code duplication

#### v0.2.0
- ✨ Remote installation script support (`curl | bash`)
- ✨ Automatic PATH configuration
- ✨ Platform detection for binary downloads
- ✨ Fallback to source build when release unavailable
- 🔧 Improved installation script error handling

#### v0.1.0
- 🎉 Initial release
- ✨ Graphical TUI interface
- ✨ Host management (add/edit/delete)
- ✨ Multi-line command support
- ✨ Auto-login with sshpass

## License

MIT License - see LICENSE file for details.

## Acknowledgments

- Inspired by [lazygit](https://github.com/jesseduffield/lazygit)
- Built with [tui-rs](https://github.com/fdehau/tui-rs) and [crossterm](https://github.com/crossterm-rs/crossterm)
