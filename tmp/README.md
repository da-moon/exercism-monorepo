# Template Rust Project

A Rust template project configured for static linking with musl and featuring OS-agnostic, multi-architecture Cargo Make workflows.

## Features

- **Dynamic Workspace Discovery**: Automatically detects workspace members without manual configuration
- **Multi-Target Support**: Build, test, and package for multiple architectures/OS combinations
- **OS-Agnostic**: Pure Duckscript implementation works on Linux, macOS, and Windows
- **Library Support**: Dedicated tasks for managing and testing library examples
- **Agent-Friendly**: Clear, consistent interface for both human developers and AI agents

## Quick Start Guide

### 1. Clone the Template

```bash
# Clone this template repository
git clone https://github.com/your-username/template-rs.git my-cli-app
cd my-cli-app

# Remove the original git history and start fresh
rm -rf .git
git init
git add .
git commit -m "feat: initial commit from template"
```

### 2. Rename Your Project

Update the project name in `Cargo.toml`:

```toml
[package]
name = "my-cli-app"  # Change from "template-rs"
version = "0.1.0"
edition = "2021"
# ... rest of configuration
```

### 3. Build Statically Linked CLI Apps

#### Setup (Ubuntu/Debian)

```bash
# Run the setup script to install musl toolchain
./scripts/musl-toolchain-ubuntu-setup.sh --test
```

#### Build Your CLI App

```bash
# Build a statically linked binary
cargo build --target x86_64-unknown-linux-musl --release

# Your binary will be at:
# target/x86_64-unknown-linux-musl/release/my-cli-app

# Test that it's statically linked
ldd target/x86_64-unknown-linux-musl/release/my-cli-app
# Should output: "not a dynamic executable"
```

#### Cross-Platform Builds

```bash
# Build for ARM64 Linux
cargo build --target aarch64-unknown-linux-musl --release

# Build for ARM7 Linux
cargo build --target armv7-unknown-linux-musleabihf --release
```

### 4. Distribute Your CLI

The resulting binaries are completely self-contained and can be distributed
without any dependencies:

```bash
# Copy binary to target system
scp target/x86_64-unknown-linux-musl/release/my-cli-app user@server:/usr/local/bin/

# Or create a release archive
tar -czf my-cli-app-linux-x64.tar.gz -C target/x86_64-unknown-linux-musl/release my-cli-app
```

## Development Scripts

### musl Toolchain Setup

The `scripts/musl-toolchain-ubuntu-setup.sh` script sets up the musl toolchain
for building static binaries on Ubuntu/Debian systems.

**Note**: This script is marked as executable via Git attributes. On Unix-like
systems, you may need to set executable permissions manually:

```bash
chmod +x scripts/musl-toolchain-ubuntu-setup.sh
```

#### Usage

```bash
# Full installation
./scripts/musl-toolchain-ubuntu-setup.sh

# Full installation with test build
./scripts/musl-toolchain-ubuntu-setup.sh --test

# Skip Rust installation (if already installed)
./scripts/musl-toolchain-ubuntu-setup.sh --skip-rust

# Skip cross-compilation tools
./scripts/musl-toolchain-ubuntu-setup.sh --no-cross

# Show help
./scripts/musl-toolchain-ubuntu-setup.sh --help
```

#### What the script does

1. **System Requirements Check**: Verifies the system is Ubuntu/Debian
   compatible
2. **Package Installation**: Installs musl development tools, build essentials,
   and dependencies
3. **Rust Setup**: Installs Rust (if not present) and adds musl targets
4. **Cargo Configuration**: Sets up cargo config for musl linking
5. **Cross-compilation Tools**: Installs additional toolchains for
   cross-compilation
6. **Verification**: Tests that the installation works correctly
7. **Optional Testing**: Can run a test build to verify everything works

## Building Static Binaries

After running the setup script, you can build static binaries using:

```bash
# Build for x86_64 Linux with musl
cargo build --target x86_64-unknown-linux-musl --release

# Build for ARM64 Linux with musl
cargo build --target aarch64-unknown-linux-musl --release

# Build for ARMv7 Linux with musl
cargo build --target armv7-unknown-linux-musleabihf --release
```

The resulting binaries will be statically linked and can run on any compatible
Linux system without external dependencies.

## Cargo Make Usage

This template includes comprehensive Cargo Make workflows for building, testing, and packaging across multiple targets.

### Basic Commands

```bash
# Install cargo-make (if not already installed)
cargo install cargo-make

# Single-crate tasks (host target)
cargo make build              # Build for host
cargo make test               # Run tests
cargo make check              # Run cargo check
cargo make clippy             # Run clippy
cargo make validate           # Run full validation (format, build, test, clippy)

# Multi-target tasks
TARGETS="x86_64-unknown-linux-gnu aarch64-unknown-linux-gnu" cargo make build_targets
TARGETS="x86_64-unknown-linux-musl x86_64-pc-windows-gnu" cargo make test_targets
cargo make package_targets    # Package binaries to dist/<triple>/

# Environment variables
RELEASE=1 cargo make build    # Build in release mode
STRICT=1 cargo make clippy    # Use strict clippy settings
FEATURES="feature1,feature2" cargo make build
ALL_FEATURES=1 cargo make test
```

### Workspace Commands

For multi-crate workspaces:

```bash
# Build all workspace members
cargo make build_all

# Target matrix builds (all members × all targets)
TARGETS="x86_64-unknown-linux-gnu aarch64-unknown-linux-gnu" cargo make build_all_members_targets

# Per-member operations
MEMBER=my-crate cargo make build_member
MEMBER=my-crate cargo make test_member

# List workspace members
cargo make list_members
```

### Library Example Commands

For library crates with examples:

```bash
# List available examples
cargo make list_examples

# Run a specific example
EXAMPLE=my_example cargo make run_example

# Build/test all examples
cargo make build_examples
cargo make test_examples

# Multi-target example builds
TARGETS="x86_64-unknown-linux-gnu aarch64-unknown-linux-gnu" cargo make build_examples_targets
```

### Cross-Compilation Notes

- Tests automatically skip for cross-compilation targets unless a runner is configured
- Set runners via environment variables: `CARGO_TARGET_<TRIPLE>_RUNNER`
- Example: `CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_RUNNER=qemu-aarch64`

### Agent Usage

For AI agents and automation:

```bash
# Prepare environment and build for multiple targets
export TARGETS="x86_64-unknown-linux-musl aarch64-unknown-linux-musl"
cargo make prepare_targets    # Installs missing targets
cargo make build_targets      # Builds for all targets
cargo make package_targets    # Creates dist/<triple>/ artifacts

# Full validation across targets
cargo make validate_targets

# Library development
cargo make list_examples
EXAMPLE=demo cargo make run_example
```

## Project Structure

```
.
├── scripts/
│   └── musl-toolchain-ubuntu-setup.sh  # Toolchain setup script
├── src/
│   └── main.rs                         # Application entry point
├── build/
│   ├── git.rs                         # Git information extraction
│   └── metadata.rs                    # Build metadata extraction
├── build.rs                           # Build script for static linking
├── Cargo.toml                         # Project configuration
├── Cargo.lock                         # Dependency lock file
├── Makefile.toml                      # Main cargo-make configuration
├── Makefile.workspaces.toml           # Workspace-specific tasks
├── Makefile.lib.toml                  # Library example tasks
├── rust-toolchain.toml               # Rust toolchain specification
└── README.md                          # This file
```

## Features

- **Static Linking**: Configured for building fully static binaries
- **musl Support**: Optimized for musl libc static linking
- **Cross-compilation**: Support for multiple target architectures
- **Build Metadata**: Automatic inclusion of Git and build information
- **Development Tools**: Comprehensive toolchain setup script

## License

[Add your license information here]
