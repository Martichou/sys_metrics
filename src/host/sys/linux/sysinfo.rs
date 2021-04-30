use std::io::Error;

/// Return the sysinfo information
pub(crate) fn sysinfo() -> Result<libc::sysinfo, Error> {
    let mut info = std::mem::MaybeUninit::<libc::sysinfo>::uninit();

    if unsafe { libc::sysinfo(info.as_mut_ptr()) } == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(unsafe { info.assume_init() })
    }
}
