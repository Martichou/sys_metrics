use criterion::Criterion;
use criterion::{criterion_group, criterion_main};
use sys_metrics::memory::*;

pub fn memory_benches(c: &mut Criterion) {
    c.bench_function("get_memory", |b| b.iter(|| get_memory()));
    c.bench_function("get_swap", |b| b.iter(|| get_swap()));
    c.bench_function("has_swap", |b| b.iter(|| has_swap()));
}

criterion_group!(benches, memory_benches);
criterion_main!(benches);
