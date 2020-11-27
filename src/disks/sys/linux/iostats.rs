use crate::models;

use models::IoStats;
use std::io::Error;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

/// Get basic [IoStats] info for each disks/partitions.
///
/// It only contains the `device_name` and the number of bytes `read`/`wrtn`.
///
/// On linux it will get them from `/proc/diskstats`.
///
/// On macOS it will use unsafes call to multiple OSX specific functions.
///
/// [IoStats]: ../struct.IoStats.html
pub fn get_iostats() -> Result<Vec<IoStats>, Error> {
    let mut viostats: Vec<IoStats> = Vec::new();
    let file = File::open("/proc/diskstats")?;
    let file = BufReader::with_capacity(2048, file);

    for line in file.lines() {
        let line = line.unwrap();
        let fields = line.split_whitespace().collect::<Vec<&str>>();
        if fields.len() < 14 {
            continue;
        }
        viostats.push(IoStats {
            device_name: fields[2].to_owned(),
            bytes_read: fields[5].parse::<i64>().unwrap() * 512,
            bytes_wrtn: fields[9].parse::<i64>().unwrap() * 512,
        });
    }

    Ok(viostats)
}
