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
  "terminal": "ghostty",
  "shell": "/usr/local/bin/fish",
  "tmux_path": null
}
```

Settings can be viewed and updated with the `muster settings` command:

```bash
# Show current settings
muster settings

# Update a setting
muster settings --terminal ghostty
muster settings --shell /usr/local/bin/fish
muster settings --tmux-path /usr/local/bin/tmux
```

### `terminal`

The terminal emulator to open when launching a session from inside tmux. If omitted, muster uses the platform default (Terminal.app on macOS; detected from PATH on Linux).

Supported values: `ghostty`, `kitty`, `alacritty`, `wezterm`, `terminal` (Terminal.app), `iterm2`.

### `shell`

Overrides the default shell for new tmux panes. If omitted, muster uses `$SHELL`. Set this if your `$SHELL` differs from the shell you actually use (common on macOS where `$SHELL` defaults to `/bin/zsh`).

### `tmux_path`

Overrides tmux discovery from `$PATH`. Set this if tmux is installed in a non-standard location.

## Profiles (`profiles.json`)

See [Profiles](profiles.md) for the full profile schema and management commands.

## Shell Integration

Muster can suggest launching profiles when you `cd` into a directory associated with one. Add the shell hook to your shell config:

**Fish** — add to `~/.config/fish/config.fish`:

```fish
muster shell-init fish | source
```

**Bash** — add to `~/.bashrc` or `~/.bash_profile`:

```bash
eval "$(muster shell-init bash)"
```

**Zsh** — add to `~/.zshrc`:

```zsh
eval "$(muster shell-init zsh)"
```

After setup, when you `cd` into a directory that matches a profile tab's CWD, muster prints a suggestion:

```
muster: profile 'webapp' matches this directory. Run: muster up webapp
```
