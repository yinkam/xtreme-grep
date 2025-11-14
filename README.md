# xerg - High-Performance Parallel Grep Tool

[![Crates.io](https://img.shields.io/crates/v/xerg.svg)](https://crates.io/crates/xerg)
[![Downloads](https://img.shields.io/crates/d/xerg.svg)](https://crates.io/crates/xerg)
[![GitHub](https://img.shields.io/github/stars/yinkam/xtreme-grep.svg)](https://github.com/yinkam/xtreme-grep)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**Published on [crates.io](https://crates.io/crates/xerg) as `xerg`**

An ultra-fast grep alternative with structured output and smart performance optimizations. Perfect for developers who need speed without sacrificing readability.

## Why xerg?

**🚀 Up to 34x faster than system grep** on large directory structures with structured output. Optional `-x` (xtreme) mode delivers maximum speedup when raw speed matters most.

## Features

- **⚡ Parallel processing**: Multi-core file scanning for maximum speed
- **🎨 Pretty output**: Syntax highlighting and structured results
- **🔍 Modern regex**: PCRE support with advanced features beyond standard grep
- **🔧 Drop-in replacement**: Familiar grep-like interface
- **🚀 Smart modes**: Default (pretty) or `-x` (raw speed)
- **📊 Statistics**: Optional detailed metrics with `--stats`

## Performance Benchmarks

xerg excels in different scenarios depending on your use case:

### Performance Comparison: xerg vs System grep

| Scenario | Tool/Mode | Time (avg) | CPU Usage | Notes |
|----------|-----------|------------:|-----------|-------|
| **Single File Search** | System grep | 4.0ms | ~58% | Baseline (single-file) |
|  | xerg (default) | 3.0ms | ~121% | 1.0ms faster (25% speedup) |
|  | xerg -x | 3.0ms | ~109% | 1.0ms faster (25% speedup) |
| **Small multi-file set (src/ ~8 files)** | System grep | 3.0ms | ~77% | Small set baseline |
|  | xerg (default) | 3.0ms | ~137% | Performance parity achieved |
|  | xerg -x | 4.0ms | ~168% | 1.0ms slower (25% overhead) |
| **Large Dataset (target/ - many files)** | System grep | 28,122ms | ~96% | Heavy single-threaded workload |
|  | xerg (default) | 1,590ms | ~484% | **17.7x faster than grep** |
|  | xerg -x | 835ms | ~745% | **33.7x faster than grep** |

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

**Test Environment**: macOS, Multi-core system, Release builds  
**Key Finding**: Default mode provides excellent performance with readability; `-x` maximizes raw speed

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

**Multi-core Processing**: Utilizes `cores - 1` threads for optimal performance  
**Smart File Reading**: Adaptive strategy based on file size (streaming/bulk/memory-mapped)  
**Memory Efficient**: Handles files of any size without excessive memory usage  

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
