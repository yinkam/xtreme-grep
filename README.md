# XGrep - A Rust Grep Implementation

*This repository is part of [Pragmatic AI Labs Rust Bootcamp](https://ds500.paiml.com/bootcamps/rust)*

A fast, colorized grep implementation written in Rust that searches for patterns in files and directories with syntax highlighting. Features comprehensive testing, CI/CD pipeline, and optimized dependencies for production use.

## Overview

XGrep is a command-line text search tool that mimics the functionality of the Unix `grep` command. It recursively searches through files and directories for specified patterns using regular expressions, with the added benefit of colorized output to highlight matches.

This project demonstrates comprehensive Rust development practices including modular code organization, error handling, command-line argument parsing, file system traversal, extensive testing (58 tests), CI/CD automation, and dependency optimization.

## Features

### Core Functionality
- ✅ Pattern matching using regular expressions
- ✅ Recursive directory traversal with symlink support
- ✅ Colorized output with customizable colors (red, green, blue, bold)
- ✅ Command-line interface with clap derive macros
- ✅ Robust error handling for file operations
- ✅ Support for both single files and directories
- ✅ Hidden file filtering (ignores files starting with '.')
- ✅ Cross-platform compatibility (Windows, macOS, Linux)

### Development & Quality
- ✅ **Comprehensive Testing**: 58 total tests across all modules
  - 36 library tests, 7 main tests, 11 integration tests, 12 individual module tests
- ✅ **CI/CD Pipeline**: Automated GitHub Actions for multi-platform builds
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
- **CI/CD**: GitHub Actions pipeline for automated testing and releases
- **Dependency Optimization**: Minimal feature flags reducing binary size by 27%

### Key Design Decisions

- **Flexible APIs**: Functions accept `&Path` instead of `&PathBuf` for broader compatibility
- **Result-based Error Handling**: Comprehensive error propagation without panics
- **Regex Power**: Uses `regex` crate with optimized features for pattern matching
- **Memory Efficiency**: Line-by-line processing handles files of any size
- **Cross-platform**: Thoroughly tested on Windows, macOS, and Linux via CI

## Testing

XGrep features comprehensive testing with **58 total tests** ensuring reliability:

### Test Suite Breakdown

- **Library Tests (36)**: Core functionality in `lib.rs`
- **Main Tests (7)**: CLI argument parsing and path resolution
- **Integration Tests (11)**: Full CLI testing using external binary execution
- **Module Tests (12)**: Individual component testing (colors, highlighter, search, crawler)

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
- **Cross-platform**: Automated testing on Windows, macOS, and Linux

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

- **`clap = "4.5.50"`** - CLI parsing with minimal features: `["derive", "std", "help", "usage"]`
- **`regex = "1.12.2"`** - Pattern matching with optimized features: `["std", "perf", "unicode-perl"]`
- **`walkdir = "2.5.0"`** - Recursive directory traversal (already minimal)

### Development Dependencies

- **`tempdir = "0.3.7"`** - Temporary directories for comprehensive testing

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

## CI/CD Pipeline

Automated quality assurance and releases via GitHub Actions:

### Continuous Integration

- **Multi-platform Testing**: Automated testing on Ubuntu, macOS, and Windows
- **Rust Version Matrix**: Tests against stable and latest Rust versions
- **Comprehensive Coverage**: All 58 tests run on every push and pull request
- **Build Verification**: Release builds tested on all target platforms

### Automated Releases

- **Binary Artifacts**: Pre-built binaries for all major platforms
- **Semantic Versioning**: Automated tagging and release notes
- **Distribution Ready**: Optimized release binaries (1.9MB) ready for deployment

## Future Enhancements

The current implementation provides a solid foundation for advanced features:

### Phase Two - Async Implementation

- **Parallel File Processing**: Tokio-based async/await for concurrent file reading
- **Producer-Consumer Pattern**: Async channels for scalable directory traversal
- **Performance Multiplier**: Significant speed improvements for large codebases
- **Resource Management**: Configurable concurrency limits and memory usage

### Planned Features

- **Configuration Files**: `.xgreprc` support for default settings
- **Output Formats**: JSON, XML output options for tooling integration
- **Advanced Patterns**: Negative patterns, multi-pattern searches
- **Performance Metrics**: Built-in timing and statistics reporting

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on how to contribute to this project.

This is a learning-focused project demonstrating comprehensive Rust development practices. Contributions that enhance the educational value are especially welcome.

## License

This project is open source and available under the MIT License.

---

*Built during the [Pragmatic AI Labs Rust Bootcamp](https://github.com/paiml/ds500-rust-bootcamp)*

**Project Status**: ✅ **Foundation Complete** - Ready for async implementation phase
