use criterion::Criterion;
use criterion::{criterion_group, criterion_main};
use sys_metrics::host::*;

pub fn miscs_benches(c: &mut Criterion) {
    c.bench_function("get_host_info", |b| b.iter(|| get_host_info()));
    c.bench_function("get_uuid", |b| b.iter(|| get_uuid()));
    c.bench_function("get_os_version", |b| b.iter(|| get_os_version()));
    c.bench_function("get_hostname", |b| b.iter(|| get_hostname()));
    c.bench_function("get_users", |b| b.iter(|| get_users()));
}

criterion_group!(benches, miscs_benches);
criterion_main!(benches);
