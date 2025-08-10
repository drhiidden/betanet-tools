use clap::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The path to the file to lint
    #[arg(value_name = "FILE")]
    path: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    println!("Linting file: {}", cli.path.display());

    let content = fs::read_to_string(&cli.path)
        .expect("could not read file");

    // Placeholder linter rule: check if the file contains "TODO"
    if content.contains("TODO") {
        println!("Linter Warning: File contains 'TODO'.");
    } else {
        println!("Linter check passed: No 'TODO' found.");
    }
}
