use crate::models;

use models::CpuStat;
use std::io::Error;
use std::{
    fs::File,
    io::{prelude::*, BufReader, ErrorKind},
};

// https://supportcenter.checkpoint.com/supportcenter/portal?eventSubmit_doGoviewsolutiondetails=&solutionid=sk65143
// read from /proc/stat and capture the 10 column with many information
// CPU info:
//	1st column : user = normal processes executing in user mode
//	2nd column : nice = niced processes executing in user mode
//	3rd column : system = processes executing in kernel mode
//	4th column : idle = twiddling thumbs
//	5th column : iowait = waiting for I/O to complete
//	6th column : irq = servicing interrupts
//	7th column : softirq = servicing softirqs
//	8th column : steal = ticks spent executing other virtual hosts
//	9th column : guest = time spent running a virtual CPU for guest operating systems under the control of the Kernel
//	10th column: guest_nice = time spent running a niced guest (virtual CPU for guest operating systems under the control of the Linux kernel)

/// Get basic [CpuStat] info the host.
///
/// It only contains row information, to get the delta we need
/// to get the diff between N and N-1.
///
/// On linux it will get them from `/proc/stat`.
///
/// On macOS it will [TODO].
///
/// [CpuStat]: ../struct.CpuStat.html
pub fn get_cpustat() -> Result<CpuStat, Error> {
    let file = File::open("/proc/stat")?;
    let mut file = BufReader::with_capacity(1024, file);

    let mut line = String::with_capacity(128);
    if file.read_line(&mut line)? != 0 {
        // Split whitespaces and get an Array of values
        let mut fields = line.split_whitespace();

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
        // Return the struct, and parse to i64
        return Ok(CpuStat {
            user: user.parse::<i64>().unwrap(),
            nice: nice.parse::<i64>().unwrap(),
            system: system.parse::<i64>().unwrap(),
            idle: idle.parse::<i64>().unwrap(),
            iowait: iowait.parse::<i64>().unwrap(),
            irq: irq.parse::<i64>().unwrap(),
            softirq: softirq.parse::<i64>().unwrap(),
            steal: steal.parse::<i64>().unwrap(),
            guest: guest.parse::<i64>().unwrap(),
            guest_nice: guest_nice.parse::<i64>().unwrap(),
        });
    }

    Err(Error::new(ErrorKind::Other, "Couldn't get the CpuStat"))
}
