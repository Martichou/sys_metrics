use crate::models;

#[cfg(target_family = "unix")]
use libc::{c_double, getloadavg};
#[cfg(target_os = "macos")]
use libc::{c_uint, c_void, sysctl};
#[cfg(target_family = "unix")]
use models::LoadAvg;
#[cfg(target_family = "unix")]
use std::io::Error;
#[cfg(target_os = "linux")]
use std::{
    fs::File,
    io::{prelude::*, BufReader, ErrorKind},
};

/// Get the cpufreq as f64.
///
/// On linux it will return the first frequency it see from `/proc/cpuinfo` (key: cpu MHz).
///
/// And on macOS it will make a syscall which will return the cpufreq (macOS doesn't seems to have per-core clock).
///
/// # Exemples
/// ```
/// use sys_metrics::cpu::get_cpufreq;
///
/// let cpufreq: f64 = match get_cpufreq() {
///     Ok(val) => val,
///     Err(x) => panic!(x),
/// };
///
/// // Should print your cpufreq as mHz
/// println!("{}", cpufreq);
/// ```
#[cfg(target_os = "linux")]
pub fn get_cpufreq() -> Result<f64, Error> {
    let file = File::open("/proc/cpuinfo")?;
    let file = BufReader::with_capacity(1024, file);

    for line in file.lines() {
        let line = line.unwrap();
        let lenght = line.len();
        if lenght > 7 && lenght < 48 && &line[..7] == "cpu MHz" {
            match line[11..lenght - 1].parse::<f64>() {
                Ok(val) => return Ok(val),
                Err(_) => continue,
            };
        }
    }

    Err(Error::new(
        ErrorKind::Other,
        "Couldn't get the avg_cpu_freq",
    ))
}

#[cfg(target_os = "macos")]
pub fn get_cpufreq() -> Result<f64, Error> {
    let mut data: c_uint = 0;
    let mib = [6, 15];

    if unsafe {
        sysctl(
            &mib[0] as *const _ as *mut _,
            mib.len() as u32,
            &mut data as *mut _ as *mut c_void,
            &mut std::mem::size_of::<c_uint>(),
            std::ptr::null_mut(),
            0,
        )
    } < 0
    {
        return Err(Error::last_os_error());
    }

    Ok(data as f64)
}

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
#[cfg(target_family = "unix")]
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
