use clap::Parser;

/// Search the guild knowledge base, curriculum docs, and common patterns.
#[derive(Parser)]
#[command(name = "guild-search", version)]
struct Cli {
    /// Search query
    query: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    guild_core::output::info("guild-search is not yet implemented");
    if !cli.query.is_empty() {
        guild_core::output::info(&format!("query: {}", cli.query.join(" ")));
    }
    Ok(())
}
