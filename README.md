**English** | [中文](README.zh.md)

# LazySSH

A cross-platform SSH management tool written in **Rust**, inspired by `lazygit`.  
It provides a **graphical TUI interface** for managing SSH hosts with support for **auto-login and command execution**.

---

## Features

- **Graphical TUI interface** for managing SSH hosts
- **Add, edit, delete hosts** with an intuitive form editor
- **Password storage** (local config file, optional sshpass auto-login)
- **Multi-line command support** with interactive shell after execution
- **Uses system SSH** - no additional SSH libraries required
- **Single executable** - cross-platform support for Linux/macOS/Windows
- **Beautiful UI** - Modern, color-coded interface with line numbers for multi-line commands

---

## Installation

### From Source

1. **Prerequisites**:

   - Rust toolchain (1.70+): [rustup.rs](https://rustup.rs/)
   - SSH client (usually pre-installed on Linux/macOS, Windows 10+ includes OpenSSH)
   - Optional: `sshpass` for auto password login (Linux/macOS)

2. **Install sshpass** (optional, for auto-login):

   ```bash
   # Debian/Ubuntu
   sudo apt install sshpass

   # macOS Homebrew
   brew install hudochenkov/sshpass/sshpass

   # Arch Linux
   sudo pacman -S sshpass
   ```

3. **Build from source**:

   ```bash
   git clone https://github.com/joel-xiao/lazyssh.git
   cd lazyssh
   cargo build --release
   ```

4. **Install binary** (optional):

   ```bash
   # Linux/macOS
   sudo cp target/release/lazyssh /usr/local/bin/

   # Or add to PATH
   export PATH=$PATH:$(pwd)/target/release
   ```

### Pre-built Binaries

Pre-built binaries for Linux, macOS, and Windows are available in the [Releases](https://github.com/joel-xiao/lazyssh/releases) section.

---

## Quick Start

1. **Run the application**:

   ```bash
   lazyssh
   ```

2. **Add your first host**:

   - Press `a` to add a new host
   - Fill in the form fields
   - Press `Enter` to save

3. **Connect to a host**:

   - Use `↑/↓` or `j/k` to navigate
   - Press `Enter` to connect

---

## Configuration

Configuration file location:

```
~/.lazyssh/config.toml
```

### Example Configuration

```toml
[[hosts]]
name = "web-server"
user = "deploy"
host = "192.0.2.10"
port = 22
password = "your_password_here"
command = "cd /var/www && ls -la"

[[hosts]]
name = "internal-box"
user = "bob"
host = "10.0.0.5"
port = 22
password = "your_password_here"
command = """
cd /var/log
tail -f app.log
"""
```

### Configuration Fields

- `name`: Host display name (required)
- `user`: SSH login username (required)
- `host`: IP address or domain name (required)
- `port`: SSH port (optional, default: 22)
- `password`: Password for auto-login (optional, requires sshpass)
- `command`: Commands to execute after login (optional, supports multi-line)

> ⚠️ **Security Warning**: Passwords are stored in plain text. Ensure file permissions are secure:
> ```bash
> chmod 600 ~/.lazyssh/config.toml
> ```
> **Recommended**: Use SSH Key authentication and leave password empty.

---

## Usage

### Main Interface

The main interface displays a list of configured SSH hosts with a beautiful TUI.

#### Navigation Shortcuts

- **↑/↓ or j/k**: Navigate through hosts
- **Enter**: Connect to selected host
- **a**: Add new host
- **e**: Edit selected host
- **d**: Delete selected host
- **q**: Quit application

### Connecting to Hosts

- If password is configured and `sshpass` is available, auto-login will be performed
- If `sshpass` is not available or password is empty, SSH will prompt for password
- After login, configured commands will be executed sequentially, then an interactive shell will be started
- You can continue working in the SSH session after commands complete

### Adding/Editing Hosts

The form editor provides an intuitive interface for managing host configurations.

#### Single-line Fields (Name, User, Host, Port, Password)

- **←/→**: Move cursor horizontally
- **Home/End**: Jump to beginning/end of field
- **Tab/↓**: Move to next field
- **Shift+Tab/↑**: Move to previous field
- **Enter**: Save and exit
- **Esc**: Cancel editing
- **Backspace/Delete**: Delete characters

#### Multi-line Command Field

- **←/→**: Move cursor horizontally within a line
- **↑/↓**: Move cursor between lines
- **Shift+Enter**: Insert new line (create line break)
- **Enter**: Save and exit
- **Esc**: Cancel editing
- **Backspace/Delete**: Delete characters

> **Note**: Some terminals may not support Shift+Enter detection. If Shift+Enter doesn't work, you can:
> - Edit the config file directly to add line breaks
> - Use a different terminal emulator that supports Shift+Enter

---

## Examples

### Example 1: Simple Connection

```toml
[[hosts]]
name = "my-server"
user = "admin"
host = "example.com"
```

### Example 2: With Auto-login and Command

```toml
[[hosts]]
name = "production"
user = "deploy"
host = "prod.example.com"
port = 2222
password = "secure_password"
command = "cd /app && git pull && npm install"
```

### Example 3: Multi-line Commands

```toml
[[hosts]]
name = "monitoring"
user = "monitor"
host = "monitor.example.com"
command = """
cd /var/log
tail -f application.log
"""
```

---

## Advantages

- **Graphical host management** - No need to remember SSH commands manually
- **Auto-login and command execution** - Solves the pain of repeated password and command input
- **Pure Rust + system SSH** - No additional libraries or complex dependencies
- **Cross-platform** - Works on Linux/macOS/Windows
- **Beautiful TUI** - Modern, intuitive interface with color-coded sections
- **Multi-line command support** - Execute complex command sequences
- **Interactive shell after commands** - Continue working in SSH session

---

## Security

- Passwords are stored in **plain text** in the configuration file
- Ensure secure file permissions: `chmod 600 ~/.lazyssh/config.toml`
- **Recommended**: Use SSH Key authentication and leave password empty
- Consider using a password manager for sensitive passwords
- Never commit the config file to version control

---

## Troubleshooting

### sshpass not found

If you see "sshpass not found" but want to use auto-login:

- Install sshpass (see Installation section)
- Or use SSH Key authentication instead

### Shift+Enter doesn't work

Some terminals don't support Shift+Enter detection. Solutions:

- Edit the config file directly to add line breaks
- Use a terminal that supports Shift+Enter (e.g., iTerm2, Alacritty)
- Type commands on a single line separated by `;`

### Permission denied

If you get permission errors:

```bash
chmod +x lazyssh
chmod 600 ~/.lazyssh/config.toml
```

---

## Building

### Requirements

- Rust 1.70 or later
- Cargo (comes with Rust)

### Build Commands

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Check code
cargo check

# Run clippy
cargo clippy
```

### Cross-compilation

For cross-platform builds, use `cargo` with appropriate targets:

```bash
# Linux
cargo build --release --target x86_64-unknown-linux-gnu

# macOS
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin

# Windows
cargo build --release --target x86_64-pc-windows-msvc
```

---

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

---

## License

This project is licensed under the MIT License - see the LICENSE file for details.

---

## Acknowledgments

- Inspired by [lazygit](https://github.com/jesseduffield/lazygit)
- Built with [tui-rs](https://github.com/fdehau/tui-rs) and [crossterm](https://github.com/crossterm-rs/crossterm)
