# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.1] - 2025-11-12

### Added

- **FileReadStrategy Architecture**: Three-tier adaptive file reading system (streaming, bulk read, memory mapping)
- **Single-File Optimization**: Bypasses thread pool overhead for single files, eliminating 175x performance penalty vs system grep  
- **Empirically-Determined Thresholds**: Smart selection based on comprehensive benchmarking (7MB and 100MB breakpoints)
- **Memory Mapping Support**: Optimal strategy for files between 7MB-100MB to prevent virtual memory pressure
- **Bulk Reading Strategy**: `fs::read_to_string` for files ≤7MB delivers 3.6x speedup vs streaming
- **Pattern Matching Elegance**: Local constant declarations enable computed values in range patterns

### Enhanced

- **Code Architecture**: Eliminated code duplication through strategy pattern with shared content processing
- **Error Handling**: Individual error messages per strategy with comprehensive fallback mechanisms  
- **Function Organization**: Private `_process_file_*` helpers following Rust naming conventions
- **Testing Coverage**: All 48 unit tests + 13 integration tests passing with new architecture
- **Performance Consistency**: Maintained existing parallel performance for multi-file operations

- **File Processing Strategy**: 
  - Files ≤7MB: `fs::read_to_string` (simple, no memory mapping overhead)
  - Files >7MB: Memory mapping (14% faster, scales with file size)
- **Single-File Detection**: Automatic bypass of thread pool for single file operations
- **Both Modes Optimized**: Single-file optimization works in both default and xtreme modes
- **Error Handling**: Robust error handling for memory mapping and file operations

### Performance

- **Single-File Speed**: Significant improvement through elimination of:
  - Thread pool overhead (2.18x penalty removed)
  - Line-by-line streaming overhead (3.6x penalty removed)  
  - Memory mapping benefits for large files (1.14x improvement)
- **Memory Efficiency**: Optimal memory usage strategy based on file size analysis
- **Large File Handling**: Superior performance on files >7MB through memory mapping

### Technical

- **Dependencies**: Added `memmap2` for memory mapping functionality
- **Architecture**: Maintained backward compatibility with existing multi-file parallel processing
- **Documentation**: Updated performance analysis with comprehensive benchmark results
- **Testing**: All existing tests pass with new single-file optimization paths

## [0.2.0] - 2025-11-10

### Added

- **Xtreme Mode**: New `-x/--xtreme` flag for maximum raw speed (33x faster than system grep)
- **Dual Architecture**: Default formatted output vs optional raw speed mode
- **Performance Benchmarks**: Comprehensive Criterion-based benchmark suite
- **Head-to-Head Comparisons**: Documented performance vs system grep across scenarios
- **Professional Documentation**: Detailed performance tables and usage recommendations

### Enhanced

- **CLI Interface**: Added `-x` short form for xtreme mode with clear help text
- **Output Formats**: Maintained readable spaced format in xtreme mode for better UX
- **Statistics**: Enhanced stats output for both modes (formatted vs summary)
- **Test Coverage**: Comprehensive integration tests for both modes
- **Development Workflow**: Added Criterion benchmarks and enhanced test suite

### Performance

- **Multi-Directory Speed**: 23x faster than grep in default mode, 33x in xtreme mode
- **Thread Utilization**: Optimized parallel processing with 6.5-6.9 cores usage
- **Memory Efficiency**: Direct output in xtreme mode reduces memory overhead

## [0.1.1] - 2025-11-01

### Fixed

- Corrected binary name references from `xgrep` to `xerg` throughout documentation
- Added proper crates.io badges and publication status
- Updated installation instructions to prioritize published version
- Fixed usage examples to reflect actual published command name

### Changed

- Enhanced README with professional presentation
- Improved project description and feature highlights

## [0.1.0] - 2025-11-01

### Initial Release

- Initial release on crates.io
- Parallel file processing with Rayon
- Colorized output with customizable highlighting
- Search statistics with `--stats` flag
- Directory traversal with symlink support
- Structured result format for automation
- Command-line interface with clap

### Core Features

- Multi-core processing with intelligent thread pool management
- Regular expression pattern matching
- Recursive directory scanning
- Machine-readable output format
- Professional CLI with comprehensive options
