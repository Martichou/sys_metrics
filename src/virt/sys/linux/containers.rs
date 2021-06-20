use crate::virt::Virtualization;

use std::io::Error;
use std::path::Path;

pub fn detect_wsl<T>(path: T) -> Result<Virtualization, Error>
where
    T: AsRef<Path>,
{
    let line = std::fs::read_to_string(path)?;

    match line {
        ref probe if probe.contains("Microsoft") => Ok(Virtualization::Wsl),
        ref probe if probe.contains("WSL") => Ok(Virtualization::Wsl),
        _ => Ok(Virtualization::Unknown),
    }
}
