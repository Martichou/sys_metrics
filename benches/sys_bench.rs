use criterion::Criterion;
use criterion::{criterion_group, criterion_main};
use sys_metrics::sys::*;

pub fn miscs_benches(c: &mut Criterion) {
    c.bench_function("get_os_version", |b| b.iter(|| get_os_version()));
    c.bench_function("get_hostname", |b| b.iter(|| get_hostname()));
}

criterion_group!(benches, miscs_benches);
criterion_main!(benches);
