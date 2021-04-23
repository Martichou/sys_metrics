mod sys;

pub use sys::*;

use serde::Serialize;
/// Struct containing the memory (ram/swap) information.
#[derive(Debug, Clone, Serialize, Default)]
pub struct Memory {
    pub total: u64,
    pub free: u64,
    pub used: u64,
    pub shared: u64,
    pub buffers: u64,
    pub cached: u64,
}

impl Memory {
    /// Return the Memory struct with used defined based on other values.
    pub(crate) fn set_used(mut self) -> Self {
        self.used = self.total - (self.free + self.buffers + self.cached);

        self
    }
}

/// Struct containing the memory swap information.
#[derive(Debug, Clone, Serialize)]
pub struct Swap {
    pub total: u64,
    pub free: u64,
    pub used: u64,
}

#[cfg(target_os = "macos")]
use mach::vm_types::natural_t;

#[doc(hidden)]
#[cfg(target_os = "macos")]
#[repr(C)]
pub struct vm_statistics64 {
    pub free_count: natural_t,
    pub active_count: natural_t,
    pub inactive_count: natural_t,
    pub wire_count: natural_t,
    pub zero_fill_count: u64,
    pub reactivations: u64,
    pub pageins: u64,
    pub pageouts: u64,
    pub faults: u64,
    pub cow_faults: u64,
    pub lookups: u64,
    pub hits: u64,
    pub purges: u64,
    pub purgeable_count: natural_t,
    pub speculative_count: natural_t,
    pub decompressions: u64,
    pub compressions: u64,
    pub swapins: u64,
    pub swapouts: u64,
    pub compressor_page_count: natural_t,
    pub throttled_count: natural_t,
    pub external_page_count: natural_t,
    pub internal_page_count: natural_t,
    pub total_uncompressed_pages_in_compressor: u64,
}
