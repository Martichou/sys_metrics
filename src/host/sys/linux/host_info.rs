use crate::cpu;
use crate::host::{self, HostInfo};
use crate::to_str;

use std::io::{Error, ErrorKind};
use std::time::Duration;

/// Get some basic [HostInfo] of the host.
///
/// [HostInfo]: ../host/struct.HostInfo.html
pub fn get_host_info() -> Result<HostInfo, Error> {
    let x = host::get_uname()?;
    let y = match host::sysinfo() {
        Ok(val) => val,
        Err(x) => return Err(Error::new(ErrorKind::Other, x)),
    };

    Ok(HostInfo {
        loadavg: cpu::get_loadavg_from_sysinfo(&y),
        system: to_str(x.sysname.as_ptr()).to_owned(),
        os_version: host::get_os_version_from_uname(&x),
        kernel_version: host::get_kernel_version_from_uname(&x),
        hostname: host::get_hostname_from_uname(&x),
        uptime: get_uptime_from_sysinfo(&y).as_secs(),
    })
}

#[inline]
#[cfg(target_os = "linux")]
fn get_uptime_from_sysinfo(y: &libc::sysinfo) -> Duration {
    Duration::from_secs(std::cmp::max(y.uptime, 0) as u64)
}
