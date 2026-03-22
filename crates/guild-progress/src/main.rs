use clap::Parser;

/// Track your curriculum progress — what you've completed, what's next.
#[derive(Parser)]
#[command(name = "guild-progress", version)]
struct Cli {
    /// Show detailed progress for a specific checkpoint
    checkpoint: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    guild_core::output::info("guild-progress is not yet implemented");
    if let Some(cp) = &cli.checkpoint {
        guild_core::output::info(&format!("checkpoint: {cp}"));
    }
    Ok(())
}
