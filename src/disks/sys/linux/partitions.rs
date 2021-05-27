use crate::disks::{disk_usage, is_physical_filesys, Disks};

use std::io::Error;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[inline]
fn _get_partitions(physical: bool) -> Result<Vec<Disks>, Error> {
    let file = File::open("/proc/mounts")?;
    let mut vdisks: Vec<Disks> = Vec::new();
    let mut file = BufReader::with_capacity(6144, file);

    let mut line = String::with_capacity(512);
    while file.read_line(&mut line)? != 0 {
        let mut fields = line.split_whitespace();
        let name = fields.next().unwrap();
        let path = fields.next().unwrap();
        let filesys = fields.next().unwrap();
        if physical && !is_physical_filesys(filesys) {
            line.clear();
            continue;
        }
        let usage: (u64, u64) = disk_usage(&path.as_bytes())?;
        vdisks.push(Disks {
            name: name.to_owned(),
            mount_point: path.to_owned(),
            total_space: usage.0 / 100000,
            avail_space: usage.1 / 100000,
        });
        line.clear()
    }

    Ok(vdisks)
}

/// Return a Vec of [Disks] (physical and virtual) with their minimal informations.
///
/// Contains `name`, `mount_point` and `total`/`free` space.
///
/// On linux it will get them from `/proc/mounts`.
///
/// [Disks]: ../disks/struct.Disks.html
pub fn get_partitions() -> Result<Vec<Disks>, Error> {
    _get_partitions(false)
}

/// Return a Vec of [Disks] (physical) with their minimal informations.
///
/// Contains `name`, `mount_point` and `total`/`free` space.
///
/// On linux it will get them from `/proc/mounts`.
///
/// [Disks]: ../disks/struct.Disks.html
pub fn get_partitions_physical() -> Result<Vec<Disks>, Error> {
    _get_partitions(true)
}
