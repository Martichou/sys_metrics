#[cfg(target_os = "linux")]
use crate::read_and_trim;
#[cfg(target_os = "macos")]
use crate::to_str;

use crate::cpu;
use crate::memory;
use crate::models;
use crate::sys;

#[cfg(target_os = "macos")]
use core_foundation_sys::{
    base::{kCFAllocatorDefault, CFRelease, CFTypeRef},
    string::{CFStringGetCString, CFStringRef},
};
#[cfg(target_os = "macos")]
use io_kit_sys::*;
#[cfg(target_os = "macos")]
use io_kit_sys::{kIOMasterPortDefault, keys::kIOPlatformUUIDKey, IOServiceMatching};
#[cfg(target_os = "macos")]
use libc::c_char;
#[cfg(target_os = "macos")]
use libc::{c_void, sysctl, timeval};
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
    let x = sys::get_uname()?;
    let y = match sys::sysinfo() {
        Ok(val) => val,
        Err(x) => return Err(Error::new(ErrorKind::Other, x)),
    };

    Ok(HostInfo {
        loadavg: cpu::get_loadavg_from_sysinfo(&y),
        memory: memory::get_memory_from_sysinfo(&y),
        uptime: get_uptime_from_sysinfo(&y).as_secs(),
        os_version: sys::get_os_version_from_uname(&x),
        hostname: sys::get_hostname_from_uname(&x),
    })
}

#[inline]
#[cfg(target_os = "linux")]
fn get_uptime_from_sysinfo(y: &libc::sysinfo) -> Duration {
    Duration::from_secs(std::cmp::max(y.uptime, 0) as u64)
}
