use sys_metrics::{cpu::*, disks::*, host::*, memory::*};

#[allow(unused_must_use)]
fn main() {
    dbg!(get_host_info());
    dbg!(get_uuid());
    dbg!(get_cpufreq());
    dbg!(get_cpustats());
    dbg!(get_partitions_physical());
    dbg!(get_iostats());
    dbg!(get_iostats_physical());
    dbg!(get_users());
    dbg!(get_cpu_logical_count());
    dbg!(get_memory());
    dbg!(get_swap());
}
