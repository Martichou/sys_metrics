use criterion::Criterion;
use criterion::{criterion_group, criterion_main};
use metrics_rs::disks::*;

pub fn disks_benches(c: &mut Criterion) {
    c.bench_function("get_partitions_info", |b| b.iter(|| get_partitions_info()));
    c.bench_function("get_iostats", |b| b.iter(|| get_iostats()));
}

criterion_group!(benches, disks_benches);
criterion_main!(benches);
