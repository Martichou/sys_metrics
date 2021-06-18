use criterion::Criterion;
use criterion::{criterion_group, criterion_main};
use sys_metrics::disks::*;

pub fn disks_benches(c: &mut Criterion) {
    c.bench_function("get_partitions", |b| b.iter(|| get_partitions()));
    c.bench_function("get_partitions_physical", |b| {
        b.iter(|| get_partitions_physical())
    });
    c.bench_function("get_ioblocks", |b| b.iter(|| get_ioblocks()));
    c.bench_function("get_physical_ioblocks", |b| {
        b.iter(|| get_physical_ioblocks())
    });
}

criterion_group!(benches, disks_benches);
criterion_main!(benches);
