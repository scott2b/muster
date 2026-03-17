# Notifications

Muster sends notifications on session events — pane exits (with last output lines) and terminal bell alerts.

## Default Behavior

By default, notifications appear as tmux status bar messages. No setup required.

## macOS Desktop Notifications

On macOS, you can enable native desktop notifications (Notification Center):

```bash
cargo install --path crates/muster-notify
muster notifications setup
```

This creates a minimal `MusterNotify.app` bundle at `~/.config/muster/MusterNotify.app/` containing the `muster-notify` helper binary. The app bundle provides a `CFBundleIdentifier` (`com.muster.notifier`) that macOS requires for persistent Notification Center access.

macOS may prompt you to allow notifications from Muster on first use.

When the helper is installed, notifications are delivered to Notification Center instead of the tmux status bar.

## SSH Fallback

Over SSH (`SSH_CONNECTION` is set), muster falls back to `tmux display-message` automatically, regardless of whether the notification helper is installed.

## Removing Notifications

```bash
muster notifications remove
```

This removes the `MusterNotify.app` bundle. Notifications revert to tmux status bar messages.

## How It Works

Muster installs tmux hooks (`pane-died` and `alert-bell`) that invoke CLI subcommands to deliver notifications. The hooks are set per-session when a session is created.

Pane death notifications include the last few lines of terminal output in the notification body, so you can see at a glance why something crashed.
