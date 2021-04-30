use crate::network::IoCounters;

use std::io::{Error, ErrorKind};

/// Return the [IoCounters] struct.
///
/// [IoCounters]: ../network/struct.IoCounters.html
pub fn get_net_iocounters() -> Result<Vec<IoCounters>, Error> {
    todo!()
}
