use crate::host;
use crate::to_str;

use libc::utsname;
use std::io::Error;

/// Get the `hostname` of the host.
pub fn get_hostname() -> Result<String, Error> {
    let x = host::get_uname()?;
    Ok(to_str(x.nodename.as_ptr()).to_owned())
}

/// Inlined function to get the `hostname` from a reference of uname.
#[inline]
pub(crate) fn get_hostname_from_uname(uts: &utsname) -> String {
    to_str(uts.nodename.as_ptr()).to_owned()
}
