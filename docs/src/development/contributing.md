# Contributing

## Development Commands

```bash
cargo t              # alias for cargo nextest run
cargo clippy         # lint
cargo fmt --check    # format check
cargo doc --no-deps  # build docs (check for warnings)
```

## Project Structure

```
crates/
├── muster/         # Library crate
├── muster-cli/     # CLI binary crate
└── muster-notify/  # macOS notification helper
docs/               # mdBook documentation (this site)
```

## Documentation

### Building Docs Locally

```bash
# mdBook user guide
mdbook serve docs

# API reference (rustdoc)
cargo doc --no-deps --open

# Regenerate CLI reference
cargo run --example gen_cli_docs -p muster-cli > docs/src/cli-reference.md
```

### Writing Doc Comments

All public types, functions, and modules should have rustdoc comments:

- `//!` for module-level docs
- `///` for public types, functions, and methods

## Code Quality

The workspace enforces:

- `unsafe_code = "forbid"` — no unsafe Rust
- `clippy::all` and `clippy::pedantic` — comprehensive linting
- All public items documented
