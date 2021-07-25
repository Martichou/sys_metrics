use crate::host;
use crate::memory::Swap;

use libc::c_ulong;
use std::{
    fs::File,
    io::{BufRead, BufReader, Error, ErrorKind},
};

/// Return the [Swap] struct.
///
/// It will get the info from syscall to sysinfo.
///
/// Used is simply the total - free.
///
/// [Swap]: ../memory/struct.Swap.html
#[allow(clippy::useless_conversion)]
pub fn get_swap() -> Result<Swap, Error> {
    // Init sysinfo
    let y = match host::sysinfo() {
        Ok(val) => val,
        Err(x) => return Err(Error::new(ErrorKind::Other, x)),
    };

    // Compute the values from the sysinfo and divide by (1024 * 1024) for MB
    let total_swap: u64 = ((y.totalswap * y.mem_unit as c_ulong) / (1024 * 1024)).into();
    let free_swap: u64 = ((y.freeswap * y.mem_unit as c_ulong) / (1024 * 1024)).into();
    let used_swap: u64 = total_swap - free_swap;
    Ok(Swap {
        total: total_swap,
        free: free_swap,
        used: used_swap,
    })
}

/// Determine if the system uses Swap.
///
/// Read /proc/swaps and count the number of lines, if more than 1 then swap is enabled.
pub fn has_swap() -> Result<bool, Error> {
    let file = File::open("/proc/swaps")?;
    let mut file = BufReader::with_capacity(512, file);

    let mut lines = 0u8;
    let mut line = String::with_capacity(128);
    while file.read_line(&mut line)? != 0 {
        lines += 1;
        line.clear();
    }

    Ok(lines > 1)
}
