use crate::models;

use core_foundation_sys::{
    base::{kCFAllocatorDefault, CFRelease},
    dictionary::{CFDictionaryGetValueIfPresent, CFDictionaryRef},
    number::{CFBooleanGetValue, CFBooleanRef, CFNumberGetValue, CFNumberRef},
    string::{CFStringGetCString, CFStringRef},
};
use io_kit_sys::{
    kIOMasterPortDefault,
    ret::kIOReturnSuccess,
    types::{io_iterator_t, io_registry_entry_t},
    IOServiceMatching, *,
};
use libc::{c_char, c_void};
use models::IoStats;
use std::ffi::CStr;
use std::io::Error;
use std::io::ErrorKind;

/// Clear the pointers for dict and release disk objects
unsafe fn release_c_ptr_iostats(
    parent_dict: *mut c_void,
    props_dict: *mut c_void,
    disk: io_registry_entry_t,
    parent: io_registry_entry_t,
) {
    CFRelease(parent_dict);
    CFRelease(props_dict);
    IOObjectRelease(disk);
    IOObjectRelease(parent);
}

/// Init the parent_dict and props_dict
unsafe fn init_dicts(
    disk: io_registry_entry_t,
    parent: io_registry_entry_t,
    parent_dict: usize,
    props_dict: usize,
) -> Result<(), Error> {
    // Create a snapchat of the registery entry (disk) creating a CFDictionary (parent_dict)
    if IORegistryEntryCreateCFProperties(disk, parent_dict as *mut _, kCFAllocatorDefault, 0)
        != kIOReturnSuccess
    {
        IOObjectRelease(disk);
        IOObjectRelease(parent);
        return Err(Error::last_os_error());
    }

    // Create a snapchat of the registery entry (parent) creating a CFDictionary (props_dict)
    if IORegistryEntryCreateCFProperties(parent, props_dict as *mut _, kCFAllocatorDefault, 0)
        != kIOReturnSuccess
    {
        release_c_ptr_iostats(
            parent_dict as *mut c_void,
            props_dict as *mut c_void,
            disk,
            parent,
        );
        return Err(Error::last_os_error());
    }

    Ok(())
}

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

    unsafe {
        let mut disk_list = std::mem::MaybeUninit::<io_iterator_t>::uninit();
        if IOServiceGetMatchingServices(
            kIOMasterPortDefault,
            IOServiceMatching(b"IOMedia\0".as_ptr() as *const c_char),
            disk_list.as_mut_ptr(),
        ) != kIOReturnSuccess
        {
            return Err(Error::last_os_error());
        }
        let disk_list = disk_list.assume_init();

        #[allow(unused_assignments)]
        let mut disk = IOIteratorNext(disk_list);
        while disk != 0 {
            let mut parent: io_registry_entry_t = 0;
            // Get the parent to which the registry (disk) was first attached to IOService
            if IORegistryEntryGetParentEntry(disk, b"IOService\0".as_ptr() as *mut i8, &mut parent)
                != kIOReturnSuccess
            {
                IOObjectRelease(disk);
                return Err(Error::last_os_error());
            }
            // Check if the object (parent) belong to the class or subclass of IOBlockStorageDriver
            if IOObjectConformsTo(parent, b"IOBlockStorageDriver\0".as_ptr() as *mut i8) == 0 {
                disk = IOIteratorNext(disk_list);
                continue;
            }

            // Null init the dict
            let mut parent_dict = std::mem::MaybeUninit::<CFDictionaryRef>::uninit();
            let mut props_dict = std::mem::MaybeUninit::<CFDictionaryRef>::uninit();
            // Init the dict in this function
            init_dicts(
                disk,
                parent,
                &mut parent_dict as *mut _ as *mut c_void as usize,
                &mut props_dict as *mut _ as *mut c_void as usize,
            )?;
            let parent_dict = parent_dict.assume_init();
            let props_dict = props_dict.assume_init();

            // Get the stats dictionnary if it exists
            let mut stats_dict = std::mem::MaybeUninit::<CFDictionaryRef>::uninit();
            if CFDictionaryGetValueIfPresent(
                props_dict as *mut _,
                CFSTR(b"Statistics\0".as_ptr() as *mut i8) as *mut c_void,
                &mut stats_dict as *mut _ as *mut *const c_void,
            ) == 0
            {
                release_c_ptr_iostats(
                    parent_dict as *mut c_void,
                    props_dict as *mut c_void,
                    disk,
                    parent,
                );
                return Err(Error::new(
                    ErrorKind::Other,
                    "CFDictionaryGetValueIfPresent: Statistics not found in the props_dict",
                ));
            }
            let stats_dict = stats_dict.assume_init();

            // Get the number of bytes read for the current disk
            let mut read_bytes = 0i64;
            let mut read_bytes_nbr = std::mem::MaybeUninit::<CFNumberRef>::uninit();
            if CFDictionaryGetValueIfPresent(
                stats_dict,
                CFSTR(b"Bytes (Read)\0".as_ptr() as *mut i8) as *mut c_void,
                read_bytes_nbr.as_mut_ptr() as *mut *const c_void,
            ) == 0
            {
                release_c_ptr_iostats(
                    parent_dict as *mut c_void,
                    props_dict as *mut c_void,
                    disk,
                    parent,
                );
                return Err(Error::new(
                    ErrorKind::Other,
                    "CFDictionaryGetValueIfPresent: Bytes Read not found in the stats_dict",
                ));
            }
            let number = read_bytes_nbr.assume_init();
            CFNumberGetValue(number, 4, &mut read_bytes as *mut _ as *mut c_void);

            // Get the number of bytes written for the current disk
            let mut write_bytes = 0i64;
            let mut write_bytes_nbr = std::mem::MaybeUninit::<CFNumberRef>::uninit();
            if CFDictionaryGetValueIfPresent(
                stats_dict,
                CFSTR(b"Bytes (Write)\0".as_ptr() as *mut i8) as *mut c_void,
                write_bytes_nbr.as_mut_ptr() as *mut *const c_void,
            ) == 0
            {
                release_c_ptr_iostats(
                    parent_dict as *mut c_void,
                    props_dict as *mut c_void,
                    disk,
                    parent,
                );
                return Err(Error::new(
                    ErrorKind::Other,
                    "CFDictionaryGetValueIfPresent: Bytes Write not found in the stats_dict",
                ));
            }
            let number = write_bytes_nbr.assume_init();
            CFNumberGetValue(number, 4, &mut write_bytes as *mut _ as *mut c_void);

            // Get the disk name (know as BSD Name)
            let mut disk_name_ref = std::mem::MaybeUninit::<CFStringRef>::uninit();
            if CFDictionaryGetValueIfPresent(
                parent_dict,
                CFSTR(b"BSD Name\0".as_ptr() as *mut i8) as *mut c_void,
                disk_name_ref.as_mut_ptr() as *mut *const c_void,
            ) == 0
            {
                release_c_ptr_iostats(
                    parent_dict as *mut c_void,
                    props_dict as *mut c_void,
                    disk,
                    parent,
                );
                return Err(Error::new(
                    ErrorKind::Other,
                    "CFDictionaryGetValueIfPresent: BSD Name not found in the parent_dict",
                ));
            }
            let disk_name_ref = disk_name_ref.assume_init();
            // Convert the CFString to String
            let mut name = [0i8; 64];
            if CFStringGetCString(disk_name_ref, name.as_mut_ptr(), 64, 134217984) == 0 {
                release_c_ptr_iostats(
                    parent_dict as *mut c_void,
                    props_dict as *mut c_void,
                    disk,
                    parent,
                );
                return Err(Error::new(
                    ErrorKind::Other,
                    "Cannot get the buffer filled to transform the name of the disk",
                ));
            }
            let name = match CStr::from_ptr(name.as_mut_ptr()).to_str() {
                Ok(val) => val.to_owned(),
                Err(_) => String::from("?"),
            };

            // Add the disk to the Vector of IoStats
            viostats.push(IoStats {
                device_name: name,
                bytes_read: read_bytes,
                bytes_wrtn: write_bytes,
            });

            // Release dicts used and disk
            release_c_ptr_iostats(
                parent_dict as *mut c_void,
                props_dict as *mut c_void,
                disk,
                parent,
            );
            // Go to the next disk
            disk = IOIteratorNext(disk_list);
        }
        IOObjectRelease(disk_list);
    }

    Ok(viostats)
}

/// Get basic [IoStats] info for each physical disks.
///
/// On linux it will get them from `/proc/diskstats` and filter the result based on the access to their `/sys/block/{}`.
///
/// On macOS it will use unsafes call and detect if the disk is marked as Removable, if it's not... it's a physical device
///
/// [IoStats]: ../struct.IoStats.html
pub fn get_iostats_physical() -> Result<Vec<IoStats>, Error> {
    let mut viostats: Vec<IoStats> = Vec::new();

    unsafe {
        let mut disk_list = std::mem::MaybeUninit::<io_iterator_t>::uninit();
        if IOServiceGetMatchingServices(
            kIOMasterPortDefault,
            IOServiceMatching(b"IOMedia\0".as_ptr() as *const c_char),
            disk_list.as_mut_ptr(),
        ) != kIOReturnSuccess
        {
            return Err(Error::last_os_error());
        }
        let disk_list = disk_list.assume_init();

        #[allow(unused_assignments)]
        let mut disk = IOIteratorNext(disk_list);
        while disk != 0 {
            let mut parent: io_registry_entry_t = 0;
            // Get the parent to which the registry (disk) was first attached to IOService
            if IORegistryEntryGetParentEntry(disk, b"IOService\0".as_ptr() as *mut i8, &mut parent)
                != kIOReturnSuccess
            {
                IOObjectRelease(disk);
                return Err(Error::last_os_error());
            }
            // Check if the object (parent) belong to the class or subclass of IOBlockStorageDriver
            if IOObjectConformsTo(parent, b"IOBlockStorageDriver\0".as_ptr() as *mut i8) == 0 {
                disk = IOIteratorNext(disk_list);
                continue;
            }

            // Null init the dict
            let mut parent_dict = std::mem::MaybeUninit::<CFDictionaryRef>::uninit();
            let mut props_dict = std::mem::MaybeUninit::<CFDictionaryRef>::uninit();
            // Init the dict in this function
            init_dicts(
                disk,
                parent,
                &mut parent_dict as *mut _ as *mut c_void as usize,
                &mut props_dict as *mut _ as *mut c_void as usize,
            )?;
            let parent_dict = parent_dict.assume_init();
            let props_dict = props_dict.assume_init();

            let mut removable_ref = std::mem::MaybeUninit::<CFBooleanRef>::uninit();
            if CFDictionaryGetValueIfPresent(
                parent_dict,
                CFSTR(b"Removable\0".as_ptr() as *mut i8) as *mut c_void,
                removable_ref.as_mut_ptr() as *mut *const c_void,
            ) == 0
            {
                release_c_ptr_iostats(
                    parent_dict as *mut c_void,
                    props_dict as *mut c_void,
                    disk,
                    parent,
                );
                return Err(Error::new(
                    ErrorKind::Other,
                    "CFDictionaryGetValueIfPresent: Removable not found in the parent_dict",
                ));
            }
            let removable_ref = removable_ref.assume_init();
            if CFBooleanGetValue(removable_ref) {
                release_c_ptr_iostats(
                    parent_dict as *mut c_void,
                    props_dict as *mut c_void,
                    disk,
                    parent,
                );
                disk = IOIteratorNext(disk_list);
                continue;
            }

            // Get the stats dictionnary if it exists
            let mut stats_dict = std::mem::MaybeUninit::<CFDictionaryRef>::uninit();
            if CFDictionaryGetValueIfPresent(
                props_dict as *mut _,
                CFSTR(b"Statistics\0".as_ptr() as *mut i8) as *mut c_void,
                &mut stats_dict as *mut _ as *mut *const c_void,
            ) == 0
            {
                release_c_ptr_iostats(
                    parent_dict as *mut c_void,
                    props_dict as *mut c_void,
                    disk,
                    parent,
                );
                return Err(Error::new(
                    ErrorKind::Other,
                    "CFDictionaryGetValueIfPresent: Statistics not found in the props_dict",
                ));
            }
            let stats_dict = stats_dict.assume_init();

            // Get the number of bytes read for the current disk
            let mut read_bytes = 0i64;
            let mut read_bytes_nbr = std::mem::MaybeUninit::<CFNumberRef>::uninit();
            if CFDictionaryGetValueIfPresent(
                stats_dict,
                CFSTR(b"Bytes (Read)\0".as_ptr() as *mut i8) as *mut c_void,
                read_bytes_nbr.as_mut_ptr() as *mut *const c_void,
            ) == 0
            {
                release_c_ptr_iostats(
                    parent_dict as *mut c_void,
                    props_dict as *mut c_void,
                    disk,
                    parent,
                );
                return Err(Error::new(
                    ErrorKind::Other,
                    "CFDictionaryGetValueIfPresent: Bytes Read not found in the stats_dict",
                ));
            }
            let number = read_bytes_nbr.assume_init();
            CFNumberGetValue(number, 4, &mut read_bytes as *mut _ as *mut c_void);

            // Get the number of bytes written for the current disk
            let mut write_bytes = 0i64;
            let mut write_bytes_nbr = std::mem::MaybeUninit::<CFNumberRef>::uninit();
            if CFDictionaryGetValueIfPresent(
                stats_dict,
                CFSTR(b"Bytes (Write)\0".as_ptr() as *mut i8) as *mut c_void,
                write_bytes_nbr.as_mut_ptr() as *mut *const c_void,
            ) == 0
            {
                release_c_ptr_iostats(
                    parent_dict as *mut c_void,
                    props_dict as *mut c_void,
                    disk,
                    parent,
                );
                return Err(Error::new(
                    ErrorKind::Other,
                    "CFDictionaryGetValueIfPresent: Bytes Write not found in the stats_dict",
                ));
            }
            let number = write_bytes_nbr.assume_init();
            CFNumberGetValue(number, 4, &mut write_bytes as *mut _ as *mut c_void);

            // Get the disk name (know as BSD Name)
            let mut disk_name_ref = std::mem::MaybeUninit::<CFStringRef>::uninit();
            if CFDictionaryGetValueIfPresent(
                parent_dict,
                CFSTR(b"BSD Name\0".as_ptr() as *mut i8) as *mut c_void,
                disk_name_ref.as_mut_ptr() as *mut *const c_void,
            ) == 0
            {
                release_c_ptr_iostats(
                    parent_dict as *mut c_void,
                    props_dict as *mut c_void,
                    disk,
                    parent,
                );
                return Err(Error::new(
                    ErrorKind::Other,
                    "CFDictionaryGetValueIfPresent: BSD Name not found in the parent_dict",
                ));
            }
            let disk_name_ref = disk_name_ref.assume_init();
            // Convert the CFString to String
            let mut name = [0i8; 64];
            if CFStringGetCString(disk_name_ref, name.as_mut_ptr(), 64, 134217984) == 0 {
                release_c_ptr_iostats(
                    parent_dict as *mut c_void,
                    props_dict as *mut c_void,
                    disk,
                    parent,
                );
                return Err(Error::new(
                    ErrorKind::Other,
                    "Cannot get the buffer filled to transform the name of the disk",
                ));
            }
            let name = match CStr::from_ptr(name.as_mut_ptr()).to_str() {
                Ok(val) => val.to_owned(),
                Err(_) => String::from("?"),
            };

            // Add the disk to the Vector of IoStats
            viostats.push(IoStats {
                device_name: name,
                bytes_read: read_bytes,
                bytes_wrtn: write_bytes,
            });

            // Release dicts used and disk
            release_c_ptr_iostats(
                parent_dict as *mut c_void,
                props_dict as *mut c_void,
                disk,
                parent,
            );
            // Go to the next disk
            disk = IOIteratorNext(disk_list);
        }
        IOObjectRelease(disk_list);
    }

    Ok(viostats)
}
