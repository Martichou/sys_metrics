use crate::virt::Virtualization;

mod containers;
mod vm_dmi;

/// Get the virtualization information of the current host
///
/// Return Unknown if it cannot determine the Virtualization used (if any).
pub fn get_virt_info() -> Virtualization {
    containers::detect_openvz()
        .or_else(|_| containers::detect_wsl())
        .or_else(|_| containers::detect_systemd_container())
        .or_else(|_| vm_dmi::detect_vm_dmi())
        .map_or(Virtualization::Unknown, |res| res)
}
