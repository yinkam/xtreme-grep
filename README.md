# XGrep - A Rust Grep Implementation

*This repository is part of [Pragmatic AI Labs Rust Bootcamp](https://ds500.paiml.com/bootcamps/rust)*

A fast, colorized grep implementation written in Rust that searches for patterns in files and directories with syntax highlighting. Features comprehensive testing and optimized dependencies for production use.

## Overview

XGrep is a command-line text search tool that mimics the functionality of the Unix `grep` command. It recursively searches through files and directories for specified patterns using regular expressions, with the added benefit of colorized output to highlight matches.

This project demonstrates comprehensive Rust development practices including modular code organization, error handling, command-line argument parsing, file system traversal, and extensive testing (58 tests).

## Features

### Core Functionality
- ✅ Pattern matching using regular expressions
- ✅ Recursive directory traversal with symlink support
- ✅ Colorized output with customizable colors (red, green, blue, bold)
- ✅ Command-line interface with clap derive macros
- ✅ Support for both single files and directories
- ✅ Hidden file filtering (ignores files starting with '.')

### Development & Quality
- ✅ **Comprehensive Testing**: 58 total tests across all modules
  - 36 library tests, 7 main tests, 11 integration tests, 12 individual module tests
- ✅ **Optimized Dependencies**: Reduced binary size by 27% (2.6MB → 1.9MB)
- ✅ **Build Automation**: Simplified Makefile with 7 essential commands
- ✅ **Integration Testing**: Full CLI testing using external binary execution

## Quick Start

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/yinkam/rust-grep.git
   cd rust-grep
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
make test        # Run all tests (58 tests)
make run         # Run with default parameters
make clean       # Clean build artifacts
make release     # Build optimized release binary
make all         # Build, test, and create release
```

## Usage

```bash
# Search for a pattern in the current directory
cargo run -- "pattern" .

# Search for a pattern in a specific file
cargo run -- "pattern" /path/to/file.txt

# Search for a pattern in a specific directory
cargo run -- "pattern" /path/to/directory

# Use custom colors for highlighting (red, green, blue, bold)
cargo run -- --color green "pattern" .

# Or use the built binary directly
./target/release/xgrep "pattern" /path/to/search
```

### Examples

```bash
# Find all "use" statements in source files
cargo run -- "use" src/

# Search for function definitions with blue highlighting
cargo run -- --color blue "fn " src/

# Search for TODO comments with bold highlighting
cargo run -- --color bold "TODO" .
```

## Architecture

The project follows a modular architecture with clear separation of concerns:

### Core Modules

- **`main.rs`**: CLI entry point with clap derive macros and Result-based error handling
- **`lib.rs`**: Core integration layer connecting all modules with optimized borrowing patterns
- **`search.rs`**: File processing with flexible API design (`&Path` vs `&PathBuf`)
- **`crawler.rs`**: Directory traversal using `walkdir` with symlink and hidden file support
- **`highlighter.rs`**: Regex-based text highlighting (renamed from colorizer for clarity)
- **`colors.rs`**: ANSI color management with simple `from_str()` method

### Quality Assurance

- **Unit Tests**: 47 focused unit tests across all modules
- **Integration Tests**: 11 comprehensive CLI tests using external binary execution
- **Dependency Optimization**: Minimal feature flags reducing binary size by 27%

### Key Design Decisions

- **Regex Power**: Uses `regex` crate with optimized features for pattern matching
- **Memory Efficiency**: Line-by-line processing handles files of any size
- **Modular Architecture**: Clean separation of concerns across focused modules

## Testing

XGrep features comprehensive testing with **58 total tests** ensuring reliability:

### Test Suite Breakdown

| Test Type | Count | Coverage |
|-----------|-------|----------|
| Library Tests | 36 | Core functionality in `lib.rs` |
| Main Tests | 7 | CLI argument parsing and path resolution |
| Integration Tests | 11 | Full CLI testing using external binary execution |
| Module Tests | 12 | Individual component testing (colors, highlighter, search, crawler) |
| **Total** | **58** | **Comprehensive reliability testing** |

### Running Tests

```bash
# Run all tests
make test
# or
cargo test

# Run with verbose output
cargo test -- --nocapture

# Run specific test modules
cargo test search::tests
cargo test integration_tests
```

### Test Categories

- **Unit Tests**: Component isolation and edge cases
- **Integration Tests**: End-to-end CLI functionality with real file operations
- **Error Handling**: Graceful handling of missing files, permissions, invalid patterns
- **Performance**: Large files, long lines, Unicode content
- **Comprehensive Coverage**: Unit tests, integration tests, and error handling scenarios

### Manual Testing

```bash
# Test basic functionality
echo "Hello World\nRust is great" | cargo run -- "Rust"

# Test with different colors
cargo run -- --color green "fn" src/

# Test error handling with non-existent files
cargo run -- "pattern" /non/existent/path
```

## Dependencies

Carefully optimized dependencies for minimal binary size and maximum performance:

### Production Dependencies

| Crate | Version | Features | Purpose |
|-------|---------|----------|---------|
| `clap` | 4.5.50 | `derive`, `std`, `help`, `usage` | CLI argument parsing |
| `regex` | 1.12.2 | `std`, `perf`, `unicode-perl` | Pattern matching engine |
| `walkdir` | 2.5.0 | _(default)_ | Directory traversal |

### Development Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `tempdir` | 0.3.7 | Temporary directories for testing |

### Optimization Results

- **Binary Size Reduction**: 27% smaller (2.6MB → 1.9MB)
- **Feature Minimization**: Only essential clap and regex features included
- **Dependency Audit**: All dependencies serve specific, necessary purposes

## Performance

XGrep is optimized for both speed and memory efficiency:

### Runtime Performance

- **Buffered I/O**: Efficient file reading with `BufReader`
- **Lazy Evaluation**: Files processed only when pattern matches are found
- **Hidden File Filtering**: Avoids unnecessary traversal of dot files
- **Regex Compilation**: Pattern compiled once and reused across all files

### Memory Efficiency

- **Line-by-line Processing**: Handles files of any size without loading into memory
- **Minimal Allocations**: Reuses buffers and compiled regex patterns
- **Optimized Binary**: Small deployment footprint (1.9MB) for fast distribution

## Future Enhancements

The current implementation provides a solid foundation for advanced features:

### Phase Two - Parallel Processing with Rayon

- **Parallel File Processing**: Process multiple files simultaneously for faster searches
- **Multi-core Utilization**: Efficiently use available CPU cores while keeping system responsive
- **Performance Multiplier**: Significant speed improvements for large codebases
- **Smart Scaling**: Optimal performance regardless of file count or size distribution

### Planned Features

#### Color Control
- **Auto-Color Detection**: Color **ON** for terminal, **OFF** when piped
- **Manual Override**: `--color` (force on) and `--no-color` (force off) flags
- **CLI Standards**: Follows Unix conventions for output handling

#### Silent Mode
- **Quiet Operation**: `-s` / `--silent` flag suppresses all **stderr** messages
- **Error Suppression**: Including permission denied and file access errors  
- **UX-Friendly**: Simple way for users to eliminate noise

#### Advanced Pattern Features
- **Negative Patterns**: Exclude matches with `-v` / `--invert-match` flag
- **Multi-pattern Support**: Search for multiple patterns simultaneously
- **Case Insensitive**: `-i` / `--ignore-case` flag for case-insensitive matching

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on how to contribute to this project.

This is a learning-focused project demonstrating comprehensive Rust development practices. Contributions that enhance the educational value are especially welcome.

## License

This project is open source and available under the MIT License.

---

*Built during the [Pragmatic AI Labs Rust Bootcamp](https://github.com/paiml/ds500-rust-bootcamp)*

**Project Status**: ✅ **Foundation Complete** - Ready for async implementation phase
