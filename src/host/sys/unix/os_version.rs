use crate::host;
use crate::to_str;

use libc::utsname;
use std::io::Error;

/// Get the os version.
pub fn get_os_version() -> Result<String, Error> {
    let x = host::get_uname()?;
    Ok(to_str(x.version.as_ptr()).to_owned())
}

/// Inlined function to get the os version from a reference of uname.
#[inline]
pub(crate) fn get_os_version_from_uname(uts: &utsname) -> String {
    to_str(uts.version.as_ptr()).to_owned()
}
