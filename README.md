# xerg - High-Performance Parallel Grep Tool

[![Crates.io](https://img.shields.io/crates/v/xerg.svg)](https://crates.io/crates/xerg)
[![Downloads](https://img.shields.io/crates/d/xerg.svg)](https://crates.io/crates/xerg)
[![GitHub](https://img.shields.io/github/stars/yinkam/xtreme-grep.svg)](https://github.com/yinkam/xtreme-grep)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**Published on [crates.io](https://crates.io/crates/xerg) as `xerg`**

An ultra-fast, parallel grep implementation in Rust with syntax highlighting and detailed search statistics. Built for performance with multi-core processing and optimized dependencies.

*This repository is part of [Pragmatic AI Labs Rust Bootcamp](https://ds500.paiml.com/bootcamps/rust)*

## Why xerg?

**🚀 23x faster than system grep** on large directory structures with beautiful structured output. Optional `-x` (xtreme) mode delivers **33x speedup** when raw speed matters most.

## Features

- ✅ **Parallel Processing**: Multi-core file processing with intelligent thread pool management
- ✅ **Pattern Matching**: Regular expression engine with optimized performance
- ✅ **Structured Streaming**: Streams structured matches per file with headers and statistics
- ✅ **Directory Traversal**: Recursive scanning with symlink support
- ✅ **Colorized Output**: Customizable syntax highlighting (red, green, blue, bold)
- ✅ **Search Statistics**: Optional detailed metrics with `--stats` flag
- ✅ **Quality Assurance**: Comprehensive test suite and optimized dependencies

## Performance Benchmarks

xerg excels in different scenarios depending on your use case:

### Performance Comparison: xerg vs System grep

| Scenario | Tool/Mode | Time | CPU Usage | Winner | Performance Gain |
|----------|-----------|------|-----------|---------|-----------------|
| **Single File Search** | System grep | 0.004s | 88% | ✅ System grep | 92x faster |
| | xerg (default) | 0.369s | 26% | | |
| | xerg -x | 0.369s | 26% | | |
| **Multi-Directory Search** | System grep | 10.194s | 87% | | |
| | **xerg (default)** | **~0.450s** | **650%** | ✅ **xerg default** | **23x faster** |
| | **xerg -x** | **0.310s** | **690%** | ✅ **xerg xtreme** | **33x faster** |
| **Large Dataset (2971 files)** | System grep | 10.194s | Single-core | | |
| | **xerg (default)** | **~0.450s** | **Multi-core** | ✅ **xerg default** | **23x faster** |
| | **xerg -x** | **0.310s** | **Multi-core** | ✅ **xerg xtreme** | **33x faster** |

### Mode Comparison: xerg (default) vs xerg -x

| Aspect | xerg (default formatted) | xerg -x (xtreme) |
|--------|--------------------------|-------------------|
| **Output Format** | Pretty headers, structured | Raw `file:line:content` |
| **Speed** | Fast (23x vs grep) | Fastest (33x vs grep) |
| **Use Case** | Most users, development | Raw speed, automation |
| **Memory Usage** | Higher (buffering) | Lower (direct output) |
| **CPU Usage** | 650% (6.5 cores) | 690% (6.9 cores) |

### Usage Recommendations

#### Use xerg (default mode)

✅ **Structured output** with pretty formatting  
✅ **Code exploration** and development work  
✅ **Human-readable results** with file headers  
✅ **Most users** - balanced speed + readability  

#### Use xerg -x (xtreme mode)

✅ **Maximum raw speed** when structure isn't needed  
✅ **Shell pipelines** and automation scripts  
✅ **CI/CD tasks** where every millisecond counts  
✅ **Grep-compatible output** for tool integration  

#### Use System grep

✅ **Single file searches**  
✅ **Simple shell scripting**  
✅ **One-off quick searches**

### Detailed Benchmark Results

| Test Case | Pattern | Files | Matches | System grep | xerg (default) | xerg -x | Best Speedup |
|-----------|---------|-------|---------|-------------|----------------|---------|--------------|
| Small project (src/) | `use` | 8 files | 90 matches | 0.004s | 0.369s | 0.369s | 0.01x |
| Large dataset (deps/) | `use` | 2971 files | 3465 matches | 10.194s | ~0.450s | **0.310s** | **32.9x** |
| Real codebase | `function` | Variable | Variable | Linear growth | Parallel + pretty | **Maximum speed** | **Scales** |

**Test Environment**: macOS, Multi-core system, Release builds  
**Methodology**: Multiple runs averaged, `time` command measurements  
**Key Finding**: Default mode provides excellent performance with readability; -x maximizes raw speed

## Quick Start

### Installation

#### Install from crates.io (Recommended)

```bash
cargo install xerg
```

#### Build from Source

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

**Using the installed binary:**

```bash
# Basic search in current directory
xerg "pattern" .

# Search with colored output, statistics, and specific path
xerg --color blue --stats "pattern" src/
```

**For development (from source):**

```bash
# Basic search in current directory
cargo run "pattern" .

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
$ xerg --stats --color blue "use" src/
--- /Users/user/rust-grep/src/lib.rs ---
    8:  use colors::Color;
    9:  use crawler::get_files;
   10:  use result::print_result;
--- /Users/user/rust-grep/src/main.rs ---
   15:  use greprs::{run_search, Config};
   16:  use std::process;
  lines: 45, matches: 2, skipped: 0
result: files:8; lines:1186; matches:207; skipped:0; errors:0; time:0.012s;
```

**Structured Result Format:** Machine-readable summary with semicolon delimiters and millisecond-precision timing. Perfect for performance analysis and automated testing.

**Metrics:** `files` = processed files, `lines` = total lines read, `matches` = pattern occurrences, `skipped` = unreadable lines, `errors` = access failures, `time` = execution time

## Architecture

The project follows a modular architecture with clear separation of concerns:

### Core Modules

- **`main.rs`**: CLI entry point and argument parsing
- **`lib.rs`**: Core integration layer connecting all modules  
- **`search.rs`**: Parallel file processing with Rayon
- **`crawler.rs`**: Directory traversal with symlink support
- **`highlighter.rs`**: Regex-based text highlighting
- **`colors.rs`**: ANSI color management
- **`result.rs`**: Message handling and statistics formatting

## Dependencies

| Crate | Purpose |
|-------|---------|
| `clap` | CLI argument parsing |
| `num_cpus` | Thread optimization |
| `rayon` | Parallel processing |
| `regex` | Pattern matching |
| `walkdir` | Directory traversal |

**Binary Size**: 2.2MB optimized release build with minimal feature flags enabled

## Implementation Details

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
