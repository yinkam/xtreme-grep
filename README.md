# XGrep - A Rust Grep Implementation

*This repository is part of [Pragmatic AI Labs Rust Bootcamp](https://ds500.paiml.com/bootcamps/rust)*

A fast, parallel grep implementation written in Rust that searches for patterns in files and directories with syntax highlighting. Features multi-core processing, comprehensive testing, and optimized dependencies for production use.

## Overview

XGrep is a command-line text search tool that mimics the functionality of the Unix `grep` command. It recursively searches through files and directories for specified patterns using regular expressions, with the added benefit of colorized output to highlight matches.

This project demonstrates comprehensive Rust development practices including modular code organization, parallel processing with Rayon, error handling, command-line argument parsing, file system traversal, and extensive testing (58 tests).

## Features

### Core Functionality

- ✅ **Parallel Processing**: Multi-core file processing with intelligent thread pool management
- ✅ **Pattern Matching**: Regular expression engine with optimized performance features
- ✅ **Streaming Results**: Real-time output as matches are discovered, grouped by file
- ✅ **Directory Traversal**: Recursive scanning with symlink support and hidden file filtering
- ✅ **Colorized Output**: Customizable syntax highlighting (red, green, blue, bold)
- ✅ **Flexible Input**: Support for both single files and directory trees
- ✅ **Smart Threading**: Automatic CPU core detection with system responsiveness (`cores - 1`)

### Development & Quality

- ✅ **Parallel Processing**: Multi-core file processing with Rayon for significant performance gains
- ✅ **Comprehensive Testing**: 58 total tests across all modules
  - 36 library tests, 7 main tests, 11 integration tests, 12 individual module tests
- ✅ **Optimized Dependencies**: Reduced binary size by 27% (2.6MB → 1.9MB)
- ✅ **Build Automation**: Simplified Makefile with 7 essential commands
- ✅ **Integration Testing**: Full CLI testing using external binary execution

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
make test        # Run all tests (58 tests)
make run         # Run with default parameters
make clean       # Clean build artifacts
make release     # Build optimized release binary
make all         # Build, test, and create release
```

## Usage

```bash
# Search for a pattern in the current directory
cargo run "pattern" .

# Search for a pattern in a specific file
cargo run "pattern" /path/to/file.txt

# Search for a pattern in a specific directory
cargo run "pattern" /path/to/directory

# Use custom colors for highlighting (red, green, blue, bold)
cargo run --color green "pattern" .

# Or use the built binary directly
./target/release/xgrep "pattern" /path/to/search
```

### Examples

```bash
# Find all "use" statements in source files
cargo run "use" src/

# Search for function definitions with blue highlighting
cargo run --color blue "fn " src/

# Search for TODO comments with bold highlighting
cargo run --color bold "TODO" .
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

- **Parallel Processing**: Rayon-based multi-threading with channel communication for streaming results
- **Regex Power**: Uses `regex` crate with optimized features for pattern matching
- **Memory Efficiency**: Line-by-line processing handles files of any size
- **Modular Architecture**: Clean separation of concerns across focused modules

## Parallel Processing Implementation

XGrep uses a sophisticated parallel processing architecture that prioritizes both performance and user experience:

### Architecture Overview

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Main Thread   │    │  Worker Threads  │    │  Output Thread  │
│                 │    │                  │    │                 │
│ 1. Discover     │    │ 3. Process files │    │ 5. Print results│
│    files        │────▶│    in parallel   │────▶│    sequentially │
│ 2. Setup thread │    │ 4. Send results  │    │ 6. Handle errors│
│    pool         │    │    via channel   │    │                 │
└─────────────────┘    └──────────────────┘    └─────────────────┘
```

### Thread Pool Configuration

- **CPU Detection**: Uses `num_cpus` to detect available cores
- **Smart Sizing**: Thread pool = `max(1, cores - 1)` to keep system responsive
- **Global Pool**: Rayon's `ThreadPoolBuilder` creates one pool for entire application
- **Bounded Parallelism**: `rayon::scope` ensures all threads complete before returning

### Parallel Processing Trade-offs

Three implementation approaches were considered:

| Approach | Output Order | Responsiveness | Memory Usage | Implementation Complexity |
|----------|--------------|----------------|--------------|---------------------------|
| **Line-by-Line Streaming** ❌ | Scattered matches across files | Immediate (fastest) | Lowest | High (complex coordination) |
| **File-by-File Streaming** ✅ | Files in completion order | Fast (file-level batching) | Low | Medium (channel messaging) |
| **Ordered Collection** ❌ | Files in original order | Slow (wait for all) | High (buffering) | Low (simple collect) |

### **✅ Chosen: File-by-File Streaming**
- **Grouped Output**: Matches organized by file with clear headers
- **Fast Feedback**: Results appear as each file completes
- **Clean Presentation**: No interleaved matches from different files
- **Balanced Trade-off**: Good performance with readable output

### **❌ Rejected: Line-by-Line Streaming**
- **Immediate Response**: Each match sent instantly via channel
- **Scattered Output**: Lines from different files mixed together
- **Poor Readability**: Hard to see which matches belong to which files
- **Complex Coordination**: Would need file headers and match grouping logic

### **❌ Rejected: Ordered Collection**
- **Predictable Order**: Files processed in input sequence
- **Delayed Results**: Must wait for all files before any output
- **Higher Memory**: Buffering all matches before printing
- **Less Interactive**: No progress indication during processing

### Message Architecture

```rust
pub enum OutputMessage {
    Header(PathBuf),              // File separator
    Line { index: usize, content: String }, // Match with highlighting
    Error(String),                // Graceful error handling
    Done,                        // Completion marker
}
```

### Benefits Achieved

- **🚀 Speed**: Multi-core utilization without system lock-up
- **⚡ Responsiveness**: Immediate streaming results  
- **🛡️ Reliability**: Graceful error handling per file
- **📊 Scalability**: Efficient for both small and large codebases
- **🔧 Maintainability**: Clean separation of parallel work and output formatting

### Alternative Implementation

A synchronous version (`search_sync.rs`) is maintained for reference, providing:
- **Ordered Output**: Files processed and displayed in predictable sequence
- **Line-by-line Streaming**: Results appear as each line is found (not file-by-file)
- **Simpler Architecture**: Direct printing without channels or threading complexity

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
cargo test --nocapture

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
echo "Hello World\nRust is great" | cargo run "Rust"

# Test with different colors
cargo run --color green "fn" src/

# Test error handling with non-existent files
cargo run "pattern" /non/existent/path
```

## Dependencies

Carefully optimized dependencies for minimal binary size and maximum performance:

### Production Dependencies

| Crate | Version | Features | Purpose |
|-------|---------|----------|---------|
| `clap` | 4.5.50 | `derive`, `std`, `help`, `usage` | CLI argument parsing |
| `num_cpus` | 1.17.0 | *default* | CPU core detection for thread pool sizing |
| `rayon` | 1.11.0 | *default* | Parallel processing and thread pool management |
| `regex` | 1.12.2 | `std`, `perf`, `unicode-perl` | Pattern matching engine |
| `walkdir` | 2.5.0 | *default* | Directory traversal |

### Development Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `tempdir` | 0.3.7 | Temporary directories for testing |

### Optimization Results

- **Binary Size Reduction**: 27% smaller (2.6MB → 1.9MB)
- **Feature Minimization**: Only essential clap and regex features included
- **Dependency Audit**: All dependencies serve specific, necessary purposes

## Performance

XGrep delivers exceptional performance through parallel processing and memory-efficient design:

### Parallel Processing Performance

- **Multi-core Utilization**: Efficiently uses available CPU cores while keeping system responsive
- **Concurrent File Processing**: Multiple files processed simultaneously using Rayon's work-stealing scheduler  
- **Immediate Results**: Streaming output provides instant feedback as matches are found
- **Scalable Architecture**: Performance scales with both file count and available CPU cores

### Runtime Optimizations

- **Smart Thread Pool**: `cores - 1` threads prevent system lock-up while maximizing performance
- **Buffered I/O**: Efficient file reading with `BufReader` for optimal disk access patterns
- **Hidden File Filtering**: Avoids unnecessary traversal of dot files during directory scanning
- **Regex Compilation**: Pattern compiled once per thread and reused across all files

### Memory Efficiency

- **Line-by-line Processing**: Handles files of any size without loading into memory
- **Channel-based Communication**: Minimal memory overhead for inter-thread messaging
- **Minimal Allocations**: Reuses buffers and compiled regex patterns within threads
- **Optimized Binary**: Small deployment footprint (2.3MB) for fast distribution

### Performance Characteristics

- **Best Case**: Large codebases with many files - see dramatic speedup from parallelization
- **Typical Case**: Mixed file sizes - faster files provide immediate feedback while larger files process
- **Worst Case**: Single large file - still benefits from optimized I/O and regex processing

## Future Enhancements

The current parallel implementation provides a solid foundation for additional features:

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

**Project Status**: ✅ **Parallel Implementation Complete** - High-performance multi-core grep with streaming results
