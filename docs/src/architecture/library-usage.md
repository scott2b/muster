# Library Usage

The `muster` library crate provides a Rust API for terminal session group management.

## Basic Usage

```rust
use muster::{Muster, Profile, TabProfile};
use std::path::Path;

let m = Muster::init(Path::new("~/.config/muster"))?;

// Create a profile
let profile = Profile {
    id: "my-project".into(),
    name: "My Project".into(),
    color: "#f97316".into(),
    tabs: vec![
        TabProfile {
            name: "Shell".into(),
            cwd: "/home/user/project".into(),
            command: None,
            layout: None,
            panes: vec![],
        },
        TabProfile {
            name: "Server".into(),
            cwd: "/home/user/project".into(),
            command: Some("npm run dev".into()),
            layout: None,
            panes: vec![],
        },
    ],
};
m.save_profile(profile.clone())?;

// Launch a session
let info = m.launch(&profile.id)?;

// List running sessions
let sessions = m.list_sessions()?;

// Subscribe to events (for GUI integration)
let rx = m.subscribe();
```

## Event Subscription

The library provides push-based event notifications via `tokio::broadcast`:

```rust
let mut rx = muster.subscribe();
tokio::spawn(async move {
    while let Ok(event) = rx.recv().await {
        match event {
            MusterEvent::TabAdded { session, window_index, name } => { /* ... */ },
            MusterEvent::TabClosed { session, window_index } => { /* ... */ },
            MusterEvent::SessionEnded { session } => { /* ... */ },
            // ...
        }
    }
});
```

## API Documentation

Full API documentation is available at the [API Reference](../api/muster/index.html), generated from rustdoc comments on all public types, functions, and modules.
