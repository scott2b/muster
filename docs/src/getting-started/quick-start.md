# Quick Start

## Create a Profile

Save a profile for a project with one or more tabs:

```bash
# Single-tab profile
muster profile save "Notes" --tab 'Shell:~/work/notes' --color '#f97316'

# Multi-tab profile
muster profile save "Web App" --color '#3b82f6' \
  --tab 'Shell:~/work/app' \
  --tab 'Server:~/work/app:npm start' \
  --tab 'Logs:~/work/app/logs'
```

## Launch a Session

```bash
muster launch "Notes"
```

This creates the tmux session and drops you in. You're now inside tmux — detach with `Ctrl-b d` to return to your shell.

If the session already exists, `launch` reattaches instead of creating a duplicate.

## Check What's Running

From another terminal:

```bash
muster status
```

## Reattach

```bash
# By profile name
muster launch "Notes"

# By session name directly
muster attach muster_notes
```

## Ad-hoc Sessions

Create a quick throwaway session without saving a profile:

```bash
muster new "Scratch"
```

## Background Sessions

Create without attaching:

```bash
muster launch "Notes" --detach
muster new "Background" --detach
```

## Modify Profiles

```bash
# Add a tab to an existing profile
muster profile add-tab "Notes" --name Editor --cwd ~/work/notes

# Edit the full profile in $EDITOR
muster profile edit "Notes"
```

## Typical Workflow

1. **`muster profile save`** — define a project (name, tabs, color)
2. **`muster launch <name>`** — start or reattach (execs `tmux attach`, replacing your shell)
3. Work inside tmux. Use `Ctrl-b d` to detach back to your regular shell.
4. **`muster launch <name>`** again to reattach later
5. **`muster status`** from another terminal to see all sessions
6. **`muster kill <session>`** when done
