use crate::disk_usage;
use crate::is_physical_filesys;
use crate::models;

use models::Disks;
use std::io::Error;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

/// Return a Vec of [Disks] with their minimal informations.
///
/// Contains `name`, `mount_point` and `total`/`free` space.
///
/// On linux it will get them from `/proc/mounts`.
///
/// On macOS it will use an unsafe call to `getfsstat64`.
///
/// [Disks]: ../struct.Disks.html
pub fn get_partitions_physical() -> Result<Vec<Disks>, Error> {
    let mut vdisks: Vec<Disks> = Vec::new();
    let file = File::open("/proc/mounts")?;
    let mut file = BufReader::with_capacity(6144, file);

    let mut line = String::with_capacity(512);
    while file.read_line(&mut line)? != 0 {
        let mut fields = line.split_whitespace();
        let name = fields.nth(0).unwrap();
        let path = fields.nth(0).unwrap();
        let filesys = fields.nth(0).unwrap();
        if !is_physical_filesys(filesys) {
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
