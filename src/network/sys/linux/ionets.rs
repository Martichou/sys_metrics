use crate::network::IoNet;

use std::{
    fs::File,
    io::{BufRead, BufReader, Error, ErrorKind},
    path::Path,
};

#[inline]
fn _get_ionets(physical: bool) -> Result<Vec<IoNet>, Error> {
    let file = File::open("/proc/net/dev")?;
    let mut v_ionets: Vec<IoNet> = Vec::new();
    let mut file = BufReader::with_capacity(2048, file);

    let mut line_skip = 0;
    let mut line = String::with_capacity(256);
    while file.read_line(&mut line)? != 0 {
        line_skip += 1;
        if line_skip < 3 {
            line.clear();
            continue;
        }
        let mut parts = line.splitn(2, ':');
        let interface = match parts.next() {
            Some(str) => str.trim(),
            None => {
                line.clear();
                continue;
            }
        };

        if physical && Path::new(&format!("/sys/devices/virtual/net/{}", interface)).exists() {
            line.clear();
            continue;
        }

        let interface = interface.to_owned();
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

        v_ionets.push(IoNet {
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

    Ok(v_ionets)
}

/// Return the [IoNet] struct.
///
/// [IoNet]: ../network/struct.IoNet.html
pub fn get_ionets() -> Result<Vec<IoNet>, Error> {
    _get_ionets(false)
}

/// Return the [IoNet] struct but keeping only those from Physical Interfaces.
///
/// [IoNet]: ../network/struct.IoNet.html
pub fn get_physical_ionets() -> Result<Vec<IoNet>, Error> {
    _get_ionets(true)
}
