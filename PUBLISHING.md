# Publishing Guide / 发布指南

This guide explains how to publish LazySSH to Homebrew and RPM repositories.

本指南说明如何将 LazySSH 发布到 Homebrew 和 RPM 仓库。

---

## Prerequisites / 前置要求

### For Homebrew / Homebrew 发布

1. GitHub repository with releases
2. Homebrew tap repository (optional, for custom tap)
3. Or submit to homebrew-core (requires approval)

### For RPM / RPM 发布

1. GitHub repository with releases
2. COPR account (Fedora) or RPM repository hosting
3. Or submit to Fedora/EPEL (requires approval)

---

## Publishing to Homebrew / 发布到 Homebrew

### Option 1: Custom Tap (Recommended) / 自定义 Tap（推荐）

1. **Create a tap repository** (if you don't have one):

   ```bash
   # Create a new repository named homebrew-lazyssh
   # Or use your existing tap repository
   ```

2. **Add the formula**:

   ```bash
   # Clone your tap repository
   git clone https://github.com/joel-xiao/homebrew-lazyssh.git
   cd homebrew-lazyssh
   
   # Copy the formula
   cp /path/to/lazyssh/Formula/lazyssh.rb Formula/
   
   # Update the URL and SHA256
   # Get SHA256: curl -sL https://github.com/joel-xiao/lazyssh/archive/v0.1.0.tar.gz | shasum -a 256
   ```

3. **Update the formula**:

   Edit `Formula/lazyssh.rb`:
   - Update `url` with your GitHub release URL
   - Update `sha256` with the actual checksum
   - Update `homepage` if needed

4. **Test the formula**:

   ```bash
   brew install --build-from-source ./Formula/lazyssh.rb
   ```

5. **Commit and push**:

   ```bash
   git add Formula/lazyssh.rb
   git commit -m "Add lazyssh formula"
   git push
   ```

6. **Users can install**:

   ```bash
   brew tap joel-xiao/lazyssh
   brew install lazyssh
   ```

### Option 2: Submit to homebrew-core / 提交到 homebrew-core

1. **Fork homebrew-core**:

   ```bash
   git clone https://github.com/Homebrew/homebrew-core.git
   cd homebrew-core
   ```

2. **Add formula**:

   ```bash
   cp /path/to/lazyssh/Formula/lazyssh.rb Formula/lazyssh.rb
   ```

3. **Test locally**:

   ```bash
   brew install --build-from-source ./Formula/lazyssh.rb
   ```

4. **Submit PR**:

   ```bash
   git checkout -b lazyssh
   git add Formula/lazyssh.rb
   git commit -m "lazyssh 0.1.0 (new formula)"
   git push origin lazyssh
   ```

   Then create a PR on GitHub.

### Option 3: Install from GitHub Releases / 从 GitHub Releases 安装

Users can install directly from GitHub releases:

```bash
# Download binary
wget https://github.com/joel-xiao/lazyssh/releases/download/v0.1.0/lazyssh-darwin-amd64.tar.gz
tar -xzf lazyssh-darwin-amd64.tar.gz
sudo mv lazyssh /usr/local/bin/
```

---

## Publishing to RPM / 发布到 RPM

### Option 1: COPR (Fedora) / COPR（Fedora）

1. **Create COPR project**:

   - Go to https://copr.fedorainfracloud.org/
   - Create a new project

2. **Prepare source**:

   ```bash
   # Create source tarball
   git archive --format=tar.gz --prefix=lazyssh-0.1.0/ -o lazyssh-0.1.0.tar.gz v0.1.0
   
   # Upload to GitHub Releases or your hosting
   ```

3. **Update spec file**:

   Edit `lazyssh.spec`:
   - Update `Source0` URL
   - Update version and release
   - Update changelog

4. **Build SRPM**:

   ```bash
   rpmbuild -bs lazyssh.spec
   ```

5. **Upload to COPR**:

   ```bash
   copr-cli build yourproject lazyssh-0.1.0-1.src.rpm
   ```

6. **Users can install**:

   ```bash
   sudo dnf copr enable joel-xiao/lazyssh
   sudo dnf install lazyssh
   ```

### Option 2: Create RPM Repository / 创建 RPM 仓库

1. **Build RPM**:

   ```bash
   # Install build dependencies
   sudo dnf install rpm-build cargo rust
   
   # Build RPM
   rpmbuild -ba lazyssh.spec
   ```

2. **Host repository**:

   - Use GitHub Releases
   - Or set up your own RPM repository

3. **Create repository metadata**:

   ```bash
   createrepo /path/to/rpm/repo
   ```

4. **Users can install**:

   ```bash
   # Add repository
   sudo tee /etc/yum.repos.d/lazyssh.repo <<EOF
   [lazyssh]
   name=LazySSH
   baseurl=https://github.com/joel-xiao/lazyssh/releases/download/v0.1.0/
   enabled=1
   gpgcheck=0
   EOF
   
   sudo dnf install lazyssh
   ```

### Option 3: Submit to Fedora/EPEL / 提交到 Fedora/EPEL

1. **Join Fedora**:

   - Create Fedora Account
   - Join packager group

2. **Create package review**:

   - Submit package for review
   - Follow Fedora packaging guidelines

---

## GitHub Actions Automation / GitHub Actions 自动化

Create `.github/workflows/release.yml` to automate builds:

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
    
    steps:
      - uses: actions/checkout@v3
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Build
        run: cargo build --release
      
      - name: Upload artifacts
        uses: actions/upload-artifact@v3
        with:
          name: lazyssh-${{ matrix.os }}
          path: target/release/lazyssh
```

---

## Version Updates / 版本更新

### Homebrew / Homebrew 更新

1. Update version in `Formula/lazyssh.rb`
2. Update `url` to point to new release
3. Update `sha256` checksum
4. Commit and push

### RPM / RPM 更新

1. Update version in `lazyssh.spec`
2. Update `Source0` URL
3. Add changelog entry
4. Build and upload new RPM

---

## Checklist / 检查清单

### Before Release / 发布前

- [ ] Update version in `Cargo.toml`
- [ ] Create git tag: `git tag v0.1.0`
- [ ] Push tag: `git push --tags`
- [ ] Create GitHub release with binaries
- [ ] Calculate SHA256 checksums
- [ ] Update formula/spec files
- [ ] Test installation locally
- [ ] Update documentation

### After Release / 发布后

- [ ] Submit Homebrew formula
- [ ] Build and upload RPM
- [ ] Update README with installation instructions
- [ ] Announce release

---

## Resources / 资源

- [Homebrew Formula Cookbook](https://docs.brew.sh/Formula-Cookbook)
- [Fedora Packaging Guidelines](https://docs.fedoraproject.org/en-US/packaging-guidelines/)
- [COPR Documentation](https://docs.pagure.org/copr.copr/)
- [RPM Packaging Guide](https://rpm-packaging-guide.github.io/)

