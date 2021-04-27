use crate::to_str;

use std::io::Error;

/// Return the users actually connected to the hosts (Only the name for now).
///
/// Utmp handling is the same on all platform, but the structure can be different.
/// Even if the following code is platform agnostic, this version is much slower on Linux.
/// That's why Linux still have it's own version of get_users().
/// Be carefull that the Error is only on Linux. If an error occur on other platform, it will return and empty Vec.
///
/// Added the Result to keep the same prototype as the Linux version.
pub fn get_users() -> Result<Vec<String>, Error> {
    let mut users = Vec::new();
    unsafe {
        libc::setutxent();
        loop {
            let entry = libc::getutxent();
            if entry.is_null() {
                break;
            }

            if (*entry).ut_type != libc::USER_PROCESS {
                continue;
            }

            users.push(to_str((*entry).ut_user.as_ptr()).to_owned());
        }
        libc::endutxent();
    }

    Ok(users)
}
