use clap::Parser;

/// Audit a project's dependencies for freshness, security advisories,
/// and license compatibility.
#[derive(Parser)]
#[command(name = "guild-deps", version)]
struct Cli {
    /// Path to the project to audit (defaults to current directory)
    #[arg(default_value = ".")]
    path: String,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    guild_core::output::info("guild-deps is not yet implemented");
    guild_core::output::info(&format!("target: {}", cli.path));
    Ok(())
}
