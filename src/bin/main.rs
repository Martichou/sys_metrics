use sys_metrics::{cpu::*, disks::*, miscs::*, users::*};

#[allow(unused_must_use)]
fn main() {
    let host_info = match get_host_info() {
        Ok(val) => val,
        Err(_) => return,
    };
    dbg!(host_info);

    let uuid = get_uuid();
    dbg!(uuid);
    let cpu_freq = get_cpufreq();
    dbg!(cpu_freq);
    let disks = get_partitions_physical();
    dbg!(disks);
    let iostats = get_iostats();
    dbg!(iostats);
    let users = get_users();
    dbg!(users);
}
