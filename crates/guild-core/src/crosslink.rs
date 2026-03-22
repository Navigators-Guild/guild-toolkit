use crate::GuildError;
use std::process::Command;

/// Run a crosslink CLI command and return its stdout.
pub fn run(args: &[&str]) -> Result<String, GuildError> {
    let output = Command::new("crosslink")
        .args(args)
        .output()
        .map_err(|e| GuildError::Crosslink(format!("failed to spawn crosslink: {e}")))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(GuildError::Crosslink(format!(
            "crosslink {} failed: {}",
            args.first().unwrap_or(&""),
            stderr.trim()
        )));
    }

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

/// Search the crosslink knowledge hub.
pub fn knowledge_search(query: &str) -> Result<String, GuildError> {
    run(&["knowledge", "search", query])
}
