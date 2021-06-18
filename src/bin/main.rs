use sys_metrics::{cpu::*, disks::*, host::*, memory::*, network::*};

#[allow(unused_must_use)]
fn main() {
    dbg!(get_cpu_logical_count());
    dbg!(get_cpufreq());
    dbg!(get_cpustats());
    dbg!(get_cputimes());
    dbg!(get_loadavg());
    dbg!(get_physical_ioblocks());
    dbg!(get_partitions_physical());
    dbg!(get_host_info());
    dbg!(get_hostname());
    dbg!(get_os_version());
    dbg!(get_logged_users());
    dbg!(get_users());
    dbg!(get_uuid());
    dbg!(get_memory());
    dbg!(get_swap());
    dbg!(has_swap());
    dbg!(get_physical_ionets());
}
