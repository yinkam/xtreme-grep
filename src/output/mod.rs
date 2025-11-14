//! Output formatting and presentation modules
//!
//! This module contains all output-related functionality including:
//! - ANSI color management and terminal formatting
//! - Text highlighting with pattern matching
//! - Result formatting, statistics, and structured output
//!
//! The output module provides consistent formatting across both
//! default and xtreme search modes while maintaining performance.

pub mod colors;
pub mod highlighter;
pub mod result;
