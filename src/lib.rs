//! `sys_metrics` is a crate used to get a system's information.
//!
//! It attempt to provide information about:
//!
//!  * CPU
//!  * Disks
//!  * Host
//!  * Memory
//!  * Networks (wip)
//!
//! ## Quick start
//! ```
//! use sys_metrics::{cpu::*, disks::*, host::*};
//!
//! fn main() -> Result<(), std::io::Error> {
//!     let host_info = match get_host_info() {
//!         Ok(val) => val,
//!         Err(x) => return Err(x),
//!     };
//!
//!     let _uuid = get_uuid().expect("Cannot retrieve UUID");
//!     let _os = host_info.os_version;
//!     let _hostname = host_info.hostname;
//!     let _uptime = host_info.uptime;
//!     let _cpufreq = match get_cpufreq() {
//!         Ok(val) => val as i64,
//!         Err(_) => -1,
//!     };
//!     let _loadavg = host_info.loadavg;
//!     let _disks = match get_partitions_physical() {
//!         Ok(val) => Some(val),
//!         Err(_) => None,
//!     };
//!     let _iostats = match get_iostats() {
//!         Ok(val) => Some(val),
//!         Err(_) => None,
//!     };
//!     let _memory = host_info.memory;
//!     let _users = get_users();
//!
//!     Ok(())
//! }
//! ```

/// CPU information
pub mod cpu;
/// Disks information
pub mod disks;
/// Host system information
pub mod host;
/// Memory and swap information
pub mod memory;

mod models;

pub use models::*;

use libc::c_char;
use std::ffi::CStr;
#[cfg(target_os = "linux")]
use std::fs;
#[cfg(target_os = "linux")]
use std::io::Error;

// Static reference to the page_size for memory
#[cfg(target_os = "macos")]
lazy_static::lazy_static! {
    static ref PAGE_SIZE: u64 = {
        unsafe {
            libc::sysconf(libc::_SC_PAGESIZE) as u64
        }
    };
}

/// Read from path to content, trim it and return the String
#[cfg(target_os = "linux")]
pub(crate) fn read_and_trim(path: &str) -> Result<String, Error> {
    let content = fs::read_to_string(path)?;
    Ok(content.trim().to_owned())
}

#[inline]
pub(crate) fn to_str<'a>(s: *const c_char) -> &'a str {
    unsafe {
        let res = CStr::from_ptr(s).to_bytes();
        std::str::from_utf8_unchecked(res)
    }
}
