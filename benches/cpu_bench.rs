use criterion::Criterion;
use criterion::{criterion_group, criterion_main};
use metrics_rs::cpu::*;

pub fn cpu_benches(c: &mut Criterion) {
    c.bench_function("get_avg_cpufreq", |b| b.iter(|| get_avg_cpufreq()));
}

criterion_group!(benches, cpu_benches);
criterion_main!(benches);
