{
  description = "Development flake for rust";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
          targets = [
            "x86_64-unknown-linux-musl"
            "aarch64-unknown-linux-musl"
          ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Rust toolchain with rust-analyzer
            rustToolchain
            rust-analyzer

            # Essential cargo extensions
            cargo-make        # Primary build orchestrator
            cargo-nextest     # Better test runner with cleaner output
            cargo-machete     # Find unused dependencies
            cargo-audit       # Security vulnerability scanning
            cargo-expand      # Macro debugging
            cargo-watch       # Auto-rebuild on file changes
            cargo-edit        # Manage dependencies (cargo add/rm)
            cargo-outdated    # Check for outdated dependencies

            # Build & performance
            sccache           # Compilation cache for faster builds
            pkg-config
            openssl.dev
            stdenv.cc         # C toolchain for linking native deps
            binutils
            glibc.dev         # Headers/libs for glibc target builds

            # Development tools
            lldb              # Debugger
            git               # Required by build.rs (uses gix)
            gh                # GitHub CLI for PR/issue management
            exercism          # Exercism CLI

            # Utilities
            jq                # JSON processing
            shellcheck        # Lint bash scripts
            shfmt             # Format shell scripts
            watchexec         # Generic file watcher
            starship          # Shell Prompt
          ];
          shellHook = ''
            echo "🦀 Rust Development Environment"
            echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
            echo "Rust: $(rustc --version | cut -d' ' -f2)"
            echo "Targets: x86_64-unknown-linux-musl, aarch64-unknown-linux-musl"
            echo ""
            echo "Key Commands:"
            echo "  cargo make check            - Quick compilation check"
            echo "  cargo make build            - Build project"
            echo "  cargo make test             - Run tests"
            echo "  cargo make validate         - Full validation suite"
            echo "  cargo make --list-all-steps - List all available tasks"
            echo ""
            echo "Development Tools:"
            echo "  cargo watch -x run          - Auto-rebuild on changes"
            echo "  cargo nextest run           - Run tests with better output"
            echo "  cargo audit                 - Check for vulnerabilities"
            echo "  cargo machete               - Find unused dependencies"
            echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

            # Enable sccache for faster rebuilds
            export RUSTC_WRAPPER=sccache
            export SCCACHE_DIR="''${SCCACHE_DIR:-$HOME/.cache/sccache}"

            # Initiate starship prompt
            eval "$(starship init bash)"
          '';

          # Development environment variables
          RUST_BACKTRACE = "1";
        };
      });
}
