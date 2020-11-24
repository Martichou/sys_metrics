use criterion::Criterion;
use criterion::{criterion_group, criterion_main};

pub fn network_benches(_c: &mut Criterion) {}

criterion_group!(benches, network_benches);
criterion_main!(benches);
