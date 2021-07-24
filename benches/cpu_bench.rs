use criterion::Criterion;
use criterion::{criterion_group, criterion_main};
use sys_metrics::cpu::*;

pub fn cpu_benches(c: &mut Criterion) {
    c.bench_function("get_cpufreq", |b| b.iter(|| get_cpufreq()));
    c.bench_function("get_cputimes", |b| b.iter(|| get_cputimes()));
    c.bench_function("get_cpustats", |b| b.iter(|| get_cpustats()));
    c.bench_function("get_loadavg", |b| b.iter(|| get_loadavg()));
    c.bench_function("get_logical_count", |b| b.iter(|| get_logical_count()));
    c.bench_function("get_physical_count", |b| b.iter(|| get_physical_count()));
}

criterion_group!(benches, cpu_benches);
criterion_main!(benches);
