#[cfg(target_os = "linux")]
use super::read_and_trim;
#[cfg(target_os = "macos")]
use super::to_str;

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

/// Return the uptime of the host for macOS.
#[cfg(target_os = "macos")]
fn get_uptime() -> Result<Duration, Error> {
    let mut data = std::mem::MaybeUninit::<timeval>::uninit();
    let mib = [1, 21];

    if unsafe {
        sysctl(
            &mib[0] as *const _ as *mut _,
            mib.len() as u32,
            &mut data as *mut _ as *mut c_void,
            &mut std::mem::size_of::<timeval>(),
            std::ptr::null_mut(),
            0,
        )
    } < 0
    {
        return Err(Error::last_os_error());
    }

    let data = unsafe { data.assume_init() };
    Ok(Duration::from_secs(data.tv_sec as u64))
}

#[inline]
#[cfg(target_os = "linux")]
fn get_uptime_from_sysinfo(y: &libc::sysinfo) -> Duration {
    Duration::from_secs(std::cmp::max(y.uptime, 0) as u64)
}

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
#[cfg(target_os = "linux")]
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

#[cfg(target_os = "macos")]
pub fn get_host_info() -> Result<HostInfo, Error> {
    let x = sys::get_uname()?;

    Ok(HostInfo {
        loadavg: cpu::get_loadavg().unwrap(),
        memory: memory::get_memory()?,
        uptime: get_uptime().unwrap().as_secs(),
        os_version: sys::get_os_version_from_uname(&x),
        hostname: sys::get_hostname_from_uname(&x),
    })
}

/// Get the machine UUID of the host.
///
/// On linux it will read it from /etc/machine-id or /var/lib/dbus/machine-id.
///
/// On macOS it will use unsafe call to OSX specific function.
#[cfg(target_os = "linux")]
pub fn get_uuid() -> Result<String, Error> {
    match read_and_trim("/etc/machine-id") {
        Ok(machine_id) => Ok(machine_id),
        Err(_) => Ok(read_and_trim("/var/lib/dbus/machine-id")?),
    }
}

#[cfg(target_os = "macos")]
pub fn get_uuid() -> Result<String, Error> {
    #[allow(unused_assignments)]
    let uuid: CFStringRef;

    unsafe {
        let platform_expert = IOServiceGetMatchingService(
            kIOMasterPortDefault,
            IOServiceMatching(b"IOPlatformExpertDevice\0".as_ptr() as *const c_char),
        );
        if platform_expert != 0 {
            let uuid_ascfstring: CFTypeRef = IORegistryEntryCreateCFProperty(
                platform_expert,
                CFSTR(kIOPlatformUUIDKey),
                kCFAllocatorDefault,
                0,
            );
            if !uuid_ascfstring.is_null() {
                uuid = uuid_ascfstring as CFStringRef;
            } else {
                return Err(Error::new(ErrorKind::Other, "Cannot get uuid_ascfstring"));
            }
            IOObjectRelease(platform_expert);
        } else {
            return Err(Error::last_os_error());
        }

        let mut buffer = [0i8; 37];
        if CFStringGetCString(uuid, buffer.as_mut_ptr(), 37, 134217984) == 0 {
            return Err(Error::new(ErrorKind::Other, "Cannot get the buffer filled"));
        }
        CFRelease(uuid as *mut c_void);

        Ok(to_str(buffer.as_ptr()).to_owned())
    }
}
