use crate::cpu::CpuStats;

use std::io::{Error, ErrorKind};

pub fn get_cpustats() -> Result<CpuStats, Error> {
    Err(Error::new(ErrorKind::Other, "Not yet implemented"))
}
