mod sys;

pub use sys::*;

#[derive(Debug, Eq, PartialEq)]
pub enum Virtualization {
    // VMs
    /// TODO

    // Containers
    /// OpenVz (https://openvz.org/)
    OpenVz,

    /// `lxc-libvirt` (https://libvirt.org/drvlxc.html)
    LxcLibvirt,

    /// Linux Containers (https://linuxcontainers.org/lxc)
    Lxc,

    /// `systemd-nspawn` container manager (https://www.freedesktop.org/wiki/Software/systemd/)
    SystemdNspawn,

    /// Docker (https://www.docker.com/)
    Docker,

    /// Podman (https://podman.io/)
    Podman,

    /// CoreOS rkt (https://coreos.com/rkt/)
    Rkt,

    /// Microsoft WSL (https://docs.microsoft.com/en-us/windows/wsl/about)
    Wsl,

    /// Unknown virtualization system.
    ///
    /// If you reach this result and you're sure you're in a virtualization system,
    /// open an issue on github to start working on a detection mechanisme.
    Unknown,
}
