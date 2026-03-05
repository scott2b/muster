# Installation

## Prerequisites

- **tmux** — installed and available in PATH
- **Rust** — 2021 edition (for building from source)

## Install from Source

```bash
cargo install --path crates/muster-cli
```

This installs the `muster` binary.

### Optional: macOS Desktop Notifications

```bash
cargo install --path crates/muster-notify
muster notifications setup
```

This creates a minimal `Muster.app` bundle at `~/.config/muster/Muster.app/` containing the notification helper binary. The app bundle provides a `CFBundleIdentifier` that macOS requires for persistent Notification Center access. See [Notifications](../user-guide/notifications.md) for details.
