#!/usr/bin/env python3
"""Benchmark Python feedparser for comparison with feedparser-rs"""

import time
import feedparser
from pathlib import Path


FIXTURES = Path(__file__).parent.parent / "fixtures"


def benchmark_parse(name: str, data: bytes, iterations: int = 100) -> float:
    """Benchmark parsing a feed multiple times"""
    start = time.perf_counter()

    for _ in range(iterations):
        feedparser.parse(data)

    end = time.perf_counter()
    total_time = end - start
    avg_time = total_time / iterations

    return avg_time * 1000  # Convert to milliseconds


def main():
    print("Python feedparser benchmarks")
    print("=" * 60)
    print(f"feedparser version: {feedparser.__version__}")
    print()

    fixtures = [
        ("small.xml", 3),
        ("medium.xml", 24),
        ("large.xml", 237),
    ]

    for filename, size_kb in fixtures:
        filepath = FIXTURES / filename

        if not filepath.exists():
            print(f"Warning: Fixture {filename} not found, skipping")
            continue

        data = filepath.read_bytes()
        actual_size = len(data) / 1024

        # Run benchmark
        avg_ms = benchmark_parse(filename, data)

        print(f"{filename:20s} ({actual_size:6.1f} KB)")
        print(f"  Average time: {avg_ms:8.3f} ms")
        print(f"  Throughput:   {actual_size/avg_ms:8.1f} KB/ms")
        print()


if __name__ == "__main__":
    main()
