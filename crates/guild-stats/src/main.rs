use clap::Parser;

/// Project health dashboard: test coverage, issue velocity,
/// commit frequency, lines changed.
#[derive(Parser)]
#[command(name = "guild-stats", version)]
struct Cli {
    /// Path to the project (defaults to current directory)
    #[arg(default_value = ".")]
    path: String,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    guild_core::output::info("guild-stats is not yet implemented");
    guild_core::output::info(&format!("target: {}", cli.path));
    Ok(())
}
