# Publishing Guide / 发布指南

This guide explains how to publish LazySSH to Homebrew and RPM repositories.

本指南说明如何将 LazySSH 发布到 Homebrew 和 RPM 仓库。

---

## Publishing to Homebrew / 发布到 Homebrew

### Option 1: Custom Tap (Recommended) / 自定义 Tap（推荐）

1. **Create a tap repository**:

   ```bash
   # Create a new repository named homebrew-lazyssh on GitHub
   ```

2. **Add the formula**:

   ```bash
   git clone https://github.com/joel-xiao/homebrew-lazyssh.git
   cd homebrew-lazyssh
   cp /path/to/lazyssh/Formula/lazyssh.rb Formula/
   
   # Get SHA256: curl -sL https://github.com/joel-xiao/lazyssh/archive/v0.1.0.tar.gz | shasum -a 256
   # Update sha256 in Formula/lazyssh.rb
   ```

3. **Commit and push**:

   ```bash
   git add Formula/lazyssh.rb
   git commit -m "Add lazyssh formula"
   git push
   ```

4. **Users can install**:

   ```bash
   brew tap joel-xiao/lazyssh
   brew install lazyssh
   ```

### Option 2: Submit to homebrew-core / 提交到 homebrew-core

1. Fork and clone `homebrew-core`
2. Add formula to `Formula/lazyssh.rb`
3. Test locally: `brew install --build-from-source ./Formula/lazyssh.rb`
4. Submit PR to homebrew-core

---

## Publishing to RPM / 发布到 RPM

### Option 1: COPR (Fedora) / COPR（Fedora）

1. **Create COPR project** at https://copr.fedorainfracloud.org/

2. **Build and upload**:

   ```bash
   # Create source tarball
   git archive --format=tar.gz --prefix=lazyssh-0.1.0/ -o lazyssh-0.1.0.tar.gz v0.1.0
   
   # Build SRPM
   rpmbuild -bs lazyssh.spec
   
   # Upload to COPR
   copr-cli build joel-xiao/lazyssh lazyssh-0.1.0-1.src.rpm
   ```

3. **Users can install**:

   ```bash
   sudo dnf copr enable joel-xiao/lazyssh
   sudo dnf install lazyssh
   ```

### Option 2: Direct RPM Build / 直接构建 RPM

```bash
# Install dependencies
sudo dnf install rpm-build cargo rust

# Build RPM
rpmbuild -ba lazyssh.spec

# RPM files will be in ~/rpmbuild/RPMS/
```

---

## Version Updates / 版本更新

### Homebrew

1. Update version in `Formula/lazyssh.rb`
2. Update `url` to point to new release
3. Update `sha256` checksum
4. Commit and push

### RPM

1. Update version in `lazyssh.spec`
2. Update `Source0` URL
3. Add changelog entry
4. Build and upload new RPM

---

## Resources / 资源

- [Homebrew Formula Cookbook](https://docs.brew.sh/Formula-Cookbook)
- [Fedora Packaging Guidelines](https://docs.fedoraproject.org/en-US/packaging-guidelines/)
- [COPR Documentation](https://docs.pagure.org/copr.copr/)
