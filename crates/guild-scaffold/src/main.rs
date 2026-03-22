use clap::Parser;

/// Set up a new project with the right structure, design doc template,
/// crosslink init, gitignore, and README skeleton.
#[derive(Parser)]
#[command(name = "guild-scaffold", version)]
struct Cli {
    /// Name of the project to scaffold
    name: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    guild_core::output::info("guild-scaffold is not yet implemented");
    if let Some(name) = &cli.name {
        guild_core::output::info(&format!("project name: {name}"));
    }
    Ok(())
}
