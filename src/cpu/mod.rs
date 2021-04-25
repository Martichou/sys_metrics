mod sys;

pub use sys::*;

use crate::cpu::CLOCK_TICKS;
use serde::Serialize;

/// Struct containing a cpu's loadavg information.
#[derive(Debug, Clone, Serialize)]
pub struct LoadAvg {
    pub one: f64,
    pub five: f64,
    pub fifteen: f64,
}

/// Struct containing cpu stat information.
#[derive(Debug, Clone, Default, Serialize)]
pub struct CpuStats {
    pub user: i64,
    pub nice: i64,
    pub system: i64,
    pub idle: i64,
    pub iowait: i64,
    pub irq: i64,
    pub softirq: i64,
    pub steal: i64,
    pub guest: i64,
    pub guest_nice: i64,
}

impl CpuStats {
    /// Return the amount of time the system CPU as been busy
    pub fn busy_time(&self) -> i64 {
        self.user + self.nice + self.system + self.irq + self.softirq + self.steal
    }

    /// Return the amount of time the system CPU as been idling
    pub fn idle_time(&self) -> i64 {
        self.idle + self.iowait
    }

    /// Return the total amount of time of the CPU since boot
    pub fn total_time(&self) -> i64 {
        self.busy_time() + self.idle_time()
    }
}

#[cfg(target_os = "macos")]
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub(crate) struct host_cpu_load_info {
    user: mach::vm_types::natural_t,
    system: mach::vm_types::natural_t,
    idle: mach::vm_types::natural_t,
    nice: mach::vm_types::natural_t,
}

#[cfg(target_os = "macos")]
impl From<host_cpu_load_info> for CpuStats {
	fn from(info: host_cpu_load_info) -> CpuStats {
		let ticks = *CLOCK_TICKS;

		CpuStats {
            // Convert to i64 is pretty safe as info.user is a u32 at first
            // we might be missing on the float part of the division...
			user: (info.user as u64 / ticks) as i64,
			system: (info.system as u64 / ticks) as i64,
			idle: (info.idle as u64 / ticks) as i64,
			nice: (info.nice as u64 / ticks) as i64,
            ..Default::default()
		}
	}
}