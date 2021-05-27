use crate::host;
use crate::memory::Swap;

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
pub fn get_swap() -> Result<Swap, Error> {
    // Init sysinfo
    let y = match host::sysinfo() {
        Ok(val) => val,
        Err(x) => return Err(Error::new(ErrorKind::Other, x)),
    };

    // Compute the values from the sysinfo and divide by 1024 for kb
    let total_swap = (y.totalswap * y.mem_unit as u64) / 1024;
    let free_swap = (y.freeswap * y.mem_unit as u64) / 1024;
    let used_swap = total_swap - free_swap;
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
