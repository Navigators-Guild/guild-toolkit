use clap::Parser;

/// Submit a project for adversarial review, pull down feedback,
/// and track review rounds.
#[derive(Parser)]
#[command(name = "guild-review", version)]
struct Cli {
    /// Subcommand: submit, status, feedback
    action: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    guild_core::output::info("guild-review is not yet implemented");
    if let Some(action) = &cli.action {
        guild_core::output::info(&format!("action: {action}"));
    }
    Ok(())
}
