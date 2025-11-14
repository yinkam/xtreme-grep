//! # File Reader
//!
//! Shared file reading approach selection logic for optimal performance
//! across different file sizes and processing contexts.

use std::path::PathBuf;

pub const BULK_READ_SIZE_THRESHOLD: u64 = 7_000_000;
pub const MEMORY_MAP_SIZE_THRESHOLD: u64 = 100_000_000;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FileReader {
    BulkRead,  // for files between 0B and 7MB
    MemoryMap, // for files between 7MB and 100MB
    Streaming, // for files larger than 100MB or multi-file contexts
}

impl FileReader {
    pub fn select(filepath: &PathBuf, is_single_file: bool) -> Self {
        if !is_single_file {
            return FileReader::Streaming;
        }

        const MEMORY_MAP_SIZE_THRESHOLD_MIN: u64 = 1 + BULK_READ_SIZE_THRESHOLD;
        match std::fs::metadata(filepath) {
            Ok(metadata) => match metadata.len() {
                0..=BULK_READ_SIZE_THRESHOLD => FileReader::BulkRead,
                MEMORY_MAP_SIZE_THRESHOLD_MIN..=MEMORY_MAP_SIZE_THRESHOLD => FileReader::MemoryMap,
                _ => FileReader::Streaming,
            },
            Err(_) => FileReader::Streaming,
        }
    }
}
