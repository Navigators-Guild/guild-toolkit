use colored::Colorize;

/// Print a success message in green.
pub fn success(msg: &str) {
    println!("{} {}", "✓".green().bold(), msg);
}

/// Print an info message in blue.
pub fn info(msg: &str) {
    println!("{} {}", "→".blue().bold(), msg);
}

/// Print a warning message in yellow.
pub fn warn(msg: &str) {
    eprintln!("{} {}", "!".yellow().bold(), msg);
}

/// Print an error message in red.
pub fn error(msg: &str) {
    eprintln!("{} {}", "✗".red().bold(), msg);
}

/// Print a section header.
pub fn header(title: &str) {
    println!("\n{}", title.bold().underline());
}
