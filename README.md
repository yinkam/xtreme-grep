# XGrep - A Rust Grep Implementation

*This repository is part of [Pragmatic AI Labs Rust Bootcamp](https://ds500.paiml.com/bootcamps/rust)*

A fast, parallel grep implementation in Rust with syntax highlighting and detailed search statistics. Built for performance with multi-core processing and optimized dependencies.

## Features

- ✅ **Parallel Processing**: Multi-core file processing with intelligent thread pool management
- ✅ **Pattern Matching**: Regular expression engine with optimized performance
- ✅ **Structured Streaming**: Streams structured matches per file with headers and statistics
- ✅ **Directory Traversal**: Recursive scanning with symlink support
- ✅ **Colorized Output**: Customizable syntax highlighting (red, green, blue, bold)
- ✅ **Search Statistics**: Optional detailed metrics with `--stats` flag
- ✅ **Quality Assurance**: Comprehensive test suite and optimized dependencies

## Quick Start

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/yinkam/xtreme-grep.git
   cd xtreme-grep
   ```

2. Build the project:

   ```bash
   cargo build --release
   ```

### Development Setup

Use the included Makefile for common development tasks:

```bash
make help        # Show all available commands
make build       # Build the project
make test        # Run all tests
make run         # Run with default parameters
make clean       # Clean build artifacts
make release     # Build optimized release binary
make all         # Build, test, and create release
```

## Usage

**Minimal Usage:**

```bash
# Basic search in current directory
cargo run "pattern" .
```

**Full Usage with All Options:**

```bash
# Search with colored output, statistics, and specific path
cargo run --color blue --stats "pattern" src/

# Or use the built binary directly
./target/release/xgrep --color green --stats "pattern" /path/to/search
```

### Command-Line Options

| Option | Description | Example |
|--------|-------------|---------|
| `pattern` | Search pattern (required) | `"use"` |
| `path` | File or directory to search (optional, defaults to current directory) | `src/` |
| `--color <COLOR>` | Highlight color: `red`, `green`, `blue`, `bold` | `--color blue` |
| `--stats` | Show detailed search statistics | `--stats` |
| `--help` | Display help information | `--help` |
| `--version` | Show version information | `--version` |

### Search Statistics

```bash
$ cargo run --stats --color blue "use" src/
--- /Users/user/rust-grep/src/lib.rs ---
    8:  use colors::Color;
    9:  use crawler::get_files;
   10:  use output::print_output;
--- /Users/user/rust-grep/src/main.rs ---
   15:  use greprs::{run_search, Config};
   16:  use std::process;
  lines: 45, matches: 2, skipped: 0
Summary: files: 8   lines: 1186   matches: 207   skipped: 0   errors: 0
```

**Metrics:** `files` = files processed, `lines` = total lines read, `matches` = pattern occurrences found, `skipped` = unreadable lines, `errors` = file access failures

## Architecture

The project follows a modular architecture with clear separation of concerns:

### Core Modules

- **`main.rs`**: CLI entry point and argument parsing
- **`lib.rs`**: Core integration layer connecting all modules  
- **`search.rs`**: Parallel file processing with Rayon
- **`crawler.rs`**: Directory traversal with symlink support
- **`highlighter.rs`**: Regex-based text highlighting
- **`colors.rs`**: ANSI color management
- **`output.rs`**: Message handling and statistics formatting

## Dependencies

| Crate | Purpose |
|-------|---------|
| `clap` | CLI argument parsing |
| `num_cpus` | Thread optimization |
| `rayon` | Parallel processing |
| `regex` | Pattern matching |
| `walkdir` | Directory traversal |

**Binary Size**: 2.2MB optimized release build with minimal feature flags enabled

## Performance

**Multi-core Processing**: Utilizes `cores - 1` threads for optimal performance without system lock-up  
**Memory Efficient**: Line-by-line processing handles files of any size  
**Structured Streaming**: Streams structured matches per file as processing completes  
**Optimized I/O**: Buffered reading and compiled regex reuse

## Planned Features

- **Auto-color detection** (`--color=auto`)
- **Silent mode** (`-s`, `--silent`)
- **Case insensitive search** (`-i`, `--ignore-case`)
- **Invert matching** (`-v`, `--invert-match`)
- **Multi-pattern support**
- **File type filtering**
- **Line number display** (`-n`, `--line-number`)

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on how to contribute to this project.

This is a learning-focused project demonstrating comprehensive Rust development practices. Contributions that enhance the educational value are especially welcome.

## License

This project is open source and available under the MIT License.

---

*Built during the [Pragmatic AI Labs Rust Bootcamp](https://github.com/paiml/ds500-rust-bootcamp)*

**Project Status**: ✅ **Parallel Implementation Complete** - High-performance multi-core grep with structured streaming
