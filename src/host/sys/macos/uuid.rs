use crate::to_str;

use core_foundation_sys::{
    base::{kCFAllocatorDefault, CFRelease, CFTypeRef},
    string::{CFStringGetCString, CFStringRef},
};
use io_kit_sys::*;
use io_kit_sys::{kIOMasterPortDefault, keys::kIOPlatformUUIDKey, IOServiceMatching};
use libc::c_char;
use libc::c_void;
use std::io::{Error, ErrorKind};

/// Get the machine UUID of the host.
///
/// On linux it will read it from /etc/machine-id or /var/lib/dbus/machine-id.
///
/// On macOS it will use unsafe call to OSX specific function.
pub fn get_uuid() -> Result<String, Error> {
    #[allow(unused_assignments)]
    let uuid: CFStringRef;

    unsafe {
        let platform_expert = IOServiceGetMatchingService(
            kIOMasterPortDefault,
            IOServiceMatching(b"IOPlatformExpertDevice\0".as_ptr() as *const c_char),
        );
        if platform_expert != 0 {
            let uuid_ascfstring: CFTypeRef = IORegistryEntryCreateCFProperty(
                platform_expert,
                CFSTR(kIOPlatformUUIDKey),
                kCFAllocatorDefault,
                0,
            );
            if !uuid_ascfstring.is_null() {
                uuid = uuid_ascfstring as CFStringRef;
            } else {
                return Err(Error::new(ErrorKind::Other, "Cannot get uuid_ascfstring"));
            }
            IOObjectRelease(platform_expert);
        } else {
            return Err(Error::last_os_error());
        }

        let mut buffer = [0i8; 37];
        if CFStringGetCString(uuid, buffer.as_mut_ptr(), 37, 134217984) == 0 {
            return Err(Error::new(ErrorKind::Other, "Cannot get the buffer filled"));
        }
        CFRelease(uuid as *mut c_void);

        Ok(to_str(buffer.as_ptr()).to_owned())
    }
}
