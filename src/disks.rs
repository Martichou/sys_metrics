use super::disk_usage;
use super::is_physical_filesys;

use crate::models;

#[cfg(target_os = "macos")]
use core_foundation_sys::{
    base::{kCFAllocatorDefault, CFRelease},
    dictionary::{CFDictionaryGetValueIfPresent, CFDictionaryRef},
    number::{CFNumberGetValue, CFNumberRef},
    string::{CFStringGetCString, CFStringRef},
};
#[cfg(target_os = "macos")]
use io_kit_sys::{
    kIOMasterPortDefault,
    ret::kIOReturnSuccess,
    types::{io_iterator_t, io_registry_entry_t},
    IOServiceMatching, *,
};
#[cfg(target_os = "macos")]
use libc::c_char;
use models::{Disks, IoStats};
#[cfg(target_os = "macos")]
use nix::libc::statfs;
#[cfg(target_os = "macos")]
use std::ffi::CStr;
#[cfg(target_family = "unix")]
use std::io::Error;
#[cfg(target_os = "macos")]
use std::io::ErrorKind;
use std::path::PathBuf;
#[cfg(target_os = "linux")]
use std::{
    fs::File,
    io::{BufRead, BufReader},
};
#[cfg(target_os = "linux")]
use unescape::unescape;

#[cfg(target_os = "macos")]
extern "C" {
    fn getfsstat64(buf: *mut statfs, bufsize: libc::c_int, flags: libc::c_int) -> libc::c_int;
}

/// Retrieve the partitions and return them as a Vec<Disks>.
/// Contains name, mount_point and total/free space.
/// LINUX => read info from /proc/mounts.
#[cfg(target_os = "linux")]
pub fn get_partitions_info() -> Result<Vec<Disks>, Error> {
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
        let usage: (u64, u64) = match disk_usage(&m_p) {
            Ok(val) => val,
            Err(x) => return Err(x),
        };
        vdisks.push(Disks {
            name: fields[0].to_owned(),
            mount_point: m_p.into_os_string().into_string().unwrap(),
            total_space: usage.0 / 100000,
            avail_space: usage.1 / 100000,
        });
    }

    Ok(vdisks)
}

/// Retrieve the partitions and return them as a Vec<Disks>.
/// Contains name, mount_point and total/free space.
/// macOS => use C's function getfsstat64.
#[cfg(target_os = "macos")]
pub fn get_partitions_info() -> Result<Vec<Disks>, Error> {
    let expected_len = unsafe { getfsstat64(std::ptr::null_mut(), 0, 2) };
    let mut mounts: Vec<statfs> = Vec::with_capacity(expected_len as usize);

    let result = unsafe {
        getfsstat64(
            mounts.as_mut_ptr(),
            std::mem::size_of::<statfs>() as libc::c_int * expected_len,
            2,
        )
    };
    if result < 0 {
        return Err(Error::last_os_error());
    }
    unsafe {
        mounts.set_len(result as usize);
    }

    let mut vdisks: Vec<Disks> = Vec::with_capacity(expected_len as usize);
    for stat in mounts {
        if !is_physical_filesys(unsafe {
            &CStr::from_ptr(stat.f_fstypename.as_ptr()).to_string_lossy()
        }) {
            continue;
        }
        let m_p = PathBuf::from(unsafe {
            CStr::from_ptr(stat.f_mntonname.as_ptr())
                .to_string_lossy()
                .to_string()
        });
        let usage: (u64, u64) = match disk_usage(&m_p) {
            Ok(val) => val,
            Err(x) => return Err(x),
        };
        vdisks.push(Disks {
            name: unsafe {
                CStr::from_ptr(stat.f_mntfromname.as_ptr())
                    .to_string_lossy()
                    .to_string()
            },
            mount_point: m_p.into_os_string().into_string().unwrap(),
            total_space: usage.0 / 100000,
            avail_space: usage.1 / 100000,
        });
    }

    Ok(vdisks)
}

/// Return the disk io usage, number of sectors read, wrtn.
/// From that you can compute the mb/s.
/// LINUX -> Read data from /proc/diskstats.
#[cfg(target_os = "linux")]
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

/// Return the disk io usage, number of sectors read, wrtn.
/// From that you can compute the mb/s.
/// macOS -> Read data using heim_disks.
#[cfg(target_os = "macos")]
pub fn get_iostats() -> Result<Vec<IoStats>, Error> {
    let mut viostats: Vec<IoStats> = Vec::new();

    unsafe {
        let mut disk_list = std::mem::MaybeUninit::<io_iterator_t>::uninit();
        if IOServiceGetMatchingServices(
            kIOMasterPortDefault,
            IOServiceMatching(b"IOMedia\0".as_ptr() as *const c_char),
            &mut disk_list as *mut _ as *mut _,
        ) != kIOReturnSuccess
        {
            return Err(Error::last_os_error());
        }
        let disk_list = disk_list.assume_init();

        #[allow(unused_assignments)]
        let mut disk: io_registry_entry_t = 0;
        let mut parent: io_registry_entry_t = 0;
        let mut parent_dict: CFDictionaryRef;
        let mut props_dict: CFDictionaryRef;
        let mut stats_dict: CFDictionaryRef;

        disk = IOIteratorNext(disk_list);
        while disk != 0 {
            parent_dict = std::ptr::null();
            props_dict = std::ptr::null();
            stats_dict = std::ptr::null();

            // Maybe pass the plane as a mut_ptr
            if IORegistryEntryGetParentEntry(disk, b"IOService\0".as_ptr() as *mut i8, &mut parent)
                != kIOReturnSuccess
            {
                IOObjectRelease(disk);
                return Err(Error::last_os_error());
            }

            // Maybe pass the className as a mut_ptr
            if IOObjectConformsTo(parent, b"IOBlockStorageDriver\0".as_ptr() as *mut i8) != 0 {
                // The parent_dict convertion was a try error, might fail
                if IORegistryEntryCreateCFProperties(
                    disk,
                    &mut parent_dict as *const _ as *mut _,
                    kCFAllocatorDefault,
                    0,
                ) != kIOReturnSuccess
                {
                    IOObjectRelease(disk);
                    IOObjectRelease(parent);
                    return Err(Error::last_os_error());
                }

                if IORegistryEntryCreateCFProperties(
                    parent,
                    &mut props_dict as *const _ as *mut _,
                    kCFAllocatorDefault,
                    0,
                ) != kIOReturnSuccess
                {
                    CFRelease(parent_dict as *mut _);
                    CFRelease(props_dict as *mut _);
                    IOObjectRelease(disk);
                    IOObjectRelease(parent);
                    return Err(Error::last_os_error());
                }

                let mut disk_name_ref = std::mem::MaybeUninit::<CFStringRef>::uninit();
                if CFDictionaryGetValueIfPresent(
                    parent_dict,
                    CFSTR(b"BSD Name\0".as_ptr() as *mut i8) as *mut _,
                    &mut disk_name_ref as *mut _ as *mut _,
                ) == 0
                {
                    CFRelease(parent_dict as *mut _);
                    CFRelease(props_dict as *mut _);
                    IOObjectRelease(disk);
                    IOObjectRelease(parent);
                    return Err(Error::new(
                        ErrorKind::Other,
                        "CFDictionaryGetValueIfPresent: BSD Name not found in the parent_dict",
                    ));
                }
                let disk_name_ref = disk_name_ref.assume_init();
                let mut name = [0i8; 64];
                if CFStringGetCString(disk_name_ref, name.as_mut_ptr(), 64, 134217984) == 0 {
                    CFRelease(parent_dict as *mut _);
                    CFRelease(props_dict as *mut _);
                    IOObjectRelease(disk);
                    IOObjectRelease(parent);
                    return Err(Error::new(ErrorKind::Other, "Cannot get the buffer filled"));
                }

                if CFDictionaryGetValueIfPresent(
                    props_dict,
                    CFSTR(b"Statistics\0".as_ptr() as *mut i8) as *mut _,
                    &mut stats_dict as *mut _ as *mut _,
                ) == 0
                {
                    CFRelease(parent_dict as *mut _);
                    CFRelease(props_dict as *mut _);
                    IOObjectRelease(disk);
                    IOObjectRelease(parent);
                    return Err(Error::new(
                        ErrorKind::Other,
                        "CFDictionaryGetValueIfPresent: Statistics not found in the props_dict",
                    ));
                }

                let mut write_bytes_nbr = std::mem::MaybeUninit::<CFNumberRef>::uninit();
                let mut read_bytes_nbr = std::mem::MaybeUninit::<CFNumberRef>::uninit();
                let mut read_bytes = 0i64;
                let mut write_bytes = 0i64;

                if CFDictionaryGetValueIfPresent(
                    stats_dict,
                    CFSTR(b"Bytes (Read)\0".as_ptr() as *mut i8) as *mut _,
                    &mut write_bytes_nbr as *mut _ as *mut _,
                ) == 0
                {
                    CFRelease(parent_dict as *mut _);
                    CFRelease(props_dict as *mut _);
                    IOObjectRelease(disk);
                    IOObjectRelease(parent);
                    return Err(Error::new(
                        ErrorKind::Other,
                        "CFDictionaryGetValueIfPresent: Bytes Read not found in the stats_dict",
                    ));
                }
                let number = write_bytes_nbr.assume_init();
                CFNumberGetValue(number, 4, &mut read_bytes as *mut _ as *mut _);

                if CFDictionaryGetValueIfPresent(
                    stats_dict,
                    CFSTR(b"Bytes (Write)\0".as_ptr() as *mut i8) as *mut _,
                    &mut read_bytes_nbr as *mut _ as *mut _,
                ) == 0
                {
                    CFRelease(parent_dict as *mut _);
                    CFRelease(props_dict as *mut _);
                    IOObjectRelease(disk);
                    IOObjectRelease(parent);
                    return Err(Error::new(
                        ErrorKind::Other,
                        "CFDictionaryGetValueIfPresent: Bytes Write not found in the stats_dict",
                    ));
                }
                let number = read_bytes_nbr.assume_init();
                CFNumberGetValue(number, 4, &mut write_bytes as *mut _ as *mut _);

                let name = match CStr::from_ptr(name.as_mut_ptr()).to_str() {
                    Ok(val) => val.to_owned(),
                    Err(_) => String::from("?"),
                };

                viostats.push(IoStats {
                    device_name: name,
                    bytes_read: read_bytes,
                    bytes_wrtn: write_bytes,
                });

                CFRelease(parent_dict as *mut _);
                CFRelease(props_dict as *mut _);
                IOObjectRelease(disk);
                IOObjectRelease(parent);
            }
            disk = IOIteratorNext(disk_list);
        }

        IOObjectRelease(disk_list);
    }

    Ok(viostats)
}
