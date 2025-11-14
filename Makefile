# Makefile for xerg
.PHONY: help build test run clean release all

help:
	@echo "Available targets:"
	@echo "  help    - Show this help"
	@echo "  build   - Build the project"
	@echo "  test    - Run all tests"
	@echo "  run     - Run with sample arguments"
	@echo "  clean   - Clean build artifacts"
	@echo "  release - Build optimized release"
	@echo "  all     - Build and test"

build:
	@echo "Building..."
	cargo build

test:
	@echo "Running tests..."
	cargo test

run:
	@echo "Running xerg..."
	cargo run -- "fn main" src/

clean:
	@echo "Cleaning..."
	cargo clean

release:
	@echo "Building release..."
	cargo build --release

all: build test