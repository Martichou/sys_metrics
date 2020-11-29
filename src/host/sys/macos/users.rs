use crate::to_str;

use libc::{getutxent, setutxent, utmpx};
use std::io::Error;

/// Get the currently logged users.
///
/// On linux it will get them from `/var/run/utmp`. It will use the C's UTMP Struct and the unsafe read C's function.
///
/// On macOS it will use unsafes call to multiple OSX specific functions [setutxent, getutxent] (the struct is UTMPX for the inner usage).
pub fn get_users() -> Result<Vec<String>, Error> {
    let mut users: Vec<String> = Vec::new();

    unsafe { setutxent() };
    let mut buffer = unsafe { getutxent() };
    while !buffer.is_null() {
        let cbuffer = unsafe { &*(buffer as *mut utmpx) as &utmpx };
        let cuser = unsafe { &*(&cbuffer.ut_user as *const [i8]) };

        if cuser[0] != 0 && cbuffer.ut_type == 7 {
            let csuser = to_str(cuser.as_ptr()).trim_matches('\0').to_owned();
            if !users.contains(&csuser) {
                users.push(csuser);
            }
        }
        buffer = unsafe { getutxent() };
    }

    Ok(users)
}
