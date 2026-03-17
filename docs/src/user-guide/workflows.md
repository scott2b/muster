# Workflows

Muster is flexible about how you get to a managed session — you can start from a saved profile or from a running tmux session. This page makes the common workflows concrete.

## The Core Concepts

- **Profile** — a saved template in `~/.config/muster/profiles.json`. Defines name, color, and tabs. Persists across reboots.
- **Session** — a live tmux session. May or may not have a backing profile.
- **Pinned tab** — a window tied to the profile. Shows the session color. Recreated when you `muster up`.
- **Unpinned tab** — a window with no profile backing. Shows a red `●`. Lost when the session dies.

---

## Profile First (The Happy Path)

You know what you want before you start.

```bash
muster profile save myproject \
  --tab 'Shell:~/work/myproject' \
  --tab 'Server:~/work/myproject:npm run dev' \
  --color blue

muster up myproject        # creates session and attaches
```

Next time:

```bash
muster up myproject        # reattaches if running, creates if not
```

When done:

```bash
muster down myproject      # kills the session (profile is kept)
```

---

## Session First (Build Then Save)

You start working without a profile, then decide to keep it.

```bash
muster new scratch          # ad-hoc session, attaches immediately
```

Inside the session, open more tabs however you like (`Ctrl-b c`). When you're happy with the layout:

```bash
muster profile save myproject --from-session scratch
```

This snapshots the current tabs, saves a profile, and pins all windows in the live session (red dots clear immediately). The session is now fully managed:

```bash
muster down scratch
muster up myproject         # recreates from profile
```

---

## Adopting a Plain tmux Session

You have existing tmux sessions that predate muster.

```bash
tmux ls
# work: 3 windows (created ...)
# scratch: 1 window (created ...)
```

Bring one under muster management:

```bash
muster adopt work --name "Work" --color orange
```

This renames `work` → `muster_work`, applies the theme, and attaches. The session keeps running.

To also save a profile so the session can be recreated:

```bash
muster adopt work --name "Work" --color orange --save
```

`--save` snapshots the tabs into a profile and pins all windows in one step.

---

## Formalizing an Ephemeral Muster Session

You have a muster-managed session (already has `muster_` prefix) but no saved profile — tabs show red `●`.

```bash
muster list
# Sessions:
#   ● muster_232 — 232 (2 windows, 2 unpinned)
```

Save it:

```bash
muster profile save 232 --from-session 232
```

This saves the profile and pins the live windows. The red dots clear immediately. Now `muster up 232` can recreate the session if it ever dies.

---

## Editing a Profile and Bouncing

Make changes to a saved profile and apply them to a fresh session.

```bash
muster profile edit myproject   # opens in $EDITOR as TOML
```

Or update inline:

```bash
muster profile update myproject --color teal
muster profile add-tab myproject --name Logs --cwd ~/work/myproject/logs
```

To pick up changes in a running session, bounce it:

```bash
muster down myproject
muster up myproject
```

---

## Releasing a Session

Remove muster management while keeping the session alive. Useful if you want to hand off to plain tmux or clean up muster's theming.

```bash
muster release myproject
# Released: muster_myproject → myproject
```

The session keeps running as plain tmux with no muster theme, hooks, or metadata. The profile is kept — you can `muster up myproject` later to create a fresh managed session.

---

## Lifecycle Summary

```
plain tmux session ──── muster adopt ────► muster session
                                                │
                                         muster release
                                                │
                                                ▼
                                        plain tmux session

muster profile save ──────────────────► profile (file only)
                                                │
                                           muster up
                                                │
                                                ▼
                                         muster session ── muster down ──► dead
```

| Goal | Command |
|---|---|
| Create profile | `muster profile save <name> --tab ...` |
| Start / reattach | `muster up <name>` |
| Tear down | `muster down <name>` |
| Ad-hoc session | `muster new <name>` |
| Save running session as profile | `muster profile save <name> --from-session <session>` |
| Adopt plain tmux session | `muster adopt <session> --name <name>` |
| Release from muster | `muster release <session>` |
| Edit profile | `muster profile edit <name>` |
