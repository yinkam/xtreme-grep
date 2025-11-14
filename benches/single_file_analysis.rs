use criterion::{Criterion, black_box, criterion_group, criterion_main};
use memmap2::MmapOptions;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;
use tempdir::TempDir;
use xerg::output::colors::Color;
use xerg::search::default::search_files;

// Test different file reading strategies for single-file optimization

fn create_test_file(temp_dir: &TempDir, size: &str) -> std::path::PathBuf {
    let file_path = temp_dir.path().join(format!("test_{}.txt", size));
    let base_line =
        "use std::collections::HashMap;\nfn main() {\n    println!(\"Hello world\");\n}\n";

    let content = match size {
        "tiny" => base_line.repeat(5),           // ~200B
        "small" => base_line.repeat(25),         // ~1KB
        "medium" => base_line.repeat(250),       // ~10KB
        "large" => base_line.repeat(2500),       // ~100KB
        "huge" => base_line.repeat(25000),       // ~1MB
        "massive" => base_line.repeat(250000),   // ~10MB
        "gigantic" => base_line.repeat(1250000), // ~50MB
        "enormous" => base_line.repeat(2500000), // ~100MB
        _ => "test line\n".to_string(),
    };
    std::fs::write(&file_path, content).unwrap();
    file_path
}

fn bench_current_approach(file_path: &Path, pattern: &str) {
    // Current approach: BufReader with line-by-line streaming
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let regex = regex::Regex::new(pattern).unwrap();
    let mut match_count = 0;

    for line in reader.lines() {
        if let Ok(line) = line {
            if regex.is_match(&line) {
                match_count += 1;
            }
        }
    }
    // Return count to prevent optimization away
    std::hint::black_box(match_count);
}

fn bench_bufreader_bulk(file_path: &Path, pattern: &str) {
    // BufReader with bulk read_to_string (not streaming)
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
    // Return count to prevent optimization away
    std::hint::black_box(match_count);
}

fn bench_memory_map(file_path: &Path, pattern: &str) {
    // Memory mapping approach
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
    // Return count to prevent optimization away
    std::hint::black_box(match_count);
}

fn bench_read_to_string(file_path: &Path, pattern: &str) {
    // Alternative: read entire file to string
    let contents = std::fs::read_to_string(file_path).unwrap();
    let regex = regex::Regex::new(pattern).unwrap();
    let mut match_count = 0;

    for line in contents.lines() {
        if regex.is_match(line) {
            match_count += 1;
        }
    }
    // Return count to prevent optimization away
    std::hint::black_box(match_count);
}

fn bench_direct_search(file_path: &Path, pattern: &str) {
    // Simple string search without regex
    let contents = std::fs::read_to_string(file_path).unwrap();
    let mut match_count = 0;

    for line in contents.lines() {
        if line.contains(pattern) {
            match_count += 1;
        }
    }
    // Return count to prevent optimization away
    std::hint::black_box(match_count);
}

fn benchmark_single_file_strategies(c: &mut Criterion) {
    let temp_dir = TempDir::new("single_file_bench").unwrap();

    // Test across different file sizes to find optimal thresholds
    // Extended range: 200B to 100MB to test memory mapping advantages
    for size in &[
        "tiny", "small", "medium", "large", "huge", "massive", "gigantic", "enormous",
    ] {
        let file_path = create_test_file(&temp_dir, size);
        let pattern = "use";
        let file_size = std::fs::metadata(&file_path).unwrap().len();

        let mut group = c.benchmark_group(format!("single_file_{}_{}_bytes", size, file_size));

        // Current xerg approach (BufReader line-by-line streaming)
        group.bench_function("bufreader_streaming", |b| {
            b.iter(|| bench_current_approach(black_box(&file_path), black_box(pattern)))
        });

        // BufReader with bulk read (not streaming)
        group.bench_function("bufreader_bulk_read", |b| {
            b.iter(|| bench_bufreader_bulk(black_box(&file_path), black_box(pattern)))
        });

        // Direct fs::read_to_string + Regex
        group.bench_function("fs_read_to_string_regex", |b| {
            b.iter(|| bench_read_to_string(black_box(&file_path), black_box(pattern)))
        });

        // Memory mapping approach (should excel at large files)
        group.bench_function("memory_mapping_regex", |b| {
            b.iter(|| bench_memory_map(black_box(&file_path), black_box(pattern)))
        });

        // read_to_string + simple contains (fastest possible)
        group.bench_function("fs_read_to_string_contains", |b| {
            b.iter(|| bench_direct_search(black_box(&file_path), black_box(pattern)))
        });

        group.finish();
    }
}

// Test thread pool overhead
fn bench_with_without_threading(c: &mut Criterion) {
    let temp_dir = TempDir::new("threading_bench").unwrap();
    let file_path = create_test_file(&temp_dir, "small");
    let files = vec![file_path.clone()];
    let pattern = "use";
    let color = Color::Red;

    let mut group = c.benchmark_group("threading_overhead");

    // Current xerg with threading
    group.bench_function("with_threading", |b| {
        b.iter(|| {
            let rx = search_files(
                black_box(&files),
                black_box(pattern),
                black_box(&color),
                false,
            );
            while rx.recv().is_ok() {}
        })
    });

    // Direct single-file processing (to be implemented)
    group.bench_function("direct_processing", |b| {
        b.iter(|| bench_current_approach(black_box(&file_path), black_box(pattern)))
    });

    group.finish();
}

// Test memory usage patterns across file sizes
fn bench_memory_usage_patterns(c: &mut Criterion) {
    let temp_dir = TempDir::new("memory_bench").unwrap();

    let mut group = c.benchmark_group("memory_pressure_analysis");

    // Test at memory pressure points - extended range
    let test_cases = vec![
        ("1kb", 1024),
        ("10kb", 10 * 1024),
        ("100kb", 100 * 1024),
        ("1mb", 1024 * 1024),
        ("10mb", 10 * 1024 * 1024),
        ("50mb", 50 * 1024 * 1024),
        ("100mb", 100 * 1024 * 1024),
    ];

    for (name, target_size) in test_cases {
        // Create file of specific size
        let file_path = temp_dir.path().join(format!("test_{}.txt", name));
        let line = "use std::collections::HashMap; // this is a test line with some content\n";
        let lines_needed = target_size / line.len();
        let content = line.repeat(lines_needed);
        std::fs::write(&file_path, content).unwrap();

        let actual_size = std::fs::metadata(&file_path).unwrap().len();
        let bench_name = format!("memory_test_{}_{}_bytes", name, actual_size);

        // Test read_to_string memory allocation
        group.bench_function(&format!("{}_read_to_string", bench_name), |b| {
            b.iter(|| {
                let _contents = std::fs::read_to_string(black_box(&file_path)).unwrap();
                // Measure allocation + deallocation time
            })
        });

        // Test BufReader streaming approach
        group.bench_function(&format!("{}_bufreader_stream", bench_name), |b| {
            b.iter(|| {
                let file = File::open(black_box(&file_path)).unwrap();
                let reader = BufReader::new(file);
                let mut _line_count = 0;
                for line in reader.lines() {
                    if line.is_ok() {
                        _line_count += 1;
                    }
                }
            })
        });

        // Test memory mapping approach (should use minimal memory)
        group.bench_function(&format!("{}_memory_mapping", bench_name), |b| {
            b.iter(|| {
                let file = File::open(black_box(&file_path)).unwrap();
                let mmap = unsafe { MmapOptions::new().map(&file).unwrap() };
                let _contents_view = std::str::from_utf8(&mmap).unwrap();
                // Memory mapping doesn't allocate, just maps
            })
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    benchmark_single_file_strategies,
    bench_with_without_threading,
    bench_memory_usage_patterns
);
criterion_main!(benches);
