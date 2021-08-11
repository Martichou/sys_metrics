use crate::cpu::CpuTimes;

use std::{
    fs::File,
    io::{prelude::*, BufReader, Error, ErrorKind},
};

/// Get basic [CpuTimes] info the host.
///
/// It only contains raw information, to get the delta we need
/// to get the diff between N and N-1.
///
/// [CpuTimes]: ../cpu/struct.CpuTimes.html
pub fn get_cputimes() -> Result<CpuTimes, Error> {
    let file = File::open("/proc/stat")?;
    let mut file = BufReader::with_capacity(1024, file);

    let mut line = String::with_capacity(128);
    if file.read_line(&mut line)? != 0 {
        // Split whitespaces and get an Array of values
        let mut fields = line.split_whitespace();

        // Skip the first columns which is the name of the stats
        let user = nth!(fields, 1)?;
        let nice = nth!(fields, 0)?;
        let system = nth!(fields, 0)?;
        let idle = nth!(fields, 0)?;
        let iowait = nth!(fields, 0)?;
        let irq = nth!(fields, 0)?;
        let softirq = nth!(fields, 0)?;
        // Unwrap_or because the 8th-10th fields are not present on old kernel
        let steal = nth!(fields, 0).unwrap_or("0");
        let guest = nth!(fields, 0).unwrap_or("0");
        let guest_nice = nth!(fields, 0).unwrap_or("0");
        // Return the struct, and parse to i64
        return Ok(CpuTimes {
            user: user.parse::<u64>().unwrap(),
            nice: nice.parse::<u64>().unwrap(),
            system: system.parse::<u64>().unwrap(),
            idle: idle.parse::<u64>().unwrap(),
            iowait: iowait.parse::<u64>().unwrap(),
            irq: irq.parse::<u64>().unwrap(),
            softirq: softirq.parse::<u64>().unwrap(),
            steal: steal.parse::<u64>().unwrap(),
            guest: guest.parse::<u64>().unwrap(),
            guest_nice: guest_nice.parse::<u64>().unwrap(),
        });
    }

    Err(Error::new(ErrorKind::Other, "Couldn't get the CpuTimes"))
}
