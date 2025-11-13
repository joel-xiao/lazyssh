# Release Checklist / 发布检查清单

## ✅ 已完成

- [x] 代码已推送到 GitHub
- [x] Git tag v0.1.0 已创建并推送
- [x] GitHub Actions 工作流已配置

## 📋 下一步操作

### 1. 创建 GitHub Release

**方法 1：使用 GitHub CLI（推荐）**

```bash
# 如果已安装 gh CLI
gh release create v0.1.0 \
  --title "v0.1.0" \
  --notes "Initial release of LazySSH - A cross-platform SSH management tool with TUI interface" \
  --draft=false
```

**方法 2：通过 GitHub 网页**

1. 访问：https://github.com/joel-xiao/lazyssh/releases/new
2. 选择 tag: `v0.1.0`
3. 标题：`v0.1.0`
4. 描述：
   ```
   Initial release of LazySSH
   
   ## Features
   - Graphical TUI interface for managing SSH hosts
   - Add, edit, delete hosts with intuitive form editor
   - Password storage (local config file, optional sshpass auto-login)
   - Multi-line command support with interactive shell after execution
   - Cross-platform support (Linux/macOS/Windows)
   ```
5. 点击 "Publish release"

### 2. 等待 GitHub Actions 构建完成

GitHub Actions 会自动：
- 构建 Linux、macOS (x86_64 和 ARM64)、Windows 二进制文件
- 上传到 Release

查看构建状态：https://github.com/joel-xiao/lazyssh/actions

### 3. 获取 SHA256 并更新 Homebrew Formula

构建完成后，获取源码包的 SHA256：

```bash
curl -sL https://github.com/joel-xiao/lazyssh/archive/v0.1.0.tar.gz | shasum -a 256
```

然后更新 `Formula/lazyssh.rb` 中的 `sha256` 字段。

### 4. 发布到 Homebrew（可选）

#### 创建 Homebrew Tap

1. 在 GitHub 创建新仓库：`homebrew-lazyssh`
2. 克隆仓库：
   ```bash
   git clone https://github.com/joel-xiao/homebrew-lazyssh.git
   cd homebrew-lazyssh
   ```
3. 复制并更新 formula：
   ```bash
   cp /path/to/lazyssh/Formula/lazyssh.rb Formula/
   # 编辑 Formula/lazyssh.rb，更新 sha256
   ```
4. 提交并推送：
   ```bash
   git add Formula/lazyssh.rb
   git commit -m "Add lazyssh formula"
   git push
   ```

用户安装：
```bash
brew tap joel-xiao/lazyssh
brew install lazyssh
```

### 5. 发布到 COPR（可选，Fedora/RHEL）

1. 访问 https://copr.fedorainfracloud.org/
2. 创建新项目：`lazyssh`
3. 安装 copr-cli：
   ```bash
   sudo dnf install copr-cli
   ```
4. 登录：
   ```bash
   copr-cli login
   ```
5. 构建 RPM：
   ```bash
   # 创建源码包
   git archive --format=tar.gz --prefix=lazyssh-0.1.0/ -o lazyssh-0.1.0.tar.gz v0.1.0
   
   # 上传到 GitHub Releases（或使用其他方式）
   
   # 更新 lazyssh.spec 中的 Source0 URL
   # 构建 SRPM
   rpmbuild -bs lazyssh.spec
   
   # 上传到 COPR
   copr-cli build joel-xiao/lazyssh lazyssh-0.1.0-1.src.rpm
   ```

用户安装：
```bash
sudo dnf copr enable joel-xiao/lazyssh
sudo dnf install lazyssh
```

## 📝 发布后检查

- [ ] GitHub Release 已创建
- [ ] 二进制文件已上传到 Release
- [ ] README 中的安装说明正确
- [ ] Homebrew formula 已更新（如果发布到 Homebrew）
- [ ] COPR 构建成功（如果发布到 COPR）

## 🔗 有用的链接

- GitHub Repository: https://github.com/joel-xiao/lazyssh
- Releases: https://github.com/joel-xiao/lazyssh/releases
- Actions: https://github.com/joel-xiao/lazyssh/actions
- COPR: https://copr.fedorainfracloud.org/

