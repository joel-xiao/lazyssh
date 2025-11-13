[English](README.md) | **中文**

# LazySSH / 懒人 SSH

一个使用 **Rust** 编写的跨平台 SSH 管理工具，灵感来自 `lazygit`。  
它提供 **图形化 TUI 界面**，可以直接选择、添加、编辑、删除 SSH 主机配置，并支持 **自动登录和自动执行命令**。

---

## 功能特点

- **图形化 TUI 界面**管理 SSH 主机
- **添加、编辑、删除主机**，支持直观的表单编辑器
- **密码存储**（本地配置文件，可选 sshpass 自动登录）
- **多行命令支持**，执行后进入交互式 shell
- 完全调用 **系统 SSH**，无需额外 SSH 库
- 打包后单文件可执行，跨平台支持 Linux/macOS/Windows
- **美观的 UI** - 现代化界面，多行命令显示行号

---

## 安装

### 从源码编译

1. **前置要求**：

   - Rust 工具链 (1.70+): [rustup.rs](https://rustup.rs/)
   - SSH 客户端（Linux/macOS 通常已预装，Windows 10+ 包含 OpenSSH）
   - 可选：`sshpass` 用于自动密码登录（Linux/macOS）

2. **安装 sshpass**（可选，用于自动登录）：

   ```bash
   # Debian/Ubuntu
   sudo apt install sshpass

   # macOS Homebrew
   brew install hudochenkov/sshpass/sshpass

   # Arch Linux
   sudo pacman -S sshpass
   ```

3. **从源码编译**：

   ```bash
   git clone https://github.com/joel-xiao/lazyssh.git
   cd lazyssh
   cargo build --release
   ```

4. **安装二进制文件**（可选）：

   ```bash
   # Linux/macOS
   sudo cp target/release/lazyssh /usr/local/bin/

   # 或添加到 PATH
   export PATH=$PATH:$(pwd)/target/release
   ```

### 预编译二进制文件

Linux、macOS 和 Windows 的预编译二进制文件可在 [Releases](https://github.com/joel-xiao/lazyssh/releases) 页面下载。

---

## 快速开始

1. **运行程序**：

   ```bash
   lazyssh
   ```

2. **添加第一个主机**：

   - 按 `a` 添加新主机
   - 填写表单字段
   - 按 `Enter` 保存

3. **连接主机**：

   - 使用 `↑/↓` 或 `j/k` 导航
   - 按 `Enter` 连接

---

## 配置文件

配置文件位置：

```
~/.lazyssh/config.toml
```

### 示例配置

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

### 配置字段

- `name`: 主机显示名称（必需）
- `user`: SSH 登录用户名（必需）
- `host`: IP 地址或域名（必需）
- `port`: SSH 端口（可选，默认 22）
- `password`: 密码（可选，使用 sshpass 自动登录）
- `command`: 登录后自动执行的命令（可选，支持多行）

> ⚠️ **安全提示**: 密码存储在本地文件，请注意安全。确保文件权限安全：
> ```bash
> chmod 600 ~/.lazyssh/config.toml
> ```
> **建议**：使用 SSH Key 登录并在配置中不填写密码。

---

## 使用方法

### 主界面

主界面以美观的 TUI 显示已配置的 SSH 主机列表。

#### 导航快捷键

- **↑/↓ 或 j/k**：在主机列表中移动
- **Enter**：连接选中的主机
- **a**：添加新主机
- **e**：编辑选中的主机
- **d**：删除选中的主机
- **q**：退出程序

### 连接主机

- 如果配置了密码且系统有 `sshpass`，会自动登录
- 如果没有 `sshpass` 或密码为空，系统 SSH 会提示输入密码
- 登录后会自动按顺序执行配置中的命令，然后进入交互式 shell
- 命令执行完成后，您可以继续在 SSH 会话中操作

### 添加/编辑主机

表单编辑器提供了直观的界面来管理主机配置。

#### 单行字段（名称、用户、主机、端口、密码）

- **←/→**：水平移动光标
- **Home/End**：跳转到字段开头/结尾
- **Tab/↓**：移动到下一个字段
- **Shift+Tab/↑**：移动到上一个字段
- **Enter**：保存并退出
- **Esc**：取消编辑
- **Backspace/Delete**：删除字符

#### 多行命令字段

- **←/→**：在同一行内水平移动光标
- **↑/↓**：在行间移动光标
- **Shift+Enter**：插入新行（创建换行）
- **Enter**：保存并退出
- **Esc**：取消编辑
- **Backspace/Delete**：删除字符

> **注意**：某些终端可能不支持 Shift+Enter 检测。如果 Shift+Enter 不工作，您可以：
> - 直接编辑配置文件添加换行符
> - 使用支持 Shift+Enter 的终端（如 iTerm2、Alacritty）
> - 使用单行命令，用 `;` 分隔

---

## 使用示例

### 示例 1：简单连接

```toml
[[hosts]]
name = "my-server"
user = "admin"
host = "example.com"
```

### 示例 2：带自动登录和命令

```toml
[[hosts]]
name = "production"
user = "deploy"
host = "prod.example.com"
port = 2222
password = "secure_password"
command = "cd /app && git pull && npm install"
```

### 示例 3：多行命令

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

## 优势

- **图形化管理主机**，无需手动记住 SSH 命令
- **自动登录和执行命令**，解决重复输入密码和命令的痛点
- **纯 Rust + 系统 SSH**，无需额外库或复杂依赖
- **跨平台**，Linux/macOS/Windows 均可使用
- **美观的 TUI**，现代化、直观的界面，带有颜色编码
- **多行命令支持**，执行复杂的命令序列
- **命令执行后进入交互式 shell**，可继续在 SSH 会话中操作

---

## 安全提示

- 配置文件中密码为**明文存储**
- 确保文件权限安全：`chmod 600 ~/.lazyssh/config.toml`
- **建议**：使用 **SSH Key** 登录并在配置中不填写密码
- 考虑使用密码管理器管理敏感密码
- 不要将配置文件提交到版本控制系统

---

## 故障排除

### sshpass 未找到

如果看到 "sshpass not found" 但想使用自动登录：

- 安装 sshpass（参见安装部分）
- 或使用 SSH Key 认证

### Shift+Enter 不工作

某些终端不支持 Shift+Enter 检测。解决方案：

- 直接编辑配置文件添加换行符
- 使用支持 Shift+Enter 的终端（如 iTerm2、Alacritty）
- 使用单行命令，用 `;` 分隔

### 权限被拒绝

如果遇到权限错误：

```bash
chmod +x lazyssh
chmod 600 ~/.lazyssh/config.toml
```

---

## 编译

### 要求

- Rust 1.70 或更高版本
- Cargo（随 Rust 一起安装）

### 编译命令

```bash
# Debug 构建
cargo build

# Release 构建（优化）
cargo build --release

# 运行测试
cargo test

# 检查代码
cargo check

# 运行 clippy
cargo clippy
```

### 交叉编译

对于跨平台构建，使用 `cargo` 配合相应的目标：

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

## 贡献

欢迎贡献！请随时提交 Pull Request。

1. Fork 本仓库
2. 创建您的功能分支 (`git checkout -b feature/amazing-feature`)
3. 提交您的更改 (`git commit -m 'Add some amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 打开 Pull Request

---

## 许可证

本项目采用 MIT 许可证 - 查看 LICENSE 文件了解详情。

---

## 致谢

- 灵感来自 [lazygit](https://github.com/jesseduffield/lazygit)
- 使用 [tui-rs](https://github.com/fdehau/tui-rs) 和 [crossterm](https://github.com/crossterm-rs/crossterm) 构建
