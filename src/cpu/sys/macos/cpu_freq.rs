use libc::{c_uint, c_void, sysctl};
use std::io::Error;

/// Get the cpufreq as f64 (in MHz).
pub fn get_cpufreq() -> Result<f64, Error> {
    let mut data: c_uint = 0;
    let mut mib = [libc::CTL_HW, libc::HW_CPU_FREQ];

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

    Ok(data as f64)
}
