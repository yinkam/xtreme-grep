use criterion::{Criterion, black_box, criterion_group, criterion_main};
use memmap2::MmapOptions;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use tempdir::TempDir;

// Focus on large file sizes where we expect to see differences
fn create_large_test_file(temp_dir: &TempDir, size: &str) -> std::path::PathBuf {
    let file_path = temp_dir.path().join(format!("large_test_{}.txt", size));
    let base_line = "use std::collections::HashMap;\nfn process_data(data: &str) -> Option<String> {\n    println!(\"Processing: {}\", data);\n    Some(data.to_string())\n}\n";

    let content = match size {
        "large" => base_line.repeat(5000),       // ~500KB
        "huge" => base_line.repeat(50000),       // ~5MB
        "massive" => base_line.repeat(250000),   // ~25MB
        "gigantic" => base_line.repeat(500000),  // ~50MB
        "enormous" => base_line.repeat(1000000), // ~100MB
        _ => "test line\n".to_string(),
    };
    std::fs::write(&file_path, content).unwrap();
    file_path
}

fn bench_bufreader_bulk(file_path: &Path, pattern: &str) {
    let file = File::open(file_path).unwrap();
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).unwrap();

    let regex = regex::Regex::new(pattern).unwrap();
    let mut match_count = 0;

    for line in contents.lines() {
        if regex.is_match(line) {
            match_count += 1;
        }
    }
    std::hint::black_box(match_count);
}

fn bench_fs_read_to_string(file_path: &Path, pattern: &str) {
    let contents = std::fs::read_to_string(file_path).unwrap();
    let regex = regex::Regex::new(pattern).unwrap();
    let mut match_count = 0;

    for line in contents.lines() {
        if regex.is_match(line) {
            match_count += 1;
        }
    }
    std::hint::black_box(match_count);
}

fn bench_memory_map(file_path: &Path, pattern: &str) {
    let file = File::open(file_path).unwrap();
    let mmap = unsafe { MmapOptions::new().map(&file).unwrap() };
    let contents = std::str::from_utf8(&mmap).unwrap();

    let regex = regex::Regex::new(pattern).unwrap();
    let mut match_count = 0;

    for line in contents.lines() {
        if regex.is_match(line) {
            match_count += 1;
        }
    }
    std::hint::black_box(match_count);
}

fn benchmark_large_files(c: &mut Criterion) {
    let temp_dir = TempDir::new("large_file_bench").unwrap();

    // Focus on large files where we expect mmap to shine
    for size in &["large", "huge", "massive", "gigantic", "enormous"] {
        let file_path = create_large_test_file(&temp_dir, size);
        let pattern = "use";
        let file_size = std::fs::metadata(&file_path).unwrap().len();

        println!(
            "Testing {} file: {} bytes ({:.2} MB)",
            size,
            file_size,
            file_size as f64 / 1_048_576.0
        );

        let mut group =
            c.benchmark_group(format!("large_file_{}_{}_mb", size, file_size / 1_048_576));

        // Set timeout for large files
        group.sample_size(10); // Reduce sample size for large files

        // BufReader bulk approach
        group.bench_function("bufreader_bulk", |b| {
            b.iter(|| bench_bufreader_bulk(black_box(&file_path), black_box(pattern)))
        });

        // fs::read_to_string approach
        group.bench_function("fs_read_to_string", |b| {
            b.iter(|| bench_fs_read_to_string(black_box(&file_path), black_box(pattern)))
        });

        // Memory mapping approach
        group.bench_function("memory_mapping", |b| {
            b.iter(|| bench_memory_map(black_box(&file_path), black_box(pattern)))
        });

        group.finish();
    }
}

criterion_group!(benches, benchmark_large_files);
criterion_main!(benches);
