mod sys;

pub use sys::*;

use serde::Serialize;

/// Struct containing the IO counters for the network interfaces.
#[derive(Debug, Clone, Serialize, Default)]
pub struct IoNet {
    pub interface: String,
    pub rx_bytes: u64,
    pub rx_packets: u64,
    pub rx_errs: u64,
    pub rx_drop: u64,
    pub tx_bytes: u64,
    pub tx_packets: u64,
    pub tx_errs: u64,
    pub tx_drop: u64,
}
