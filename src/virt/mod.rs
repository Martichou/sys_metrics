mod sys;

pub use sys::*;

#[derive(Debug, Eq, PartialEq)]
pub enum Virtualization {
    /// Microsoft WSL (https://docs.microsoft.com/en-us/windows/wsl/about)
    Wsl,

    /// Unknown virtualization system.
    ///
    /// If you reach this result and you're sure you're in a virtualization system,
    /// open an issue on github.
    Unknown,
}
