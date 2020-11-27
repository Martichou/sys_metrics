use std::io::Error;

/// Return the number of logical core the system has.
///
/// On linux it will gather the info from libc's sysconf or sched_getaffinity as a fallback.
///
/// And on macOS it will make a syscall to sysctl with hw.logicalcpu.
pub fn get_cpu_logical_count() -> Result<i64, Error> {
    let cpus = unsafe { libc::sysconf(libc::_SC_NPROCESSORS_ONLN) };
    if cpus >= 0 {
        return Ok(cpus);
    } else {
        let mut set: libc::cpu_set_t = unsafe { std::mem::zeroed() };
        if unsafe { libc::sched_getaffinity(0, std::mem::size_of::<libc::cpu_set_t>(), &mut set) }
            == 0
        {
            let mut count: u32 = 0;
            for i in 0..libc::CPU_SETSIZE as usize {
                if unsafe { libc::CPU_ISSET(i, &set) } {
                    count += 1
                }
            }
            Ok(count.into())
        } else {
            Err(Error::last_os_error())
        }
    }
}
