use libc::{c_uint, c_void, sysctl};
use std::io::Error;

/// Return the number of logical core the system has.
///
/// And on macOS it will make a syscall to sysctl with hw.logicalcpu.
pub fn get_cpu_logical_count() -> Result<u32, Error> {
    let mut data: c_uint = 0;
    let mut mib: [i32; 2] = [6, 25];

    if unsafe {
        sysctl(
            mib.as_mut_ptr(),
            mib.len() as u32,
            &mut data as *mut _ as *mut c_void,
            &mut std::mem::size_of::<c_uint>(),
            std::ptr::null_mut(),
            0,
        )
    } < 0
    {
        let mut mib: [i32; 2] = [6, 3];
        if unsafe {
            sysctl(
                mib.as_mut_ptr(),
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

    Ok(data)
}
