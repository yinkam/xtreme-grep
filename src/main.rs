use clap::Parser;
use num_cpus;
use rayon::ThreadPoolBuilder;
use std::env::current_dir;
use std::fs::canonicalize;
use std::path::{Path, PathBuf};
use xgrep::{colors::Color, run};

fn resolve_path(path: Option<PathBuf>) -> Result<PathBuf, std::io::Error> {
    let final_path = match path {
        Some(path) => path,
        None => current_dir()?,
    };

    canonicalize(final_path)
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    pattern: String,
    path: Option<PathBuf>,

    #[arg(long, value_name = "COLOR_NAME", default_value = "red")]
    color: String,

    #[arg(long, help = "Show search stats per file and total stats summary")]
    stats: bool,
}

fn main() {
    let cores = num_cpus::get();
    let num_threads = std::cmp::max(1, cores - 1);
    ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
        .unwrap();

    let cli = Cli::parse();

    if cli.path.is_none() && Path::new(&cli.pattern).exists() {
        eprintln!("error: Pattern missing. You provided a path but no search pattern.");
        eprintln!("Usage: xgrep <PATTERN> [PATH] [-- <options>...]");
        std::process::exit(1)
    }

    let path = match resolve_path(cli.path) {
        Ok(path) => path,
        Err(_) => {
            eprintln!("error: file or directory does not exist");
            std::process::exit(1);
        }
    };

    let color = Color::from_str(&cli.color).unwrap_or_else(|| {
        eprintln!(
            "Warning: Invalid color name '{}'. Defaulting to Red.",
            &cli.color
        );
        Color::Red
    });

    run(&path, &cli.pattern, &color, cli.stats);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempdir::TempDir;

    #[test]
    fn test_resolve_path_with_current_dir() {
        // Test resolve_path when no path is provided (should use current dir)
        let result = resolve_path(None).unwrap();

        // Should resolve to current directory
        assert!(result.is_absolute());
        assert!(result.exists());
    }

    #[test]
    fn test_resolve_path_with_valid_path() {
        // Test resolve_path with a valid path
        let temp_dir = TempDir::new("resolve_test").unwrap();

        let result = resolve_path(Some(temp_dir.path().to_path_buf())).unwrap();

        // Should resolve to an absolute path that exists
        assert!(result.is_absolute());
        assert!(result.exists());
    }

    #[test]
    fn test_resolve_path_with_existing_file() {
        // Test resolve_path with an existing file
        let temp_dir = TempDir::new("resolve_file_test").unwrap();
        let temp_file = temp_dir.path().join("test.txt");
        File::create(&temp_file).unwrap();

        let result = resolve_path(Some(temp_file.clone())).unwrap();

        // Should resolve to absolute path
        assert!(result.is_absolute());
        assert!(result.exists());
        assert!(result.is_file());
    }

    #[test]
    fn test_resolve_path_nonexistent() {
        // Test resolve_path with nonexistent path (should return Err)
        let nonexistent = PathBuf::from("/definitely/does/not/exist/path");
        let result = resolve_path(Some(nonexistent));

        // Should return an error
        assert!(result.is_err());
    }

    #[test]
    fn test_cli_structure() {
        // Test that CLI structure is properly defined
        // This is more of a compile-time test, but ensures the structure is valid

        let args = vec!["xgrep", "pattern", "/path"];
        let cli = Cli::try_parse_from(args).unwrap();

        assert_eq!(cli.pattern, "pattern");
        assert_eq!(cli.path, Some(PathBuf::from("/path")));
        assert_eq!(cli.color, "red"); // default value
    }

    #[test]
    fn test_cli_with_color_flag() {
        // Test CLI parsing with color flag
        let args = vec!["xgrep", "pattern", "/path", "--color", "blue"];
        let cli = Cli::try_parse_from(args).unwrap();

        assert_eq!(cli.pattern, "pattern");
        assert_eq!(cli.path, Some(PathBuf::from("/path")));
        assert_eq!(cli.color, "blue");
    }

    #[test]
    fn test_cli_pattern_only() {
        // Test CLI with just pattern (no path)
        let args = vec!["xgrep", "pattern"];
        let cli = Cli::try_parse_from(args).unwrap();

        assert_eq!(cli.pattern, "pattern");
        assert_eq!(cli.path, None);
        assert_eq!(cli.color, "red");
    }
}
