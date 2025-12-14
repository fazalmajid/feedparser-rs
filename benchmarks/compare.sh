#!/bin/bash
# Compare feedparser-rs vs Python feedparser

set -e

echo "Running feedparser-rs benchmarks (Rust)..."
cd "$(dirname "$0")/.."
cargo bench -p feedparser-rs-core

echo ""
echo "Running feedparser benchmarks (Python)..."
cd benchmarks/python
python3 -m venv .venv
source .venv/bin/activate
pip install -q -r requirements.txt
python bench_feedparser.py

echo ""
echo "Results saved to:"
echo "  - Rust: target/criterion/parse/"
echo "  - Python: (console output above)"
