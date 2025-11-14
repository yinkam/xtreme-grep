use criterion::{Criterion, black_box, criterion_group, criterion_main};
use memmap2::MmapOptions;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use tempdir::TempDir;

// Test memory mapping overhead on small files
fn create_small_test_file(temp_dir: &TempDir, size: &str) -> std::path::PathBuf {
    let file_path = temp_dir.path().join(format!("small_test_{}.txt", size));
    let base_line =
        "use std::collections::HashMap;\nfn main() {\n    println!(\"Hello world\");\n}\n";

    let content = match size {
        "tiny" => base_line.repeat(2),        // ~100B
        "micro" => base_line.repeat(5),       // ~250B
        "mini" => base_line.repeat(10),       // ~500B
        "small" => base_line.repeat(25),      // ~1.25KB
        "medium" => base_line.repeat(50),     // ~2.5KB
        "regular" => base_line.repeat(100),   // ~5KB
        "modest" => base_line.repeat(250),    // ~12.5KB
        "decent" => base_line.repeat(500),    // ~25KB
        "sizeable" => base_line.repeat(1000), // ~50KB
        "notable" => base_line.repeat(2000),  // ~100KB
        _ => "test line\n".to_string(),
    };
    std::fs::write(&file_path, content).unwrap();
    file_path
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

fn benchmark_small_file_overhead(c: &mut Criterion) {
    let temp_dir = TempDir::new("small_file_bench").unwrap();

    // Test very small files to detect memory mapping overhead
    for size in &[
        "tiny", "micro", "mini", "small", "medium", "regular", "modest", "decent", "sizeable",
        "notable",
    ] {
        let file_path = create_small_test_file(&temp_dir, size);
        let pattern = "use";
        let file_size = std::fs::metadata(&file_path).unwrap().len();

        println!(
            "Testing {} file: {} bytes ({:.2} KB)",
            size,
            file_size,
            file_size as f64 / 1024.0
        );

        let mut group = c.benchmark_group(format!("small_file_{}_{}_bytes", size, file_size));

        // Increase sample size for small files to detect micro-differences
        group.sample_size(100);

        // fs::read_to_string (current recommendation for small files)
        group.bench_function("fs_read_to_string", |b| {
            b.iter(|| bench_fs_read_to_string(black_box(&file_path), black_box(pattern)))
        });

        // BufReader bulk (equivalent to fs::read_to_string)
        group.bench_function("bufreader_bulk", |b| {
            b.iter(|| bench_bufreader_bulk(black_box(&file_path), black_box(pattern)))
        });

        // Memory mapping (test for overhead on small files)
        group.bench_function("memory_mapping", |b| {
            b.iter(|| bench_memory_map(black_box(&file_path), black_box(pattern)))
        });

        group.finish();
    }
}

criterion_group!(benches, benchmark_small_file_overhead);
criterion_main!(benches);
