use std::io::Error;
use std::mem::{size_of, zeroed};

/// Return the number of logical core the system has.
pub fn get_logical_count() -> Result<u32, Error> {
    let cpus = unsafe { libc::sysconf(libc::_SC_NPROCESSORS_ONLN) };
    if cpus >= 0 {
        return Ok(cpus as u32);
    }
    let mut set: libc::cpu_set_t = unsafe { zeroed() };
    if unsafe { libc::sched_getaffinity(0, size_of::<libc::cpu_set_t>(), &mut set) } == 0 {
        let mut count: u32 = 0;
        for i in 0..libc::CPU_SETSIZE as usize {
            if unsafe { libc::CPU_ISSET(i, &set) } {
                count += 1
            }
        }
        Ok(count)
    } else {
        Err(Error::last_os_error())
    }
}
