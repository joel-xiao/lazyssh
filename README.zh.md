[English](README.md) | **中文**

# LazySSH / 懒人 SSH

一个使用 **Rust** 编写的跨平台 SSH 管理工具，灵感来自 `lazygit`。  
提供 **图形化 TUI 界面**，支持 **自动登录和自动执行命令**。

## 功能特点

- **图形化 TUI 界面**管理 SSH 主机
- **添加、编辑、删除主机**，支持直观的表单编辑器
- **密码存储**（本地配置文件，可选 sshpass 自动登录）
- **多行命令支持**，执行后进入交互式 shell
- 完全调用 **系统 SSH**，无需额外 SSH 库
- **跨平台**支持 Linux/macOS/Windows

## 安装

### 快速安装（推荐）

```bash
curl -fsSL https://raw.githubusercontent.com/joel-xiao/lazyssh/main/install.sh | bash
```

脚本会自动检测平台，下载最新版本，安装到 `/usr/local/bin` 并配置 PATH。如果发布版本不可用，将自动回退到从源码构建。

### 预编译二进制文件

从 [Releases](https://github.com/joel-xiao/lazyssh/releases) 下载并解压：

```bash
tar -xzf lazyssh-linux-x86_64.tar.gz
sudo mv lazyssh /usr/local/bin/
sudo chmod +x /usr/local/bin/lazyssh
```

### 从源码编译

**前置要求：** Rust 1.70+ ([rustup.rs](https://rustup.rs/))

```bash
git clone https://github.com/joel-xiao/lazyssh.git
cd lazyssh
cargo build --release
sudo cp target/release/lazyssh /usr/local/bin/
```

**可选：** 安装 `sshpass` 用于自动登录：
- Debian/Ubuntu: `sudo apt install sshpass`
- macOS: `brew install sshpass`
- Arch: `sudo pacman -S sshpass`
- Fedora: `sudo dnf install sshpass`

## 快速开始

1. 运行 `lazyssh`
2. 按 `a` 添加主机，填写表单，按 `Enter` 保存
3. 使用 `↑/↓` 或 `j/k` 导航，按 `Enter` 连接

## 配置文件

配置文件：`~/.lazyssh/config.toml`

### 示例

```toml
[[hosts]]
name = "web-server"
user = "deploy"
host = "192.0.2.10"
port = 22
password = "your_password_here"  # 可选，需要 sshpass
command = "cd /var/www && ls -la"  # 可选，支持多行

[[hosts]]
name = "monitoring"
user = "monitor"
host = "monitor.example.com"
command = """
cd /var/log
tail -f application.log
"""
```

### 字段说明

- `name`: 主机显示名称（必需）
- `user`: SSH 用户名（必需）
- `host`: IP 或域名（必需）
- `port`: SSH 端口（可选，默认 22）
- `password`: 密码（可选，需要 sshpass）
- `command`: 登录后执行的命令（可选，支持多行）

> ⚠️ **安全提示**：密码以明文存储。使用 `chmod 600 ~/.lazyssh/config.toml`。  
> **建议**：使用 SSH Key 认证，不填写密码。

## 使用方法

### 快捷键

**主界面：**
- `↑/↓` 或 `j/k`：导航主机
- `Enter`：连接选中主机
- `a`：添加，`e`：编辑，`d`：删除，`q`：退出

**表单编辑器：**
- `Tab/↓`：下一个字段，`Shift+Tab/↑`：上一个字段
- `Enter`：保存，`Esc`：取消
- `Shift+Enter`：换行（命令字段）

### 行为说明

- 如果配置了密码且系统有 `sshpass`，会自动登录
- 命令按顺序执行，然后进入交互式 shell
- 命令执行完成后可继续在 SSH 会话中操作

## 故障排除

**安装失败：**
- 确保已安装 `curl`/`wget` 且网络正常
- 下载失败时脚本会自动尝试从源码构建

**找不到二进制文件：**
- 检查 PATH：`echo $PATH | grep /usr/local/bin`
- 重启终端或运行 `source ~/.bashrc`（或 `~/.zshrc`）

**sshpass 未找到：**
- 安装 sshpass（参见安装部分）或使用 SSH Key 认证

**Shift+Enter 不工作：**
- 直接编辑配置文件或使用支持的终端（iTerm2、Alacritty）

**权限被拒绝：**
```bash
chmod +x lazyssh
chmod 600 ~/.lazyssh/config.toml
```

## 编译

```bash
# Debug 构建
cargo build

# Release 构建
cargo build --release

# 交叉编译
cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin
cargo build --release --target x86_64-pc-windows-msvc
```

## 贡献

欢迎贡献！Fork 仓库，创建功能分支，提交更改并打开 Pull Request。

## 版本

当前版本：**v0.2.0**

### 更新日志

#### v0.2.0
- ✨ 远程安装脚本支持（`curl | bash`）
- ✨ 自动 PATH 配置
- ✨ 平台检测下载二进制文件
- ✨ 发布版本不可用时回退到源码构建
- 🔧 改进安装脚本错误处理

#### v0.1.0
- 🎉 首次发布
- ✨ 图形化 TUI 界面
- ✨ 主机管理（添加/编辑/删除）
- ✨ 多行命令支持
- ✨ sshpass 自动登录

## 许可证

MIT 许可证 - 查看 LICENSE 文件了解详情。

## 致谢

- 灵感来自 [lazygit](https://github.com/jesseduffield/lazygit)
- 使用 [tui-rs](https://github.com/fdehau/tui-rs) 和 [crossterm](https://github.com/crossterm-rs/crossterm) 构建
