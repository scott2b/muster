# Command-Line Help for `muster`

This document contains the help content for the `muster` command-line program.

**Command Overview:**

* [`muster`↴](#muster)
* [`muster list`↴](#muster-list)
* [`muster launch`↴](#muster-launch)
* [`muster attach`↴](#muster-attach)
* [`muster kill`↴](#muster-kill)
* [`muster new`↴](#muster-new)
* [`muster color`↴](#muster-color)
* [`muster ps`↴](#muster-ps)
* [`muster ports`↴](#muster-ports)
* [`muster status`↴](#muster-status)
* [`muster peek`↴](#muster-peek)
* [`muster pin`↴](#muster-pin)
* [`muster unpin`↴](#muster-unpin)
* [`muster profile`↴](#muster-profile)
* [`muster profile list`↴](#muster-profile-list)
* [`muster profile delete`↴](#muster-profile-delete)
* [`muster profile save`↴](#muster-profile-save)
* [`muster profile add-tab`↴](#muster-profile-add-tab)
* [`muster profile show`↴](#muster-profile-show)
* [`muster profile edit`↴](#muster-profile-edit)
* [`muster profile update`↴](#muster-profile-update)
* [`muster profile remove-tab`↴](#muster-profile-remove-tab)
* [`muster notifications`↴](#muster-notifications)
* [`muster notifications setup`↴](#muster-notifications-setup)
* [`muster notifications remove`↴](#muster-notifications-remove)

## `muster`

Terminal session group management built on tmux.

Muster organizes terminal sessions into named, color-coded groups with saved profiles, runtime theming, and push-based state synchronization via tmux control mode.

**Usage:** `muster [OPTIONS] <COMMAND>`

###### **Subcommands:**

* `list` — List profiles and running sessions
* `launch` — Launch or attach to a profile's session
* `attach` — Attach to a running session
* `kill` — Destroy a session
* `new` — Create an ad-hoc session
* `color` — Change session color live
* `ps` — Show processes running inside sessions
* `ports` — Show listening ports inside sessions
* `status` — Show all sessions with details
* `peek` — Peek at recent terminal output
* `pin` — Pin the current window to the session's profile
* `unpin` — Unpin the current window from the session's profile
* `profile` — Profile management
* `notifications` — Notification management

###### **Options:**

* `--config-dir <CONFIG_DIR>` — Path to the config directory
* `--json` — Output in JSON format



## `muster list`

List profiles and running sessions

**Usage:** `muster list`



## `muster launch`

Launch or attach to a profile's session

**Usage:** `muster launch [OPTIONS] <PROFILE>`

###### **Arguments:**

* `<PROFILE>` — Profile name or ID

###### **Options:**

* `--detach` — Create session but don't attach



## `muster attach`

Attach to a running session

**Usage:** `muster attach [OPTIONS] <SESSION>`

###### **Arguments:**

* `<SESSION>` — Profile name, ID, or session name

###### **Options:**

* `--window <WINDOW>` — Window index to switch to



## `muster kill`

Destroy a session

**Usage:** `muster kill <SESSION>`

###### **Arguments:**

* `<SESSION>` — Profile name, ID, or session name



## `muster new`

Create an ad-hoc session

**Usage:** `muster new [OPTIONS] <NAME>`

###### **Arguments:**

* `<NAME>` — Display name

###### **Options:**

* `--tab <TAB>` — Tab definition (`name:cwd[:command]`), repeatable
* `--color <COLOR>` — Color (hex)

  Default value: `#808080`
* `--detach` — Create session but don't attach



## `muster color`

Change session color live

**Usage:** `muster color <SESSION> <COLOR>`

###### **Arguments:**

* `<SESSION>` — Profile name, ID, or session name
* `<COLOR>` — Hex color (e.g. #f97316)



## `muster ps`

Show processes running inside sessions

**Usage:** `muster ps [PROFILE]`

###### **Arguments:**

* `<PROFILE>` — Profile name or ID (shows all sessions if omitted)



## `muster ports`

Show listening ports inside sessions

**Usage:** `muster ports [PROFILE]`

###### **Arguments:**

* `<PROFILE>` — Profile name or ID (shows all sessions if omitted)



## `muster status`

Show all sessions with details

**Usage:** `muster status`



## `muster peek`

Peek at recent terminal output

**Usage:** `muster peek [OPTIONS] <SESSION> [WINDOWS]...`

###### **Arguments:**

* `<SESSION>` — Profile name, ID, or session name
* `<WINDOWS>` — Window names to show (all if omitted)

###### **Options:**

* `-n`, `--lines <LINES>` — Lines of output per window

  Default value: `50`



## `muster pin`

Pin the current window to the session's profile

**Usage:** `muster pin`



## `muster unpin`

Unpin the current window from the session's profile

**Usage:** `muster unpin`



## `muster profile`

Profile management

**Usage:** `muster profile <COMMAND>`

###### **Subcommands:**

* `list` — List all profiles
* `delete` — Delete a profile
* `save` — Save a new profile
* `add-tab` — Add a tab to an existing profile
* `show` — Show a profile's full definition
* `edit` — Edit a profile in $EDITOR
* `update` — Update profile fields inline
* `remove-tab` — Remove a tab from a profile



## `muster profile list`

List all profiles

**Usage:** `muster profile list`



## `muster profile delete`

Delete a profile

**Usage:** `muster profile delete <ID>`

###### **Arguments:**

* `<ID>` — Profile name or ID



## `muster profile save`

Save a new profile

**Usage:** `muster profile save [OPTIONS] <NAME>`

###### **Arguments:**

* `<NAME>` — Profile name

###### **Options:**

* `--tab <TAB>` — Tab definition (`name:cwd[:command]`), repeatable
* `--color <COLOR>` — Color (hex)

  Default value: `#808080`



## `muster profile add-tab`

Add a tab to an existing profile

**Usage:** `muster profile add-tab [OPTIONS] --name <NAME> --cwd <CWD> <PROFILE>`

###### **Arguments:**

* `<PROFILE>` — Profile name or ID

###### **Options:**

* `--name <NAME>` — Tab name
* `--cwd <CWD>` — Working directory
* `--command <COMMAND>` — Startup command



## `muster profile show`

Show a profile's full definition

**Usage:** `muster profile show <ID>`

###### **Arguments:**

* `<ID>` — Profile name or ID



## `muster profile edit`

Edit a profile in $EDITOR

**Usage:** `muster profile edit <ID>`

###### **Arguments:**

* `<ID>` — Profile name or ID



## `muster profile update`

Update profile fields inline

**Usage:** `muster profile update [OPTIONS] <ID>`

###### **Arguments:**

* `<ID>` — Profile name or ID

###### **Options:**

* `--name <NAME>` — New display name
* `--color <COLOR>` — New color (hex or named)



## `muster profile remove-tab`

Remove a tab from a profile

**Usage:** `muster profile remove-tab <PROFILE> <TAB>`

###### **Arguments:**

* `<PROFILE>` — Profile name or ID
* `<TAB>` — Tab name or 0-based index



## `muster notifications`

Notification management

**Usage:** `muster notifications <COMMAND>`

###### **Subcommands:**

* `setup` — Install macOS notification app bundle
* `remove` — Remove macOS notification app bundle



## `muster notifications setup`

Install macOS notification app bundle

**Usage:** `muster notifications setup`



## `muster notifications remove`

Remove macOS notification app bundle

**Usage:** `muster notifications remove`



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>
