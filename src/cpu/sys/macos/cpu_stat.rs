use crate::models;

use models::CpuStat;
use std::io::Error;
use std::{fs::File, io::ErrorKind};

pub fn get_cpustat() -> Result<CpuStat, Error> {
    Err(Error::new(ErrorKind::Other, "Couldn't get the CpuStat"))
}
