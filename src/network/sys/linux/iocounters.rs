use crate::network::IoCounters;

use std::{
    fs::File,
    io::{BufRead, BufReader, Error, ErrorKind},
};

/// Return the [IoCounters] struct.
///
/// It will get the info from `/proc/net/dev`.
///
/// [IoCounters]: ../network/struct.IoCounters.html
pub fn get_net_iocounters() -> Result<Vec<IoCounters>, Error> {
    let file = File::open("/proc/net/dev")?;
    let mut file = BufReader::with_capacity(2048, file);

    let mut line_skip = 0;
    let mut line = String::with_capacity(512);
    let mut v_iocounters: Vec<IoCounters> = Vec::new();
    while file.read_line(&mut line)? != 0 {
        line_skip += 1;
        if line_skip < 3 {
            line.clear();
            continue;
        }
        let mut parts = line.splitn(2, ':');
        let interface = match parts.next() {
            Some(str) => str.trim().to_string(),
            None => {
                line.clear();
                continue;
            }
        };
        let mut parts = match parts.next() {
            Some(rest) => rest.split_whitespace(),
            None => {
                return Err(Error::new(
                    ErrorKind::Other,
                    "Fatal error, /proc/net/dev parsing failed",
                ));
            }
        };

        let rx_bytes = parts.next().unwrap_or("0").parse::<u64>().unwrap();
        let rx_packets = parts.next().unwrap_or("0").parse::<u64>().unwrap();
        let rx_errs = parts.next().unwrap_or("0").parse::<u64>().unwrap();
        let rx_drop = parts.next().unwrap_or("0").parse::<u64>().unwrap();
        let mut parts = parts.skip(4);
        let tx_bytes = parts.next().unwrap_or("0").parse::<u64>().unwrap();
        let tx_packets = parts.next().unwrap_or("0").parse::<u64>().unwrap();
        let tx_errs = parts.next().unwrap_or("0").parse::<u64>().unwrap();
        let tx_drop = parts.next().unwrap_or("0").parse::<u64>().unwrap();

        v_iocounters.push(IoCounters {
            interface,
            rx_bytes,
            rx_packets,
            rx_errs,
            rx_drop,
            tx_bytes,
            tx_packets,
            tx_errs,
            tx_drop,
        });
        line.clear();
    }

    Ok(v_iocounters)
}
