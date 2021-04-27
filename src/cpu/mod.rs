mod sys;

pub use sys::*;

use serde::Serialize;

/// Struct containing a cpu's loadavg information.
#[derive(Debug, Clone, Serialize)]
pub struct LoadAvg {
    pub one: f64,
    pub five: f64,
    pub fifteen: f64,
}

/// Struct containing cpu times information.
#[derive(Debug, Clone, Default, Serialize)]
pub struct CpuTimes {
    pub user: u64,
    pub nice: u64,
    pub system: u64,
    pub idle: u64,
    pub iowait: u64,
    pub irq: u64,
    pub softirq: u64,
    pub steal: u64,
    pub guest: u64,
    pub guest_nice: u64,
}

impl CpuTimes {
    /// Return the amount of time the system CPU as been busy
    pub fn busy_time(&self) -> u64 {
        self.user + self.nice + self.system + self.irq + self.softirq + self.steal
    }

    /// Return the amount of time the system CPU as been idling
    pub fn idle_time(&self) -> u64 {
        self.idle + self.iowait
    }

    /// Return the total amount of time of the CPU since boot
    pub fn total_time(&self) -> u64 {
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
impl From<host_cpu_load_info> for CpuTimes {
    fn from(info: host_cpu_load_info) -> CpuTimes {
        CpuTimes {
            // Convert to u64 is pretty safe as info.user is a u32 at first
            user: info.user.into(),
            system: info.system.into(),
            idle: info.idle.into(),
            nice: info.nice.into(),
            ..Default::default()
        }
    }
}
/// Struct containing cpu stats information.
///
/// TODO - Details what each interrupts are:
/// - intr contains a LOT of different interrupts, might be worth detailling the important one
/// - softirq contains 10 types of softirq
#[derive(Debug, Clone, Default, Serialize)]
pub struct CpuStats {
    pub interrupts: u64,
    pub ctx_switches: u64,
    pub soft_interrupts: u64,
}
