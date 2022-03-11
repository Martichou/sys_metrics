use crate::host;
use crate::to_str;

#[cfg(target_os = "linux")]
use libc::utsname;
use std::io::Error;

/// Get the kernel version.
pub fn get_kernel_version() -> Result<String, Error> {
    let x = host::get_uname()?;
    Ok(to_str(x.release.as_ptr()).to_owned())
}

/// Inlined function to get the kernel version from a reference of uname.
#[cfg(target_os = "linux")]
#[inline]
pub(crate) fn get_kernel_version_from_uname(uts: &utsname) -> String {
    to_str(uts.release.as_ptr()).to_owned()
}
