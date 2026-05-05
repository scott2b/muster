//! CLI entry point for muster: terminal session group management built on tmux.

use std::path::PathBuf;
use std::process;

use clap::Parser;
use muster::Muster;
use muster_cli::{Cli, Command, NotificationAction, ProfileAction};

mod commands;
mod editing;
mod error;
mod format;
mod ports;
mod proctree;
mod resources;
mod tabs;
mod terminal;

/// Portable cli config root (profiles, settings, logs). Hardcoded to
/// `~/.config/muster/` on every platform so the tree is identical across
/// Linux and macOS. The macOS notification bundle deliberately does *not*
/// live here — it goes under `dirs::config_dir()` (`~/Library/Application
/// Support/`) per macOS convention. See SPECIFICATION.md §3.1.
fn default_config_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join(".config")
        .join("muster")
}

#[allow(clippy::too_many_lines)]
fn run() -> error::Result {
    let cli = Cli::parse();
    let config_dir = cli.config_dir.unwrap_or_else(default_config_dir);
    let m = Muster::init(&config_dir)?;
    let settings = m.settings().unwrap_or_default();

    let ctx = commands::CommandContext {
        muster: m,
        settings,
        config_dir,
        json: cli.json,
    };

    match cli.command {
        Command::List => commands::list::execute(&ctx),
        Command::Up { profile, tab, detach } => commands::launch::execute(&ctx, &profile, tab, detach),
        Command::Attach { session, tab } => commands::attach::execute(&ctx, &session, tab),
        Command::Down { session } => commands::kill::execute(&ctx, &session),
        Command::New {
            name,
            tab,
            color,
            detach,
        } => commands::new::execute(&ctx, &name, &tab, &color, detach),
        Command::Color {
            session,
            color,
            list,
        } => commands::color::execute(&ctx, session.as_deref(), color.as_deref(), list),
        Command::Ps { profile } => commands::inspect::execute_ps(&ctx, profile.as_deref()),
        Command::Ports { profile } => commands::inspect::execute_ports(&ctx, profile.as_deref()),
        Command::Top { profile } => commands::inspect::execute_top(&ctx, profile.as_deref()),
        Command::Status => commands::status::execute(&ctx),
        Command::Peek {
            session,
            tabs,
            lines,
        } => commands::peek::execute(&ctx, &session, &tabs, lines),
        Command::Pin => commands::pin::execute_pin(&ctx),
        Command::Unpin => commands::pin::execute_unpin(&ctx),
        Command::SyncRename {
            session,
            window,
            name,
        } => commands::hooks::execute_sync_rename(&ctx, &session, window, &name),
        Command::PaneDied {
            session_name,
            window_name,
            pane_id,
            exit_code,
        } => commands::hooks::execute_pane_died(
            &ctx,
            &session_name,
            &window_name,
            &pane_id,
            exit_code,
        ),
        Command::Bell {
            session_name,
            window_name,
        } => commands::hooks::execute_bell(&ctx, &session_name, &window_name),
        Command::Notifications { action } => match action {
            NotificationAction::Setup => commands::notifications::execute_setup(),
            NotificationAction::Remove => commands::notifications::execute_remove(),
            NotificationAction::Test => commands::notifications::execute_test(&ctx),
        },
        Command::Profile { action } => match action {
            ProfileAction::List => commands::profile::execute_list(&ctx),
            ProfileAction::Delete { id } => commands::profile::execute_delete(&ctx, &id),
            ProfileAction::Save {
                name,
                tab,
                color,
                from_session,
            } => commands::profile::execute_save(
                &ctx,
                &name,
                &tab,
                &color,
                from_session.as_deref(),
            ),
            ProfileAction::AddTab {
                profile,
                name,
                cwd,
                command,
            } => commands::profile::execute_add_tab(&ctx, &profile, name, cwd, command),
            ProfileAction::Show { id } => commands::profile::execute_show(&ctx, &id),
            ProfileAction::Edit { id } => commands::profile::execute_edit(&ctx, &id),
            ProfileAction::Update { id, name, color } => {
                commands::profile::execute_update(&ctx, &id, name.as_deref(), color.as_deref())
            }
            ProfileAction::RemoveTab { profile, tab } => {
                commands::profile::execute_remove_tab(&ctx, &profile, &tab)
            }
        },
        Command::Release { session, name } => {
            commands::release::execute(&ctx, &session, name.as_deref())
        }
        Command::Adopt {
            session,
            name,
            color,
            save,
            detach,
        } => commands::adopt::execute(&ctx, &session, name.as_deref(), &color, save, detach),
        Command::ShellInit { shell } => commands::shell_init::execute(&shell),
        Command::ShellSuggest { dir } => commands::adopt::execute_suggest(&ctx, &dir),
        Command::Settings {
            terminal,
            shell,
            tmux_path,
        } => commands::settings::execute(
            &ctx,
            terminal.as_deref(),
            shell.as_deref(),
            tmux_path.as_deref(),
        ),
    }
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
