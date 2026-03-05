# Architecture Overview

Muster is organized as a Cargo workspace with three crates:

```
crates/
├── muster/         # Library — tmux bindings, profiles, theming, control mode
├── muster-cli/     # CLI binary
└── muster-notify/  # macOS notification helper (minimal binary for Muster.app bundle)
```

## Design Principles

1. **tmux is the runtime** — muster is an organizational layer on top of tmux, not a replacement
2. **Library-first** — the CLI and GUI are independent consumers of the same `muster` library crate
3. **No application state** — running session metadata lives in tmux user options, not in files
4. **Push-based sync** — control mode provides structured notifications; no polling for state

## Library Modules

| Module | Purpose |
|--------|---------|
| `tmux::client` | Command execution, output parsing, session/window CRUD |
| `tmux::control` | Control mode connection, event stream parsing (`MusterEvent`) |
| `tmux::types` | `TmuxSession`, `TmuxWindow`, `SessionInfo` |
| `config::profile` | Profile CRUD with atomic JSON persistence |
| `config::settings` | Settings (tmux path, shell preference) |
| `session` | Session lifecycle — create from profile, destroy |
| `session::theme` | Hex color parsing, dimming, tmux status bar styling |
| `muster` | `Muster` facade tying everything together |

## Data Flow

```
Commands:    GUI/CLI → library → tmux / config
State:       Runtime state always from tmux, metadata from config files
Events:      tmux control mode → library → GUI (via Tauri events)
```

Control mode notifications cover window lifecycle, session lifecycle, and active tab changes. CWD tracking uses tmux's native `pane_current_path` with on-demand queries.

## Component Diagram

```
┌─────────────────────────────────────────────────────────┐
│                    Tauri Application                     │
│  ┌──────────┐ ┌──────────────┐ ┌─────────────────────┐  │
│  │   File   │ │    Group     │ │    Search            │  │
│  │ Browser  │ │  Launcher UI │ │  (ParavaneFS)        │  │
│  │ (beacons)│ │              │ │                      │  │
│  └────┬─────┘ └──────┬───────┘ └──────────────────────┘  │
│       └──────┬───────┘                                    │
│         ┌────▼────┐                                       │
│         │ Tauri   │                                       │
│         │Commands │                                       │
│         └────┬────┘                                       │
└──────────────┼────────────────────────────────────────────┘
               │
        ┌──────▼──────┐         ┌──────────────┐
        │             │         │              │
        │   muster    │◄────────│  CLI binary  │
        │  (library)  │         │   (muster)   │
        │             │         │              │
        └──┬───────┬──┘         └──────────────┘
           │       │
     ┌─────▼──┐ ┌──▼──────────────┐
     │ Config │ │      tmux       │
     │  dir   │ │  (sessions,     │
     │ (JSON) │ │   control mode) │
     └────────┘ └─────────────────┘
```
