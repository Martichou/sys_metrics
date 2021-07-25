mod sys;

pub use sys::*;

use libc::statvfs;
use serde::Serialize;
use std::ffi::CString;
use std::io::Error;

/// Struct containing a disk' information.
#[derive(Debug, Clone, Serialize)]
pub struct Disks {
    pub name: String,
    pub mount_point: String,
    /// Value is in MB
    pub total_space: u64,
    /// Value is in MB
    pub avail_space: u64,
}

/// Struct containing a disk_io (bytes read/wrtn) information.
#[derive(Debug, Clone, Serialize)]
pub struct IoBlock {
    pub device_name: String,
    pub read_count: u64,
    pub read_bytes: u64,
    pub write_count: u64,
    pub write_bytes: u64,
    pub busy_time: u64,
}

/// Return the total/free space of a Disk from it's path (mount_point).
pub fn disk_usage<P>(path: P) -> Result<(u64, u64), Error>
where
    P: AsRef<[u8]>,
{
    let mut statvfs = std::mem::MaybeUninit::<statvfs>::uninit();

    if unsafe { libc::statvfs(CString::new(path.as_ref())?.as_ptr(), statvfs.as_mut_ptr()) } == -1 {
        return Err(Error::last_os_error());
    }

    let statvfs = unsafe { statvfs.assume_init() };
    let total = statvfs.f_blocks as u64 * statvfs.f_frsize as u64;
    let free = statvfs.f_bavail as u64 * statvfs.f_frsize as u64;

    Ok((total, free))
}

/// Detect if a filesysteme is for a physical drive or not.
/// This is not 100% true, but it's true enough for me.
/// The better approach would be to read the filesystems from /proc/filesystems
/// maybe construct a lazy_static array of filesystems.
pub(crate) fn is_physical_filesys(filesysteme: &str) -> bool {
    matches!(
        filesysteme,
        "ext2"
            | "ext3"
            | "ext4"
            | "vfat"
            | "ntfs"
            | "zfs"
            | "hfs"
            | "reiserfs"
            | "reiser4"
            | "exfat"
            | "f2fs"
            | "hfsplus"
            | "jfs"
            | "btrfs"
            | "minix"
            | "nilfs"
            | "xfs"
            | "apfs"
            | "fuseblk"
    )
}
