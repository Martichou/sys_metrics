use crate::cpu::{host_cpu_load_info, CpuStats};

use mach::mach_port::mach_port_deallocate;
use mach::traps::mach_task_self;
use mach::{
    kern_return::kern_return_t,
    mach_types::{host_name_port_t, host_t},
    message::mach_msg_type_number_t,
    vm_types::integer_t,
};
use std::io::Error;

extern "C" {
    fn mach_host_self() -> host_name_port_t;

    fn host_statistics64(
        host_priv: host_t,
        flavor: integer_t,
        host_info_out: *mut integer_t,
        host_info_outCnt: *const mach_msg_type_number_t,
    ) -> kern_return_t;
}

/// Get basic [CpuStats] info the host.
///
/// It only contains row information, to get the delta we need
/// to get the diff between N and N-1.
///
/// [CpuStats]: ../cpu/struct.CpuStats.html
pub fn get_cpustats() -> Result<CpuStats, Error> {
    let count = 4u32;
    // ALLOCATE A PORT
    let port = unsafe { mach_host_self() };
    let mut stats = std::mem::MaybeUninit::<host_cpu_load_info>::uninit();
    // GET CPU STATS INFO & SAVE THE RETURN VALUE OF host_statistics64
    let result =
        unsafe { host_statistics64(port, 3, stats.as_mut_ptr() as *mut integer_t, &count) };

    // Everybody seems to deallocate the port when it's for the cpustats
    // so let's be dumb and do the same without searching...
    if unsafe { mach_port_deallocate(mach_task_self(), port) } != 0 || result != 0 {
        return Err(Error::last_os_error());
    }

    // ASSUME VM_STATS IS INIT
    let stats = unsafe { stats.assume_init() };
    Ok(stats.into())
}
