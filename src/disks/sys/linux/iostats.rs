use crate::disks::IoStats;

use std::io::{Error, ErrorKind};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

// https://github.com/heim-rs/heim/blob/ad691385940babcab857b1e19ebe95af35b8d70e/heim-disk/src/sys/linux/counters.rs#L21
// Magic value shared accross almost all Linux source code
// Despite this value can be queried at runtime
// via /sys/block/{DISK}/queue/hw_sector_size and results may vary
// between 1k, 2k, or 4k... 512 appears to be a magic constant used.
const DISK_SECTOR_SIZE: u64 = 512;

fn _get_iostats(physical: bool) -> Result<Vec<IoStats>, Error> {
    let file = File::open("/proc/diskstats")?;
    let mut viostats: Vec<IoStats> = Vec::new();
    let mut file = BufReader::with_capacity(2048, file);

    let mut line = String::with_capacity(256);
    while file.read_line(&mut line)? != 0 {
        let mut fields = line.split_whitespace().skip(2);

        let name = fields.next().unwrap();
        // Based on the sysstat code:
        // https://github.com/sysstat/sysstat/blob/1c711c1fd03ac638cfc1b25cdf700625c173fd2c/common.c#L200
        // Some devices may have a slash in their name (eg. cciss/c0d0...) so replace them with `!`
        if physical {
            let path =
                std::ffi::CString::new(format!("/sys/block/{}/device", name.replace("/", "!")))?;
            if unsafe { libc::access(path.as_ptr(), libc::F_OK) } != 0 {
                line.clear();
                continue;
            }
        }
        let read_count = fields.next().unwrap();
        let mut fields = fields.skip(1);
        let read_bytes = fields.next().unwrap();
        let write_count = fields.next().unwrap();
        let mut fields = fields.skip(1);
        let write_bytes = fields.next().unwrap();
        let mut fields = fields.skip(2);
        // Seconds
        let busy_time = fields.next().unwrap();

        if fields.count() < 3 {
            return Err(Error::new(ErrorKind::Other, "Invalid /proc/diskstats"));
        }
        viostats.push(IoStats {
            device_name: name.to_owned(),
            read_count: read_count.parse().unwrap(),
            read_bytes: read_bytes.parse::<u64>().unwrap() * DISK_SECTOR_SIZE,
            write_count: write_count.parse().unwrap(),
            write_bytes: write_bytes.parse::<u64>().unwrap() * DISK_SECTOR_SIZE,
            busy_time: busy_time.parse().unwrap(),
        });
        line.clear();
    }

    Ok(viostats)
}

/// Get basic [IoStats] (physical and virtual) info for each disks/partitions.
///
/// It only contains the `device_name` and the number of bytes `read`/`wrtn`.
///
/// On linux it will get them from `/proc/diskstats`.
///
/// [IoStats]: ../disks/struct.IoStats.html
pub fn get_iostats() -> Result<Vec<IoStats>, Error> {
    _get_iostats(false)
}

/// Get basic [IoStats] (physical) info for each physical disks.
///
/// On linux it will get them from `/proc/diskstats` and filter the result based on the access to their `/sys/block/{}`.
///
/// [IoStats]: ../struct.IoStats.html
pub fn get_iostats_physical() -> Result<Vec<IoStats>, Error> {
    _get_iostats(true)
}
