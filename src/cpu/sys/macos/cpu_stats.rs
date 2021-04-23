use crate::cpu::CpuStats;

use std::io::{Error, ErrorKind};

/// Get basic [CpuStats] info the host.
///
/// It only contains row information, to get the delta we need
/// to get the diff between N and N-1.
///
/// [CpuStats]: ../cpu/struct.CpuStats.html
pub fn get_cpustats() -> Result<CpuStats, Error> {
    Err(Error::new(ErrorKind::Other, "Couldn't get the CpuStat"))
}
