use libc::{c_uint, c_void, sysctl};
use std::io::Error;

/// Get the cpufreq as f64.
///
/// On linux it will return the first frequency it see from `/proc/cpuinfo` (key: cpu MHz).
///
/// And on macOS it will make a syscall which will return the cpufreq (macOS doesn't seems to have per-core clock).
///
/// # Exemples
/// ```
/// use sys_metrics::cpu::get_cpufreq;
///
/// let cpufreq: f64 = match get_cpufreq() {
///     Ok(val) => val,
///     Err(x) => panic!(x),
/// };
///
/// // Should print your cpufreq as mHz
/// println!("{}", cpufreq);
/// ```
pub fn get_cpufreq() -> Result<f64, Error> {
    let mut data: c_uint = 0;
    let mut mib = [6, 15];

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
