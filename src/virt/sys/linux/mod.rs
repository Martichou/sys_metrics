use crate::virt::Virtualization;

mod containers;

/// Get the virtualization information of the current host
///
/// Return Unknown if it cannot determine the Virtualization used (if any).
pub fn get_virt_info() -> Virtualization {
    containers::detect_openvz()
        .or_else(|_| containers::detect_wsl())
        .or_else(|_| containers::detect_systemd_container())
        .map_or(Virtualization::Unknown, |res| res)
}
