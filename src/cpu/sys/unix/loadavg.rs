use crate::models;

use libc::{c_double, getloadavg};
use models::LoadAvg;
use std::io::Error;

/// Returns the [LoadAvg] over the last 1, 5 and 15 minutes.
///
/// In Linux, the [LoadAvg] is technically believed to be a running average
/// of processes in it’s (kernel) execution queue tagged as running or uninterruptible.
///
/// # Exemples
/// ```
/// use sys_metrics::LoadAvg;
/// use sys_metrics::cpu::get_loadavg;
///
/// let loadavg: LoadAvg = match get_loadavg() {
///     Ok(val) => val,
///     Err(x) => panic!(x),
/// };
///
/// // Should print your system load avg
/// println!("{:?}", loadavg);
/// ```
///
/// [LoadAvg]: ../struct.LoadAvg.html
pub fn get_loadavg() -> Result<LoadAvg, Error> {
    let mut data: [c_double; 3] = [0.0, 0.0, 0.0];

    if unsafe { getloadavg(data.as_mut_ptr(), 3) } == -1 {
        return Err(Error::last_os_error());
    }

    Ok(LoadAvg {
        one: data[0],
        five: data[1],
        fifteen: data[2],
    })
}
