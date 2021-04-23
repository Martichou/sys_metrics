use crate::read_and_trim;

use std::io::Error;

/// Get the machine UUID of the host.
///
/// On linux it will read it from /etc/machine-id or /var/lib/dbus/machine-id.
pub fn get_uuid() -> Result<String, Error> {
    match read_and_trim("/etc/machine-id") {
        Ok(machine_id) => Ok(machine_id),
        Err(_) => Ok(read_and_trim("/var/lib/dbus/machine-id")?),
    }
}
