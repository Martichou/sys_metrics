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
    let mut file = BufReader::with_capacity(2048, file);

    let mut line = String::with_capacity(512);
    while file.read_line(&mut line)? != 0 {
        let mut fields = line.split_whitespace();
        let name = fields.nth(2).unwrap();
        let byte_r = fields.nth(2).unwrap();
        let byte_w = fields.nth(3).unwrap();
        if fields.count() < 4 {
            line.clear();
            continue;
        }
        viostats.push(IoStats {
            device_name: name.to_owned(),
            bytes_read: byte_r.parse::<i64>().unwrap() * 512,
            bytes_wrtn: byte_w.parse::<i64>().unwrap() * 512,
        });
        line.clear();
    }

    Ok(viostats)
}
