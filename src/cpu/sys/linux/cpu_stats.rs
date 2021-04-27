use crate::cpu::CpuStats;

use std::{
    fs::File,
    io::{prelude::*, BufReader, Error, ErrorKind},
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

    let mut matched_lines = 0u8;
    let mut cpuctx = CpuStats::default();
    let mut line = String::with_capacity(128);
    while file.read_line(&mut line)? != 0 {
        // We only need 6 values which can be detected by their 4first bytes
        let first_bytes = &line.as_bytes()[..4];
        match first_bytes {
            b"intr" | b"ctxt" | b"soft" => {}
            _ => {
                line.clear();
                continue;
            }
        }

        // Split the line at the ' ' separator
        let mut parts = line.splitn(3, ' ');
        // Check if if the value we search is the splitted one
        // if so, return a pointer to the memory zone we'll modify.
        let field = match parts.next() {
            Some("intr") => &mut cpuctx.interrupts,
            Some("ctxt") => &mut cpuctx.ctx_switches,
            Some("softirq") => &mut cpuctx.soft_interrupts,
            _ => {
                line.clear();
                continue;
            }
        };

        // Get the value part
        match parts.next() {
            Some(value) => {
                // Increment the field we previously got (pointer)
                *field = {
                    // Trim to only get the bytes value
                    let rval = value.trim().parse::<u64>().unwrap();

                    matched_lines += 1;
                    rval
                }
            }

            None => {
                line.clear();
                continue;
            }
        }

        // If we've found all our information, we can return.
        if matched_lines == 3 {
            return Ok(cpuctx);
        }

        line.clear();
    }

    Err(Error::new(
        ErrorKind::Other,
        "Couldn't find all the informations for CpuStats",
    ))
}
