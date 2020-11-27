use crate::to_str;

use libc::{getutxent, setutxent, utmpx};
use std::io::Error;
use std::mem;

const _UTX_USERSIZE: usize = 256;
const _UTX_LINESIZE: usize = 32;
const _UTX_IDSIZE: usize = 4;
const _UTX_HOSTSIZE: usize = 256;

#[doc(hidden)]
#[repr(C)]
#[derive(Debug)]
pub struct ut_tv {
    pub tv_sec: i32,
    pub tv_usec: i32,
}

impl Default for ut_tv {
    fn default() -> ut_tv {
        ut_tv {
            tv_sec: 0,
            tv_usec: 0,
        }
    }
}

/// Get the currently logged users.
///
/// On linux it will get them from `/var/run/utmp`. It will use the C's UTMP Struct and the unsafe read C's function.
///
/// On macOS it will use unsafes call to multiple OSX specific functions [setutxent, getutxent] (the struct is UTMPX for the inner usage).
pub fn get_users() -> Result<Vec<String>, Error> {
    let mut users: Vec<String> = Vec::new();
    #[allow(unused_assignments)]
    let mut buffer: *mut utmpx = unsafe { mem::zeroed() };

    unsafe {
        setutxent();
        buffer = getutxent();
        while !buffer.is_null() {
            let cbuffer = &*(buffer as *mut utmpx) as &utmpx;
            let cuser = &*(&cbuffer.ut_user as *const [i8]);

            if cuser[0] != 0 && cbuffer.ut_type == 7 {
                let csuser = to_str(cuser.as_ptr()).trim_matches('\0').to_owned();
                if !users.contains(&csuser) {
                    users.push(csuser);
                }
            }
            buffer = getutxent();
        }
    }

    Ok(users)
}
