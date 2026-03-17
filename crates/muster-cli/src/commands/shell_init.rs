use crate::error::bail;

/// Output shell integration code for the requested shell.
///
/// Pipe to your shell's config file or eval directly:
///   fish: `muster shell-init fish | source`
///   bash: `eval "$(muster shell-init bash)"`
///   zsh:  `eval "$(muster shell-init zsh)"`
pub(crate) fn execute(shell: &str) -> crate::error::Result {
    match shell {
        "fish" => print!("{FISH_INIT}"),
        "bash" => print!("{BASH_INIT}"),
        "zsh" => print!("{ZSH_INIT}"),
        other => bail!("Unsupported shell: {other}. Supported: fish, bash, zsh"),
    }
    Ok(())
}

const FISH_INIT: &str = r#"
# muster shell integration
# Add to ~/.config/fish/config.fish:
#   muster shell-init fish | source

function __muster_suggest --on-variable PWD
    set -l matches (muster shell-suggest $PWD 2>/dev/null)
    if test -n "$matches"
        for profile in $matches
            echo "muster: profile '$profile' matches this directory. Run: muster up $profile"
        end
    end
end
"#;

const BASH_INIT: &str = r#"
# muster shell integration
# Add to ~/.bashrc or ~/.bash_profile:
#   eval "$(muster shell-init bash)"

__muster_suggest() {
    local matches
    matches=$(muster shell-suggest "$PWD" 2>/dev/null)
    if [ -n "$matches" ]; then
        while IFS= read -r profile; do
            echo "muster: profile '$profile' matches this directory. Run: muster up $profile"
        done <<< "$matches"
    fi
}

# Hook into PROMPT_COMMAND so it runs before each prompt
if [[ "$PROMPT_COMMAND" != *"__muster_suggest"* ]]; then
    PROMPT_COMMAND="__muster_suggest${PROMPT_COMMAND:+; $PROMPT_COMMAND}"
fi
"#;

const ZSH_INIT: &str = r#"
# muster shell integration
# Add to ~/.zshrc:
#   eval "$(muster shell-init zsh)"

__muster_suggest() {
    local matches
    matches=$(muster shell-suggest "$PWD" 2>/dev/null)
    if [ -n "$matches" ]; then
        while IFS= read -r profile; do
            echo "muster: profile '$profile' matches this directory. Run: muster up $profile"
        done <<< "$matches"
    fi
}

# Hook into chpwd so it runs on directory change
autoload -Uz add-zsh-hook
add-zsh-hook chpwd __muster_suggest
"#;
