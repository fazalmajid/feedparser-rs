#![allow(missing_docs)]

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use feedparser_rs::parse;
use std::hint::black_box;

const SMALL_FEED: &[u8] = include_bytes!("../../../benchmarks/fixtures/small.xml");
const MEDIUM_FEED: &[u8] = include_bytes!("../../../benchmarks/fixtures/medium.xml");
const LARGE_FEED: &[u8] = include_bytes!("../../../benchmarks/fixtures/large.xml");

fn bench_parse_feeds(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse");

    group.bench_with_input(BenchmarkId::new("rss", "small"), &SMALL_FEED, |b, data| {
        b.iter(|| parse(black_box(data)));
    });

    group.bench_with_input(
        BenchmarkId::new("rss", "medium"),
        &MEDIUM_FEED,
        |b, data| b.iter(|| parse(black_box(data))),
    );

    group.bench_with_input(BenchmarkId::new("rss", "large"), &LARGE_FEED, |b, data| {
        b.iter(|| parse(black_box(data)));
    });

    group.finish();
}

fn bench_detect_format(c: &mut Criterion) {
    use feedparser_rs::detect_format;

    let mut group = c.benchmark_group("detect_format");

    group.bench_with_input(
        BenchmarkId::new("detect", "small"),
        &SMALL_FEED,
        |b, data| b.iter(|| detect_format(black_box(data))),
    );

    group.bench_with_input(
        BenchmarkId::new("detect", "medium"),
        &MEDIUM_FEED,
        |b, data| b.iter(|| detect_format(black_box(data))),
    );

    group.finish();
}

criterion_group!(benches, bench_parse_feeds, bench_detect_format);
criterion_main!(benches);
