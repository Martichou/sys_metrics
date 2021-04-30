use crate::binding::{host_statistics64, mach_host_self, vm_statistics64};
use crate::memory::Memory;
use crate::PAGE_SIZE;

use mach::mach_port::mach_port_deallocate;
use mach::traps::mach_task_self;
use mach::vm_types::integer_t;
use std::io::Error;

/// Return the [Memory] struct.
///
/// Will use unsafe syscall due to specific OSX implementation.
///
/// Note that shared, buffers and cached are not used nor declared on MacOS, and so those value are zeroed.
///
/// [Memory]: ../memory/struct.Memory.html
pub fn get_memory() -> Result<Memory, Error> {
    let count = 38;
    // ALLOCATE A PORT
    let port = unsafe { mach_host_self() };
    let mut vm_stats = std::mem::MaybeUninit::<vm_statistics64>::uninit();
    // GET HOST INFO ABOUT MEMORY & CHECK THE RETURN VALUE OF host_statistics64
    if unsafe { host_statistics64(port, 4, vm_stats.as_mut_ptr() as *mut integer_t, &count) } != 0 {
        return Err(Error::last_os_error());
    }
    // Is the port_deallocate really usefull ?
    // If I try to stay like vm_stat (https://opensource.apple.com/source/system_cmds/system_cmds-498.2/vm_stat.tproj/vm_stat.c)
    // there is not a single deallocation for the port whatsoever.
    if unsafe { mach_port_deallocate(mach_task_self(), port) } != 0 {
        return Err(Error::last_os_error());
    }

    // ASSUME VM_STATS IS INIT
    let vm_stats = unsafe { vm_stats.assume_init() };

    // TOTAL VIRTUAL MEMORY
    let mut name: [i32; 2] = [6, 24];
    let mut total = 0u64;
    let mut length = std::mem::size_of::<u64>();
    if unsafe {
        libc::sysctl(
            name.as_mut_ptr(),
            2,
            &mut total as *mut u64 as *mut libc::c_void,
            &mut length,
            std::ptr::null_mut(),
            0,
        )
    } != 0
    {
        return Err(Error::last_os_error());
    }

    Ok(Memory {
        total,
        free: u64::from(vm_stats.free_count + vm_stats.speculative_count) * (*PAGE_SIZE),
        used: u64::from(vm_stats.active_count + vm_stats.wire_count) * (*PAGE_SIZE),
        shared: 0,
        buffers: 0,
        cached: 0,
    })
}
