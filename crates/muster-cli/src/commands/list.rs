use super::CommandContext;
use crate::format::{color_dot, red};

pub(crate) fn execute(ctx: &CommandContext) -> crate::error::Result {
    let profiles = ctx.muster.list_profiles()?;
    let sessions = ctx.muster.list_sessions()?;

    if ctx.json {
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
                println!("  {} {} ({}){}", color_dot(&p.color), p.name, p.id, marker);
            }
        }
        if !sessions.is_empty() {
            println!("\nSessions:");
            for s in &sessions {
                let unpinned = ctx
                    .muster
                    .count_unpinned_windows(&s.session_name)
                    .unwrap_or(0);
                let window_info = if unpinned > 0 {
                    format!(
                        "{} windows, {}",
                        s.window_count,
                        red(&format!("{unpinned} unpinned"))
                    )
                } else {
                    format!("{} windows", s.window_count)
                };
                println!(
                    "  {} {} — {} ({}){}",
                    color_dot(&s.color),
                    s.session_name,
                    s.display_name,
                    window_info,
                    if s.attached { " [attached]" } else { "" }
                );
            }
        }
        if profiles.is_empty() && sessions.is_empty() {
            println!("No profiles or sessions.");
        }
    }

    Ok(())
}
