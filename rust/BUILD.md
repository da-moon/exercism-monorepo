# Build System & Development Commands

## Workspace Layout
- Root `Cargo.toml` is now a workspace; every Exercism exercise directory is a member crate.
- Legacy template makefiles are preserved as `.bak` for reference only.
- All automation lives in modular Cargo Make files under `contrib/cargo-make/`, grouped by cargo command:
  - `fmt/` (cargo fmt), `build/` (build/check/run incl. targets), `test/` (cargo test), `clippy/` (lint), plus supporting `init/`, `core/`, `validation/`, `packaging/`, `shortcuts/`.

## Critical Build Rules
- **Never** call `cargo build|check|test|fmt|clippy` directly; always go through `cargo make` tasks.
- Prefer workspace-aware tasks (`*all`) when touching multiple exercises; use `MEMBER=<crate>` for single-crate tasks.
- Run formatting + checks after any change: `cargo make format_all` then `cargo make check_member MEMBER=<crate>` (or `check_all`).

## Core Cargo Make Tasks
- `cargo make list_members` — show all exercise crates detected in the workspace.
- `cargo make build_member MEMBER=<crate>` — build one exercise (host target).
- `cargo make test_member MEMBER=<crate>` — test one exercise (host target).
- `cargo make clippy_member MEMBER=<crate>` — clippy one exercise with configured lints.
- `cargo make build_all` / `test_all` / `clippy_all` — run across the entire workspace (host target).
- `cargo make format_all` / `format_fix_all` — workspace formatting check / fix.
- Cross-target matrix: set `TARGETS="x86_64-unknown-linux-gnu aarch64-apple-darwin"` then run `cargo make build_targets` for a single crate or `cargo make build_all_members_targets` for every crate.
- Packaging: `cargo make package_targets MEMBER=<crate>` (per crate) or `cargo make package_all_members_targets` (workspace sweep). `cargo make dist` is a shortcut for `RELEASE=1` + `package_targets` and requires `MEMBER` to be set.
- Shortcuts: `cargo make quick` (format+build+test current crate), `cargo make quick_all` (workspace), `cargo make release_targets` (release builds for all TARGETS).
- Cross presets: `cargo make build_linux`, `build_musl`, `build_windows`, `build_mac`, `build_cross_all` (preconfigures TARGETS env).

## Validation Sequences
- Full workspace: `cargo make validate_all` (format → build → test → clippy).
- Full workspace across targets: `cargo make validate_all_targets` (requires `TARGETS` + installed targets).
- Strict clippy (pedantic/nursery): `cargo make validate_all_strict`.
- Per-crate workflow: `cargo make format_all` → `cargo make build_member MEMBER=<crate>` → `cargo make test_member MEMBER=<crate>` → `cargo make clippy_member MEMBER=<crate>`.

## Helpful Queries
- Show every available task: `cargo make --list-all-steps`.
- List configured targets: `cargo make list_targets` (uses the `TARGETS` env var discovered in `init`).
- Dump and exercise every target automatically: `scripts/test-make-targets.sh` (writes logs to `tmp/cargo-make-target-logs/`).

## Notes
- Target installation is handled via `cargo make prepare_targets` (automatically pulled in by cross-target tasks).
- Init detects host triple, computes `TARGETS`, `CARGO_BUILD_ARGS`, `CLIPPY_ARGS`, and ensures `dist/` exists. Tasks depend on `init`; the global `init_task` is a no-op to avoid double runs.
- Workspace-aware commands respect `RELEASE=1` and `FEATURES`/`ALL_FEATURES` environment overrides when set.
