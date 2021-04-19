use crate::models;

use models::CpuStats;
use std::io::Error;
use std::{fs::File, io::ErrorKind};

pub fn get_cpustats() -> Result<CpuStats, Error> {
    Err(Error::new(ErrorKind::Other, "Couldn't get the CpuStat"))
}
