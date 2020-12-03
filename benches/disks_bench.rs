use criterion::Criterion;
use criterion::{criterion_group, criterion_main};
use sys_metrics::disks::*;

pub fn disks_benches(c: &mut Criterion) {
    c.bench_function("get_partitions_info", |b| {
        b.iter(|| get_partitions_physical())
    });
    c.bench_function("get_iostats", |b| b.iter(|| get_iostats()));
    c.bench_function("get_iostats_physical", |b| b.iter(|| get_iostats_physical()));
}

criterion_group!(benches, disks_benches);
criterion_main!(benches);
