use crate::binding::{host_statistics64, mach_host_self};
use crate::cpu::{host_cpu_load_info, CpuTimes};

use mach::mach_port::mach_port_deallocate;
use mach::traps::mach_task_self;
use mach::vm_types::integer_t;
use std::io::Error;

/// Get basic [CpuTimes] info the host.
///
/// It only contains raw information, to get the delta we need
/// to get the diff between N and N-1.
///
/// [CpuTimes]: ../cpu/struct.CpuTimes.html
pub fn get_cputimes() -> Result<CpuTimes, Error> {
    let count = 4u32;
    // ALLOCATE A PORT
    let port = unsafe { mach_host_self() };
    let mut stats = std::mem::MaybeUninit::<host_cpu_load_info>::uninit();
    // GET CPU STATS INFO & SAVE THE RETURN VALUE OF host_statistics64
    let result =
        unsafe { host_statistics64(port, 3, stats.as_mut_ptr() as *mut integer_t, &count) };

    // Everybody seems to deallocate the port when it's for the cputimes
    // so let's be dumb and do the same without searching...
    if unsafe { mach_port_deallocate(mach_task_self(), port) } != 0 || result != 0 {
        return Err(Error::last_os_error());
    }

    // ASSUME VM_STATS IS INIT
    let stats = unsafe { stats.assume_init() };
    Ok(stats.into())
}

/// Get per core [CpuTimes] info the host.
///
/// It only contains raw information, to get the delta we need
/// to get the diff between N and N-1.
///
/// Note that with this call, the core field will correspond
/// to the number of the related core.
///
/// [CpuTimes]: ../cpu/struct.CpuTimes.html
pub fn get_each_cputimes() -> Result<Vec<CpuTimes>, Error> {
    // TODO
    Ok(Vec::new())
}
