//! `sys_metrics` is a crate used to get a system's information.
//!
//! It provide information about:
//!
//!  * CPU
//!  * Disks
//!  * Host
//!  * Memory
//!  * Network
//!  * Virtualization
//!
//! ## Quick start
//! ```
//! use sys_metrics::{cpu::*};
//!
//! // This is just a very basic example of the CPU part.
//! // Check the doc, this crate can do much more.
//! let cpufreq = get_cpufreq().unwrap();
//! println!("CPU Speed: {:13}MHz\n", cpufreq as u64);
//!
//! let cpu_logical = get_logical_count().unwrap();
//! let cpu_physical = get_physical_count().unwrap();
//!
//! println!("CPU Core: {:12}\nLogical processors: {}", cpu_physical, cpu_logical);
//!
//! let loadavg = get_loadavg().unwrap();
//! println!("Load average: {:10} {} {}", loadavg.one, loadavg.five, loadavg.fifteen);
//! ```

#[cfg(target_os = "linux")]
macro_rules! nth {
    ($a:expr, $b:expr) => {
        match $a.nth($b) {
            Some(val) => Ok(val),
            None => Err(Error::new(
                ErrorKind::Other,
                "The fields asked for does not exists",
            )),
        }
    };
}

/// CPU information
pub mod cpu;
/// Disks information
pub mod disks;
/// Host system information
pub mod host;
/// Memory information
pub mod memory;
/// Network information
pub mod network;
/// Virtualization information
pub mod virt;

#[cfg(target_os = "macos")]
pub mod macos_binding;
#[cfg(target_os = "macos")]
pub mod macos_utils;
#[cfg(target_os = "macos")]
pub(crate) use macos_binding as binding;
#[cfg(target_os = "macos")]
pub(crate) use macos_utils as utils;

use libc::c_char;
use std::ffi::CStr;
#[cfg(target_os = "linux")]
use std::fs;
use std::io::Error;

#[cfg(target_os = "macos")]
lazy_static::lazy_static! {
    static ref PAGE_SIZE: u64 = {
        unsafe {
            libc::sysconf(libc::_SC_PAGESIZE) as u64
        }
    };
}

/// Function used if you want to divide ticked value by the host's jiffies (USER_HZ) value. (like CpuStats)
///
/// See <https://en.wikipedia.org/wiki/Jiffy_(time)> for more information.
pub fn clock_ticks() -> Result<u64, Error> {
    let result = unsafe { libc::sysconf(libc::_SC_CLK_TCK) };

    if result > 0 {
        Ok(result as u64)
    } else {
        Err(Error::last_os_error())
    }
}

lazy_static::lazy_static! {
    /// Time units in jiffies (USER_HZ) (https://en.wikipedia.org/wiki/Jiffy_(time))
    pub(crate) static ref CLOCK_TICKS: u64 = clock_ticks().expect("Unable to determine CPU number of ticks per second");
}

/// Read from path to content, trim it and return the String
#[cfg(target_os = "linux")]
pub(crate) fn read_and_trim<P>(path: P) -> Result<String, Error>
where
    P: AsRef<std::path::Path>,
{
    let content = fs::read_to_string(path)?;
    Ok(content.trim().to_owned())
}

/// Convert c_char (string in C) to a str in Rust
#[inline]
pub(crate) fn to_str<'a>(s: *const c_char) -> &'a str {
    unsafe {
        let res = CStr::from_ptr(s).to_bytes();
        std::str::from_utf8_unchecked(res)
    }
}
