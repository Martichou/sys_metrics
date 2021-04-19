use std::io::Error;
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
pub fn get_cpufreq() -> Result<f64, Error> {
    let file = File::open("/proc/cpuinfo")?;
    let mut file = BufReader::with_capacity(1024, file);

    let mut line = String::with_capacity(256);
    while file.read_line(&mut line)? != 0 {
        let lenght = line.len();
        if lenght > 7 && lenght < 48 && &line[..7] == "cpu MHz" {
            match line[11..lenght - 1].parse::<f64>() {
                Ok(val) => return Ok(val),
                Err(_) => {
                    line.clear();
                    continue;
                }
            };
        }
        line.clear();
    }

    Err(Error::new(ErrorKind::Other, "Couldn't get the cpufreq"))
}
