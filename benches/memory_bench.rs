use criterion::Criterion;
use criterion::{criterion_group, criterion_main};
use sys_metrics::memory::*;

pub fn memory_benches(c: &mut Criterion) {
    #[cfg(target_os = "macos")]
    c.bench_function("get_memory", |b| b.iter(|| get_memory()));
    #[cfg(target_os = "linux")]
    c.bench_function("get_memory", |b| b.iter(|| get_memory()));
}

criterion_group!(benches, memory_benches);
criterion_main!(benches);
