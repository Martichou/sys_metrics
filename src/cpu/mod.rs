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
