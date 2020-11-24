pub mod cpu;
pub mod disks;
pub mod memory;
pub mod miscs;
pub mod models;
pub mod network;
pub mod users;

#[cfg(target_os = "macos")]
use mach::vm_types::integer_t;
use std::{fs, io::Error};

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
pub fn read_and_trim(path: &str) -> Result<String, Error> {
    let content = fs::read_to_string(path)?;
    Ok(content.trim().to_owned())
}

/// Detect if a filesysteme is for a physical drive or not.
/// This is not 100% true, but it's true enough for me.
pub fn is_physical_filesys(filesysteme: &str) -> bool {
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
