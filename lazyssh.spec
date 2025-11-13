%define debug_package %{nil}

Name:           lazyssh
Version:        0.2.0
Release:        1%{?dist}
Summary:        A cross-platform SSH management tool with TUI interface
License:        MIT
URL:            https://github.com/joel-xiao/lazyssh
Source0:        https://github.com/joel-xiao/lazyssh/archive/v%{version}.tar.gz

BuildRequires:  rust
BuildRequires:  cargo
BuildRequires:  gcc
BuildRequires:  make

%description
LazySSH is a cross-platform SSH management tool written in Rust, inspired by lazygit.
It provides a graphical TUI interface for managing SSH hosts with support for
auto-login and command execution.

%prep
%setup -q

%build
export CARGO_HOME=%{_builddir}/cargo-home
cargo build --release

%install
install -d %{buildroot}%{_bindir}
install -m 755 target/release/lazyssh %{buildroot}%{_bindir}/lazyssh

%files
%{_bindir}/lazyssh

%changelog
* Wed Nov 13 2024 Your Name <your.email@example.com> - 0.2.0-1
- Add remote installation script support
- Add automatic PATH configuration
- Improve installation experience

