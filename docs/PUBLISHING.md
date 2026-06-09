# Publishing confrisk to Linux Repositories

A comprehensive guide to distributing `confrisk` and `confrisk-npm` via package managers.

## Table of Contents

1. [Quick Overview](#quick-overview)
2. [Debian/Ubuntu (.deb packages)](#debianubuntu-deb-packages)
3. [Fedora/RHEL (.rpm packages)](#fedorarhel-rpm-packages)
4. [Arch Linux (AUR)](#arch-linux-aur)
5. [Distribution Options](#distribution-options)
6. [Alternative Methods](#alternative-methods)
7. [Automated Publishing](#automated-publishing)

---

## Quick Overview

### Repository Types

| Method | Difficulty | Time to Availability | Best For |
|--------|-----------|---------------------|----------|
| **PPA (Ubuntu)** | Medium | Days | Ubuntu/Debian users, quick start |
| **Snapcraft** | Easy | Hours | Cross-distro, sandboxed apps |
| **cargo install** | Easy | Immediate | Rust developers |
| **GitHub Releases** | Easy | Immediate | All platforms, manual install |
| **Official Debian** | Hard | Months/Years | Long-term official support |
| **Flatpak** | Medium | Days | Cross-distro GUI apps |
| **AUR (Arch)** | Easy | Hours | Arch Linux users |

### Recommended Approach (Fastest)

```bash
# 1. GitHub Releases (immediate)
# 2. PPA for Ubuntu (days)
# 3. AUR for Arch (hours)
# 4. cargo install (immediate, for Rust users)
```

---

## Debian/Ubuntu (.deb packages)

### Prerequisites

```bash
# Install packaging tools
sudo apt-get install devscripts debhelper dh-cargo
```

### Step 1: Create Package Structure

```bash
cd confrisk
mkdir -p debian

# Create required files
touch debian/control
touch debian/changelog
touch debian/rules
touch debian/compat
touch debian/copyright
```

### Step 2: debian/control

```
Source: confrisk
Section: admin
Priority: optional
Maintainer: Your Name <your.email@example.com>
Build-Depends: debhelper (>= 11), cargo, rustc
Standards-Version: 4.5.0
Homepage: https://github.com/yourusername/confrisk

Package: confrisk
Architecture: amd64
Depends: ${shlibs:Depends}, ${misc:Depends}
Description: Linux security configuration scanner with risk-based scoring
 confrisk is a security scanner that provides contextual risk assessment
 for Linux systems. It includes:
  - Risk-based prioritization
  - Config-driven security checks
  - HTML report generation
  - NPM dependency scanning
  - Git hooks integration

Package: confrisk-npm
Architecture: amd64
Depends: ${shlibs:Depends}, ${misc:Depends}, nodejs, npm
Description: NPM dependency security scanner
 A dedicated npm dependency scanner that integrates with git hooks
 and CI/CD pipelines. Scans for vulnerable packages, outdated
 dependencies, and enforces custom blocklists.
```

### Step 3: debian/changelog

```
confrisk (0.2.0-1) unstable; urgency=medium

  * Initial release
  * Linux security scanner with risk-based scoring
  * NPM dependency scanner with git hooks support
  * Config-driven architecture
  * HTML report generation

 -- Your Name <your.email@example.com>  Sat, 24 May 2026 10:00:00 +0000
```

### Step 4: debian/rules

```makefile
#!/usr/bin/make -f

%:
	dh $@

override_dh_auto_build:
	cargo build --release --bin confrisk
	cargo build --release --bin confrisk-npm

override_dh_auto_install:
	install -D -m 755 target/release/confrisk debian/confrisk/usr/bin/confrisk
	install -D -m 755 target/release/confrisk-npm debian/confrisk-npm/usr/bin/confrisk-npm
	install -D -m 644 README.md debian/confrisk/usr/share/doc/confrisk/README.md
	install -D -m 644 CONFRISK_NPM.md debian/confrisk-npm/usr/share/doc/confrisk-npm/README.md

	# Install config files
	mkdir -p debian/confrisk/etc/confrisk
	cp -r config/* debian/confrisk/etc/confrisk/

override_dh_auto_clean:
	cargo clean
```

### Step 5: debian/compat

```
11
```

### Step 6: debian/copyright

```
Format: https://www.debian.org/doc/packaging-manuals/copyright-format/1.0/
Upstream-Name: confrisk
Upstream-Contact: Your Name <your.email@example.com>
Source: https://github.com/yourusername/confrisk

Files: *
Copyright: 2026 Your Name
License: MIT

License: MIT
 Permission is hereby granted, free of charge, to any person obtaining a
 copy of this software and associated documentation files (the "Software"),
 to deal in the Software without restriction, including without limitation
 the rights to use, copy, modify, merge, publish, distribute, sublicense,
 and/or sell copies of the Software, and to permit persons to whom the
 Software is furnished to do so, subject to the following conditions:
 .
 The above copyright notice and this permission notice shall be included
 in all copies or substantial portions of the Software.
 .
 THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
 OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 DEALINGS IN THE SOFTWARE.
```

### Step 7: Build .deb Package

```bash
# Build the package
debuild -us -uc -b

# This creates files in parent directory:
# - confrisk_0.2.0-1_amd64.deb
# - confrisk-npm_0.2.0-1_amd64.deb
```

### Step 8: Test Installation

```bash
# Test installation locally
sudo dpkg -i ../confrisk_0.2.0-1_amd64.deb
sudo dpkg -i ../confrisk-npm_0.2.0-1_amd64.deb

# Verify
which confrisk
which confrisk-npm
confrisk --help
confrisk-npm --help

# Check installed files
dpkg -L confrisk
dpkg -L confrisk-npm

# Uninstall
sudo apt-get remove confrisk confrisk-npm
```

---

## Distribution Options

### Option 1: Ubuntu PPA (Personal Package Archive)

**Best for:** Ubuntu users, easiest official-ish method

#### Step 1: Create Launchpad Account

1. Go to https://launchpad.net/
2. Create account and set up GPG key
3. Create a new PPA: https://launchpad.net/~/+activate-ppa

#### Step 2: Create GPG Key

```bash
# Generate GPG key
gpg --full-generate-key
# Choose: RSA, 4096 bits, name and email matching Launchpad

# Upload to Ubuntu keyserver
gpg --list-keys
gpg --keyserver keyserver.ubuntu.com --send-keys YOUR_KEY_ID
```

#### Step 3: Build Source Package

```bash
# Build source package (not binary)
debuild -S -sa

# Sign with GPG
debsign -k YOUR_KEY_ID ../confrisk_0.2.0-1_source.changes
```

#### Step 4: Upload to PPA

```bash
# Install upload tool
sudo apt-get install dput

# Upload to your PPA
dput ppa:yourusername/confrisk ../confrisk_0.2.0-1_source.changes
```

#### Step 5: Users Install From PPA

```bash
# Users can now install with:
sudo add-apt-repository ppa:yourusername/confrisk
sudo apt-get update
sudo apt-get install confrisk confrisk-npm
```

**Timeline:**
- Upload: Minutes
- Build on Launchpad: 15-60 minutes
- Available to users: 1-2 hours

**Pros:**
- Official Ubuntu infrastructure
- Automatic builds for multiple Ubuntu versions
- Trusted by Ubuntu users
- Free

**Cons:**
- Ubuntu/Debian only
- Requires learning Launchpad
- Must maintain source packages

---

### Option 2: Snapcraft (Cross-Distribution)

**Best for:** Quick cross-distro distribution with automatic updates

#### Step 1: Create snapcraft.yaml

```yaml
name: confrisk
version: '0.2.0'
summary: Linux security configuration scanner
description: |
  confrisk is a security scanner that provides contextual risk assessment
  for Linux systems with config-driven checks and risk-based prioritization.

base: core22
confinement: strict
grade: stable

apps:
  confrisk:
    command: bin/confrisk
    plugs: [network, home]

  confrisk-npm:
    command: bin/confrisk-npm
    plugs: [network, home]

parts:
  confrisk:
    plugin: rust
    source: .
    build-packages:
      - cargo
      - rustc
    override-build: |
      cargo build --release --bin confrisk
      cargo build --release --bin confrisk-npm
      install -D -m755 target/release/confrisk $SNAPCRAFT_PART_INSTALL/bin/confrisk
      install -D -m755 target/release/confrisk-npm $SNAPCRAFT_PART_INSTALL/bin/confrisk-npm

      # Install config
      mkdir -p $SNAPCRAFT_PART_INSTALL/etc/confrisk
      cp -r config/* $SNAPCRAFT_PART_INSTALL/etc/confrisk/
```

#### Step 2: Build and Publish Snap

```bash
# Install snapcraft
sudo snap install snapcraft --classic

# Build snap
snapcraft

# Test locally
sudo snap install confrisk_0.2.0_amd64.snap --dangerous

# Login to Snap Store
snapcraft login

# Upload to Snap Store
snapcraft upload confrisk_0.2.0_amd64.snap --release=stable
```

#### Step 3: Users Install Snap

```bash
# Users can install with:
sudo snap install confrisk
confrisk --help
confrisk-npm --help
```

**Timeline:**
- Build: 5-10 minutes
- Upload and review: 1-24 hours (first submission)
- Updates: Automatic

**Pros:**
- Works on all major distros (Ubuntu, Fedora, Arch, etc.)
- Automatic updates
- Sandboxed (more secure)
- Easy to publish

**Cons:**
- Snap daemon required
- Larger package size
- Some users dislike snaps

---

### Option 3: GitHub Releases + Static Binaries

**Best for:** Immediate availability, all platforms

#### Step 1: Cross-Compile Binaries

```bash
# Install cross-compilation targets
rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-unknown-linux-musl
rustup target add aarch64-unknown-linux-gnu

# Build static binaries (musl for portability)
cargo build --release --target x86_64-unknown-linux-musl --bin confrisk
cargo build --release --target x86_64-unknown-linux-musl --bin confrisk-npm

# Package with config
mkdir -p release/confrisk-0.2.0-linux-x86_64
cp target/x86_64-unknown-linux-musl/release/confrisk release/confrisk-0.2.0-linux-x86_64/
cp target/x86_64-unknown-linux-musl/release/confrisk-npm release/confrisk-0.2.0-linux-x86_64/
cp -r config release/confrisk-0.2.0-linux-x86_64/
cp README.md CONFRISK_NPM.md release/confrisk-0.2.0-linux-x86_64/

# Create tarball
cd release
tar czf confrisk-0.2.0-linux-x86_64.tar.gz confrisk-0.2.0-linux-x86_64/

# Create checksums
sha256sum confrisk-0.2.0-linux-x86_64.tar.gz > confrisk-0.2.0-linux-x86_64.tar.gz.sha256
```

#### Step 2: Create GitHub Release

```bash
# Tag release
git tag -a v0.2.0 -m "Release v0.2.0"
git push origin v0.2.0

# Upload to GitHub:
# 1. Go to https://github.com/yourusername/confrisk/releases
# 2. Click "Draft a new release"
# 3. Choose tag v0.2.0
# 4. Upload: confrisk-0.2.0-linux-x86_64.tar.gz
# 5. Upload: confrisk-0.2.0-linux-x86_64.tar.gz.sha256
# 6. Publish release
```

#### Step 3: Create Install Script

Create `install.sh`:

```bash
#!/bin/bash
set -e

VERSION="0.2.0"
ARCH="x86_64"
URL="https://github.com/yourusername/confrisk/releases/download/v${VERSION}/confrisk-${VERSION}-linux-${ARCH}.tar.gz"

echo "Installing confrisk v${VERSION}..."

# Download
curl -L "$URL" -o /tmp/confrisk.tar.gz

# Extract
cd /tmp
tar xzf confrisk.tar.gz

# Install binaries
sudo cp confrisk-${VERSION}-linux-${ARCH}/confrisk /usr/local/bin/
sudo cp confrisk-${VERSION}-linux-${ARCH}/confrisk-npm /usr/local/bin/
sudo chmod +x /usr/local/bin/confrisk /usr/local/bin/confrisk-npm

# Install config
sudo mkdir -p /etc/confrisk
sudo cp -r confrisk-${VERSION}-linux-${ARCH}/config/* /etc/confrisk/

# Cleanup
rm -rf /tmp/confrisk.tar.gz /tmp/confrisk-${VERSION}-linux-${ARCH}

echo "✅ confrisk installed successfully!"
echo "Run: confrisk --help"
```

#### Step 4: Users Install

```bash
# One-line install
curl -L https://raw.githubusercontent.com/yourusername/confrisk/main/install.sh | bash

# Or manual:
wget https://github.com/yourusername/confrisk/releases/download/v0.2.0/confrisk-0.2.0-linux-x86_64.tar.gz
tar xzf confrisk-0.2.0-linux-x86_64.tar.gz
sudo cp confrisk-0.2.0-linux-x86_64/confrisk* /usr/local/bin/
```

**Timeline:** Immediate (minutes)

**Pros:**
- Immediate availability
- Works on all Linux distros
- No repository approval needed
- Full control

**Cons:**
- Manual updates
- No package manager integration
- Users must trust your binaries

---

### Option 4: cargo install (Rust Users)

**Best for:** Developers with Rust installed

#### Step 1: Publish to crates.io

```bash
# Add metadata to Cargo.toml
[package]
name = "confrisk"
version = "0.2.0"
edition = "2021"
authors = ["Your Name <email@example.com>"]
description = "Linux security configuration scanner with risk-based scoring"
license = "MIT"
repository = "https://github.com/yourusername/confrisk"
keywords = ["security", "scanner", "linux", "audit", "risk"]
categories = ["command-line-utilities", "development-tools"]

# Login to crates.io
cargo login YOUR_API_TOKEN

# Publish
cargo publish
```

#### Step 2: Users Install

```bash
# Users can install with:
cargo install confrisk

# Binaries installed to ~/.cargo/bin/
confrisk --help
confrisk-npm --help
```

**Timeline:** Immediate

**Pros:**
- Immediate availability
- Automatic compilation for user's platform
- Integrated with Rust ecosystem
- Free

**Cons:**
- Requires Rust toolchain
- Limited to Rust users
- Config files need manual setup

---

## Fedora/RHEL (.rpm packages)

### Create .spec File

Create `confrisk.spec`:

```spec
Name:           confrisk
Version:        0.2.0
Release:        1%{?dist}
Summary:        Linux security configuration scanner

License:        MIT
URL:            https://github.com/yourusername/confrisk
Source0:        %{name}-%{version}.tar.gz

BuildRequires:  cargo
BuildRequires:  rust

%description
confrisk is a security scanner that provides contextual risk assessment
for Linux systems with config-driven checks and risk-based prioritization.

%prep
%autosetup

%build
cargo build --release --bin confrisk
cargo build --release --bin confrisk-npm

%install
install -D -m 755 target/release/confrisk %{buildroot}%{_bindir}/confrisk
install -D -m 755 target/release/confrisk-npm %{buildroot}%{_bindir}/confrisk-npm
mkdir -p %{buildroot}%{_sysconfdir}/confrisk
cp -r config/* %{buildroot}%{_sysconfdir}/confrisk/

%files
%{_bindir}/confrisk
%{_bindir}/confrisk-npm
%{_sysconfdir}/confrisk/*
%doc README.md
%license LICENSE

%changelog
* Sat May 24 2026 Your Name <email@example.com> - 0.2.0-1
- Initial RPM release
```

### Build RPM

```bash
# Install build tools
sudo dnf install rpm-build rpmdevtools

# Setup build environment
rpmdev-setuptree

# Create source tarball
tar czf ~/rpmbuild/SOURCES/confrisk-0.2.0.tar.gz .

# Copy spec file
cp confrisk.spec ~/rpmbuild/SPECS/

# Build RPM
rpmbuild -ba ~/rpmbuild/SPECS/confrisk.spec

# RPM created at:
# ~/rpmbuild/RPMS/x86_64/confrisk-0.2.0-1.x86_64.rpm
```

### Distribute via COPR (Fedora PPA equivalent)

1. Go to https://copr.fedorainfracloud.org/
2. Create new project
3. Upload .spec file
4. Users install with:

```bash
sudo dnf copr enable yourusername/confrisk
sudo dnf install confrisk
```

---

## Arch Linux (AUR)

### Step 1: Create PKGBUILD

Create `PKGBUILD`:

```bash
# Maintainer: Your Name <email@example.com>
pkgname=confrisk
pkgver=0.2.0
pkgrel=1
pkgdesc="Linux security configuration scanner with risk-based scoring"
arch=('x86_64')
url="https://github.com/yourusername/confrisk"
license=('MIT')
depends=('gcc-libs')
makedepends=('cargo' 'rust')
source=("$pkgname-$pkgver.tar.gz::$url/archive/v$pkgver.tar.gz")
sha256sums=('SKIP')

build() {
    cd "$srcdir/$pkgname-$pkgver"
    cargo build --release --locked --bin confrisk
    cargo build --release --locked --bin confrisk-npm
}

package() {
    cd "$srcdir/$pkgname-$pkgver"

    # Install binaries
    install -Dm755 target/release/confrisk "$pkgdir/usr/bin/confrisk"
    install -Dm755 target/release/confrisk-npm "$pkgdir/usr/bin/confrisk-npm"

    # Install config
    mkdir -p "$pkgdir/etc/confrisk"
    cp -r config/* "$pkgdir/etc/confrisk/"

    # Install docs
    install -Dm644 README.md "$pkgdir/usr/share/doc/$pkgname/README.md"
    install -Dm644 CONFRISK_NPM.md "$pkgdir/usr/share/doc/$pkgname/CONFRISK_NPM.md"

    # Install license
    install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
```

### Step 2: Publish to AUR

```bash
# Test build locally
makepkg -si

# Create AUR repository
git clone ssh://aur@aur.archlinux.org/confrisk.git
cd confrisk
cp ../PKGBUILD .

# Update checksums
updpkgsums

# Generate .SRCINFO
makepkg --printsrcinfo > .SRCINFO

# Commit and push
git add PKGBUILD .SRCINFO
git commit -m "Initial commit: confrisk 0.2.0"
git push
```

### Step 3: Users Install

```bash
# Users install with:
yay -S confrisk
# or
paru -S confrisk
```

**Timeline:** Hours (immediate once pushed)

**Pros:**
- Easy to publish
- Arch users love AUR
- Automatic updates via AUR helpers
- Free

**Cons:**
- Arch Linux only
- Users compile from source (slow first install)

---

## Automated Publishing

### GitHub Actions Workflow

Create `.github/workflows/release.yml`:

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build-binaries:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        target:
          - x86_64-unknown-linux-gnu
          - x86_64-unknown-linux-musl
          - aarch64-unknown-linux-gnu

    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}

      - name: Install cross
        run: cargo install cross --git https://github.com/cross-rs/cross

      - name: Build
        run: |
          cross build --release --target ${{ matrix.target }} --bin confrisk
          cross build --release --target ${{ matrix.target }} --bin confrisk-npm

      - name: Package
        run: |
          mkdir -p release/confrisk-${{ github.ref_name }}-linux-${{ matrix.target }}
          cp target/${{ matrix.target }}/release/confrisk release/confrisk-${{ github.ref_name }}-linux-${{ matrix.target }}/
          cp target/${{ matrix.target }}/release/confrisk-npm release/confrisk-${{ github.ref_name }}-linux-${{ matrix.target }}/
          cp -r config release/confrisk-${{ github.ref_name }}-linux-${{ matrix.target }}/
          cp README.md CONFRISK_NPM.md release/confrisk-${{ github.ref_name }}-linux-${{ matrix.target }}/
          cd release
          tar czf confrisk-${{ github.ref_name }}-linux-${{ matrix.target }}.tar.gz confrisk-${{ github.ref_name }}-linux-${{ matrix.target }}/
          sha256sum confrisk-${{ github.ref_name }}-linux-${{ matrix.target }}.tar.gz > confrisk-${{ github.ref_name }}-linux-${{ matrix.target }}.tar.gz.sha256

      - name: Upload artifacts
        uses: actions/upload-artifact@v3
        with:
          name: binaries-${{ matrix.target }}
          path: release/*.tar.gz*

  create-release:
    needs: build-binaries
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v4

      - name: Download artifacts
        uses: actions/download-artifact@v3
        with:
          path: artifacts

      - name: Create Release
        uses: softprops/action-gh-release@v1
        with:
          files: artifacts/**/*.tar.gz*
          generate_release_notes: true
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}

  publish-crates:
    needs: build-binaries
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Publish to crates.io
        run: cargo publish --token ${{ secrets.CARGO_TOKEN }}
```

---

## Recommended Publishing Strategy

### Phase 1: Immediate (Day 1)
```bash
1. GitHub Releases with static binaries
2. cargo publish to crates.io
3. Create install.sh script
```

### Phase 2: Week 1
```bash
4. Create AUR package (Arch Linux)
5. Create Snapcraft package (cross-distro)
```

### Phase 3: Month 1
```bash
6. Create Ubuntu PPA
7. Create Fedora COPR
8. Document all installation methods
```

### Phase 4: Long-term
```bash
9. Submit to official Debian repository
10. Submit to official Fedora repository
11. Create Homebrew formula (macOS)
12. Create Chocolatey package (Windows via WSL)
```

---

## Testing Installation

### Test Script

Create `test-install.sh`:

```bash
#!/bin/bash

echo "Testing confrisk installation methods..."

# Test 1: GitHub releases
echo "1. Testing GitHub binary install..."
curl -L https://github.com/yourusername/confrisk/releases/latest/download/confrisk-linux-x86_64.tar.gz -o /tmp/test.tar.gz
tar xzf /tmp/test.tar.gz -C /tmp/
/tmp/confrisk-*/confrisk --help && echo "✅ Binary works"

# Test 2: Snap
echo "2. Testing snap install..."
sudo snap install confrisk
confrisk --help && echo "✅ Snap works"

# Test 3: Cargo
echo "3. Testing cargo install..."
cargo install confrisk
~/.cargo/bin/confrisk --help && echo "✅ Cargo works"

# Test 4: PPA
echo "4. Testing PPA install..."
sudo add-apt-repository ppa:yourusername/confrisk -y
sudo apt-get update
sudo apt-get install confrisk -y
confrisk --help && echo "✅ PPA works"
```

---

## Summary: Quick Start Publishing

```bash
# 1. Create GitHub release (immediate)
git tag v0.2.0
git push origin v0.2.0
# Upload binaries manually or via GH Actions

# 2. Publish to crates.io (Rust users)
cargo publish

# 3. Create AUR package (Arch users, ~1 hour)
# Follow AUR section above

# 4. Create Snap (all distros, ~1 day)
snapcraft upload confrisk_*.snap --release=stable

# 5. Create Ubuntu PPA (Ubuntu users, ~1 day)
# Follow PPA section above
```

After these 5 steps, users can install with:

```bash
# GitHub releases
curl -L install.sh | bash

# Cargo
cargo install confrisk

# Snap (Ubuntu, Fedora, etc.)
sudo snap install confrisk

# Arch
yay -S confrisk

# Ubuntu PPA
sudo add-apt-repository ppa:yourusername/confrisk
sudo apt install confrisk
```

---

**Version:** 0.2.0
**Last Updated:** May 24, 2026
**Next Steps:** Choose your distribution method based on target audience
