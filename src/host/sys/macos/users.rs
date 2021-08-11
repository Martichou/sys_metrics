use crate::to_str;

use libc::{getutxent, setutxent, utmpx};
use std::io::Error;

/// Get the currently logged users.
///
/// Use unsafe calls to multiple Unix specific functions [setutxent, getutxent].
pub fn get_logged_users() -> Result<Vec<String>, Error> {
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

/// TODO - Return empty [] for now
pub fn get_users() -> Result<Vec<String>, Error> {
    Ok(vec![])
}
