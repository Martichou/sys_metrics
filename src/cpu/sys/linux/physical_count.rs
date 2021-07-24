use crate::read_and_trim;

use std::collections::HashSet;
use std::{
    fs::File,
    io::{prelude::*, BufReader, Error, ErrorKind},
};

/// Return the number of physcial core the system has using topology (glob).
fn get_from_glob() -> Result<u32, Error> {
    let path = "/sys/devices/system/cpu/cpu*/topology/core_id";
    let entries = glob::glob(path).expect("Invalid glob pattern");
    let mut acc = HashSet::<u32>::new();

    for entry in entries {
        let entry = entry.map_err(|e| e.into_error())?;
        // Read the content & trim the result and parse to u32
        match read_and_trim(entry)?.parse() {
            Ok(val) => acc.insert(val),
            Err(e) => return Err(Error::new(ErrorKind::InvalidData, e)),
        };
    }

    // This error will not be propagated to caller,
    // since `physical_count` will call `or_else()` on it
    if !acc.is_empty() {
        Ok(acc.len() as u32)
    } else {
        Err(Error::from(ErrorKind::NotFound))
    }
}

/// Return the number of physcial core the system has using /proc/cpuinfo.
fn get_from_cpuinfo() -> Result<u32, Error> {
    let file = File::open("/proc/cpuinfo")?;
    let mut file = BufReader::with_capacity(1024, file);

    let mut line = String::with_capacity(256);
    while file.read_line(&mut line)? != 0 {
        let lenght = line.len();
        if lenght > 12 && lenght < 24 && &line[..9] == "cpu cores" {
            match line[12..lenght - 1].parse::<u32>() {
                Ok(val) => return Ok(val),
                Err(_) => {
                    line.clear();
                    continue;
                }
            };
        }
        line.clear();
    }

    Err(Error::new(
        ErrorKind::Other,
        "Cannot determine the number of physical core",
    ))
}

/// Return the number of physcial core the system has.
pub fn get_physical_count() -> Result<u32, Error> {
    match get_from_glob() {
        Ok(val) => Ok(val),
        Err(..) => get_from_cpuinfo(),
    }
}
