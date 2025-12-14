use criterion::{Criterion, black_box, criterion_group, criterion_main};
use feedparser_rs_core::parse;

fn benchmark_parse(_c: &mut Criterion) {
    // TODO: Implement benchmarks in Phase 2
}

criterion_group!(benches, benchmark_parse);
criterion_main!(benches);
