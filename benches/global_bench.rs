use criterion::Criterion;
use criterion::{criterion_group, criterion_main};
use sys_metrics::{cpu::*, disks::*, host::*, users::*};

fn all_gather() {
    let _host_info = match get_host_info() {
        Ok(val) => val,
        Err(_) => return,
    };

    {
        let _uuid = get_uuid();
        let _cpu_freq = get_cpufreq();
        let _disks = get_partitions_physical();
        let _iostats = get_iostats();
        let _users = get_users();
    };
}

pub fn global_benche(c: &mut Criterion) {
    c.bench_function("all_gather", |b| b.iter(|| all_gather()));
}

criterion_group!(benches, global_benche);
criterion_main!(benches);
