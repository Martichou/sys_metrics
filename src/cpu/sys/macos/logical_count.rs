use libc::{c_uint, c_void, sysctl};
use std::io::Error;

/// Return the number of logical core the system has.
///
/// On linux it will gather the info from libc's sysconf or sched_getaffinity as a fallback.
///
/// And on macOS it will make a syscall to sysctl with hw.logicalcpu.
pub fn get_cpu_logical_count() -> Result<i64, Error> {
    let mut data: c_uint = 0;
    let mib = [6, 25];

    if unsafe {
        sysctl(
            &mib[0] as *const _ as *mut _,
            mib.len() as u32,
            &mut data as *mut _ as *mut c_void,
            &mut std::mem::size_of::<c_uint>(),
            std::ptr::null_mut(),
            0,
        )
    } < 0
    {
        let mib = [6, 3];
        if unsafe {
            sysctl(
                &mib[0] as *const _ as *mut _,
                mib.len() as u32,
                &mut data as *mut _ as *mut c_void,
                &mut std::mem::size_of::<c_uint>(),
                std::ptr::null_mut(),
                0,
            )
        } < 0
        {
            return Err(Error::last_os_error());
        }
    }

    Ok(data.into())
}
