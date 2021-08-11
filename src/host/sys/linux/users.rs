use crate::to_str;

use libc::{c_char, c_void, read, utmpx};
use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
    mem,
    os::unix::prelude::*,
};

/// Get the currently logged users.
///
/// Will get them from `/var/run/utmp`. If the file does not exist, it will return and Error.
/// Be carefull that the Error is only on Linux (and only if /var/run/utmp is not found).
pub fn get_logged_users() -> Result<Vec<String>, Error> {
    let utmp_file = File::open("/var/run/utmp")?;
    let mut users: Vec<String> = Vec::new();
    let buffer = std::mem::MaybeUninit::<utmpx>::uninit().as_mut_ptr();

    while unsafe {
        read(
            utmp_file.as_raw_fd(),
            buffer as *mut c_void,
            mem::size_of::<utmpx>(),
        )
    } != 0
    {
        let cbuffer = unsafe { &*(buffer as *mut utmpx) as &utmpx };
        let cuser = unsafe { &*(&cbuffer.ut_user as *const [c_char]) };

        if cuser[0] != 0 && cbuffer.ut_type == libc::USER_PROCESS {
            let csuser = to_str(cuser.as_ptr() as *const c_char)
                .trim_matches('\0')
                .to_owned();
            users.push(csuser);
        }
    }

    Ok(users)
}

// Range of user IDs used for the creation of regular users by useradd or newusers.
//
// The default value for UID_MIN (resp.  UID_MAX) is 1000 (resp. 60000).
const UID_MIN: u16 = 1000;
const UID_MAX: u16 = 60000;

/// Get the list of real users.
///
/// Read them from /etc/passwd and extract 'real users' based on their ID
/// following convention from /etc/login.defs (UID_MIN & UID_MAX).
pub fn get_users() -> Result<Vec<String>, Error> {
    let file = File::open("/etc/passwd")?;
    let mut users: Vec<String> = Vec::new();
    let mut file = BufReader::with_capacity(2048, file);

    let mut line = String::with_capacity(128);
    while file.read_line(&mut line)? != 0 {
        // Split the line with the ':' sep and to get 4 parts (username:x:gid:rest)
        let mut parts = line.splitn(4, ':');
        let username = match parts.next() {
            // Don't do name.to_owned yet to avoid an allocation
            Some(name) => name,
            None => {
                line.clear();
                continue;
            }
        };
        // Check if the id (3th elements) is between UID_MIN and UID_MAX
        if let Some(id) = parts.nth(1) {
            if let Ok(parsed) = id.parse::<u16>() {
                // If the value is not between our range, we clear and continue
                if (UID_MIN..UID_MAX).contains(&parsed) {
                    // If we reach this parts, it means we can assume the username is a real user
                    users.push(username.to_owned());
                }
            }
        }

        line.clear();
    }

    Ok(users)
}
