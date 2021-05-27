use criterion::Criterion;
use criterion::{criterion_group, criterion_main};
use sys_metrics::network::*;

pub fn network_benches(c: &mut Criterion) {
    c.bench_function("get_net_iocounters", |b| b.iter(|| get_net_iocounters()));
    c.bench_function("get_net_physical_iocounters", |b| {
        b.iter(|| get_net_physical_iocounters())
    });
}

criterion_group!(benches, network_benches);
criterion_main!(benches);
