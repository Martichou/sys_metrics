use libc::utsname;
use std::io::Error;

/// Return a utsname instance
///
/// Use it with [get_os_version_from_uname] or [get_hostname_from_uname]
///
/// [get_os_version_from_uname]: ../host/fn.get_os_version_from_uname.html
/// [get_hostname_from_uname]: ../host/fn.get_hostname_from_uname.html
pub fn get_uname() -> Result<utsname, Error> {
    unsafe {
        let mut ret = std::mem::MaybeUninit::uninit();

        if libc::uname(ret.as_mut_ptr()) == -1 {
            return Err(Error::last_os_error());
        }

        Ok(ret.assume_init())
    }
}
