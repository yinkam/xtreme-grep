use clap::Parser;
use std::env::current_dir;
use std::fs::canonicalize;
use std::path::{Path, PathBuf};
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

    if cli.path.is_none() && Path::new(&cli.pattern).exists() {
        eprintln!("error: Pattern missing. You provided a path but no search pattern.");
        eprintln!("Usage: xgrep <PATTERN> [PATH] [-- <options>...]");
        std::process::exit(1)
    }

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
