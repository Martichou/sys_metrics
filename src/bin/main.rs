use sys_metrics::{cpu::*, disks::*, host::*};

#[allow(unused_must_use)]
fn main() {
    dbg!(get_host_info());
    dbg!(get_uuid());
    dbg!(get_cpufreq());
    dbg!(get_cpustat());
    dbg!(get_partitions_physical());
    dbg!(get_iostats());
    dbg!(get_iostats_physical());
    dbg!(get_users());
    dbg!(get_cpu_logical_count());
}
