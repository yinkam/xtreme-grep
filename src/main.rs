use clap::Parser;
use std::env::current_dir;
use std::fs::canonicalize;
use std::path::PathBuf;
use xgrep::{colors::Color, run};

fn resolve_path(path: Option<PathBuf>) -> PathBuf {
    let final_path = match path {
        Some(path) => path,
        None => current_dir().unwrap(),
    };

    canonicalize(final_path).unwrap()
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    pattern: String,
    path: Option<PathBuf>,

    #[arg(long, value_name = "COLOR_NAME", default_value = "red")]
    color: String,
}

fn main() {
    let cli = Cli::parse();

    let path = resolve_path(cli.path);

    let color = Color::from_str(&cli.color).unwrap_or_else(|| {
        eprintln!(
            "Warning: Invalid color name '{}'. Defaulting to Red.",
            &cli.color
        );
        Color::Red
    });

    run(&path, &cli.pattern, &color);
}
