# Pin & Unpin

Pin and unpin sync the current tmux window state back to the session's profile.

## Pin

```bash
muster pin
```

Run this from inside a muster-managed tmux session. It saves the current window's name, working directory, and command to the session's profile. This is useful when you've customized a window at runtime and want those changes persisted.

## Unpin

```bash
muster unpin
```

Removes the current window from the session's profile. The window continues to exist in the running session but won't be recreated when the profile is launched again.

## Window Rename Sync

Muster installs a tmux hook that automatically syncs window renames to the profile when a window is pinned. This means renaming a tab with `Ctrl-b ,` (tmux's rename-window) updates the profile if the window is pinned.
