use criterion::Criterion;
use criterion::{criterion_group, criterion_main};
use sys_metrics::virt::*;

pub fn virt_benches(c: &mut Criterion) {
    c.bench_function("get_virt_info", |b| b.iter(|| get_virt_info()));
}

criterion_group!(benches, virt_benches);
criterion_main!(benches);
