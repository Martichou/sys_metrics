use serde::{Deserialize, Serialize};

mod sys;

pub use sys::*;

/// Struct containing the IO counters for the network interfaces.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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
