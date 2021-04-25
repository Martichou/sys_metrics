use crate::cpu::CpuStats;
use crate::cpu::CLOCK_TICKS;

use std::io::Error;
use std::{
    fs::File,
    io::{prelude::*, BufReader, ErrorKind},
};

/// Get basic [CpuStats] info the host.
///
/// It only contains row information, to get the delta we need
/// to get the diff between N and N-1.
///
/// On linux it will get them from `/proc/stat`.
///
/// [CpuStats]: ../cpu/struct.CpuStats.html
pub fn get_cpustats() -> Result<CpuStats, Error> {
    let file = File::open("/proc/stat")?;
    let mut file = BufReader::with_capacity(1024, file);

    let mut line = String::with_capacity(128);
    if file.read_line(&mut line)? != 0 {
        // Split whitespaces and get an Array of values
        let mut fields = line.split_whitespace();

        // TODO - Add guard if less than 7 fields
        // Skip the first columns which is the name of the stats
        let user = fields.nth(1).unwrap();
        let nice = fields.next().unwrap();
        let system = fields.next().unwrap();
        let idle = fields.next().unwrap();
        let iowait = fields.next().unwrap();
        let irq = fields.next().unwrap();
        let softirq = fields.next().unwrap();
        // Unwrap_or because the 8th-10th fields are not present on old kernel
        let steal = fields.next().unwrap_or("0");
        let guest = fields.next().unwrap_or("0");
        let guest_nice = fields.next().unwrap_or("0");

        let ticks = *CLOCK_TICKS as i64;
        // Return the struct, and parse to i64
        return Ok(CpuStats {
            user: user.parse::<i64>().unwrap() / ticks,
            nice: nice.parse::<i64>().unwrap() / ticks,
            system: system.parse::<i64>().unwrap() / ticks,
            idle: idle.parse::<i64>().unwrap() / ticks,
            iowait: iowait.parse::<i64>().unwrap() / ticks,
            irq: irq.parse::<i64>().unwrap() / ticks,
            softirq: softirq.parse::<i64>().unwrap() / ticks,
            steal: steal.parse::<i64>().unwrap() / ticks,
            guest: guest.parse::<i64>().unwrap() / ticks,
            guest_nice: guest_nice.parse::<i64>().unwrap() / ticks,
        });
    }

    Err(Error::new(ErrorKind::Other, "Couldn't get the CpuStat"))
}
