mod sys;

pub use sys::*;

#[derive(Debug, Eq, PartialEq)]
pub enum Virtualization {
    // VMs
    /// Xen project (<https://xenproject.org/>)
    Xen,

    /// Kernel Virtual Machine (<https://www.linux-kvm.org>)
    Kvm,

    /// QEMU (<https://www.qemu.org/>)
    Qemu,

    /// VMware (<https://www.vmware.com>)
    Vmware,

    /// Oracle virtualization (<https://www.oracle.com/virtualization/>)
    Oracle,

    /// Bochs IA-32 emulator (<http://bochs.sourceforge.net/>)
    Bochs,

    /// Parallels (<https://www.parallels.com/>)
    Parallels,

    /// FreeBSD bhyve (<https://wiki.freebsd.org/bhyve>)
    Bhyve,

    // Containers
    /// OpenVz (<https://openvz.org/>)
    OpenVz,

    /// `lxc-libvirt` (<https://libvirt.org/drvlxc.html>)
    LxcLibvirt,

    /// Linux Containers (<https://linuxcontainers.org/lxc>)
    Lxc,

    /// `systemd-nspawn` container manager (<https://www.freedesktop.org/wiki/Software/systemd/>)
    SystemdNspawn,

    /// Docker (<https://www.docker.com/>)
    Docker,

    /// Podman (<https://podman.io/>)
    Podman,

    /// CoreOS rkt (<https://coreos.com/rkt/>)
    Rkt,

    /// Microsoft WSL (<https://docs.microsoft.com/en-us/windows/wsl/about>)
    Wsl,

    /// Unknown virtualization system.
    ///
    /// If you reach this result and you're sure you're in a virtualization system,
    /// open an issue on github to start working on a detection mechanisme.
    Unknown,
}
