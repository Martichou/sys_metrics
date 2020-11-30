mod sys;

pub use sys::*;

use libc::statvfs;
use std::ffi::CString;
use std::io::Error;

/// Return the total/free space of a Disk from it's path (mount_point).
pub(crate) fn disk_usage<P>(path: P) -> Result<(u64, u64), Error>
where
    P: AsRef<[u8]>,
{
    let mut statvfs = std::mem::MaybeUninit::<statvfs>::uninit();

    if unsafe { libc::statvfs(CString::new(path.as_ref())?.as_ptr(), statvfs.as_mut_ptr()) } == -1 {
        return Err(Error::last_os_error());
    }

    let statvfs = unsafe { statvfs.assume_init() };
    let total = statvfs.f_blocks as u64 * statvfs.f_bsize as u64;
    let free = statvfs.f_bavail as u64 * statvfs.f_bsize as u64;

    Ok((total, free))
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
