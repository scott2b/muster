# tmux Interface

## Sources of Truth

| Source | Owns |
|--------|------|
| **tmux** | All running state: windows, CWDs, active window, plus `@muster_*` metadata |
| **Config directory** | Saved profiles and settings — never runtime state |

There is no application-level cache of tmux state. When a consumer needs to know what tabs a group has, it asks tmux.

## Session Metadata

Running session metadata is stored as tmux user options:

| tmux User Option | Value | Example |
|-----------------|-------|---------|
| `@muster_name` | Display name | `"Web App"` |
| `@muster_color` | Hex color | `"#f97316"` |
| `@muster_profile` | Profile ID | `"web-app"` |

Set on creation, queryable at any time:

```bash
tmux show-option -t muster_web-app -v @muster_color
tmux list-sessions -F '#{session_name} #{@muster_name} #{@muster_color}'
```

## Control Mode

For long-running consumers (GUI applications), muster establishes tmux control mode connections. Control mode is a persistent stdin/stdout pipe via `tmux -CC attach -t <session>` that provides push-based notifications:

### Events Consumed

**Window lifecycle:**

| Notification | Library Action |
|-------------|----------------|
| `%window-add` | Query window details, emit tab-added event |
| `%window-close` | Emit tab-closed event |
| `%window-renamed` | Emit tab-renamed event |
| `%session-window-changed` | Emit active-tab-changed event |

**Session lifecycle:**

| Notification | Library Action |
|-------------|----------------|
| `%sessions-changed` | Re-query session list |
| `%session-changed` | Update active session tracking |

Output notifications (`%output`) are suppressed via `refresh-client -f no-output`.

### Response Framing

Commands sent through control mode produce structured output blocks:

```
%begin <timestamp> <command_number> <flags>
<output lines>
%end <timestamp> <command_number> <flags>
```

## tmux Hooks

Muster uses tmux hooks selectively for fire-and-forget actions:

- **`pane-died`** — invokes `muster _pane-died` to capture death snapshots and send notifications
- **`alert-bell`** — invokes `muster _bell` to send bell notifications
- **`window-renamed`** — invokes `muster sync-rename` to sync pinned window names to profiles

Hooks are appropriate here because they trigger one-shot external commands rather than feeding a stateful event stream.

## Color Theming

Each session's color is applied to the tmux status bar:

```bash
tmux set-option -t <session> status-style "bg=<color>,fg=#000000"
tmux set-option -t <session> status-left "#[bg=<darker>,fg=#ffffff,bold] <name> #[default]"
tmux set-option -t <session> window-status-current-format "#[fg=<color>,bg=#000000,bold] #I: #W #[default]"
```

Color changes are live — no session restart required.
