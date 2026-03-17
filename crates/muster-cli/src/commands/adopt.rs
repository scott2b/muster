use super::CommandContext;
use crate::terminal::exec_tmux_attach;

pub(crate) fn execute(
    ctx: &CommandContext,
    session: &str,
    name: Option<&str>,
    color: &str,
    save: bool,
    detach: bool,
) -> crate::error::Result {
    let display_name = name.unwrap_or(session);

    let info = ctx.muster.adopt(session, display_name, color)?;

    if save {
        let tabs = ctx.muster.snapshot_session(&info.session_name)?;
        let profile = muster::Profile {
            id: info.profile_id.clone().unwrap_or_default(),
            name: info.display_name.clone(),
            color: info.color.clone(),
            tabs,
            ..muster::Profile::default()
        };
        // upsert: try create, fall back to update if already exists
        let saved = match ctx.muster.save_profile(profile.clone()) {
            Ok(p) => p,
            Err(_) => ctx.muster.update_profile(profile)?,
        };
        // Mark all windows as pinned now that the profile exists
        ctx.muster.pin_session_windows(&info.session_name)?;
        if ctx.json {
            println!("{}", serde_json::to_string_pretty(&saved)?);
            return Ok(());
        }
        if detach {
            println!("Adopted and saved: {} ({})", saved.name, saved.id);
            return Ok(());
        }
    } else if ctx.json {
        println!("{}", serde_json::to_string_pretty(&info)?);
        return Ok(());
    } else if detach {
        println!("Adopted: {} → {}", session, info.session_name);
        return Ok(());
    }

    exec_tmux_attach(&info.session_name, &ctx.settings)
}

pub(crate) fn execute_suggest(ctx: &CommandContext, dir: &str) -> crate::error::Result {
    let profiles = ctx.muster.list_profiles()?;
    let matches: Vec<_> = profiles
        .iter()
        .filter(|p| p.tabs.iter().any(|t| t.cwd == dir || dir.starts_with(&t.cwd)))
        .collect();

    if matches.is_empty() {
        return Ok(());
    }

    if ctx.json {
        println!("{}", serde_json::to_string_pretty(&matches)?);
    } else {
        for p in matches {
            println!("{}", p.name);
        }
    }
    Ok(())
}
