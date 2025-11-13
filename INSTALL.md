# Installation Guide / 安装指南

## Homebrew Installation / Homebrew 安装

### Option 1: Custom Tap / 自定义 Tap

```bash
brew tap joel-xiao/lazyssh
brew install lazyssh
```

### Option 2: Install from Source / 从源码安装

```bash
brew install --build-from-source lazyssh
```

### Option 3: Direct Binary / 直接安装二进制

```bash
# Download binary
curl -L https://github.com/joel-xiao/lazyssh/releases/download/v0.1.0/lazyssh-darwin-amd64.tar.gz -o lazyssh.tar.gz
tar -xzf lazyssh.tar.gz
sudo mv lazyssh /usr/local/bin/
```

---

## RPM Installation / RPM 安装

### Option 1: COPR (Fedora) / COPR（Fedora）

```bash
sudo dnf copr enable joel-xiao/lazyssh
sudo dnf install lazyssh
```

### Option 2: Direct RPM Install / 直接安装 RPM

```bash
# Download RPM
wget https://github.com/joel-xiao/lazyssh/releases/download/v0.1.0/lazyssh-0.1.0-1.x86_64.rpm

# Install
sudo rpm -ivh lazyssh-0.1.0-1.x86_64.rpm
```

### Option 3: YUM Repository / YUM 仓库

```bash
# Add repository
sudo tee /etc/yum.repos.d/lazyssh.repo <<EOF
[lazyssh]
name=LazySSH
baseurl=https://github.com/joel-xiao/lazyssh/releases/download/v0.1.0/
enabled=1
gpgcheck=0
EOF

# Install
sudo yum install lazyssh
```

---

## Manual Installation / 手动安装

### Linux / macOS

```bash
# Download binary
wget https://github.com/joel-xiao/lazyssh/releases/download/v0.1.0/lazyssh-linux-x86_64.tar.gz
tar -xzf lazyssh-linux-x86_64.tar.gz

# Install
sudo mv lazyssh /usr/local/bin/
chmod +x /usr/local/bin/lazyssh
```

### Windows

1. Download `lazyssh-windows-x86_64.exe` from releases
2. Rename to `lazyssh.exe`
3. Add to PATH or place in desired directory

---

## Verify Installation / 验证安装

```bash
lazyssh --version
```

Or simply run:

```bash
lazyssh
```

