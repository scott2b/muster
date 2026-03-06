# Muster: Engineering Standards Compliance Plan

Reference: `~/.claude/guides/rust-engineering.md`
Official style authority: https://doc.rust-lang.org/style-guide/

This plan brings muster into alignment with the shared Rust engineering standards.
Each phase is independently verifiable and should be committed separately.

All formatting is enforced by `cargo fmt` (configured via `rustfmt.toml`) and
`cargo clippy` (configured via workspace lints in `Cargo.toml`). No manual
formatting rules — if `cargo fmt` and `cargo clippy` pass, the code is compliant.

---

## Phase 1: Configuration Alignment [DONE]

Lowest risk. Configuration changes, then auto-formatting.

### 1a. Update Cargo.toml workspace metadata [DONE]
- Changed `edition = "2021"` to `edition = "2024"`
- Added `rust-version = "1.85"`
- Changed `unsafe_code = "forbid"` to `unsafe_code = "deny"`
- Added `similar_names = "allow"` to workspace clippy lints (common pedantic
  noise for intentional parallel naming like cpu/gpu, pid/ppid)
- muster-notify retains its per-crate `unsafe_code = "allow"` for objc2 FFI

### 1b. Update rustfmt.toml [DONE]
- Added `edition = "2024"` and `use_try_shorthand = true`
- Ran `cargo fmt --all` — reformatted 8 source files (let-chains, closures,
  expression formatting per edition 2024 rules)

### 1c. Fix clippy warnings from edition upgrade [DONE]
- Auto-fixed via `cargo clippy --fix`: collapsible_if with let-chains, midpoint,
  doc_markdown backticks, format args inlining, map_unwrap_or, if_not_else
- Manual fixes: `clone_from` for 4 assignment clones, `_app` -> `app` rename,
  `WindowStats` struct moved before statements, `#[allow(cast_possible_truncation)]`
  on color math, `#[allow(cast_precision_loss)]` on memory formatting
- Result: zero clippy warnings

### 1d. Update nextest config [DONE]
- Added `[store]`, `[profile.default]`, `[profile.ci]` sections
- Existing test-group configuration preserved

**Verified**: `cargo fmt --check` passes. `cargo clippy --workspace` zero
warnings. `cargo nextest run` 76 passed, 26 skipped.

---

## Phase 2: Structured Tracing [DONE]

Added structured tracing to core library operations.

### 2a. Audit [DONE]
- Core library had zero tracing calls. CLI `eprintln!` usage is all user-facing
  error/validation messages — appropriate as-is, not converted.
- No `println!` in core library (clean separation).

### 2b. Tracing added to key operations [DONE]
- `Muster::init` — debug: config_dir, tmux path discovery
- `Muster::launch` — info: profile name + session name; debug: session reuse
- `Muster::destroy` — info: session name
- `Muster::save_profile` — info: id + name
- `Muster::update_profile` — debug: id
- `Muster::rename_profile` — info: old_id + new_id
- `Muster::delete_profile` — info: id
- `TmuxClient::cmd` — debug: command args; warn: failed commands with stderr
- `TmuxClient::source_file` — debug: batch command count
- tracing-subscriber already initialized in CLI main() with env-filter

### 2c. muster-notify review [DONE]
- Left as-is. The custom `log()` function writes to stderr + `/tmp/muster-notify.log`.
  This is necessary because muster-notify runs as a background macOS notification
  helper with no terminal attached. tracing-subscriber's default stderr output
  would be lost. A file appender could work but adds dependency complexity for
  a single-file helper binary — not worth it.

**Verified**: `cargo clippy --workspace` zero warnings. `cargo fmt --check` passes.
76 tests passed, 26 skipped.

---

## Phase 3: Code Organization Polish

### 3a. Review pub vs pub(crate) usage
- Audit functions/types that are `pub` but only used within the crate.
- Change to `pub(crate)` where appropriate.
- This may surface unused public API — that's fine to clean up.

**Verify**: `cargo check --workspace` passes (downstream crates still compile).
`cargo nextest run` passes.

---

## Phase 4: Testing Infrastructure

### 4a. Add insta for snapshot testing
- Add `insta` to workspace dev-dependencies.
- Identify CLI output or serialization formats that benefit from snapshot testing.
- Add snapshot tests for profile serialization and CLI output formatting.

**Verify**: `cargo nextest run` passes. Snapshots reviewed and accepted.

---

## Ongoing Compliance

After all phases are complete, compliance is maintained by:
- `cargo fmt --all --check` — formatting (run in CI)
- `cargo clippy --workspace` — linting with zero warnings (run in CI)
- `cargo nextest run` — tests pass (run in CI)

These three commands are the single source of truth for code compliance.

---

## Notes

- Each phase should be a separate commit (or small set of commits).
- Run the full test suite after each phase:
  `cargo nextest run && cargo nextest run --run-ignored all`
- If any phase causes unexpected breakage, stop and investigate before continuing.
- No backwards compatibility concerns — this is pre-release software.
