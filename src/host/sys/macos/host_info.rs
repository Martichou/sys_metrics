use crate::cpu;
use crate::host;
use crate::memory;
use crate::models;
use crate::to_str;

use libc::{c_void, sysctl, timeval};
use models::HostInfo;
use std::io::Error;
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

    Ok(HostInfo {
        loadavg: cpu::get_loadavg().unwrap(),
        memory: memory::get_memory()?,
        uptime: get_uptime().unwrap().as_secs(),
        system: to_str(x.sysname.as_ptr()).to_owned(),
        os_version: host::get_os_version_from_uname(&x),
        hostname: host::get_hostname_from_uname(&x),
    })
}

fn get_uptime() -> Result<Duration, Error> {
    let mut data = std::mem::MaybeUninit::<timeval>::uninit();
    let mut mib: [i32; 2] = [1, 21];

    if unsafe {
        sysctl(
            mib.as_mut_ptr(),
            mib.len() as u32,
            data.as_mut_ptr() as *mut c_void,
            &mut std::mem::size_of::<timeval>(),
            std::ptr::null_mut(),
            0,
        )
    } < 0
    {
        return Err(Error::last_os_error());
    }

    Ok(Duration::from_secs(
        unsafe { data.assume_init() }.tv_sec as u64,
    ))
}
