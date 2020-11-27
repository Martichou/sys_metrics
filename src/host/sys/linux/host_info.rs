use crate::cpu;
use crate::host;
use crate::memory;
use crate::models;

use models::HostInfo;
use std::io::{Error, ErrorKind};
use std::time::Duration;

/// Get some basic [HostInfo] of the host.
///
/// On linux and macOS it will get the `os_version` and `hostname` from uname.
///
/// For the `uptime`/`loadavg`/`memory` on linux it will get them from sysinfo.
/// But on macOS it will use the crate [get_loadavg] and [get_memory] and a special get_uptime function using an unsafe syscall.
///
/// [get_loadavg]: ../cpu/fn.get_loadavg.html
/// [get_memory]: ../memory/fn.get_memory.html
/// [HostInfo]: ../struct.HostInfo.html
pub fn get_host_info() -> Result<HostInfo, Error> {
    let x = host::get_uname()?;
    let y = match host::sysinfo() {
        Ok(val) => val,
        Err(x) => return Err(Error::new(ErrorKind::Other, x)),
    };

    Ok(HostInfo {
        loadavg: cpu::get_loadavg_from_sysinfo(&y),
        memory: memory::get_memory_from_sysinfo(&y),
        uptime: get_uptime_from_sysinfo(&y).as_secs(),
        os_version: host::get_os_version_from_uname(&x),
        hostname: host::get_hostname_from_uname(&x),
    })
}

#[inline]
#[cfg(target_os = "linux")]
fn get_uptime_from_sysinfo(y: &libc::sysinfo) -> Duration {
    Duration::from_secs(std::cmp::max(y.uptime, 0) as u64)
}
