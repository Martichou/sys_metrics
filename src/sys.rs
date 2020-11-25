//! ## Example
//! ```
//! let uts = get_uname()?;
//! 
//! let os_version = get_os_version_from_uname(&uts);
//! let hostname = get_hostname_from_uname(&uts);
//! ```

use super::to_str;

use libc::utsname;
use std::io::Error;

/// Return a utsname instance
///
/// Use it with [get_os_version_from_uname] or [get_hostname_from_uname]
///
/// [get_os_version_from_uname]: ../sys/fn.get_os_version_from_uname.html
/// [get_hostname_from_uname]: ../sys/fn.get_hostname_from_uname.html
pub fn get_uname() -> Result<utsname, Error> {
    unsafe {
        let mut ret = std::mem::MaybeUninit::uninit();

        if libc::uname(ret.as_mut_ptr()) == -1 {
            return Err(Error::last_os_error());
        }

        Ok(ret.assume_init())
    }
}

/// Get the `os_version` (distro + release).
pub fn get_os_version() -> Result<String, Error> {
    let x = get_uname()?;
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

/// Get the `hostname` of the host.
pub fn get_hostname() -> Result<String, Error> {
    let x = get_uname()?;
    Ok(to_str(x.nodename.as_ptr()).to_owned())
}

/// Inlined function to get the `hostname` from a reference of uname.
#[inline]
pub fn get_hostname_from_uname(uts: &utsname) -> String {
    to_str(uts.nodename.as_ptr()).to_owned()
}
