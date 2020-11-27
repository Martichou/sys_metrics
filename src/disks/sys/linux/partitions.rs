use crate::disk_usage;
use crate::is_physical_filesys;
use crate::models;

use models::Disks;
use std::io::Error;
use std::path::PathBuf;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};
use unescape::unescape;

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
    let file = BufReader::with_capacity(6144, file);

    for line in file.lines() {
        let line = line.unwrap();
        let fields = line.split_whitespace().collect::<Vec<&str>>();
        if !is_physical_filesys(fields[2]) {
            continue;
        }
        let m_p = PathBuf::from(unescape(fields[1]).unwrap());
        let usage: (u64, u64) = disk_usage(&m_p)?;
        vdisks.push(Disks {
            name: fields[0].to_owned(),
            mount_point: m_p.into_os_string().into_string().unwrap(),
            total_space: usage.0 / 100000,
            avail_space: usage.1 / 100000,
        });
    }

    Ok(vdisks)
}
