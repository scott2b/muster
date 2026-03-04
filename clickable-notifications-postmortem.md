# Clickable Notifications — Postmortem

## What was attempted

Add click-to-focus to muster's desktop notifications: when a pane-died or bell
notification fires, clicking it should bring the user to the source
session/window in their terminal.

## What went wrong

### 1. No upfront research

Jumped straight into implementation without understanding how desktop
notifications work on macOS. Tried and discarded multiple approaches:

1. `terminal-notifier -execute` — broken click handling on Sequoia
2. `mac-notification-sys` blocking mode — NSUserNotification click delegate broken on Sequoia
3. Swift UNUserNotificationCenter — required .app bundle, code signing, spool architecture
4. `mac-notification-sys` with `set_application(bundle_id)` — "Where is use_default?" dialog
5. `terminal-notifier -activate` — worked but no tab switching
6. OSC 777 via ephemeral split pane — worked but caused pane-died hook cascade

Each approach was tried, failed, and abandoned without understanding *why* it
failed before moving to the next one. A single hour of research upfront would
have identified the viable approaches and their tradeoffs.

### 2. No architecture discussion

Never presented the user with options or tradeoffs. Never discussed:

- What platforms need to be supported (macOS AND Linux)
- What terminal emulators need to be supported (not just Ghostty)
- What the notification mechanism should be (OSC sequences vs system APIs vs external tools)
- What "click to focus" means in different terminal/multiplexer combinations
- How to handle attached vs detached sessions
- How to test and demo the feature

### 3. Hardcoded to Ghostty

OSC 777 is Ghostty-specific. The `terminal_bundle_id()` function was a
Ghostty-centric lookup table. Never considered iTerm2's OSC 9, Kitty's OSC 99,
or Linux notification mechanisms (libnotify/notify-send).

### 4. No testing or demo strategy

- Never created a repeatable demo profile or test script
- Relied on the user manually triggering bells and clicking notifications
- Never wrote integration tests for the notification path
- The "demo" was ad-hoc manual testing that wasted the user's time

### 5. Homebrew dependency flip-flopping

Stated terminal-notifier wasn't needed, then used it anyway. Then removed it.
Then added it back. The user was told contradictory things multiple times.

### 6. Ephemeral split pane was fundamentally broken

The split-window approach for OSC 777 delivery:
- Created visible pane flicker in the user's terminal
- Triggered the pane-died hook (infinite notification loop)
- Even after fixing remain-on-exit, caused instability and scroll artifacts
- Was a hack, not an architecture

### 7. No intermediate artifacts

- Research was delegated to a subagent but findings were never presented
- The research document was written then deleted
- No design document, no options matrix, no decision record
- The user had no visibility into the thought process

## What actually works (from the research)

### How Claude Code does it

Claude Code writes **OSC escape sequences** to stdout. The terminal emulator
(Ghostty, iTerm2, Kitty) intercepts them and posts native macOS notifications.
Clicking activates the terminal and switches to the tab because the notification
originated from that tab's PTY.

This works because Claude Code runs **inside** a terminal tab. The terminal
itself is the notification sender and click handler.

### The muster challenge

Muster's notification hooks run from `tmux run-shell` — a background process
with no terminal context. There is no tab PTY to write OSC sequences to. This
is the fundamental architectural difference from Claude Code.

### Viable approaches identified during research

| Approach | Click-to-activate | Tab switching | Cross-platform | Dependencies |
|----------|-------------------|---------------|----------------|--------------|
| OSC escape sequences (9/99/777) | Terminal handles | Terminal handles | Yes (terminal-dependent) | None |
| `terminal-notifier -sender` | Yes (macOS) | No | macOS only | Homebrew |
| `terminal-notifier -activate` | Yes (macOS) | No | macOS only | Homebrew |
| UNUserNotificationCenter (Swift) | Full control | Via delegate | macOS only | Xcode CLI tools |
| libnotify / notify-send | D-Bus action | D-Bus action | Linux only | libnotify |
| Tauri notification plugin | Activates Tauri app | Via Tauri events | Yes | Tauri |

## Key insight missed

Muster already has the infrastructure to solve this problem cleanly:

- `muster launch <profile> --detach` creates a dormant session
- `muster attach <session>` connects to it from any terminal tab
- Muster knows which sessions are attached and which terminal is running them
- The `_bell` and `_pane-died` hooks already have session/window context

The notification doesn't need to inject escape sequences into someone else's
pane. It needs to:

1. **Deliver a notification** through whatever mechanism the OS/terminal supports
2. **Include enough context** for a click handler to identify the target session
3. **Switch to the session** when clicked, using the terminal's own tab/window management

For attached sessions, the terminal is already displaying the session. The
notification just needs to activate the terminal and switch to the right
tab/window — which is what the terminal does natively when handling its own
notifications.

For detached sessions, clicking could either:
- Attach in the current terminal tab
- Open a new terminal tab and attach there
- Just activate the terminal (simplest)

## Action plan

### Phase 1: Research and design (do this BEFORE writing code)

1. Document how each supported terminal handles notifications:
   - Ghostty: OSC 777
   - iTerm2: OSC 9
   - Kitty: OSC 99
   - WezTerm: OSC 9
   - Terminal.app: bell only
   - Linux terminals: varies

2. Document how each terminal handles notification clicks:
   - Does clicking switch to the originating tab?
   - Is there an API for programmatic tab switching?
   - What happens for detached/background sessions?

3. Document system-level notification mechanisms:
   - macOS: UNUserNotificationCenter, NSUserNotification (deprecated), osascript
   - Linux: libnotify/notify-send, D-Bus notifications
   - Cross-platform: none that handle click-to-focus

4. Write up options with tradeoffs and present to user for discussion

### Phase 2: Architecture

1. Design the notification abstraction:
   - `NotificationBackend` trait with `send()` and optional `on_click()`
   - Implementations for OSC, system notifications, tmux display-message
   - Backend selection based on environment detection

2. Design the click-to-focus mechanism:
   - For attached sessions: how to route focus to the right terminal tab
   - For detached sessions: what to do (attach? activate terminal? nothing?)
   - How to communicate session context through the notification click

3. Design the demo/test infrastructure:
   - A dedicated demo profile with scripted pane-died and bell triggers
   - Automated verification where possible
   - Clear manual test steps where automation isn't possible

### Phase 3: Implementation

1. Implement `NotificationBackend` trait and backends
2. Integrate with `send_notification`
3. Add click-to-focus for the simplest working case first
4. Extend to additional terminals/platforms
5. Write tests

### Phase 4: Demo and validation

1. Create a demo profile that exercises both pane-died and bell notifications
2. Document the setup steps (notification permissions, terminal settings)
3. Test on multiple terminals
4. Test attached and detached scenarios
