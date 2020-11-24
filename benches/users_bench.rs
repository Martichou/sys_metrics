use criterion::Criterion;
use criterion::{criterion_group, criterion_main};
use sys_metrics::users::*;

pub fn users_benches(c: &mut Criterion) {
    c.bench_function("get_users", |b| b.iter(|| get_users()));
}

criterion_group!(benches, users_benches);
criterion_main!(benches);
