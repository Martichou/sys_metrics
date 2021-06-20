use std::io::Error;

/// Return the number of logical core the system has.
///
/// On linux it will gather the info from libc's sysconf or sched_getaffinity as a fallback.
pub fn get_cpu_logical_count() -> Result<u32, Error> {
    let cpus = unsafe { libc::sysconf(libc::_SC_NPROCESSORS_ONLN) };
    if cpus >= 0 {
        return Ok(cpus as u32);
    }
    let mut set = std::mem::MaybeUninit::<libc::cpu_set_t>::uninit();
    if unsafe {
        libc::sched_getaffinity(0, std::mem::size_of::<libc::cpu_set_t>(), set.as_mut_ptr())
    } == 0
    {
        let mut count: u32 = 0;
        for i in 0..libc::CPU_SETSIZE as usize {
            if unsafe { libc::CPU_ISSET(i, &set.assume_init()) } {
                count += 1
            }
        }
        Ok(count)
    } else {
        Err(Error::last_os_error())
    }
}
