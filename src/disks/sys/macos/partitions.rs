use crate::disks::{disk_usage, is_physical_filesys, Disks};
use crate::to_str;

use libc::statfs;
use std::io::Error;

extern "C" {
    fn getfsstat64(buf: *mut statfs, bufsize: libc::c_int, flags: libc::c_int) -> libc::c_int;
}

/// Return a Vec of [Disks] with their minimal informations.
///
/// Contains `name`, `mount_point` and `total`/`free` space.
///
/// On linux it will get them from `/proc/mounts`.
///
/// On macOS it will use an unsafe call to `getfsstat64`.
///
/// [Disks]: ../disks/struct.Disks.html
pub fn get_partitions_physical() -> Result<Vec<Disks>, Error> {
    let expected_len = unsafe { getfsstat64(std::ptr::null_mut(), 0, 2) };
    let mut mounts: Vec<statfs> = Vec::with_capacity(expected_len as usize);

    let result = unsafe {
        getfsstat64(
            mounts.as_mut_ptr(),
            std::mem::size_of::<statfs>() as libc::c_int * expected_len,
            2,
        )
    };

    if result < 0 {
        return Err(Error::last_os_error());
    }

    unsafe {
        mounts.set_len(result as usize);
    }

    let mut vdisks: Vec<Disks> = Vec::with_capacity(expected_len as usize);
    for stat in mounts {
        if !is_physical_filesys(to_str(stat.f_fstypename.as_ptr())) {
            continue;
        }
        let path = to_str(stat.f_mntonname.as_ptr());
        let usage: (u64, u64) = match disk_usage(&path.as_bytes()) {
            Ok(val) => val,
            Err(x) => return Err(x),
        };
        vdisks.push(Disks {
            name: to_str(stat.f_mntfromname.as_ptr()).to_owned(),
            mount_point: path.to_owned(),
            total_space: usage.0 / 100000,
            avail_space: usage.1 / 100000,
        });
    }

    Ok(vdisks)
}
