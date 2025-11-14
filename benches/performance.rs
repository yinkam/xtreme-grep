use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use tempdir::TempDir;

// Import our modules
use xerg::output::colors::Color;
use xerg::search::crawler::get_files;
use xerg::search::default::search_files;
use xerg::search::xtreme::search_files as search_files_xtreme;

/// Create test files of different sizes for benchmarking
fn create_test_files(temp_dir: &TempDir) -> Vec<(String, PathBuf)> {
    let mut test_files = Vec::new();

    // Small file: ~1KB
    let small_file = temp_dir.path().join("small.txt");
    let mut file = File::create(&small_file).unwrap();
    for i in 0..50 {
        writeln!(
            file,
            "Line {} with some text to match against pattern search",
            i
        )
        .unwrap();
    }
    test_files.push(("small_1kb".to_string(), small_file));

    // Medium file: ~100KB
    let medium_file = temp_dir.path().join("medium.txt");
    let mut file = File::create(&medium_file).unwrap();
    for i in 0..5000 {
        writeln!(
            file,
            "Line {} with various content including patterns and text for searching through",
            i
        )
        .unwrap();
    }
    test_files.push(("medium_100kb".to_string(), medium_file));

    // Large file: ~1MB
    let large_file = temp_dir.path().join("large.txt");
    let mut file = File::create(&large_file).unwrap();
    for i in 0..50000 {
        writeln!(file, "Line {} contains different types of content with patterns embedded for comprehensive testing", i).unwrap();
    }
    test_files.push(("large_1mb".to_string(), large_file));

    // Code-like file with realistic patterns
    let code_file = temp_dir.path().join("code.rs");
    let mut file = File::create(&code_file).unwrap();
    for i in 0..1000 {
        writeln!(file, "fn function_{}() {{", i).unwrap();
        writeln!(file, "    let variable = {};", i).unwrap();
        writeln!(file, "    use std::collections::HashMap;").unwrap();
        writeln!(file, "    println!(\"Debug message {}\");", i).unwrap();
        writeln!(file, "}}").unwrap();
        writeln!(file, "").unwrap();
    }
    test_files.push(("code_rust".to_string(), code_file));

    test_files
}

/// Benchmark our channel-based search
fn bench_xerg_regular(files: &[PathBuf], pattern: &str) {
    let rx = search_files(files, pattern, &Color::Blue, false);
    // Consume all results
    while rx.recv().is_ok() {}
}

/// Benchmark our xtreme mode
fn bench_xerg_xtreme(files: &[PathBuf], pattern: &str) {
    // Capture stdout to avoid polluting benchmark output
    let _result = search_files_xtreme(files, pattern, &Color::Blue, false);
}

/// Benchmark system grep for comparison
fn bench_system_grep(file: &PathBuf, pattern: &str) -> bool {
    let output = Command::new("grep").arg(pattern).arg(file).output();

    match output {
        Ok(result) => result.status.success(),
        Err(_) => false, // grep not available
    }
}

/// Run comprehensive benchmarks
fn benchmark_search_performance(c: &mut Criterion) {
    let temp_dir = TempDir::new("xerg_bench").unwrap();
    let test_files = create_test_files(&temp_dir);

    let patterns = vec![
        ("common_word", "text"),       // High match count
        ("specific_term", "function"), // Medium match count
        ("rare_pattern", "xyz123"),    // Low/no matches
        ("regex_pattern", r"\d+"),     // Regex complexity
    ];

    let mut group = c.benchmark_group("search_comparison");

    for (file_desc, file_path) in &test_files {
        for (pattern_desc, pattern) in &patterns {
            let bench_id = format!("{}_{}", file_desc, pattern_desc);

            // Benchmark our regular mode
            group.bench_with_input(
                BenchmarkId::new("xerg_regular", &bench_id),
                &(vec![file_path.clone()], pattern),
                |b, (files, pattern)| {
                    b.iter(|| bench_xerg_regular(black_box(files), black_box(pattern)))
                },
            );

            // Benchmark our xtreme mode
            group.bench_with_input(
                BenchmarkId::new("xerg_xtreme", &bench_id),
                &(vec![file_path.clone()], pattern),
                |b, (files, pattern)| {
                    b.iter(|| bench_xerg_xtreme(black_box(files), black_box(pattern)))
                },
            );

            // Benchmark system grep (if available)
            if bench_system_grep(file_path, pattern) {
                group.bench_with_input(
                    BenchmarkId::new("system_grep", &bench_id),
                    &(file_path, pattern),
                    |b, (file, pattern)| {
                        b.iter(|| bench_system_grep(black_box(file), black_box(pattern)))
                    },
                );
            }
        }
    }

    group.finish();
}

/// Benchmark different file reading strategies
fn benchmark_file_reading_strategies(c: &mut Criterion) {
    let temp_dir = TempDir::new("xerg_bench_io").unwrap();
    let test_files = create_test_files(&temp_dir);

    let mut group = c.benchmark_group("file_reading_strategies");

    for (file_desc, file_path) in &test_files {
        let pattern = "text";

        // Current line-by-line approach
        group.bench_with_input(
            BenchmarkId::new("line_by_line", file_desc),
            &(file_path, pattern),
            |b, (file, pattern)| {
                b.iter(|| {
                    use std::fs::File;
                    use std::io::{BufRead, BufReader};

                    let file = File::open(black_box(file)).unwrap();
                    let reader = BufReader::new(file);
                    let mut matches = 0;

                    for line_result in reader.lines() {
                        if let Ok(line) = line_result {
                            if line.contains(black_box(pattern)) {
                                matches += 1;
                            }
                        }
                    }
                    matches
                })
            },
        );

        // Read entire file at once
        group.bench_with_input(
            BenchmarkId::new("read_to_string", file_desc),
            &(file_path, pattern),
            |b, (file, pattern)| {
                b.iter(|| {
                    let contents = std::fs::read_to_string(black_box(file)).unwrap();
                    let matches = contents
                        .lines()
                        .filter(|line| line.contains(black_box(pattern)))
                        .count();
                    matches
                })
            },
        );
    }

    group.finish();
}

// Head-to-head comparison: xerg vs system grep
fn bench_head_to_head_comparison(c: &mut Criterion) {
    let temp_dir = TempDir::new("xerg_bench").unwrap();
    let test_files = create_test_files(&temp_dir);

    // Create a more realistic directory structure with multiple files
    let multi_dir = temp_dir.path().join("multi_test");
    std::fs::create_dir_all(&multi_dir).unwrap();

    // Create nested directories with multiple files
    for i in 0..5 {
        let sub_dir = multi_dir.join(format!("subdir_{}", i));
        std::fs::create_dir_all(&sub_dir).unwrap();

        for j in 0..3 {
            let file_path = sub_dir.join(format!("test_file_{}.rs", j));
            let content = format!(
                "fn main() {{\n    println!(\"Hello from file {} in dir {}\");\n    function_call();\n}}\n\nfn function_call() {{\n    // Some code here\n}}\n",
                j, i
            );
            std::fs::write(&file_path, content).unwrap();
        }
    }

    let mut group = c.benchmark_group("head_to_head");
    group.sample_size(20);

    // Single file tests
    if let Some((_, file_path)) = test_files.iter().find(|(name, _)| name == "code_rust") {
        let pattern = "function";

        group.bench_function("single_file/xerg_regular", |b| {
            b.iter(|| bench_xerg_regular(&vec![file_path.clone()], pattern))
        });

        group.bench_function("single_file/xerg_xtreme", |b| {
            b.iter(|| bench_xerg_xtreme(&vec![file_path.clone()], pattern))
        });

        group.bench_function("single_file/system_grep", |b| {
            b.iter(|| {
                let output = std::process::Command::new("grep")
                    .arg("-n")
                    .arg(pattern)
                    .arg(file_path)
                    .output()
                    .expect("Failed to execute grep");
                output.stdout.len() > 0
            })
        });
    }

    // Multi-directory tests - this is where xerg should shine!
    let pattern = "function";

    group.bench_function("multi_dir/xerg_regular", |b| {
        b.iter(|| {
            // Use actual xerg directory search
            let files = get_files(&multi_dir);
            bench_xerg_regular(&files, pattern)
        })
    });

    group.bench_function("multi_dir/xerg_xtreme", |b| {
        b.iter(|| {
            // Use actual xerg directory search
            let files = get_files(&multi_dir);
            bench_xerg_xtreme(&files, pattern)
        })
    });

    group.bench_function("multi_dir/system_grep_recursive", |b| {
        b.iter(|| {
            let output = std::process::Command::new("grep")
                .arg("-rn") // Recursive + line numbers
                .arg(pattern)
                .arg(&multi_dir)
                .output()
                .expect("Failed to execute grep");
            output.stdout.len() > 0
        })
    });

    // Test on a real project directory (src/) - most realistic benchmark
    group.bench_function("real_project/xerg_xtreme", |b| {
        b.iter(|| {
            let src_dir = std::path::PathBuf::from("src/");
            if src_dir.exists() {
                let files = get_files(&src_dir);
                bench_xerg_xtreme(&files, "use");
            }
        })
    });

    group.bench_function("real_project/system_grep_recursive", |b| {
        b.iter(|| {
            let output = std::process::Command::new("grep")
                .arg("-rn")
                .arg("use")
                .arg("src/")
                .output()
                .expect("Failed to execute grep");
            output.stdout.len() > 0
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_search_performance,
    benchmark_file_reading_strategies,
    bench_head_to_head_comparison
);
criterion_main!(benches);
