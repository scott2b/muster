# Muster

Terminal session group management built on tmux. A Rust library and CLI for organizing terminal sessions into named, color-coded groups with saved profiles, runtime theming, and push-based state synchronization via tmux control mode.

## Prerequisites

- **tmux** — installed and available in PATH
- **Rust** — 2021 edition (for building from source)

## Installation

```bash
cargo install --path crates/muster-cli
```

## Quick Start

```bash
# Create a profile for a project
muster profile save "My Project" --cwd ~/work/myproject --color '#f97316'

# Launch a session from the profile
muster launch "My Project"

# See what's running
muster status

# Change a session's color live
muster color muster_profile_abc123 '#00ff00'

# Create an ad-hoc session (no saved profile)
muster new "Scratch" --cwd /tmp --color '#808080'

# Destroy a session
muster kill muster_profile_abc123
```

## CLI Reference

```
muster list                                    # List profiles and running sessions
muster launch <profile-name-or-id>             # Launch or attach to a profile's session
muster attach <session-name> [--window N]      # Attach to a running session
muster kill <session-name>                     # Destroy a session
muster new <name> [--cwd <dir>] [--color <hex>]  # Create ad-hoc session
muster color <session> <hex-color>             # Change session color live
muster status                                  # Show all sessions with window details
muster profile save <name> [--cwd <dir>] [--color <hex>]
muster profile list
muster profile delete <name-or-id>
```

All commands accept `--json` for machine-readable output. Use `--config-dir` or `MUSTER_CONFIG_DIR` to override the default config directory (`~/.config/muster/`).

## Concepts

**Terminal group** — A named tmux session containing one or more windows (tabs), each with a working directory and optional startup command. Groups have a display name, color, and optional profile reference.

**Profile** — A saved template for creating a group. Stored in `~/.config/muster/profiles.json`. Contains the group's name, color, and tab definitions.

**Session** — A running tmux session managed by muster. Session names are prefixed with `muster_` to distinguish them from personal tmux sessions. Application metadata (name, color, profile ID) is stored as tmux user options (`@muster_name`, `@muster_color`, `@muster_profile`) on the session itself — no separate state file.

## Architecture

Muster is organized as a Cargo workspace with two crates:

```
crates/
├── muster/         # Library — tmux bindings, profiles, theming, control mode
└── muster-cli/     # CLI binary
```

### Library Modules

| Module | Purpose |
|--------|---------|
| `tmux::client` | Command execution, output parsing, session/window CRUD |
| `tmux::control` | Control mode connection, event stream parsing (`MusterEvent`) |
| `tmux::types` | `TmuxSession`, `TmuxWindow`, `SessionInfo` |
| `config::profile` | Profile CRUD with atomic JSON persistence |
| `config::settings` | Settings (emulator preference, tmux path) |
| `session` | Session lifecycle — create from profile, destroy |
| `session::theme` | Hex color parsing, dimming, tmux status bar styling |
| `emulator` | `Emulator` trait + Ghostty implementation |
| `muster` | `Muster` facade tying everything together |

### Library Usage

```rust
use muster::{Muster, Profile, TabProfile};
use std::path::Path;

let m = Muster::init(Path::new("~/.config/muster"))?;

// Create a profile
let profile = Profile {
    id: "profile_myproject".into(),
    name: "My Project".into(),
    color: "#f97316".into(),
    tabs: vec![
        TabProfile { name: "Shell".into(), cwd: "/home/user/project".into(), command: None },
        TabProfile { name: "Server".into(), cwd: "/home/user/project".into(), command: Some("npm run dev".into()) },
    ],
};
m.save_profile(profile.clone())?;

// Launch a session
let info = m.launch(&profile.id)?;

// List running sessions
let sessions = m.list_sessions()?;

// Subscribe to events (for GUI integration)
let rx = m.subscribe();
```

### Sources of Truth

| Source | Owns |
|--------|------|
| **tmux** | All running state: windows, CWDs, active window, plus `@muster_*` metadata |
| **Config directory** | Saved profiles and settings — never runtime state |

There is no application-level cache. When you need session state, you ask tmux.

### Control Mode

For long-running consumers (GUI applications), muster can establish tmux control mode connections that provide push-based notifications:

- `%window-add`, `%window-close`, `%window-renamed` — tab lifecycle
- `%session-window-changed` — active tab changes
- `%sessions-changed` — session lifecycle
- Response block framing (`%begin`/`%end`) for command output

These are parsed into `MusterEvent` variants and distributed via `tokio::broadcast`.

## Configuration

### `~/.config/muster/profiles.json`

```json
{
  "profiles": {
    "profile_abc123": {
      "id": "profile_abc123",
      "name": "PKM Project",
      "color": "#f97316",
      "tabs": [
        { "name": "Shell", "cwd": "/Users/sbb/work/pkm", "command": null },
        { "name": "Server", "cwd": "/Users/sbb/work/pkm", "command": "npm run dev" }
      ]
    }
  }
}
```

### `~/.config/muster/settings.json`

```json
{
  "emulator": "ghostty",
  "emulator_path": null,
  "tmux_path": null
}
```

## Testing

```bash
# Unit tests (no tmux required)
cargo nextest run

# All tests including integration (requires tmux)
cargo nextest run --run-ignored all

# Or with cargo test
cargo test                          # unit tests
cargo test -- --ignored             # integration tests
```

Integration tests create sessions with unique names and clean up after themselves. They do not interfere with your personal tmux sessions.

## Development

```bash
cargo t              # alias for cargo nextest run
cargo clippy         # lint
cargo fmt --check    # format check
```
