use super::CommandContext;

pub(crate) fn execute(
    ctx: &CommandContext,
    session: &str,
    name: Option<&str>,
) -> crate::error::Result {
    let session_name = ctx.muster.resolve_session(session)?;
    let new_name = ctx.muster.release(&session_name, name)?;

    if ctx.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "released": session_name,
                "new_name": new_name,
            }))?
        );
    } else {
        println!("Released: {session_name} → {new_name}");
    }
    Ok(())
}
