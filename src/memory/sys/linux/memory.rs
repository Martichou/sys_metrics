use crate::host;
use crate::models;

use models::Memory;
use std::io::Error;
use std::io::ErrorKind;

/// Return the [Memory] struct.
///
/// Only contains the virtual/swap memory total/available.
///
/// On linux it will get them from the sysinfo.
///
/// On macOS it will use unsafe syscall due to specific OSX implementation.
///
/// [Memory]: ../struct.Memory.html
pub fn get_memory() -> Result<Memory, Error> {
    let y = match host::sysinfo() {
        Ok(val) => val,
        Err(x) => return Err(Error::new(ErrorKind::Other, x)),
    };

    Ok(Memory {
        total_virt: y.totalram as u64 * y.mem_unit as u64,
        total_swap: y.totalswap as u64 * y.mem_unit as u64,
        avail_virt: y.freeram as u64 * y.mem_unit as u64,
        avail_swap: y.freeswap as u64 * y.mem_unit as u64,
    })
}

#[inline]
pub(crate) fn get_memory_from_sysinfo(y: &libc::sysinfo) -> Memory {
    Memory {
        total_virt: y.totalram as u64 * y.mem_unit as u64,
        total_swap: y.totalswap as u64 * y.mem_unit as u64,
        avail_virt: y.freeram as u64 * y.mem_unit as u64,
        avail_swap: y.freeswap as u64 * y.mem_unit as u64,
    }
}
