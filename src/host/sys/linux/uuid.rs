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

/// Get the machine UUID of the host.
///
/// On linux it will read it from /etc/machine-id or /var/lib/dbus/machine-id.
///
/// On macOS it will use unsafe call to OSX specific function.
pub fn get_uuid() -> Result<String, Error> {
    match read_and_trim("/etc/machine-id") {
        Ok(machine_id) => Ok(machine_id),
        Err(_) => Ok(read_and_trim("/var/lib/dbus/machine-id")?),
    }
}
