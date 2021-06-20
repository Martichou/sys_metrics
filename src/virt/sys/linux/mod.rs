use crate::virt::Virtualization;

mod containers;

pub fn get_virt_info() -> Option<Virtualization> {
    match containers::detect_wsl("/proc/sys/kernel/osrelease") {
        Ok(res) => Some(res),
        Err(_) => None,
    }
}
