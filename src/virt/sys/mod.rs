#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::*;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::*;

#[cfg(target_os = "linux")]
#[inline]
pub(crate) fn err_not_found() -> std::io::Error {
    std::io::Error::new(
        std::io::ErrorKind::Other,
        "Content of WSL's path doesn't match our criteria.",
    )
}
