use std::path::PathBuf;
use std::process;

use clap::{Parser, Subcommand};
use muster::Muster;

#[derive(Parser)]
#[command(name = "muster", version, about = "Terminal session group management")]
struct Cli {
    /// Path to the config directory
    #[arg(long, env = "MUSTER_CONFIG_DIR")]
    config_dir: Option<PathBuf>,

    /// Output in JSON format
    #[arg(long, global = true)]
    json: bool,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// List profiles and running sessions
    List,

    /// Launch or attach to a profile's session
    Launch {
        /// Profile name or ID
        profile: String,
    },

    /// Attach to a running session
    Attach {
        /// Session name
        session: String,
        /// Window index to switch to
        #[arg(long)]
        window: Option<u32>,
    },

    /// Destroy a session
    Kill {
        /// Session name
        session: String,
    },

    /// Create an ad-hoc session
    New {
        /// Display name
        name: String,
        /// Working directory
        #[arg(long, default_value = ".")]
        cwd: String,
        /// Color (hex)
        #[arg(long, default_value = "#808080")]
        color: String,
    },

    /// Change session color live
    Color {
        /// Session name
        session: String,
        /// Hex color (e.g. #f97316)
        color: String,
    },

    /// Show all sessions with details
    Status,

    /// Profile management
    Profile {
        #[command(subcommand)]
        action: ProfileAction,
    },
}

#[derive(Subcommand)]
enum ProfileAction {
    /// List all profiles
    List,

    /// Delete a profile
    Delete {
        /// Profile name or ID
        id: String,
    },

    /// Save a new profile
    Save {
        /// Profile name
        name: String,
        /// Working directory for the first tab
        #[arg(long, default_value = ".")]
        cwd: String,
        /// Color (hex)
        #[arg(long, default_value = "#808080")]
        color: String,
    },
}

fn default_config_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("~/.config"))
        .join("muster")
}

#[allow(clippy::too_many_lines)]
fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let config_dir = cli.config_dir.unwrap_or_else(default_config_dir);
    let m = Muster::init(&config_dir)?;

    match cli.command {
        Command::List => {
            let profiles = m.list_profiles()?;
            let sessions = m.list_sessions()?;

            if cli.json {
                let output = serde_json::json!({
                    "profiles": profiles,
                    "sessions": sessions,
                });
                println!("{}", serde_json::to_string_pretty(&output)?);
            } else {
                if !profiles.is_empty() {
                    println!("Profiles:");
                    for p in &profiles {
                        let active = sessions
                            .iter()
                            .any(|s| s.profile_id.as_deref() == Some(&p.id));
                        let marker = if active { " [active]" } else { "" };
                        println!("  {} ({}){}", p.name, p.id, marker);
                    }
                }
                if !sessions.is_empty() {
                    println!("\nSessions:");
                    for s in &sessions {
                        println!(
                            "  {} — {} ({} windows){}",
                            s.session_name,
                            s.display_name,
                            s.window_count,
                            if s.attached { " [attached]" } else { "" }
                        );
                    }
                }
                if profiles.is_empty() && sessions.is_empty() {
                    println!("No profiles or sessions.");
                }
            }
        }

        Command::Launch { profile } => {
            // Try to find profile by name first, then by ID
            let profiles = m.list_profiles()?;
            let found = profiles
                .iter()
                .find(|p| p.name == profile || p.id == profile);

            let Some(p) = found else {
                eprintln!("Profile not found: {profile}");
                process::exit(1);
            };
            let profile_id = p.id.clone();

            let info = m.launch(&profile_id)?;
            if cli.json {
                println!("{}", serde_json::to_string_pretty(&info)?);
            } else {
                println!("Launched: {} ({})", info.display_name, info.session_name);
            }
        }

        Command::Attach { session, window } => {
            if let Some(idx) = window {
                m.switch_window(&session, idx)?;
            }
            // In a real terminal, we'd exec tmux attach. For now, print the command.
            println!("tmux attach -t {session}");
        }

        Command::Kill { session } => {
            m.destroy(&session)?;
            if !cli.json {
                println!("Destroyed: {session}");
            }
        }

        Command::New { name, cwd, color } => {
            let cwd = if cwd == "." {
                std::env::current_dir()?.to_string_lossy().to_string()
            } else {
                cwd
            };

            let profile = muster::Profile {
                id: format!("adhoc_{}", uuid::Uuid::new_v4()),
                name: name.clone(),
                color,
                tabs: vec![muster::TabProfile {
                    name: "Shell".to_string(),
                    cwd,
                    command: None,
                }],
            };

            m.save_profile(profile.clone())?;
            let info = m.launch(&profile.id)?;
            if cli.json {
                println!("{}", serde_json::to_string_pretty(&info)?);
            } else {
                println!("Created: {} ({})", info.display_name, info.session_name);
            }
        }

        Command::Color { session, color } => {
            m.set_color(&session, &color)?;
            if !cli.json {
                println!("Color updated: {session} → {color}");
            }
        }

        Command::Status => {
            let sessions = m.list_sessions()?;
            if cli.json {
                println!("{}", serde_json::to_string_pretty(&sessions)?);
            } else if sessions.is_empty() {
                println!("No active sessions.");
            } else {
                for s in &sessions {
                    println!(
                        "{} — {} [{} windows] {}",
                        s.session_name, s.display_name, s.window_count, s.color
                    );
                    // Get detailed window info
                    if let Ok(windows) = m.client().list_windows(&s.session_name) {
                        for w in &windows {
                            let marker = if w.active { "→" } else { " " };
                            println!("  {marker} {}: {} ({})", w.index, w.name, w.cwd);
                        }
                    }
                }
            }
        }

        Command::Profile { action } => match action {
            ProfileAction::List => {
                let profiles = m.list_profiles()?;
                if cli.json {
                    println!("{}", serde_json::to_string_pretty(&profiles)?);
                } else if profiles.is_empty() {
                    println!("No profiles.");
                } else {
                    for p in &profiles {
                        println!(
                            "  {} ({}) — {} tab(s), color: {}",
                            p.name,
                            p.id,
                            p.tabs.len(),
                            p.color
                        );
                    }
                }
            }

            ProfileAction::Delete { id } => {
                // Find by name or ID
                let profiles = m.list_profiles()?;
                let found = profiles.iter().find(|p| p.name == id || p.id == id);

                if let Some(p) = found {
                    let name = p.name.clone();
                    m.delete_profile(&p.id)?;
                    if !cli.json {
                        println!("Deleted: {name}");
                    }
                } else {
                    eprintln!("Profile not found: {id}");
                    process::exit(1);
                }
            }

            ProfileAction::Save { name, cwd, color } => {
                let cwd = if cwd == "." {
                    std::env::current_dir()?.to_string_lossy().to_string()
                } else {
                    cwd
                };

                let profile = muster::Profile {
                    id: format!("profile_{}", uuid::Uuid::new_v4()),
                    name: name.clone(),
                    color,
                    tabs: vec![muster::TabProfile {
                        name: "Shell".to_string(),
                        cwd,
                        command: None,
                    }],
                };

                let saved = m.save_profile(profile)?;
                if cli.json {
                    println!("{}", serde_json::to_string_pretty(&saved)?);
                } else {
                    println!("Saved: {} ({})", saved.name, saved.id);
                }
            }
        },
    }

    Ok(())
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    if let Err(e) = run() {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}
