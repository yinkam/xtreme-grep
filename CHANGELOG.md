# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
