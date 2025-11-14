# 发布指南

本指南说明如何将 LazySSH 发布到 GitHub Releases、Homebrew 和 RPM 仓库。

---

## 创建 GitHub Release

### 自动发布（推荐）

项目已配置 GitHub Actions，推送 tag 时会自动创建 release。

1. **推送 tag 到远程**：

   ```bash
   # 如果 tag 已存在于本地
   git push origin v0.3.0
   
   # 或推送所有 tags
   git push origin --tags
   ```

2. **GitHub Actions 会自动**：
   - 为所有平台构建二进制文件（Linux、macOS x86_64/ARM64、Windows）
   - 创建 GitHub Release 并附加所有二进制文件
   - 自动生成 release 说明

### 手动发布

如果需要手动创建 release（例如，tag 已存在于远程但未创建 release）：

#### 方法 1：使用 GitHub CLI

1. **安装 GitHub CLI**：

   ```bash
   # macOS
   brew install gh
   
   # Linux (Debian/Ubuntu)
   curl -fsSL https://cli.github.com/packages/githubcli-archive-keyring.gpg | sudo dd of=/usr/share/keyrings/githubcli-archive-keyring.gpg
   echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main" | sudo tee /etc/apt/sources.list.d/github-cli.list > /dev/null
   sudo apt update && sudo apt install gh
   ```

2. **认证**：

   ```bash
   gh auth login
   ```

3. **从已有 tag 创建 release**：

   ```bash
   # 使用自动生成的说明创建 release
   gh release create v0.3.0 --generate-notes
   
   # 或使用自定义标题和说明
   gh release create v0.3.0 \
     --title "v0.3.0" \
     --notes "发布说明"
   
   # 或附加文件
   gh release create v0.3.0 \
     --title "v0.3.0" \
     --notes "发布说明" \
     lazyssh-linux-x86_64.tar.gz \
     lazyssh-darwin-x86_64.tar.gz \
     lazyssh-darwin-arm64.tar.gz \
     lazyssh-windows-x86_64.exe.zip
   ```

#### 方法 2：使用 GitHub Web UI

1. 访问 GitHub 仓库页面：`https://github.com/joel-xiao/lazyssh`
2. 点击 **"Releases"** → **"Draft a new release"**
3. 从下拉菜单中选择 tag（例如 `v0.3.0`）
4. 填写 release 标题和说明
5. 如需要，附加二进制文件
6. 点击 **"Publish release"**

---

## 发布到 Homebrew

### 方法 1：自定义 Tap（推荐）

1. **创建 tap 仓库**：

   ```bash
   # 在 GitHub 上创建一个名为 homebrew-lazyssh 的新仓库
   ```

2. **添加 formula**：

   ```bash
   git clone https://github.com/joel-xiao/homebrew-lazyssh.git
   cd homebrew-lazyssh
   cp /path/to/lazyssh/Formula/lazyssh.rb Formula/
   
   # 获取 SHA256: curl -sL https://github.com/joel-xiao/lazyssh/archive/v0.3.0.tar.gz | shasum -a 256
   # 更新 Formula/lazyssh.rb 中的 sha256
   ```

3. **提交并推送**：

   ```bash
   git add Formula/lazyssh.rb
   git commit -m "Add lazyssh formula"
   git push
   ```

4. **用户可以安装**：

   ```bash
   brew tap joel-xiao/lazyssh
   brew install lazyssh
   ```

### 方法 2：提交到 homebrew-core

1. Fork 并克隆 `homebrew-core`
2. 添加 formula 到 `Formula/lazyssh.rb`
3. 本地测试：`brew install --build-from-source ./Formula/lazyssh.rb`
4. 提交 PR 到 homebrew-core

---

## 发布到 RPM

### 方法 1：COPR（Fedora）

1. **创建 COPR 项目**：访问 https://copr.fedorainfracloud.org/

2. **构建并上传**：

   ```bash
   # 创建源码压缩包
   git archive --format=tar.gz --prefix=lazyssh-0.3.0/ -o lazyssh-0.3.0.tar.gz v0.3.0
   
   # 构建 SRPM
   rpmbuild -bs lazyssh.spec
   
   # 上传到 COPR
   copr-cli build joel-xiao/lazyssh lazyssh-0.3.0-1.src.rpm
   ```

3. **用户可以安装**：

   ```bash
   sudo dnf copr enable joel-xiao/lazyssh
   sudo dnf install lazyssh
   ```

### 方法 2：直接构建 RPM

```bash
# 安装依赖
sudo dnf install rpm-build cargo rust

# 构建 RPM
rpmbuild -ba lazyssh.spec

# RPM 文件将在 ~/rpmbuild/RPMS/ 目录中
```

---

## 版本更新

### Homebrew

1. 更新 `Formula/lazyssh.rb` 中的版本号
2. 更新 `url` 指向新 release
3. 更新 `sha256` 校验和
4. 提交并推送

### RPM

1. 更新 `lazyssh.spec` 中的版本号
2. 更新 `Source0` URL
3. 添加 changelog 条目
4. 构建并上传新的 RPM

---

## 资源

- [Homebrew Formula Cookbook](https://docs.brew.sh/Formula-Cookbook)
- [Fedora Packaging Guidelines](https://docs.fedoraproject.org/en-US/packaging-guidelines/)
- [COPR Documentation](https://docs.pagure.org/copr.copr/)
