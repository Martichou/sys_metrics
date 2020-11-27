use crate::host;
use crate::to_str;

use libc::utsname;
use std::io::Error;

/// Get the `os_version` (distro + release).
pub fn get_os_version() -> Result<String, Error> {
    let x = host::get_uname()?;
    let mut ret = String::with_capacity(x.sysname.len() + x.release.len() + 1);
    ret.push_str(to_str(x.sysname.as_ptr()));
    ret.push_str(to_str(x.release.as_ptr()));
    Ok(ret)
}

/// Inlined function to get the `os_version` from a reference of uname.
#[inline]
pub fn get_os_version_from_uname(uts: &utsname) -> String {
    let mut ret = String::with_capacity(uts.sysname.len() + uts.release.len() + 1);
    ret.push_str(to_str(uts.sysname.as_ptr()));
    ret.push_str(to_str(uts.release.as_ptr()));
    ret
}
