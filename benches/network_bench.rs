use criterion::Criterion;
use criterion::{criterion_group, criterion_main};
use sys_metrics::network::*;

pub fn network_benches(c: &mut Criterion) {
    c.bench_function("get_ionets", |b| b.iter(|| get_ionets()));
    c.bench_function("get_physical_ionets", |b| b.iter(|| get_physical_ionets()));
}

criterion_group!(benches, network_benches);
criterion_main!(benches);
