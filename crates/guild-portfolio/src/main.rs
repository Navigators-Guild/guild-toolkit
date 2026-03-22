use clap::Parser;

/// Generate a portfolio site from your project repos.
#[derive(Parser)]
#[command(name = "guild-portfolio", version)]
struct Cli {
    /// Output directory for generated site
    #[arg(short, long, default_value = "./site")]
    output: String,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    guild_core::output::info("guild-portfolio is not yet implemented");
    guild_core::output::info(&format!("output dir: {}", cli.output));
    Ok(())
}
