use serde::{Deserialize, Serialize};

mod sys;

pub use sys::*;

/// Struct containing the memory virtual information.
///
/// All values are in MB.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Memory {
    pub total: u64,
    pub free: u64,
    pub used: u64,
    pub shared: u64,
    pub buffers: u64,
    pub cached: u64,
}

#[cfg(target_os = "linux")]
impl Memory {
    /// Return the Memory struct with used defined based on other values.
    pub(crate) fn set_used(mut self) -> Self {
        self.used = self.total - (self.free + self.buffers + self.cached);

        self
    }
}

/// Struct containing the memory swap information.
///
/// All values are in MB.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Swap {
    pub total: u64,
    pub free: u64,
    pub used: u64,
}
