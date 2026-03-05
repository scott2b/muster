# Sessions

Sessions are running tmux sessions managed by muster.

## Launching

```bash
# From a profile (creates or reattaches)
muster launch "My Project"

# Create without attaching
muster launch "My Project" --detach
```

`launch` is idempotent — if the session already exists, it attaches. If not, it creates from the profile and attaches.

`launch` and `attach` replace the current process with `exec tmux attach`. Use `--detach` to create the session in the background without attaching.

## Ad-hoc Sessions

Create a session without a saved profile:

```bash
muster new "Scratch" --tab 'Shell:~/work' --color '#808080'
muster new "Quick" --detach
```

If `--tab` is omitted, defaults to a single "Shell" tab at `$HOME`.

## Attaching

```bash
# By profile name or session name
muster attach muster_my-project

# Switch to a specific window on attach
muster attach muster_my-project --window 2
```

## Status

```bash
# Show all sessions with window details
muster status

# List profiles and running sessions
muster list
```

## Inspecting Sessions

```bash
# Show processes running inside sessions
muster ps
muster ps "My Project"

# Show listening ports
muster ports
muster ports "My Project"
```

## Changing Colors

Change a running session's color without restarting:

```bash
muster color muster_my-project '#22c55e'
```

The tmux status bar updates instantly. This does not update the profile — to persist the color change, use `muster profile update`.

## Destroying Sessions

```bash
muster kill muster_my-project
```

Session metadata dies with the tmux session — no file cleanup needed.

## JSON Output

All commands support `--json` for machine-readable output:

```bash
muster status --json
muster list --json
muster peek my-project --json
```
