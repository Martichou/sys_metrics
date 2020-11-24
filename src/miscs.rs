#[cfg(target_os = "linux")]
use super::read_and_trim;

#[cfg(target_os = "macos")]
use crate::cpu;
#[cfg(target_os = "macos")]
use crate::memory;
use crate::models;

#[cfg(target_os = "macos")]
use core_foundation_sys::{
    base::{kCFAllocatorDefault, CFRelease, CFTypeRef},
    string::{CFStringGetCString, CFStringRef},
};
#[cfg(target_os = "macos")]
use cpu::get_loadavg;
#[cfg(target_os = "macos")]
use io_kit_sys::*;
#[cfg(target_os = "macos")]
use io_kit_sys::{kIOMasterPortDefault, keys::kIOPlatformUUIDKey, IOServiceMatching};
#[cfg(target_os = "macos")]
use libc::c_char;
#[cfg(target_os = "macos")]
use libc::{c_void, sysctl, timeval};
#[cfg(target_os = "macos")]
use memory::get_memory;
use models::HostInfo;
#[cfg(target_os = "linux")]
use models::{LoadAvg, Memory};
use nix::sys;
#[cfg(target_os = "macos")]
use std::ffi::CStr;
use std::io::{Error, ErrorKind};
#[cfg(target_os = "macos")]
use std::time::Duration;

/// Get the os version (Mac/Linux/Windows) in a safe String.
pub fn get_os_version() -> String {
    let x = sys::utsname::uname();
    x.sysname().to_owned() + "/" + x.release()
}

/// Get the hostname (Mac/Linux/Windows) in a safe String.
pub fn get_hostname() -> String {
    let x = sys::utsname::uname();
    x.nodename().to_owned()
}

/// Return the uptime of the host for macOS.
#[cfg(target_os = "macos")]
pub fn get_uptime() -> Result<Duration, Error> {
    let mut data = std::mem::MaybeUninit::<timeval>::uninit();
    let mib = [1, 21];

    let ret = unsafe {
        sysctl(
            &mib[0] as *const _ as *mut _,
            mib.len() as u32,
            &mut data as *mut _ as *mut c_void,
            &mut std::mem::size_of::<timeval>(),
            std::ptr::null_mut(),
            0,
        )
    };

    if ret < 0 {
        Err(Error::new(ErrorKind::Other, "Invalid return for sysctl"))
    } else {
        let data = unsafe { data.assume_init() };
        Ok(Duration::from_secs(data.tv_sec as u64))
    }
}

/// Get both hostname and os_version from the same single uname instance.
#[cfg(target_os = "linux")]
pub fn get_host_info() -> Result<HostInfo, Error> {
    let x = sys::utsname::uname();
    let y = match sys::sysinfo::sysinfo() {
        Ok(val) => val,
        Err(x) => return Err(Error::new(ErrorKind::Other, x)),
    };
    let uptime = y.uptime().as_secs() as i64;
    let loadavg_raw = y.load_average();
    let loadavg = LoadAvg {
        one: loadavg_raw.0,
        five: loadavg_raw.1,
        fifteen: loadavg_raw.2,
    };
    let memory = Memory {
        total_virt: y.ram_total() as i64,
        total_swap: y.swap_total() as i64,
        avail_virt: y.ram_unused() as i64,
        avail_swap: y.swap_free() as i64,
    };

    Ok(HostInfo {
        loadavg,
        memory,
        uptime,
        os_version: x.sysname().to_owned() + "/" + x.release(),
        hostname: x.nodename().to_owned(),
    })
}

/// Get both hostname and os_version from the same single uname instance.
#[cfg(target_os = "macos")]
pub fn get_host_info() -> Result<HostInfo, Error> {
    let x = sys::utsname::uname();
    let uptime = get_uptime().unwrap().as_secs() as i64;
    let loadavg = get_loadavg().unwrap();
    let memory = unsafe { get_memory() }?;

    Ok(HostInfo {
        loadavg,
        memory,
        uptime,
        os_version: x.sysname().to_owned() + "/" + x.release(),
        hostname: x.nodename().to_owned(),
    })
}

/// Get the machine UUID (Linux) as a String.
/// LINUX => Will read it from /etc/machine-id or /var/lib/dbus/machine-id.
#[cfg(target_os = "linux")]
pub fn get_uuid() -> Result<String, Error> {
    match read_and_trim("/etc/machine-id") {
        Ok(machine_id) => Ok(machine_id),
        Err(_) => Ok(read_and_trim("/var/lib/dbus/machine-id")?),
    }
}

/// Get the machine Serial Number (macOS) as a String.
/// macOS => Will get it from some black magic extern C function.
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

        match CStr::from_ptr(buffer.as_mut_ptr()).to_str() {
            Ok(val) => Ok(val.to_owned()),
            Err(x) => Err(Error::new(ErrorKind::Other, x)),
        }
    }
}
