mod sys;

pub use sys::*;

use crate::cpu::LoadAvg;
use serde::Serialize;

/// Struct containing the principal host's information.
#[derive(Debug, Clone, Serialize)]
pub struct HostInfo {
    pub loadavg: LoadAvg,
    pub system: String,
    pub os_version: String,
    pub hostname: String,
    pub uptime: u64,
}
