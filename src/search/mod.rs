//! Search functionality and related modules
//!
//! This module contains all search-related functionality including:
//! - File system crawling and traversal
//! - File reading strategies with adaptive performance
//! - Core search operations with pattern matching
//!
//! The search module uses a three-tier file reading system:
//! - Streaming for small files (<7MB)
//! - Bulk reading for medium files (7MB-100MB)  
//! - Memory mapping for large files (>100MB)

pub mod crawler;
pub mod default;
pub mod reader;
pub mod xtreme;
