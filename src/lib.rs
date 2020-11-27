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

use libc::{c_char, statvfs, PATH_MAX};
#[cfg(target_os = "macos")]
use mach::vm_types::integer_t;
use std::ffi::CStr;
#[cfg(target_os = "linux")]
use std::fs;
use std::io::{Error, ErrorKind};
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

#[allow(non_camel_case_types)]
#[cfg(target_os = "macos")]
type host_flavor_t = integer_t;
#[allow(non_camel_case_types)]
#[cfg(target_os = "macos")]
type host_info64_t = *mut integer_t;

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

/// Detect if a filesysteme is for a physical drive or not.
/// This is not 100% true, but it's true enough for me.
pub(crate) fn is_physical_filesys(filesysteme: &str) -> bool {
    match filesysteme {
        "ext2" => true,
        "ext3" => true,
        "ext4" => true,
        "vfat" => true,
        "ntfs" => true,
        "zfs" => true,
        "hfs" => true,
        "reiserfs" => true,
        "reiser4" => true,
        "exfat" => true,
        "f2fs" => true,
        "hfsplus" => true,
        "jfs" => true,
        "btrfs" => true,
        "minix" => true,
        "nilfs" => true,
        "xfs" => true,
        "apfs" => true,
        "fuseblk" => true,
        _ => false,
    }
}

/// Return the total/free space of a Disk from it's path (mount_point).
pub(crate) fn disk_usage<P>(path: P) -> Result<(u64, u64), Error>
where
    P: AsRef<Path>,
{
    let mut statvfs = std::mem::MaybeUninit::<statvfs>::uninit();

    let mpath = path.as_ref().as_os_str().as_bytes();
    if mpath.len() >= PATH_MAX as usize {
        return Err(Error::new(ErrorKind::Other, "Invalid path lenght"));
    }
    let mut buf = [0u8; PATH_MAX as usize];

    unsafe {
        std::ptr::copy_nonoverlapping(mpath.as_ptr(), buf.as_mut_ptr(), mpath.len());
        if libc::statvfs(
            CStr::from_ptr(buf.as_ptr() as *const c_char).as_ptr(),
            statvfs.as_mut_ptr(),
        ) == -1
        {
            return Err(Error::last_os_error());
        }
    }

    let statvfs = unsafe { statvfs.assume_init() };
    let total = statvfs.f_blocks as u64 * statvfs.f_bsize as u64;
    let free = statvfs.f_bavail as u64 * statvfs.f_bsize as u64;

    Ok((total, free))
}

#[allow(dead_code)]
#[inline]
pub(crate) fn to_str_mut<'a>(s: *mut c_char) -> &'a str {
    unsafe {
        let res = CStr::from_ptr(s).to_bytes();
        std::str::from_utf8_unchecked(res)
    }
}

#[inline]
pub(crate) fn to_str<'a>(s: *const c_char) -> &'a str {
    unsafe {
        let res = CStr::from_ptr(s).to_bytes();
        std::str::from_utf8_unchecked(res)
    }
}
