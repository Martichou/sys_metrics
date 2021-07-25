// Based on https://github.com/heim-rs/heim/blob/master/heim-virt/src/sys/linux/containers.rs

use crate::virt::Virtualization;

use std::{
    fs::File,
    io::{BufRead, BufReader, Error, ErrorKind},
    path::Path,
};

#[inline]
fn err_not_found() -> Error {
    Error::new(
        ErrorKind::Other,
        "Content of WSL's path doesn't match our criteria.",
    )
}

fn try_guess_container(value: &str) -> Result<Virtualization, Error> {
    match value {
        "lxc" => Ok(Virtualization::Lxc),
        "lxc-libvirt" => Ok(Virtualization::LxcLibvirt),
        "systemd-nspawn" => Ok(Virtualization::SystemdNspawn),
        "docker" => Ok(Virtualization::Docker),
        "podman" => Ok(Virtualization::Podman),
        "rkt" => Ok(Virtualization::Rkt),
        "wsl" => Ok(Virtualization::Wsl),
        _ => Err(err_not_found()),
    }
}

pub(crate) fn detect_wsl() -> Result<Virtualization, Error> {
    let line = std::fs::read_to_string("/proc/sys/kernel/osrelease")?;

    match line {
        ref probe if probe.contains("Microsoft") => Ok(Virtualization::Wsl),
        ref probe if probe.contains("WSL") => Ok(Virtualization::Wsl),
        _ => Err(err_not_found()),
    }
}

pub(crate) fn detect_openvz() -> Result<Virtualization, Error> {
    let f1 = Path::new("/proc/vz").exists();
    let f2 = Path::new("/proc/bc").exists();

    match (f1, f2) {
        // `/proc/vz` exists in container and outside of the container,
        // `/proc/bc` only outside of the container.
        (true, false) => Ok(Virtualization::OpenVz),
        _ => Err(err_not_found()),
    }
}

pub(crate) fn detect_systemd_container() -> Result<Virtualization, Error> {
    // systemd PID 1 might have dropped this information into a file in `/run`.
    // This is better than accessing `/proc/1/environ`,
    // since we don't need `CAP_SYS_PTRACE` for that.
    let path = Path::new("/run/systemd/container");
    // If the path doesn't exist, no need to continue.
    // Doing so is faster than letting File::open fail (by 15%)
    // in case where the file doesn't exist.
    if !path.exists() {
        return Err(err_not_found());
    }
    let file = File::open("/run/systemd/container")?;
    let mut file = BufReader::with_capacity(512, file);
    // Construct a String using 64 chars line capacity.
    let mut line = String::with_capacity(64);
    // Read the first line into the line "buffer"
    file.read_line(&mut line)?;
    // Return a guess about the container of an Error
    try_guess_container(&line)
}
