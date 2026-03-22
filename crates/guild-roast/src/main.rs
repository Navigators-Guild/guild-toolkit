use clap::Parser;

/// Point it at a codebase and get an adversarial review checklist.
/// Automated first-pass before human review.
#[derive(Parser)]
#[command(name = "guild-roast", version)]
struct Cli {
    /// Path to the project to roast (defaults to current directory)
    #[arg(default_value = ".")]
    path: String,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    guild_core::output::info("guild-roast is not yet implemented");
    guild_core::output::info(&format!("target: {}", cli.path));
    Ok(())
}
