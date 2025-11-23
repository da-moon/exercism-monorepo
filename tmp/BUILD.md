# Build System & Development Commands

## Critical Build Rules

- **NEVER** use `cargo build`, `cargo test`, `cargo check`, `cargo fmt` and
  `cargo clippy` directly
- **ALWAYS** use `cargo make` targets when possible : `cargo make build` ,
  `cargo make format` , `cargo make check`, `cargo make test` ,
  `cargo make clippy`
- **IMMEDIATELY** run `cargo make check` and `cargo make format` after any code
  change to ensure it compiles
- **MANDATORY** validation sequence before any commit (see Validation Commands
  section)

## Cargo Make targets

**Note**: While direct `cargo` commands are shown below for reference, always
use `cargo make` commands for consistency and to ensure proper configuration.

the following are some ( not all ) of the targets

```bash
cargo make build                  # Recommended
# Direct: cargo build --message-format=short

cargo make test                   # Recommended
# Direct: cargo test --message-format=short

cargo make check                  # Recommended
# Direct: cargo check --message-format=short

cargo make clippy                 # Recommended
# Direct: cargo clippy --message-format=short --all-targets --all-features -- -D warnings

cargo make format                 # Recommended (check only)
# Direct: cargo fmt -- --check

cargo make strict-clippy          # Recommended
# Direct: cargo clippy --all-features --message-format=short -- --deny warnings --deny clippy::pedantic --deny clippy::nursery --allow clippy::wildcard_imports --allow clippy::used_underscore_binding --allow clippy::missing_docs_in_private_items --allow clippy::missing_panics_doc --allow clippy::missing_errors_doc --allow clippy::missing_safety_doc --allow clippy::doc_markdown
```

**run the following to see the complete target list**

```bash
cargo make --list-all-steps
```

## Mandatory Validation Sequence

After making ANY changes, you MUST execute these validation commands in the
specified order:

```bash
cargo make validate_initial
```

Before final commit , you should run the following

```bash
cargo make validate
```

## DeepSource Integration

For additional code quality analysis, use DeepSource validation:

```bash
# Authenticate with DeepSource
deepsource auth login --with-token $DEEPSOURCE_TOKEN

# List issues found by Rust analyzer in SARIF format
deepsource issues list --analyzer rust --sarif
```
