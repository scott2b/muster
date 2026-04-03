use super::CommandContext;
use crate::error::bail;
use crate::terminal::exec_tmux_attach;

pub(crate) fn execute(
    ctx: &CommandContext,
    profile: &str,
    tab: Option<u32>,
    detach: bool,
) -> crate::error::Result {
    let profiles = ctx.muster.list_profiles()?;
    let found = profiles
        .iter()
        .find(|p| p.name == profile || p.id == profile);

    // If no profile matches, try resolving as a live session name (e.g. adopted sessions)
    let info = if let Some(p) = found {
        ctx.muster.launch(&p.id)?
    } else {
        match ctx.muster.resolve_session(profile) {
            Ok(session_name) => {
                let sessions = ctx.muster.list_sessions()?;
                sessions
                    .into_iter()
                    .find(|s| s.session_name == session_name)
                    .ok_or_else(|| muster::Error::SessionNotFound(profile.to_string()))?
            }
            Err(_) => bail!("Profile not found: {profile}"),
        }
    };

    if let Some(idx) = tab {
        ctx.muster.switch_window(&info.session_name, idx)?;
    }

    if ctx.json {
        println!("{}", serde_json::to_string_pretty(&info)?);
    } else if detach {
        println!("Launched: {} ({})", info.display_name, info.session_name);
    } else {
        exec_tmux_attach(&info.session_name, Some(&info.display_name), &ctx.settings);
    }

    Ok(())
}
