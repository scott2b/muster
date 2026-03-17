# Quick Start

## Try It: System Dashboard

Here's a ready-to-use profile you can launch right now. It opens three tabs — a shell, a live process monitor, and a live disk usage view:

```bash
muster profile save sysmon --color indigo \
  --tab 'Shell:~' \
  --tab 'Processes:~:top' \
  --tab 'Disk:~:sh -c "while true; do clear; df -h; sleep 5; done"'

muster up sysmon
```

You're now inside tmux with three tabs. Switch between them with `Ctrl-b n` (next) or `Ctrl-b 0/1/2`. Detach with `Ctrl-b d` to return to your shell — the session keeps running.

> If you have `htop` installed, substitute `htop` for `top` in the Processes tab for a better experience.

## Create Your Own Profile

Save a profile for a project:

```bash
muster profile save myproject --tab 'Shell:~/work/myproject' --color orange
```

Multi-tab example for a web project:

```bash
muster profile save webapp --color '#3b82f6' \
  --tab 'Shell:~/work/app' \
  --tab 'Server:~/work/app:npm start' \
  --tab 'Logs:~/work/app/logs'
```

## Start a Session

```bash
muster up myproject
```

This creates the tmux session and drops you in. If the session already exists, `up` reattaches instead of creating a duplicate.

## Check What's Running

From another terminal:

```bash
muster status
```

## Reattach

```bash
muster up sysmon
```

## Ad-hoc Sessions

Create a throwaway session without saving a profile:

```bash
muster new scratch
```

## Typical Workflow

1. **`muster profile save`** — define a project (name, tabs, color)
2. **`muster up <name>`** — start or reattach (execs `tmux attach`, replacing your shell)
3. Work inside tmux. Use `Ctrl-b d` to detach back to your regular shell.
4. **`muster up <name>`** again to reattach later
5. **`muster status`** from another terminal to see all sessions
6. **`muster down <name>`** when done
