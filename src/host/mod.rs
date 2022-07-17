use crate::cpu::LoadAvg;

use serde::{Deserialize, Serialize};

mod sys;

pub use sys::*;

/// Struct containing the principal host's information.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HostInfo {
    pub loadavg: LoadAvg,
    pub system: String,
    pub os_version: String,
    pub kernel_version: String,
    pub hostname: String,
    pub uptime: u64,
}
