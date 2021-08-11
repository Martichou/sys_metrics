use crate::disks::IoBlock;
use crate::utils::KeyVal;

use core_foundation_sys::{
    base::{kCFAllocatorDefault, CFRelease},
    dictionary::CFDictionaryRef,
};
use io_kit_sys::{
    kIOMasterPortDefault,
    ret::kIOReturnSuccess,
    types::{io_iterator_t, io_registry_entry_t},
    IOServiceMatching, *,
};
use libc::{c_char, c_void};
use std::io::Error;

/// Clear the pointers for dict and release disk objects
unsafe fn release_c_ptr_ioblocks(
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
        release_c_ptr_ioblocks(
            parent_dict as *mut c_void,
            props_dict as *mut c_void,
            disk,
            parent,
        );
        return Err(Error::last_os_error());
    }

    Ok(())
}

#[inline]
unsafe fn _get_ioblocks(physical: bool) -> Result<Vec<IoBlock>, Error> {
    let mut v_ioblocks: Vec<IoBlock> = Vec::new();

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

        if physical {
            let is_one = parent_dict.get_bool("Removable\0")?;
            if is_one {
                release_c_ptr_ioblocks(
                    parent_dict as *mut c_void,
                    props_dict as *mut c_void,
                    disk,
                    parent,
                );
                disk = IOIteratorNext(disk_list);
                continue;
            }
        }

        // Get the value we're interested in from the stats_dict
        let result = || -> Result<IoBlock, Error> {
            // Get the stats dictionnary if it exists
            let stats_dict = props_dict.get_dict("Statistics\0")?;
            // Get the values from the stats_dict
            let read_count = stats_dict.get_i64("Operations (Read)\0")? as u64;
            let read_bytes = stats_dict.get_i64("Bytes (Read)\0")? as u64;
            let write_count = stats_dict.get_i64("Operations (Write)\0")? as u64;
            let write_bytes = stats_dict.get_i64("Bytes (Write)\0")? as u64;
            let busy_time = (stats_dict.get_i64("Total Time (Read)\0")?
                + stats_dict.get_i64("Total Time (Write)\0")?) as u64;
            let device_name = parent_dict.get_string("BSD Name\0")?;

            Ok(IoBlock {
                device_name,
                read_count,
                read_bytes,
                write_count,
                write_bytes,
                busy_time,
            })
        };

        let curr_io = match result() {
            Ok(val) => val,
            Err(err) => {
                release_c_ptr_ioblocks(
                    parent_dict as *mut c_void,
                    props_dict as *mut c_void,
                    disk,
                    parent,
                );
                return Err(err);
            }
        };

        // Add the disk to the Vector of IoBlocks
        v_ioblocks.push(curr_io);

        // Release dicts used and disk
        release_c_ptr_ioblocks(
            parent_dict as *mut c_void,
            props_dict as *mut c_void,
            disk,
            parent,
        );
        // Go to the next disk
        disk = IOIteratorNext(disk_list);
    }
    IOObjectRelease(disk_list);

    Ok(v_ioblocks)
}

/// Get basic [IoBlock] (physical and virtual) info for each disks/partitions.
///
/// [IoBlock]: ../struct.IoBlock.html
pub fn get_ioblocks() -> Result<Vec<IoBlock>, Error> {
    unsafe { _get_ioblocks(false) }
}

/// Get basic [IoBlock] (physical) info for each physical disks.
///
/// [IoBlock]: ../disks/struct.IoBlock.html
pub fn get_physical_ioblocks() -> Result<Vec<IoBlock>, Error> {
    unsafe { _get_ioblocks(true) }
}
