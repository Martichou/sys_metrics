#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::*;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::*;

#[cfg(target_family = "unix")]
mod unix;
#[cfg(target_family = "unix")]
pub use unix::*;

use crate::models::LoadAvg;

#[inline]
#[cfg(target_os = "linux")]
pub(crate) fn get_loadavg_from_sysinfo(y: &libc::sysinfo) -> LoadAvg {
    LoadAvg {
        one: y.loads[0] as f64 / (1 << libc::SI_LOAD_SHIFT) as f64,
        five: y.loads[1] as f64 / (1 << libc::SI_LOAD_SHIFT) as f64,
        fifteen: y.loads[2] as f64 / (1 << libc::SI_LOAD_SHIFT) as f64,
    }
}
