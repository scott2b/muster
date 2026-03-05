# Configuration

## Config Directory

Muster stores configuration in `~/.config/muster/` by default. Override with `--config-dir` or the `MUSTER_CONFIG_DIR` environment variable.

```
~/.config/muster/
├── profiles.json             # Saved terminal group profiles
├── settings.json             # Global settings
├── logs/                     # Death snapshots
│   └── <session_name>/
│       └── <window_name>.last
└── Muster.app/               # macOS notification helper (optional)
```

## Settings (`settings.json`)

```json
{
  "tmux_path": null,
  "shell": "/usr/local/bin/fish"
}
```

### `shell`

Overrides the default shell for new tmux panes. If omitted, muster uses `$SHELL`. Set this if your `$SHELL` differs from the shell you actually use (common on macOS where `$SHELL` defaults to `/bin/zsh`).

### `tmux_path`

Overrides tmux discovery from `$PATH`. Set this if tmux is installed in a non-standard location.

## Profiles (`profiles.json`)

See [Profiles](profiles.md) for the full profile schema and management commands.
