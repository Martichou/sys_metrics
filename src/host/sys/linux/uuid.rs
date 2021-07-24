use crate::read_and_trim;

use std::io::Error;

/// Get the machine UUID of the host.
///
/// On linux it will read it from /etc/machine-id or /var/lib/dbus/machine-id.
/// Be careful when transmitting machine-id over the network (https://man7.org/linux/man-pages/man5/machine-id.5.html).
pub fn get_uuid() -> Result<String, Error> {
    match read_and_trim("/etc/machine-id") {
        Ok(machine_id) => match machine_id.is_empty() {
            false => Ok(machine_id),
            true => Ok(read_and_trim("/var/lib/dbus/machine-id")?),
        },
        Err(_) => Ok(read_and_trim("/var/lib/dbus/machine-id")?),
    }
}
