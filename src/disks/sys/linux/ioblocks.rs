use crate::disks::IoBlock;

use std::io::{Error, ErrorKind};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

// https://github.com/heim-rs/heim/blob/ad691385940babcab857b1e19ebe95af35b8d70e/heim-disk/src/sys/linux/counters.rs#L21
// Magic value shared accross almost all Linux source code
// Despite this value can be queried at runtime
// via /sys/block/{DISK}/queue/hw_sector_size and results may vary
// between 1k, 2k, or 4k... 512 appears to be a magic constant used.
const DISK_SECTOR_SIZE: u64 = 512;

#[inline]
fn _get_ioblocks(physical: bool) -> Result<Vec<IoBlock>, Error> {
    let file = File::open("/proc/diskstats")?;
    let mut v_ioblocks: Vec<IoBlock> = Vec::new();
    let mut file = BufReader::with_capacity(2048, file);

    let mut line = String::with_capacity(256);
    while file.read_line(&mut line)? != 0 {
        let mut fields = line.split_whitespace();

        let name = nth!(fields, 2)?;
        // Based on the sysstat code:
        // https://github.com/sysstat/sysstat/blob/1c711c1fd03ac638cfc1b25cdf700625c173fd2c/common.c#L200
        // Some devices may have a slash in their name (eg. cciss/c0d0...) so replace them with `!`
        if physical && !Path::new(&format!("/sys/block/{}/device", name.replace("/", "!"))).exists()
        {
            line.clear();
            continue;
        }
        let read_count = nth!(fields, 0)?;
        let read_bytes = nth!(fields, 1)?;
        let write_count = nth!(fields, 0)?;
        let write_bytes = nth!(fields, 1)?;
        // Seconds
        let busy_time = nth!(fields, 2)?;

        if fields.count() < 2 {
            return Err(Error::new(ErrorKind::Other, "Invalid /proc/diskstats"));
        }
        v_ioblocks.push(IoBlock {
            device_name: name.to_owned(),
            read_count: read_count.parse().unwrap(),
            read_bytes: read_bytes.parse::<u64>().unwrap() * DISK_SECTOR_SIZE,
            write_count: write_count.parse().unwrap(),
            write_bytes: write_bytes.parse::<u64>().unwrap() * DISK_SECTOR_SIZE,
            busy_time: busy_time.parse().unwrap(),
        });
        line.clear();
    }

    Ok(v_ioblocks)
}

/// Get basic [IoBlock] (physical and virtual) info for each disks/partitions.
///
/// It only contains the `device_name` and the number of bytes `read`/`wrtn`.
///
/// On linux it will get them from `/proc/diskstats`.
///
/// [IoBlock]: ../disks/struct.IoBlock.html
pub fn get_ioblocks() -> Result<Vec<IoBlock>, Error> {
    _get_ioblocks(false)
}

/// Get basic [IoBlock] (physical) info for each physical disks.
///
/// On linux it will get them from `/proc/diskstats` and filter the result based on the access to their `/sys/block/{}`.
///
/// [IoBlock]: ../struct.IoBlock.html
pub fn get_physical_ioblocks() -> Result<Vec<IoBlock>, Error> {
    _get_ioblocks(true)
}
