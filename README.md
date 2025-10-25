# XGrep - A Rust Grep Implementation

*This repository is part of [Pragmatic AI Labs Rust Bootcamp](https://ds500.paiml.com/bootcamps/rust)*

A fast, colorized grep implementation written in Rust that searches for patterns in files and directories with syntax highlighting.

## Overview

XGrep is a command-line text search tool that mimics the functionality of the Unix `grep` command. It recursively searches through files and directories for specified patterns using regular expressions, with the added benefit of colorized output to highlight matches.

This project demonstrates real-world Rust development practices including modular code organization, error handling, command-line argument parsing, and file system traversal.

## Features

- ✅ Pattern matching using regular expressions
- ✅ Recursive directory traversal
- ✅ Colorized output with customizable colors
- ✅ Command-line interface with clap
- ✅ Error handling for file operations
- ✅ Support for both single files and directories
- ✅ Hidden file filtering (ignores files starting with '.')
- ✅ Cross-platform compatibility

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/yinkam/rust-grep.git
   cd rust-grep
   ```

2. Build the project:
   ```bash
   cargo build --release
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

The project is organized into several focused modules:

- **`main.rs`**: Entry point with command-line argument parsing using `clap`
- **`lib.rs`**: Core library interface and main run logic
- **`search.rs`**: File and directory searching functionality
- **`crawler.rs`**: Directory traversal using `walkdir` crate
- **`colorizer.rs`**: Text highlighting using regex pattern matching
- **`colors.rs`**: Color definitions and ANSI escape code management

### Key Design Decisions

- **Modular Architecture**: Each component has a single responsibility
- **Error Handling**: Graceful error handling for file operations
- **Regular Expressions**: Uses the `regex` crate for powerful pattern matching
- **Memory Efficiency**: Processes files line-by-line to handle large files
- **Cross-platform**: Uses standard library and well-tested crates for compatibility

## Testing

Run the test suite with:

```bash
cargo test
```

The project includes unit tests for core functionality. To run tests with output:

```bash
cargo test -- --nocapture
```

### Manual Testing

You can test the application with various scenarios:

```bash
# Test basic functionality
echo "Hello World\nRust is great" | cargo run -- "Rust"

# Test with different colors
cargo run -- --color green "fn" src/

# Test error handling with non-existent files
cargo run -- "pattern" /non/existent/path
```

## Dependencies

This project uses the following key dependencies:

- **`clap`** - Command line argument parsing with derive macros
- **`regex`** - Regular expression engine for pattern matching
- **`walkdir`** - Recursive directory traversal

## Performance

XGrep is designed for performance:

- Uses buffered reading for efficient file processing
- Filters hidden files during traversal to avoid unnecessary work
- Compiled binary provides fast startup and execution times
- Memory-efficient line-by-line processing

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on how to contribute to this project.

## License

This project is open source and available under the MIT License.

---

*Built during the [Pragmatic AI Labs Rust Bootcamp](https://github.com/paiml/ds500-rust-bootcamp)*
