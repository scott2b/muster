# Terminal Notification Protocols & Click-to-Focus Research

Researched 2026-03-04. Verified findings from documentation, GitHub issues, and specifications.

---

## 1. Terminal OSC Notification Protocols

### Summary Table

| Terminal | OSC 9 | OSC 777 | OSC 99 | Click-to-focus | tmux passthrough |
|---|---|---|---|---|---|
| Ghostty | Yes | Yes | No (planned 1.3) | Yes (fixed in 1.3) | Yes, with allow-passthrough |
| iTerm2 | Yes | No | No | Opens iTerm2 | Yes, with DCS wrapping |
| Kitty | No (has OSC 99) | No | Yes | Yes (native) | Yes, with allow-passthrough |
| WezTerm | Yes | Yes | No | No (not documented) | Yes, with allow-passthrough |
| Alacritty | No | No | No | N/A | N/A |
| Terminal.app | No | No | No | N/A | N/A |
| GNOME Terminal (VTE) | No | Yes (Fedora patches) | No | No (highlights tab) | Yes, with allow-passthrough |
| Konsole | No | Yes (MR #761, status unclear) | No | Unknown | Unknown |
| foot | No | Yes | No | No (inhibits when focused) | Yes, with allow-passthrough |

### Per-Terminal Details

#### Ghostty

- **Supported:** OSC 9, OSC 777
- **OSC 9 format:** `\033]9;body\007`
- **OSC 777 format:** `\033]777;notify;title;body\007`
- **OSC 99:** Not yet supported. Tracked in issue #5634, assigned to milestone 1.3.0.
- **Click behavior:** As of PR #9146 (merged Oct 2025, milestone 1.3.0), clicking a notification focuses the originating Ghostty window. The fix corrected surface targeting and added `present()` call on the window.
- **Configuration:** `desktop-notifications` config option controls whether notifications are shown. Ghostty also has `notify-on-command-finish` for shell integration.
- **tmux:** Works with `allow-passthrough on`.
- **Platform note:** On Linux, Ghostty must be launched from a `.desktop` file for notification permissions.

Sources: [Ghostty VT Reference](https://ghostty.org/docs/vt/reference), [Discussion #3555](https://github.com/ghostty-org/ghostty/discussions/3555), [Issue #9145](https://github.com/ghostty-org/ghostty/issues/9145), [Discussion #4405](https://github.com/ghostty-org/ghostty/discussions/4405)

#### iTerm2

- **Supported:** OSC 9
- **Format:** `\033]9;message\007`
- **History:** Originally posted Growl notifications; now uses macOS native notifications.
- **Click behavior:** Clicking the notification activates iTerm2 (brings it to front). Does NOT focus the specific tab/session that originated the notification.
- **No OSC 777 or OSC 99 support.**
- **tmux:** Works with DCS passthrough wrapping: `\033Ptmux;\033\033]9;message\007\033\\`
- **Scripting:** iTerm2 has extensive AppleScript and Python API for tab/session activation. Can `tell application "iTerm2" tell tab N of window M` etc.

Sources: [iTerm2 Escape Codes](https://iterm2.com/documentation-escape-codes.html), [iTerm2 Scripting](https://iterm2.com/documentation-scripting.html)

#### Kitty

- **Supported:** OSC 99 (its own protocol, the most capable)
- **Format:** `\033]99;metadata;payload\033\\`
- **Metadata format:** Colon-separated `key=value` pairs. Keys are single characters.
- **Key metadata fields:**
  - `p=title|body|close|icon|buttons|alive|?` - payload type
  - `d=0|1` - done flag (1 = display now)
  - `i=identifier` - globally unique notification ID
  - `a=focus|report|-focus` - action on click. Default is `focus` (activates originating window). `report` sends `\033]99;i=<id>;\033\\` back to the application.
  - `o=always|unfocused|invisible` - when to show
  - `u=0|1|2` - urgency (low/normal/critical)
  - `c=1` - request close notification callback
  - `w=milliseconds` - auto-expiration (-1=OS default, 0=never)
- **Click behavior:** With default `a=focus`, clicking focuses the originating kitty window. With `a=report`, sends escape sequence back to the application. With `a=report:focus`, does both.
- **Close events:** With `c=1`, terminal sends `\033]99;i=<id>:p=close;\033\\` when notification is dismissed.
- **Query support:** Send `\033]99;i=<id>:p=?;\033\\` to query terminal capabilities.
- **Button support:** Can define custom buttons; clicking sends `\033]99;i=<id>;button_number\033\\`.
- **tmux:** Works with `allow-passthrough on`. The `i=` identifier helps multiplex responses to correct panes.
- **Shell helper:** `kitten notify` command wraps the protocol.

Sources: [Kitty Desktop Notifications Spec](https://sw.kovidgoyal.net/kitty/desktop-notifications/), [Kitty Notify Kitten](https://sw.kovidgoyal.net/kitty/kittens/notify/)

#### WezTerm

- **Supported:** OSC 9, OSC 777
- **OSC 9 format:** `\033]9;message\033\\`
- **OSC 777 format:** `\033]777;notify;title;body\033\\`
- **Configuration:** `notification_handling` option with values:
  - `AlwaysShow` (default)
  - `NeverShow`
  - `SuppressFromFocusedPane`
  - `SuppressFromFocusedTab`
  - `SuppressFromFocusedWindow`
- **Click behavior:** No documented click-to-focus. The configuration only controls whether notifications appear, not interaction behavior.
- **tmux:** Works with `allow-passthrough on`.

Sources: [WezTerm notification_handling](https://wezterm.org/config/lua/config/notification_handling.html), [WezTerm Escape Sequences](https://wezterm.org/escape-sequences.html)

#### Alacritty

- **Supported:** NONE. No OSC 9, 777, or 99.
- **Status:** Open issue [#7105](https://github.com/alacritty/alacritty/issues/7105). Maintainer @chrisduerr is opposed ("I really don't like the idea of Alacritty messing with notifications"). Maintainer @kchibisov suggested delegating to user-configurable script (similar to bell handling). Issue remains open with no implementation plan.
- **Workaround:** The configurable bell feature (`program` config) can run an external command when BEL is received. This is the only notification path.

Sources: [Alacritty Issue #7105](https://github.com/alacritty/alacritty/issues/7105)

#### Terminal.app (macOS)

- **Supported:** NONE. No OSC notification sequences.
- **Capabilities:** Supports OSC 7 (CWD), basic VT/ANSI. No proprietary notification extensions.
- **Workaround:** None via escape sequences. Must use external tools (terminal-notifier, osascript).

#### GNOME Terminal (VTE-based)

- **Supported:** OSC 777 (with Fedora VTE patches)
- **Format:** `\033]777;notify;title;body\033\\` or `\033]777;precmd\007`
- **History:** VTE added OSC 777 support via Fedora patches. The `precmd` sub-command notifies when a prompt is about to be shown (command completed). GNOME Terminal 3.16+ uses this to post desktop notifications when a command completes in an unfocused tab.
- **Availability:** Requires Fedora's VTE patches. Not in upstream VTE by default in all distros.
- **Click behavior:** The notification highlights the tab in the status bar. Clicking the notification brings GNOME Terminal to front but does NOT switch to the specific tab.
- **tmux:** Works with `allow-passthrough on`.
- **Also applies to:** Tilix, and other VTE-based terminals.

Sources: [Fedora Magazine](https://fedoramagazine.org/terminal-job-notifications-in-fedora-22-workstation/), [VTE commit](https://mail.gnome.org/archives/commits-list/2016-May/msg01684.html)

#### Konsole (KDE)

- **Supported:** OSC 777 (merged via MR #761, Nov 2022)
- **Format:** `\033]777;notify;title;body\033\\`
- **Status:** Merge request was filed by Christian Muehlhaeuser. The exact merge status and release version are unclear from available sources, but the MR exists and was marked for review.
- **Existing features:** Konsole has built-in "Monitor for Activity/Silence" which triggers KDE desktop notifications via KDE's Plasma notification system.
- **Click behavior:** Unknown/undocumented for OSC 777.

Sources: [Konsole MR #761](https://invent.kde.org/utilities/konsole/-/merge_requests/761)

#### foot

- **Supported:** OSC 777 (only the `notify` sub-command)
- **Format:** `\033]777;notify;title;body\033\\`
- **Configuration:** `notify` option in `foot.ini` specifies the command to execute when a notification is received. Uses `${title}` and `${body}` template variables. Example: `notify=notify-send -a ${app-id} -i ${app-id} ${title} ${body}`
- **Behavior:** Notifications are inhibited when the foot window is focused (spam prevention).
- **Click behavior:** Delegated to whatever notification daemon handles the command (e.g., dunst, mako). foot itself does not manage click-to-focus.
- **tmux:** Works with `allow-passthrough on`.

Sources: [foot Issue #224](https://codeberg.org/dnkl/foot/issues/224), [foot PR #236](https://codeberg.org/dnkl/foot/pulls/236)

---

## 2. tmux Passthrough

### allow-passthrough (tmux 3.2+)

**Configuration:**
```
set -g allow-passthrough on    # tmux 3.3+
set -g allow-passthrough all   # alternative value
```

**How it works:**
- When enabled, applications in panes can send escape sequences that tmux passes through to the outer terminal instead of interpreting them.
- The application must wrap sequences in DCS passthrough format.
- tmux itself is NOT aware of state changes made by passthrough sequences and may undo them.

### DCS Passthrough Format

```
\033Ptmux;\033<escaped-sequence>\033\\
```

Rules:
- Prefix with `\033Ptmux;\033`
- Double any `\033` characters in the inner sequence
- Terminate with `\033\\`

**Example - OSC 9 through tmux:**
```
# Plain:
printf '\033]9;message\007'

# DCS-wrapped for tmux:
printf '\033Ptmux;\033\033]9;message\007\033\\'
```

**Example - OSC 777 through tmux:**
```
printf '\033Ptmux;\033\033]777;notify;title;body\007\033\\'
```

### run-shell Output: Does NOT Reach the Terminal

**Critical finding:** `tmux run-shell` output does NOT go to any pane's terminal stream. It is displayed as a message on the tmux status line (similar to `display-message`). Therefore:

- OSC escape sequences in `run-shell` output will NOT trigger terminal notifications.
- They are consumed by tmux's status line renderer, which strips/ignores escape sequences.

**Workaround (confirmed by tmux maintainer Nicholas Marriott in [Issue #2136](https://github.com/tmux/tmux/issues/2136)):**

Write directly to the client TTY:
```bash
printf '\033]9;message\007' > $(tmux display-message -p '#{client_tty}')
```

This bypasses tmux entirely and writes the escape sequence directly to the terminal device file (e.g., `/dev/ttys003`). The terminal emulator receives it as if it came from a pane.

**Caveat:** This only works when there IS an attached client. If the session is detached, `#{client_tty}` is empty.

### Unfocused Pane Limitation

Escape sequences printed by a process in an unfocused (non-visible) pane do NOT reach the terminal. tmux only forwards output from the currently visible pane of the attached client's active window.

**This means:** A background hook process cannot send OSC notifications by printing to stdout. It must either:
1. Write directly to `#{client_tty}` (bypasses tmux)
2. Use `pipe-pane -I` to inject into the active pane's output stream

### pipe-pane -I

`pipe-pane -I` connects a command's stdout to a pane's input (as if typed). With `-O`, it connects pane output to a command's stdin. The `-I` flag could theoretically inject escape sequences into a pane, but this is equivalent to typing the escape sequence into the pane's shell, which is NOT the same as the terminal receiving it as output.

**Conclusion:** `pipe-pane -I` is NOT suitable for injecting OSC sequences to the terminal. It sends to the pane's input (stdin), not to the terminal's display stream.

### display-popup

`tmux display-popup -E` runs a command in a popup pane. The popup IS a real pane, so output from the command running inside it DOES reach the terminal (as long as the popup is visible). However:
- The popup pane is ephemeral (closes when the command exits).
- The popup pane IS the active pane while open, so its output reaches the terminal.
- You could theoretically `display-popup -E 'printf "\033]9;done\007"'` and it would work, but it would also flash a popup briefly.

### Summary: Viable Approaches from tmux Hooks

| Approach | Works? | Notes |
|---|---|---|
| `run-shell` printing OSC | NO | Output goes to status line, not terminal |
| `run-shell` writing to `#{client_tty}` | YES | Direct write to terminal device file |
| `send-keys` with OSC | NO | Goes to pane stdin, not terminal output |
| `pipe-pane -I` with OSC | NO | Goes to pane stdin |
| `display-popup -E` with OSC | YES | But flashes a popup briefly |
| Process in active pane printing OSC | YES | Standard passthrough path |

---

## 3. System-Level Notification Mechanisms

### macOS

#### terminal-notifier

- **Repo:** [julienXX/terminal-notifier](https://github.com/julienXX/terminal-notifier)
- **API:** Uses Apple's `UNUserNotificationCenter` (previously `NSUserNotification`, deprecated)
- **Install:** `brew install terminal-notifier`

**Key options:**
| Option | Effect |
|---|---|
| `-title "Title"` | Notification title |
| `-message "Body"` | Notification body |
| `-activate com.bundle.id` | Activate specified app on click (sender remains terminal-notifier) |
| `-sender com.bundle.id` | Fake the sender app (uses its icon, launches it on click) |
| `-execute "command"` | Run shell command on click |
| `-open "URL"` | Open URL on click (web, file, or custom scheme) |
| `-appIcon path` | Custom icon (private API, subject to breakage) |

**Critical constraint:** `-sender` CANNOT be combined with `-execute` or `-activate`. When using `-sender`, the specified app launches on click, but you lose the ability to run custom commands.

**macOS Sequoia status:** BROKEN for many users. Issue [#312](https://github.com/julienXX/terminal-notifier/issues/312) reports:
- No notifications appear on M4/Sequoia 15.3.1
- No error output
- Workaround: Ensure "Allow Notifications" is enabled in System Settings > Notifications > terminal-notifier
- Workaround: Disable Focus modes (especially "Sleep")
- Workaround: Rebuild from source with updated deployment target in Xcode
- Affects M1, M2, M4 across Sequoia 15.0.1 through 15.5

#### alerter

- **Repo:** [vjeantet/alerter](https://github.com/vjeantet/alerter)
- **Version:** 26.5 (released Feb 19, 2026) - completely rewritten in Swift
- **Requires:** macOS 13.0+
- **Install:** `brew install vjeantet/tap/alerter`

**Key features:**
- Displays notifications with custom action buttons
- Captures user interaction as stdout output:
  - `@TIMEOUT` - auto-closed
  - `@CLOSED` - user clicked close
  - `@CONTENTCLICKED` - user clicked notification body
  - `@ACTIONCLICKED` - user clicked action button
- Supports `--json` flag for structured output
- CLI uses double-dash syntax: `--message`, `--title`, `--json`

**Advantage over terminal-notifier:** alerter gives you synchronous feedback about which action the user took. You can script click responses:
```bash
result=$(alerter --message "Task done" --title "muster" --actions "Go to session")
if [ "$result" = "@ACTIONCLICKED" ]; then
    # activate the terminal and switch to the session
fi
```

**Sequoia compatibility:** Not explicitly documented, but requires macOS 13+ and was updated Feb 2026, suggesting active maintenance.

#### osascript display notification

```bash
osascript -e 'display notification "body" with title "title"'
```

- **Click behavior:** Clicking the notification opens the sending application (Script Editor or the `.app` that ran the script). There is NO way to specify a custom click action with vanilla AppleScript.
- **Limitation:** `display notification` does not support actions, buttons, or callbacks.
- **Sequoia issue:** Reports that `osascript` notifications from Terminal.app may not work on Sequoia.
- **Alternative:** AppleScript-ObjC can use `UNUserNotificationCenter` for more control, but requires building an actual application bundle.

### Linux

#### notify-send (libnotify)

```bash
notify-send "Title" "Body"
```

- **API:** Sends D-Bus messages to `org.freedesktop.Notifications`
- **Actions (libnotify 0.8+):**
  ```bash
  notify-send "Title" "Body" --action="default=Click here" --action="dismiss=Dismiss"
  ```
  The `--action` flag implies `--wait` (process blocks until user interacts). The action NAME is printed to stdout when clicked.
- **Default action:** The action key `"default"` represents clicking the notification body itself (no visible button).

**Example with click handling:**
```bash
ACTION=$(notify-send "Task Complete" "Session: myproject" \
    --action="activate=Go to session" \
    --action="default=Open")
case "$ACTION" in
    activate) swaymsg '[app_id="foot"] focus' ;;
    default) tmux attach -t myproject ;;
esac
```

#### D-Bus Notification Protocol (org.freedesktop.Notifications)

**Notify method signature:**
```
Notify(app_name: STRING, replaces_id: UINT32, app_icon: STRING,
       summary: STRING, body: STRING, actions: ARRAY[STRING],
       hints: DICT, expire_timeout: INT32) -> UINT32
```

**Actions format:** Array of string pairs: `["action_key", "Label", "action_key2", "Label2"]`

**Signals:**
- `ActionInvoked(id: UINT32, action_key: STRING)` - emitted when user clicks action
- `NotificationClosed(id: UINT32, reason: UINT32)` - emitted when notification closes
  - reason: 1=expired, 2=dismissed, 3=closed programmatically, 4=undefined

**Using gdbus directly:**
```bash
gdbus call --session --dest org.freedesktop.Notifications \
    --object-path /org/freedesktop/Notifications \
    --method org.freedesktop.Notifications.Notify \
    "muster" 0 "" "Task Complete" "Session ready" \
    "['default', 'Open', 'activate', 'Go to session']" '{}' 5000
```

#### Notification Daemons

**dunst:**
- Implements full freedesktop spec including actions
- Click behavior configurable: `mouse_left_click = do_action`, `close_current`, `close_all`, `none`
- Default: left click = close, middle click = invoke default action
- Context keybinding shows action menu (requires dmenu)
- `dunstctl action` invokes default action programmatically

**mako (Wayland):**
- Implements freedesktop spec
- Right-click shows action list via wofi/dmenu
- Default click invokes default action
- Lightweight, designed for wlroots compositors

**swaync, fnott, etc.** - all implement the same freedesktop spec with varying levels of action support.

---

## 4. How Other Tools Handle Notifications

### Zellij

- **No desktop notification API.** The Zellij plugin API (WASM-based) has NO `send_notification` or similar command.
- **No OSC notification passthrough.** Zellij does not forward OSC 9/99/777 from panes to the terminal.
- **Workaround:** The [zellij-attention](https://github.com/KiryuuLight/zellij-attention) plugin adds visual notification icons to tab names (in-band notification within the Zellij UI).
- **Feature request:** Issue [#350](https://github.com/zellij-org/zellij/issues/350) requests a notification system for errors, but it was about internal UI notifications, not desktop notifications.

### tmux

- **Bell forwarding:** tmux supports `monitor-bell on` and `bell-action any|other|current|none`. When a pane sends BEL (`\007`), tmux can:
  - Highlight the window in the status bar
  - Send a bell to the terminal (`visual-bell off`)
  - Show a message (`visual-bell on`)
- **Activity monitoring:** `monitor-activity on` watches for output in background windows. `activity-action` controls the response.
- **Silence monitoring:** `monitor-silence N` triggers after N seconds of no output.
- **display-message:** Shows text in the status line. Not a desktop notification.
- **No desktop notification support.** tmux has no built-in mechanism to trigger OS-level notifications. It can only signal through the bell character or status bar highlighting.

### noti

- **Repo:** [variadico/noti](https://github.com/variadico/noti)
- **Latest:** v3.8.0 (March 2025), written in Go
- **How it works:** Wraps a command and triggers a notification when it exits.
  ```bash
  noti make build         # notify when make finishes
  noti --pwatch --pid 123 # notify when PID exits
  ```
- **macOS mechanism:** Uses `NSUserNotification` / banner notifications (labeled "nsuser" in config)
- **Linux mechanism:** Uses freedesktop D-Bus notifications
- **Click actions:** NOT supported. noti only sends fire-and-forget notifications.
- **Remote services:** Also supports Slack, Telegram, Pushover, etc.

### tnotify (soloterm)

- **Repo:** [soloterm/tnotify](https://github.com/soloterm/tnotify)
- **Written in Go**
- **Detection strategy:** Reads `$TERM_PROGRAM` and other environment variables to detect terminal
- **Protocol selection:**
  - Kitty -> OSC 99
  - iTerm2 -> OSC 9
  - WezTerm, Ghostty, VTE, foot -> OSC 777
  - Windows Terminal -> OSC 9
  - Unknown -> native fallback (osascript/notify-send/PowerShell)
- **tmux handling:** Detects `$TMUX` environment variable, wraps sequences in DCS passthrough automatically. Requires tmux 3.2+ with `allow-passthrough on`.
- **Fallback:** When OSC detection fails, uses platform-native tools.

---

## 5. Click-to-Focus Challenge

### The Problem Restated

muster's tmux hooks run from `tmux run-shell`. This context has:
- No terminal (stdout goes to tmux status line)
- No window/GUI context
- No knowledge of which terminal emulator is running
- No guarantee the session is even attached

When a notification is clicked, we need to:
1. Activate the terminal emulator application
2. Switch to the correct tab/window showing the tmux session
3. If detached, attach the session

### Approach A: System-Level Notifications with Click Actions

**macOS with alerter:**
```bash
# From tmux hook (run-shell):
CLIENT_TTY=$(tmux display-message -p '#{client_tty}')
if [ -n "$CLIENT_TTY" ]; then
    # Session is attached, identify the terminal
    TERM_PID=$(tmux display-message -p '#{client_pid}')
    TERM_BUNDLE=$(ps -p $TERM_PID -o comm= | # derive bundle ID)

    result=$(alerter --message "Task complete in session: foo" \
        --title "muster" --actions "Go to session")
    if [ "$result" = "@ACTIONCLICKED" ]; then
        # Activate the terminal emulator
        osascript -e "tell application id \"$TERM_BUNDLE\" to activate"
    fi
fi
```

**macOS with terminal-notifier:**
```bash
# Use -activate with the terminal's bundle ID
BUNDLE_ID="com.mitchellh.ghostty"  # or detect dynamically
terminal-notifier -title "muster" -message "Task complete" \
    -activate "$BUNDLE_ID"
```

**Linux with notify-send (libnotify 0.8+):**
```bash
ACTION=$(notify-send "muster" "Task complete" \
    --action="activate=Go to session")
if [ "$ACTION" = "activate" ]; then
    # X11:
    xdotool search --name "tmux" windowactivate
    # Wayland/Sway:
    swaymsg '[app_id="foot"] focus'
    # Wayland/Hyprland:
    hyprctl dispatch focuswindow "class:foot"
fi
```

### Approach B: OSC Notification via client_tty

Write an OSC notification escape sequence directly to the terminal:
```bash
CLIENT_TTY=$(tmux display-message -p '#{client_tty}')
if [ -n "$CLIENT_TTY" ]; then
    # For Kitty (OSC 99 with focus action):
    printf '\033]99;i=muster-task-1:a=focus:d=1;Task complete\033\\' > "$CLIENT_TTY"

    # For Ghostty/WezTerm/VTE (OSC 777):
    printf '\033]777;notify;muster;Task complete\007' > "$CLIENT_TTY"

    # For iTerm2 (OSC 9):
    printf '\033]9;Task complete\007' > "$CLIENT_TTY"
fi
```

**Advantage:** Terminal-native notification. For Kitty, click-to-focus is built in (`a=focus`). For Ghostty 1.3+, click-to-focus works.

**Disadvantage:** Must detect which terminal is running to choose the right OSC code.

### Approach C: Hybrid (Best of Both)

1. Detect the terminal emulator from the tmux client
2. If the terminal supports OSC notifications with click-to-focus (Kitty, Ghostty 1.3+), write OSC to `#{client_tty}`
3. Otherwise, use system-level notifications (alerter/terminal-notifier on macOS, notify-send on Linux) with the appropriate click action to activate the terminal

### Detecting the Terminal Emulator from tmux

```bash
# Method 1: Check TERM_PROGRAM from the client environment
TERM_PROG=$(tmux show-environment -g TERM_PROGRAM 2>/dev/null | cut -d= -f2)

# Method 2: Check the client's parent process
CLIENT_PID=$(tmux display-message -p '#{client_pid}')
PARENT_PID=$(ps -p "$CLIENT_PID" -o ppid= | tr -d ' ')
PARENT_CMD=$(ps -p "$PARENT_PID" -o comm=)

# Method 3: Check TERM_PROGRAM in the session/global environment
tmux show-environment TERM_PROGRAM 2>/dev/null
```

### Tab Switching Problem

Even if we activate the terminal emulator, we still need to switch to the correct tab. This is the hardest part:

**Terminals with scriptable tab switching:**
- **iTerm2:** AppleScript/Python API can select tabs: `tell application "iTerm2" to tell tab N of window M`
- **Kitty:** `kitten @ focus-tab --match` and `kitten @ focus-window`
- **WezTerm:** Has `wezterm cli activate-tab`
- **Ghostty:** No external tab control API (as of 1.2)

**Terminals without scriptable tab switching:**
- Alacritty, Terminal.app, GNOME Terminal, Konsole, foot - no external API for tab switching

**Nuclear option:** For Kitty's OSC 99 with `a=focus`, the terminal itself handles focusing the correct window/tab that originated the notification. This is the only protocol that solves the tab-switching problem natively.

### Handling Detached Sessions

If the tmux session is detached (`#{client_tty}` is empty):
- No client TTY to write OSC to
- System notification is the only option
- Click action must either:
  - Open a new terminal window and `tmux attach -t session`
  - Use `open -a Ghostty` / `open -a iTerm` and then attach

**macOS approach:**
```bash
terminal-notifier -title "muster" -message "Task complete" \
    -execute "tmux attach -t sessionname"  # runs in sh, not in a terminal!
```
The `-execute` option runs the command in a shell, but NOT in a terminal window. This is useless for `tmux attach`.

Better:
```bash
terminal-notifier -title "muster" -message "Task complete" \
    -open "muster://attach/sessionname"  # custom URL scheme handled by a helper app
```

Or:
```bash
# Use -activate to bring terminal to front, then use tmux's select-window
terminal-notifier -title "muster" -message "Task complete" \
    -activate "com.mitchellh.ghostty" \
    -execute "tmux select-window -t sessionname:0"
```
Note: `-activate` and `-execute` CAN be combined (unlike `-sender`).

### Window Activation on Linux (X11 vs Wayland)

**X11:**
```bash
# Find window by class and activate
WID=$(xdotool search --class "ghostty" | head -1)
xdotool windowactivate "$WID"

# Or with wmctrl:
wmctrl -a "ghostty"
```

**Wayland:** No universal protocol. Each compositor has its own mechanism:
- **Sway:** `swaymsg '[app_id="ghostty"] focus'`
- **Hyprland:** `hyprctl dispatch focuswindow "class:ghostty"`
- **GNOME/Mutter:** No external window activation API (by design, for security)
- **KDE/KWin:** `kdotool` or D-Bus: `qdbus org.kde.KWin /KWin org.kde.KWin.activateWindow`
- **wlroots-based:** `wlrctl window focus` (requires wlr-foreign-toplevel-management-v1 protocol)

**Key limitation:** On Wayland, there is NO universal way to activate a window from outside. The notification daemon itself can request activation (via XDG activation tokens on wlroots), but a script running in a tmux hook cannot.

### Recommended Architecture for muster

Given all the above constraints, the most viable architecture is:

1. **Notification dispatch from tmux hook:**
   ```
   tmux run-shell 'muster-notify "Task complete" "session:myproject"'
   ```

2. **`muster-notify` binary** (Rust or Go) that:
   - Detects whether the session is attached (checks `#{client_tty}`)
   - If attached, detects the terminal emulator (`$TERM_PROGRAM`)
   - Chooses the best notification path:
     - Kitty: OSC 99 via `client_tty` (native click-to-focus)
     - Ghostty 1.3+: OSC 777 via `client_tty` (native click-to-focus)
     - Others with attached client: System notification with `-activate $BUNDLE_ID`
     - Detached session: System notification with click action to open terminal + attach

3. **On macOS:** Use `alerter` (or `terminal-notifier` if alerter unavailable) for system notifications. alerter's synchronous output model (`@CONTENTCLICKED`, etc.) enables scripted responses.

4. **On Linux:** Use `notify-send --action` (requires libnotify 0.8+) or direct D-Bus calls. Handle the `ActionInvoked` signal to trigger window activation via compositor-specific commands.

5. **Terminal bundle ID detection on macOS:**
   ```bash
   # From the client PID, walk up to find the .app bundle
   CLIENT_PID=$(tmux display-message -p '#{client_pid}')
   APP_PATH=$(ps -p $CLIENT_PID -o comm=)
   # Map known paths to bundle IDs
   ```

6. **Fall back gracefully:** If detection fails, send a basic system notification without click-to-focus. The user can manually switch to the terminal.
