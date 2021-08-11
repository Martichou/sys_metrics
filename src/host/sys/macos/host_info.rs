use crate::cpu;
use crate::host::{self, HostInfo};
use crate::to_str;

use libc::{c_void, sysctl, timeval};
use std::io::Error;
use std::time::Duration;

/// Get some basic [HostInfo] of the host.
///
/// [HostInfo]: ../host/struct.HostInfo.html
pub fn get_host_info() -> Result<HostInfo, Error> {
    let x = host::get_uname()?;

    Ok(HostInfo {
        loadavg: cpu::get_loadavg().unwrap(),
        system: to_str(x.sysname.as_ptr()).to_owned(),
        os_version: host::get_os_version_from_uname(&x),
        kernel_version: host::get_os_version_from_uname(&x),
        hostname: host::get_hostname_from_uname(&x),
        uptime: get_uptime().unwrap().as_secs(),
    })
}

fn get_uptime() -> Result<Duration, Error> {
    let mut data = std::mem::MaybeUninit::<timeval>::uninit();
    let mut mib: [i32; 2] = [libc::CTL_KERN, libc::KERN_BOOTTIME];

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
